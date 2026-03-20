pub mod constants;

use axum::{
    http,
    response::{self, IntoResponse},
    routing,
};
use tokio::sync::broadcast;
use tokio_stream::StreamExt;
use tower_http::cors;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PopulateEvent {
    pub name: String,
    pub data: String,
}

#[derive(Debug, Clone)]
pub struct ServerState {
    pub app_env: Environment,
    pub populate_event_tx: broadcast::Sender<PopulateEvent>,
}
impl Default for ServerState {
    fn default() -> Self {
        Self::new()
    }
}
impl ServerState {
    pub fn new() -> Self {
        let (populate_event_tx, _) = broadcast::channel::<PopulateEvent>(constants::EVENT_CAP);
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
            populate_event_tx,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Environment {
    DEV,
    PROD,
}

async fn begin_populate(
    axum::extract::State(server_state): axum::extract::State<ServerState>,
) -> impl IntoResponse {
    let tx = server_state.populate_event_tx.clone();

    tokio::spawn(async move {
        for i in 0..10 {
            let event = PopulateEvent {
                name: format!("event_{}", i),
                data: "this_is_some_data_from_backend".to_string(),
            };
            let _ = tx.send(event);
            tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
        }
    });

    http::StatusCode::OK
}
async fn stream_populate(
    axum::extract::State(server_state): axum::extract::State<ServerState>,
) -> response::Sse<
    impl futures::stream::Stream<Item = Result<response::sse::Event, std::convert::Infallible>>,
> {
    let rx = server_state.populate_event_tx.subscribe();

    let stream = tokio_stream::wrappers::BroadcastStream::new(rx).filter_map(|event| match event {
        Ok(data) => Some(Ok(response::sse::Event::default()
            .event("populate_event")
            .data(serde_json::to_string(&data).unwrap()))),
        Err(_) => None,
    });

    response::Sse::new(stream).keep_alive(response::sse::KeepAlive::default())
}
async fn cancel_populate() -> impl IntoResponse {
    String::from("cancel_populate")
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
        .route("/begin_populate", routing::get(begin_populate))
        .route("/stream_populate", routing::get(stream_populate))
        .route("/cancel_populate", routing::get(cancel_populate))
        .with_state(server_state)
        .layer(cors);

    let listener =
        tokio::net::TcpListener::bind((constants::SERVER_ADDRESS, constants::SERVER_PORT))
            .await
            .unwrap();
    log::info!("raesan web server started on {}", constants::SERVER_ADDRESS);
    axum::serve(listener, router).await.unwrap();
}
