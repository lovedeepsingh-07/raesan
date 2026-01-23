use crate::error;
use tokio::sync::mpsc;

#[derive(Debug)]
pub(crate) enum Command {
    API,
}

pub(crate) async fn run_daemon(mut rx: mpsc::Receiver<Command>) -> Result<(), error::Error> {
    while let Some(command) = rx.recv().await {
        log::debug!("API request with command: {:#?}", command);
    }
    Ok(())
}
