use crate::{command, web_server};
use axum::response::IntoResponse;

pub(crate) async fn route(
    axum::extract::State(server_state): axum::extract::State<web_server::ServerState>,
) -> impl IntoResponse {
    server_state
        .command_tx
        .send(command::Command::API)
        .await
        .unwrap();
    String::from("API_RESPONSE")
}
