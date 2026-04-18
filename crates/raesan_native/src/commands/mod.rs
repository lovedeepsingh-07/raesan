use crate::state;
use tokio::sync::RwLock;

#[tauri::command(rename_all = "snake_case")]
pub async fn filter_metadata(
    app_state: tauri::State<'_, RwLock<state::AppState>>,
) -> Result<Vec<schema::Exam>, error::Error> {
    let app_state = app_state.read().await;
    Ok(app_state.app.get_filter_metadata().await?)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn create_test(
    app_state: tauri::State<'_, RwLock<state::AppState>>,
) -> Result<String, error::Error> {
    let _ = app_state;
    Ok("CREATING_TEST".into())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn chapter_data(
    app_state: tauri::State<'_, RwLock<state::AppState>>,
    chapter_id: String,
) -> Result<schema::Chapter, error::Error> {
    let app_state = app_state.read().await;
    Ok(app_state.app.get_chapter_data(&chapter_id).await?)
}
