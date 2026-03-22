pub mod commands;

use commands::populate;
use tauri::Manager;
use tokio::sync::RwLock;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(RwLock::new(raesan::AppState::new()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            populate::populate_database,
            populate::cancel_populate
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
