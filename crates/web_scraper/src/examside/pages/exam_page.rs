use crate::examside;
use tokio::sync::mpsc;

pub async fn extract(
    db_pool: &sqlx::Pool<sqlx::Sqlite>,
    log_tx: mpsc::Sender<crate::ScraperLog>,
    exam_page_metadata: &serde_json::Value,
) -> Result<schema::Exam, error::Error> {
    let mut curr_exam = examside::exam_from_json(db_pool, exam_page_metadata).await?;

    let subject_json_array = exam_page_metadata
        .get("subjects")
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the exam[subjects] field".to_string())
        })?
        .as_array()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the exam[subjects] field as an array".to_string(),
            )
        })?;
    curr_exam.total_subjects = subject_json_array.len() as i64;
    log_tx
        .send(crate::ScraperLog::Info(format!(
            "Extracted Exam: {:#?}",
            curr_exam
        )))
        .await?;

    for curr_subject_json in subject_json_array.iter() {
        let mut curr_subject =
            examside::subject_from_json(db_pool, curr_subject_json, &curr_exam.id).await?;

        let mut chapter_json_array: Vec<&serde_json::Value> = Vec::new();

        let chapter_groups_array = curr_subject_json
            .get("chapterGroups")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the exam[subjects][.][chapterGroups] field".to_string(),
                )
            })?
            .as_array()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the exam[subjects][.][chapterGroups] field as an array"
                        .to_string(),
                )
            })?;
        for curr_group in chapter_groups_array {
            let curr_group_chapter_json_array = curr_group
                .get("chapters")
                .ok_or_else(|| {
                    error::Error::DeserializeError(
                        "Failed to get the exam[subjects][.][chapterGroups][.][chapters] field".to_string(),
                    )
                })?
                .as_array()
                .ok_or_else(|| {
                    error::Error::DeserializeError(
                        "Failed to get the exam[subjects][.][chapterGroups][.][chapters] field as an array".to_string(),
                    )
                })?;
            chapter_json_array.extend(curr_group_chapter_json_array);
        }

        curr_subject.total_chapters = chapter_json_array.len() as i64;
        log_tx
            .send(crate::ScraperLog::Info(format!(
                "Extracted Subject: {:#?}",
                curr_subject
            )))
            .await?;

        for chapter_json in chapter_json_array {
            let _ = examside::chapter_from_json(db_pool, chapter_json, &curr_subject.id).await?;
        }

        sqlx::query("UPDATE subject SET total_chapters = $1 WHERE id = $2")
            .bind(curr_subject.total_chapters)
            .bind(&curr_subject.id)
            .execute(db_pool)
            .await?;
    }
    sqlx::query("UPDATE exam SET total_subjects = $1 WHERE id = $2")
        .bind(curr_exam.total_subjects)
        .bind(&curr_exam.id)
        .execute(db_pool)
        .await?;

    Ok(curr_exam)
}
