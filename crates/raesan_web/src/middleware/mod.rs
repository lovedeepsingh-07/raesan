use crate::state;
use std::sync::Arc;

#[axum::debug_middleware]
pub async fn latency_simulator(
    axum::extract::State(server_state): axum::extract::State<Arc<state::ServerState>>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    if server_state.app.env == raesan::Environment::DEV {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
    next.run(request).await
}
