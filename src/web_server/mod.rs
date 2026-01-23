pub(crate) mod routes;

use crate::{command, constants, error};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub(crate) struct ServerState {
    pub(crate) command_tx: mpsc::Sender<command::Command>,
}

pub(crate) async fn run(command_tx: mpsc::Sender<command::Command>) -> Result<(), error::Error> {
    let server_state = ServerState { command_tx };

    let router = axum::Router::new()
        .route("/", axum::routing::get(routes::home))
        .route("/api", axum::routing::get(routes::api))
        .route(
            "/static/{*file_path}",
            axum::routing::get(routes::static_files),
        )
        .with_state(server_state);

    let listener = tokio::net::TcpListener::bind(constants::SERVER_ADDRESS).await?;
    log::info!("Web Server started on {}", constants::SERVER_ADDRESS);
    axum::serve(listener, router).await?;
    Ok(())
}
