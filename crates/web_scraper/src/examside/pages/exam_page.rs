use crate::examside;

pub async fn extract(
    db_pool: &sqlx::Pool<sqlx::Sqlite>,
    exam_page_metadata: &serde_json::Value,
) -> Result<(), error::Error> {
    let exam_id = uuid::Uuid::new_v4().to_string();
    examside::exam_from_json(db_pool, exam_page_metadata, &exam_id).await?;

    let subjects_array = exam_page_metadata
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

    for subject_json in subjects_array {
        let subject_id = uuid::Uuid::new_v4().to_string();
        examside::subject_from_json(db_pool, subject_json, &exam_id, &subject_id).await?;

        let mut chapter_json_array: Vec<&serde_json::Value> = Vec::new();
        let chapter_groups_array = subject_json
            .get("chapterGroups")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the exam[subjects][.][chapterGroups] field".to_string(),
                )
            })?
            .as_array()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the exam[subjects][.][chapterGroups] field as an array".to_string(),
                )
            })?;
        for curr_group in chapter_groups_array {
            let chapters_array = curr_group
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
            chapter_json_array.extend(chapters_array);
        }
        for chapter_json in chapter_json_array {
            let chapter_id = uuid::Uuid::new_v4().to_string();
            examside::chapter_from_json(db_pool, chapter_json, &subject_id, &chapter_id).await?;
        }
    }

    Ok(())
}
