pub mod constants;
pub mod middleware;
pub mod routes;
pub mod state;

use axum::{http, routing};
use std::sync::Arc;
use tower_http::cors;

async fn run() -> Result<(), error::Error> {
    let frontend_url: String =
        std::env::var("FRONTEND_URL").unwrap_or_else(|_| "https://raesan.pages.dev".to_string());

    let server_state = Arc::new(state::ServerState::new("./test.db").await?);
    let cors = match server_state.app.env {
        raesan::Environment::DEV => cors::CorsLayer::permissive(),
        raesan::Environment::PROD => cors::CorsLayer::new()
            .allow_origin(frontend_url.parse::<http::HeaderValue>()?)
            .allow_methods([http::Method::GET, http::Method::POST])
            .allow_headers(cors::Any),
    };

    let router = axum::Router::new()
        .route("/health", routing::get(routes::health))
        .route(
            "/api/filter_metadata",
            routing::get(routes::api::filter_metadata),
        )
        .route("/api/create_test", routing::post(routes::api::create_test))
        .route_layer(axum::middleware::from_fn_with_state(
            server_state.clone(),
            middleware::latency_simulator,
        ))
        .with_state(server_state)
        .layer(cors);

    let listener =
        tokio::net::TcpListener::bind((constants::SERVER_ADDRESS, constants::SERVER_PORT)).await?;
    log::info!(
        "raesan web server started on {}:{}",
        constants::SERVER_ADDRESS,
        constants::SERVER_PORT
    );
    axum::serve(listener, router).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_module("raesan", log::LevelFilter::Debug)
        .filter_module("raesan_web", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    match run().await {
        Ok(_) => {}
        Err(e) => {
            log::error!("{}", e);
        }
    }
}
