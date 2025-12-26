use std::collections::HashMap;

use anyhow::{Result, anyhow};
use argon2::PasswordHash;
use argon2::PasswordVerifier;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::body::Body;
use axum::extract::Path;
use axum::extract::Query;
use axum::http::Request;
use axum::http::StatusCode;
use axum::http::header;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use base64::Engine;
use tracing::error;
use tracing::info;
use tracing::warn;

use crate::db;
use crate::errors::ApiError;

pub async fn login(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let return_to = params
        .get("return")
        .map(String::as_str)
        .unwrap_or("http://localhost:5173");

    // TODO: Only bounce back if allowed URL (localhost or actual URL)
    if !return_to.starts_with("http://localhost:5173") {
        warn!("The redirect is {return_to}");
    }

    (
        StatusCode::FOUND,
        [(header::LOCATION, return_to.to_string())],
    )
}

pub async fn authenticate(req: Request<Body>, next: Next) -> Response {
    let Some(auth) = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
    else {
        error!("Did not find authorization header");
        return unauthorized();
    };

    let Some(basic) = auth.strip_prefix("Basic ") else {
        error!("No Basic authorization in header");
        return unauthorized();
    };

    let Ok(decoded) = base64_decode(basic) else {
        error!("Unable to decode header");
        return unauthorized();
    };

    let Ok(creds) = String::from_utf8(decoded) else {
        error!("Unable to parse header from utf8");
        return unauthorized();
    };

    let Some((user, pass)) = creds.split_once(':') else {
        error!("Unable to parse {creds} as user:pass");
        return unauthorized();
    };

    if let Ok(user_data) = db::users::get_user_data(user).await {
        if verify_password(pass, &user_data.password).is_ok() {
            info!("Logged in as {}", user_data.username);
            return next.run(req).await;
        }
    }
    unauthorized()
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
