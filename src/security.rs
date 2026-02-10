use axum::{
    body::Body,
    extract::{Request, State},
    http::{Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::config::*;
use crate::rate_limiter::RateLimitStatus;
use crate::state::AppState;
use crate::utils;

const HTML_RATE_LIMITED: &str = r#"<!DOCTYPE html>
<html><head><title>429 Too Many Requests</title></head>
<body><h1>Too Many Requests</h1><p>Silakan coba lagi nanti.</p></body></html>"#;

pub fn get_client_ip(req: &Request) -> String {
    if let Some(forwarded) = req.headers().get("x-forwarded-for") {
        if let Ok(val) = forwarded.to_str() {
            if let Some(first) = val.split(',').next() {
                return first.trim().to_string();
            }
        }
    }

    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(val) = real_ip.to_str() {
            return val.trim().to_string();
        }
    }

    req.extensions()
        .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
        .map(|ci| ci.0.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn is_suspicious_path(path: &str) -> bool {
    let path_lower = path.to_lowercase();
    SUSPICIOUS_PATHS
        .iter()
        .any(|&sus| path_lower.contains(sus))
}

fn rate_limited_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::TOO_MANY_REQUESTS)
        .header("Content-Type", "text/html; charset=utf-8")
        .header("Retry-After", "60")
        .body(Body::from(HTML_RATE_LIMITED))
        .unwrap()
}

fn forbidden_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::FORBIDDEN)
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"error":"forbidden"}"#))
        .unwrap()
}

pub async fn security_middleware(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> impl IntoResponse {
    let client_ip = get_client_ip(&req);
    let path = req.uri().path().to_string();

    // Check blocked
    if state.is_ip_blocked(&client_ip) {
        return rate_limited_response();
    }

    // Rate limit (skip whitelist)
    let is_whitelisted = RATE_LIMIT_WHITELIST.iter().any(|&w| path == w);
    if !is_whitelisted {
        let (allowed, _count, status) = state.rate_limiter.check(&client_ip);

        match status {
            RateLimitStatus::Blocked => {
                state.block_ip(&client_ip, 600);
                return rate_limited_response();
            }
            RateLimitStatus::Limited => {
                return rate_limited_response();
            }
            RateLimitStatus::Ok => {}
        }
    }

    // Suspicious path check
    if is_suspicious_path(&path) {
        state.record_failed_attempt(&client_ip, 3);
        return forbidden_response();
    }

    // Suspicious aturts patterns
    let path_lower = path.to_lowercase();
    if path_lower.starts_with("/aturts")
        && path_lower != "/aturts"
        && !path_lower.starts_with("/aturts/")
    {
        state.record_failed_attempt(&client_ip, 2);
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"error":"invalid"}"#))
            .unwrap();
    }

    next.run(req).await.into_response()
}
