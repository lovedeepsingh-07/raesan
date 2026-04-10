use crate::state;
use axum::response::IntoResponse;
use std::sync::Arc;

// GET (/api/metadata)
pub async fn metadata(
    axum::extract::State(server_state): axum::extract::State<Arc<state::ServerState>>,
) -> impl IntoResponse {
    let _ = server_state;
    String::from("METADATA")
}
