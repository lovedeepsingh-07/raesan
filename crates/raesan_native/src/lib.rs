pub mod commands;
pub mod state;

use tauri::Manager;
use tokio::sync::RwLock;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let app_state = state::AppState::new("../../test.db").await.unwrap();
    tauri::Builder::default()
        .setup(|app| {
            app.manage(RwLock::new(app_state));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::filter_metadata])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
