use crate::error;

#[derive(Debug, clap::Parser)]
pub struct CliArgs {
    pub data_folder_path: Option<String>,
}

pub fn get_data_folder_path(cli_args: CliArgs) -> Result<std::path::PathBuf, error::Error> {
    let current_dir = std::env::current_dir()?;
    match cli_args.data_folder_path {
        Some(data_folder_path_str) => {
            let data_folder_path = current_dir.join(data_folder_path_str);
            return Ok(data_folder_path);
        }
        None => return Ok(current_dir.join("data")),
    }
}
