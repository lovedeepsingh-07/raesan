pub mod constants;
pub mod middleware;
pub mod routes;
pub mod state;

use axum::{http, routing};
use std::sync::Arc;
use tower_http::cors;

async fn run() -> Result<(), error::Error> {
    let frontend_url: String =
        std::env::var(raesan::environment::FRONTEND_URL__NAME).map_err(|e| {
            error::Error::NotFoundError(format!(
                "Failed to get the {:#?} environment variable, {}",
                raesan::environment::FRONTEND_URL__NAME,
                e
            ))
        })?;
    let db_url = format!("./{}.db", raesan::constants::DB_NAME);
    let server_state = Arc::new(state::ServerState::new(db_url.as_str()).await?);

    // on development, I allow nothing, on production, I allow the "frontend_url" to pass through
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
        .route(
            "/api/chapter_data/{chapter_id}",
            routing::get(routes::api::chapter_data),
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
