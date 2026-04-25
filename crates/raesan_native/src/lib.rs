pub mod commands;
pub mod state;

use tauri::Manager;
use tokio::sync::RwLock;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let app_env = raesan::Environment::from(
        match std::env::var("PUBLIC_APP_ENV") {
            Ok(out) => out,
            Err(_) => {
                log::warn!("Unable to get the app environment, assuming development environment");
                String::from("development")
            }
        }
        .as_str(),
    );
    let db_url: String = match app_env {
        raesan::Environment::DEV => format!("../../{}.db", raesan::constants::DB_NAME),
        raesan::Environment::PROD => format!("./{}.db", raesan::constants::DB_NAME),
    };
    let app_state = state::AppState::new(db_url.as_str(), app_env)
        .await
        .unwrap();
    tauri::Builder::default()
        .setup(|app| {
            app.manage(RwLock::new(app_state));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::filter_metadata,
            commands::create_test,
            commands::chapter_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
