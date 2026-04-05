pub mod populate;

use crate::state;
use tokio::sync::RwLock;

const CHAPTER_QUERY: &str = r#"SELECT
    chapter.id,
    chapter.key,
    exam.key as exam_key,
    chapter.subject_id,
    subject.key as subject_key,
    chapter.title,
    chapter."group" FROM chapter
INNER JOIN subject on chapter.subject_id = subject.id
INNER JOIN exam on subject.exam_id = exam.id"#;

#[tauri::command(rename_all = "snake_case")]
pub async fn metadata(
    app_state: tauri::State<'_, RwLock<state::AppState>>,
) -> Result<String, error::Error> {
    let app_state = app_state.read().await;
    let chapter_rows: Vec<schema::Chapter> = sqlx::query_as::<_, schema::Chapter>(CHAPTER_QUERY)
        .fetch_all(&app_state.db_pool)
        .await?;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    Ok(serde_json::to_string(&chapter_rows)?)
}
