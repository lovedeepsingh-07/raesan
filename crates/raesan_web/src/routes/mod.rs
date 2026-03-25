pub mod api;

use axum::response::IntoResponse;

// GET (/health)
pub async fn health() -> impl IntoResponse {
    String::from("HEALTHY")
}
