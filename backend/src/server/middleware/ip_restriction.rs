use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::net::{IpAddr, SocketAddr};
use tracing::warn;

pub async fn restrict_ip(req: Request<Body>, next: Next, allowed_ips: Vec<IpAddr>) -> Response {
    // Extract the remote IP from the request
    let connect_info = req
        .extensions()
        .get::<axum::extract::ConnectInfo<SocketAddr>>();

    let remote_ip = match connect_info {
        Some(ConnectInfo(addr)) => addr.ip(),
        None => return forbidden(),
    };

    if allowed_ips.contains(&remote_ip) {
        next.run(req).await
    } else {
        warn!("Got login request from IP {remote_ip:?}");
        forbidden()
    }
}

fn forbidden() -> Response {
    let mut response = Response::new(Body::from("Forbidden"));
    *response.status_mut() = StatusCode::FORBIDDEN;
    response
}
