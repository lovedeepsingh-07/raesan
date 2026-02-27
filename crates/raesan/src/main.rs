mod cli;
mod command;
mod constants;
mod daemon;
mod web_server;

use clap::Parser;
use tokio::{
    sync::{mpsc, watch},
    task::JoinSet,
};

#[derive(Debug)]
enum TaskResult {
    Completed,
    Shutdown,
    Failed(error::Error),
}

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_module("raesan", log::LevelFilter::Debug)
        .filter_module("web_scraper", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .init();

    let cli_args = cli::CliArgs::parse();
    let data_folder_path = match cli::get_data_folder_path(cli_args) {
        Ok(out) => out,
        Err(e) => {
            log::error!("Failed to get data folder path, {}", e);
            return;
        }
    };

    run_tasks(data_folder_path).await;
}

async fn run_tasks(data_folder_path: std::path::PathBuf) {
    let mut tasks: JoinSet<TaskResult> = JoinSet::new();
    let (command_tx, command_rx) = mpsc::channel::<command::Command>(constants::COMMAND_CAP);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let mut daemon_shutdown_rx = shutdown_rx.clone();
    tasks.spawn(async move {
        tokio::select! {
            res = daemon::run(command_rx, data_folder_path) => {
                match res {
                    Ok(_) => TaskResult::Completed,
                    Err(e) => TaskResult::Failed(e),
                }
            },
            _ = daemon_shutdown_rx.wait_for(|val| *val) => TaskResult::Shutdown,
        }
    });
    let mut web_server_shutdown_rx = shutdown_rx.clone();
    tasks.spawn(async move {
        tokio::select! {
            res = web_server::run(command_tx) => {
                match res {
                    Ok(_) => TaskResult::Completed,
                    Err(e) => TaskResult::Failed(e),
                }
            },
            _ = web_server_shutdown_rx.wait_for(|val| *val) => TaskResult::Shutdown,
        }
    });

    tokio::select! {
        join_res = tasks.join_next() => {
            if let Some(join_res) = join_res {
                let _ = shutdown_tx.send(true);
                match join_res {
                    Ok(TaskResult::Completed) => {
                        log::error!("Task returned unexpectedly");
                    },
                    Ok(TaskResult::Shutdown) => {
                        log::error!("Task shut down");
                    },
                    Ok(TaskResult::Failed(e)) => {
                        log::error!("Task failed with error: {}", e);
                    },
                    Err(e) => {
                        log::error!("Failed to join task, {}", e);
                    },
                }
            }
        },
        _ = tokio::signal::ctrl_c() => {
            let _ = shutdown_tx.send(true);
            log::info!("Shutting down...");
        }
    };
}
