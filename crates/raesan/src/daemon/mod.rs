use crate::{command, error};
use tokio::sync::mpsc;

pub async fn run(
    mut command_rx: mpsc::Receiver<command::Command>,
    data_folder_path: std::path::PathBuf,
) -> Result<(), error::Error> {
    // if !data_folder_path.exists() {
    //     std::fs::create_dir_all(&data_folder_path)?;
    // } else if !data_folder_path.is_dir() {
    //     return Err(error::Error::FSError(format!("Provided data folder path is not a directory, {:#?}", data_folder_path)));
    // }
    //
    let _ = data_folder_path;
    while let Some(command) = command_rx.recv().await {
        log::debug!("API request with command: {:#?}", command);
    }
    Ok(())
}
