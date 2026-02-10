#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod config;
mod handlers;
mod rate_limiter;
mod security;
mod state;
mod template;
mod treasury;
mod usd_idr;
mod utils;
mod ws_manager;

use axum::{
    Router,
    middleware as axum_middleware,
    extract::Request,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tokio::signal;
use tower_http::compression::CompressionLayer;
use tracing_subscriber::EnvFilter;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("warn")),
        )
        .compact()
        .init();

    let state = Arc::new(AppState::new());

    // Spawn background tasks
    let treasury_state = state.clone();
    tokio::spawn(async move {
        treasury::treasury_ws_loop(treasury_state).await;
    });

    let usd_state = state.clone();
    tokio::spawn(async move {
        usd_idr::usd_idr_loop(usd_state).await;
    });

    let heartbeat_state = state.clone();
    tokio::spawn(async move {
        ws_manager::heartbeat_loop(heartbeat_state).await;
    });

    let app = Router::new()
        .merge(handlers::routes())
        .layer(CompressionLayer::new().gzip(true).br(true).deflate(true))
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            security::security_middleware,
        ))
        .with_state(state.clone());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .unwrap_or(8000);

    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received");
}
