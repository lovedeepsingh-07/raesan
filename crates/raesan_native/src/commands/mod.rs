use crate::state;
use tokio::sync::RwLock;

#[tauri::command(rename_all = "snake_case")]
pub async fn filter_metadata(
    app_state: tauri::State<'_, RwLock<state::AppState>>,
) -> Result<Vec<schema::Exam>, error::Error> {
    let app_state = app_state.read().await;
    let mut exams: Vec<schema::Exam> = sqlx::query_as::<_, schema::Exam>("SELECT * FROM exam")
        .fetch_all(&app_state.app.db_pool)
        .await
        .unwrap();

    for curr_exam in exams.iter_mut() {
        let mut subjects: Vec<schema::Subject> =
            sqlx::query_as::<_, schema::Subject>("SELECT * FROM subject WHERE exam_id = $1")
                .bind(&curr_exam.id)
                .fetch_all(&app_state.app.db_pool)
                .await
                .unwrap();

        for curr_subject in subjects.iter_mut() {
            let chapters: Vec<schema::Chapter> =
                sqlx::query_as::<_, schema::Chapter>("SELECT * FROM chapter WHERE subject_id = $1")
                    .bind(&curr_subject.id)
                    .fetch_all(&app_state.app.db_pool)
                    .await
                    .unwrap();
            curr_subject.chapters = chapters;
        }

        curr_exam.subjects = subjects;
    }

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    Ok(exams)
}
