pub mod constants;
pub mod middleware;
pub mod routes;
pub mod state;

use axum::{http, routing};
use std::sync::Arc;
use tower_http::cors;

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_module("raesan", log::LevelFilter::Debug)
        .filter_module("raesan_web", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    let frontend_url =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "https://raesan.pages.dev".to_string());

    let server_state = Arc::new(state::ServerState::new("./test.db").await.unwrap());
    let cors = match server_state.app.env {
        raesan::Environment::DEV => cors::CorsLayer::permissive(),
        raesan::Environment::PROD => cors::CorsLayer::new()
            .allow_origin(frontend_url.parse::<http::HeaderValue>().unwrap())
            .allow_methods([http::Method::GET, http::Method::POST])
            .allow_headers(cors::Any),
    };

    let router = axum::Router::new()
        .route("/health", routing::get(routes::health))
        .route("/api/metadata", routing::get(routes::api::metadata))
        .route_layer(axum::middleware::from_fn_with_state(
            server_state.clone(),
            middleware::latency_simulator,
        ))
        .with_state(server_state)
        .layer(cors);

    let listener =
        tokio::net::TcpListener::bind((constants::SERVER_ADDRESS, constants::SERVER_PORT))
            .await
            .unwrap();
    log::info!(
        "raesan web server started on {}:{}",
        constants::SERVER_ADDRESS,
        constants::SERVER_PORT
    );
    axum::serve(listener, router).await.unwrap();
}
