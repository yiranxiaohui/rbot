mod qq;
mod config;
mod utils;
mod features;

use crate::config::{get_config_clone};
use axum::{routing::get, Router};
use std::net::SocketAddr;
use axum::routing::post;
use tower_http::services::ServeDir;
use crate::qq::webhook;

#[tokio::main]
async fn main() {
    let config = get_config_clone().await;
    env_logger::Builder::from_env(env_logger::Env::default()
        .default_filter_or(&config.log_level))
        .init();
    qq::init().await;
    let address = config.address.parse().unwrap();
    let port = config.port;
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest_service("/resources", ServeDir::new("resources"))
        .nest_service("/check", ServeDir::new("resources/102149350.json"))
        .route("/qq/event", post(webhook));
    let listener = tokio::net::TcpListener::bind(SocketAddr::new(address, port)).await.unwrap();
    log::info!("服务器启动在 https://{}:{}", address, port);
    axum::serve(listener, app).await.unwrap();
}
