pub mod api;

use axum::response::IntoResponse;

// GET (/health)
#[axum::debug_handler]
pub async fn health() -> impl IntoResponse {
    String::from("HEALTHY")
}
