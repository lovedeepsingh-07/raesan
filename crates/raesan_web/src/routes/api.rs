use crate::state;
use axum::response::IntoResponse;
use std::sync::Arc;

// const CHAPTER_QUERY: &str = r#"SELECT
//     chapter.id,
//     chapter.key,
//     exam.key as exam_key,
//     chapter.subject_id,
//     subject.key as subject_key,
//     chapter.title,
//     chapter."group" FROM chapter
// INNER JOIN subject on chapter.subject_id = subject.id
// INNER JOIN exam on subject.exam_id = exam.id"#;

// GET (/api/metadata)
pub async fn metadata(
    axum::extract::State(server_state): axum::extract::State<Arc<state::ServerState>>,
) -> impl IntoResponse {
    // let mut exam_rows: Vec<tree_schema::T_Exam> = sqlx::query_as::<_, tree_schema::T_Exam>(EXAM_QUERY).fetch_all(&mut *tx).await?;
    // for curr_exam in exam_rows.iter_mut() {
    //     let mut subject_rows: Vec<tree_schema::T_Subject> = sqlx::query_as::<_, tree_schema::T_Subject>(SUBJECT_QUERY)
    //         .bind(&curr_exam.id)
    //         .fetch_all(&mut *tx)
    //         .await
    //         .unwrap();
    //     for curr_subject in subject_rows.iter_mut() {
    //         let mut chapter_rows: Vec<tree_schema::T_Chapter> = sqlx::query_as::<_, tree_schema::T_Chapter>(CHAPTER_QUERY)
    //             .bind(&curr_subject.id)
    //             .fetch_all(&mut *tx)
    //             .await
    //             .unwrap();
    //         for curr_chapter in chapter_rows.iter_mut() {
    //             let mut question_rows: Vec<schema::Question> = sqlx::query_as::<_, schema::Question>(QUESTION_QUERY)
    //                 .bind(&curr_chapter.id)
    //                 .fetch_all(&mut *tx)
    //                 .await
    //                 .unwrap();
    //             for curr_question in question_rows.iter_mut() {
    //                 let question_option_rows: Vec<schema::QuestionOption> = sqlx::query_as::<_, schema::QuestionOption>(QUESTION_OPTION_QUERY)
    //                     .bind(&curr_question.id)
    //                     .fetch_all(&mut *tx)
    //                     .await
    //                     .unwrap();
    //                 curr_question.options = question_option_rows.into_iter().map(|item| (item.key.clone(), item)).collect();
    //             }
    //             curr_chapter.questions.extend(question_rows);
    //         }
    //         curr_subject.chapters.extend(chapter_rows);
    //     }
    //     curr_exam.subjects.extend(subject_rows);
    // }
    // serde_json::to_string(&chapter_rows).unwrap()
    let _ = server_state;
    String::from("METADATA")
}
