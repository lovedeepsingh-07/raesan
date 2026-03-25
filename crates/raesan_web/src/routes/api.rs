use crate::state;
use axum::response::IntoResponse;
use std::sync::Arc;

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

// GET (/api/metadata)
pub async fn metadata(
    axum::extract::State(server_state): axum::extract::State<Arc<state::ServerState>>,
) -> impl IntoResponse {
    let chapter_rows: Vec<schema::Chapter> = sqlx::query_as::<_, schema::Chapter>(CHAPTER_QUERY)
        .fetch_all(&server_state.db_pool)
        .await
        .unwrap();
    serde_json::to_string(&chapter_rows).unwrap()
}
