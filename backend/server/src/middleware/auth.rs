use crate::middleware::rate_limiting::AuthLimit;
use anyhow::{Result, anyhow};
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{
    body::Body,
    extract::{Path, Query, Request, State},
    http::{
        StatusCode,
        header::{self, RETRY_AFTER},
    },
    middleware::Next,
    response::{IntoResponse, Response},
};
use base64::Engine;
use db::users::UserData;
use governor::clock::{Clock, DefaultClock};
use std::collections::HashMap;
use tower_governor::key_extractor::{KeyExtractor, SmartIpKeyExtractor};
use tracing::error;
use types::errors::ApiError;

/// Hash used for password verification if user does not enter a valid username. Prevents timing
/// attacks.
const DUMMY_HASH: &str = "$argon2i$v=19$m=16,t=2,p=1$ZGFkc2Fkd2Fkc2E$7gECsfaOVWjDZJXDzqy92g";

enum AuthResult {
    Valid,
    Invalid,
    NoHeader,
}

pub async fn protected() -> String {
    String::from("You successfully reached a protected route!")
}

pub async fn login(Query(params): Query<HashMap<String, String>>) -> Response {
    let return_to = params
        .get("return")
        .map(String::as_str)
        .unwrap_or("http://localhost:5173");

    (
        StatusCode::FOUND,
        [(header::LOCATION, return_to.to_string())],
    )
        .into_response()
}

pub async fn authenticate_with_limit(
    State(state): State<AuthLimit>,
    req: Request,
    next: Next,
) -> Response {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    match authenticate(auth_header).await {
        AuthResult::Valid => next.run(req).await,
        AuthResult::NoHeader => unauthorized(),
        // Only consume an auth limit token on auth fail
        AuthResult::Invalid => match SmartIpKeyExtractor.extract(&req) {
            Ok(key) => {
                if let Err(negative) = state.arc.limiter().check_key(&key) {
                    let wait_time = negative.wait_time_from(DefaultClock::default().now());
                    return (
                        StatusCode::TOO_MANY_REQUESTS,
                        [(RETRY_AFTER, wait_time.as_secs().to_string())],
                    )
                        .into_response();
                }
                unauthorized()
            }
            Err(_) => unauthorized(),
        },
    }
}

async fn authenticate(auth_header: Option<String>) -> AuthResult {
    let Some(auth) = auth_header else {
        error!("No Authorization header at all");
        return AuthResult::NoHeader;
    };

    let Some(basic) = auth.strip_prefix("Basic ") else {
        error!("No Basic authorization in header");
        return AuthResult::Invalid;
    };

    let Ok(decoded) = base64_decode(basic) else {
        error!("Unable to decode header");
        return AuthResult::Invalid;
    };

    let Ok(creds) = String::from_utf8(decoded) else {
        error!("Unable to parse header from utf8");
        return AuthResult::Invalid;
    };

    let Some((user, pass)) = creds.split_once(':') else {
        error!("Unable to parse credentials as user:pass");
        return AuthResult::Invalid;
    };

    let (user_data, found) = match db::users::get_user_data(user).await {
        Ok(data) => (data, true),
        Err(_) => (
            UserData {
                username: "not_found".to_string(),
                password: DUMMY_HASH.to_string(),
            },
            false,
        ),
    };

    if found && verify_password(pass, &user_data.password).is_ok() {
        return AuthResult::Valid;
    }
    AuthResult::Invalid
}

fn unauthorized() -> Response {
    let mut response = Response::new(Body::from("Unauthorized"));
    *response.status_mut() = StatusCode::UNAUTHORIZED;
    response.headers_mut().insert(
        "WWW-Authenticate",
        "Basic realm=\"Protected Area\"".parse().unwrap(),
    );

    response
}

pub async fn create_user(
    Path((user, pass)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    let hashed_pass = hash_password(&pass).map_err(|e| ApiError::BadRequest(e.to_string()))?;
    let status = db::users::create_user(&user, &hashed_pass).await;
    match status {
        Ok(user) => Ok((StatusCode::OK, format!("Created user {user}"))),
        Err(e) => Err(ApiError::BadRequest(e.to_string())),
    }
}

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hashed.to_string())
}

fn verify_password(password: &str, hash: &str) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}

fn base64_decode(input: &str) -> Result<Vec<u8>> {
    base64::engine::general_purpose::STANDARD
        .decode(input)
        .map_err(|_| anyhow!("Could not decode"))
}
