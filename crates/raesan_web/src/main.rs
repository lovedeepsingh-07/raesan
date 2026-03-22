pub mod constants;

use tower_http::cors;
use axum::{http, response::IntoResponse, routing};

#[derive(Debug, Clone)]
pub struct ServerState { pub app_env: Environment }
impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}
impl ServerState {
    pub fn new() -> Self {
        let app_env = match std::env::var("APP_ENV") {
            Ok(out) => match out.as_str() {
                "production" => Environment::PROD,
                _ => Environment::DEV,
            },
            Err(e) => {
                log::warn!("Failed to get APP_ENV, assuming DEV, error: {}", e);
                Environment::DEV
            }
        };
        Self {
            app_env,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Environment {
    DEV,
    PROD,
}

// GET (/health)
async fn health(
    axum::extract::State(server_state): axum::extract::State<ServerState>,
) -> impl IntoResponse {
    match server_state.app_env {
        Environment::PROD => {
            String::from("HEALTHY")
        },
        Environment::DEV => {
            // NOTE: simulating latency
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            String::from("HEALTHY")
        },
    }
}

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

    let server_state = ServerState::new();
    let cors = match server_state.app_env {
        Environment::DEV => cors::CorsLayer::permissive(),
        Environment::PROD => cors::CorsLayer::new()
            .allow_origin(frontend_url.parse::<http::HeaderValue>().unwrap())
            .allow_methods([http::Method::GET, http::Method::POST])
            .allow_headers(cors::Any),
    };

    let router = axum::Router::new()
        .route("/health", routing::get(health))
        .with_state(server_state)
        .layer(cors);

    let listener =
        tokio::net::TcpListener::bind((constants::SERVER_ADDRESS, constants::SERVER_PORT))
            .await
            .unwrap();
    log::info!("raesan web server started on {}", constants::SERVER_ADDRESS);
    axum::serve(listener, router).await.unwrap();
}
