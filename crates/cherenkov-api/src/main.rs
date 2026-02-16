use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

mod graphql;
mod rest;
mod auth;
mod websocket;

use websocket::{create_websocket_state, create_websocket_router};


use graphql::schema::build_schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    info!("Starting Cherenkov API Server");
    
    let schema = build_schema().await?;
    let ws_state = create_websocket_state();
    
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/graphql", get(graphql::handler).post(graphql::handler))
        .nest("/ws", create_websocket_router(ws_state.clone()))
        .layer(CorsLayer::permissive())
        .with_state(ws_state);

    
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Listening on {}", addr);
    
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    
    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
