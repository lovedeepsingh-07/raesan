use crate::state;
use axum::response::IntoResponse;
use std::sync::Arc;

// GET (/api/filter_metadata)
pub async fn filter_metadata(
    axum::extract::State(server_state): axum::extract::State<Arc<state::ServerState>>,
) -> impl IntoResponse {
    let mut exams: Vec<schema::Exam> = sqlx::query_as::<_, schema::Exam>("SELECT * FROM exam")
        .fetch_all(&server_state.app.db_pool)
        .await
        .unwrap();

    for curr_exam in exams.iter_mut() {
        let mut subjects: Vec<schema::Subject> =
            sqlx::query_as::<_, schema::Subject>("SELECT * FROM subject WHERE exam_id = $1")
                .bind(&curr_exam.id)
                .fetch_all(&server_state.app.db_pool)
                .await
                .unwrap();

        for curr_subject in subjects.iter_mut() {
            let chapters: Vec<schema::Chapter> =
                sqlx::query_as::<_, schema::Chapter>("SELECT * FROM chapter WHERE subject_id = $1")
                    .bind(&curr_subject.id)
                    .fetch_all(&server_state.app.db_pool)
                    .await
                    .unwrap();
            curr_subject.chapters = chapters;
        }

        curr_exam.subjects = subjects;
    }

    axum::Json(exams)
}

// POSt (/api/create_test)
pub async fn create_test(
    axum::extract::State(server_state): axum::extract::State<Arc<state::ServerState>>,
) -> impl IntoResponse {
    let _ = server_state;
    "CREATING_TEST".into_response()
}
