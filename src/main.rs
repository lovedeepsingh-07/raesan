mod command;
mod constants;
mod error;
mod ui;
mod web_server;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let logger_env = env_logger::Env::default().filter_or("RUST_LOG", "raesan=debug");
    env_logger::init_from_env(logger_env);

    let (command_tx, command_rx) = mpsc::channel::<command::Command>(constants::COMMAND_CAP);

    tokio::spawn(async move {
        match command::run_daemon(command_rx).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("Failed to run command daemon, {}", e.to_string());
                return;
            }
        }
    });
    match web_server::run(command_tx).await {
        Ok(_) => {}
        Err(e) => {
            log::error!("Failed to run web server, {}", e.to_string());
            return;
        }
    };
}
