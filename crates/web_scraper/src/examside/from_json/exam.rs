// JSON: { key: String, tite: String }
pub async fn from_json(
    db_pool: &sqlx::Pool<sqlx::Sqlite>,
    json: &serde_json::Value,
) -> Result<schema::Exam, error::Error> {
    let exam_id = uuid::Uuid::new_v4().to_string();
    let exam_key = json
        .get("key")
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the exam[key] field".to_string())
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the exam[key] field as a string".to_string(),
            )
        })?
        .to_string();
    let exam_title = json
        .get("title")
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the exam[title] field".to_string())
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the exam[title] field as a string".to_string(),
            )
        })?
        .to_string();

    sqlx::query(schema::Exam::INSERT_QUERY)
        .bind(&exam_id)
        .bind(&exam_title)
        .execute(db_pool)
        .await?;
    sqlx::query(schema::SourceRecord::INSERT_QUERY)
        .bind(&exam_id)
        .bind(&schema::EntityType::Exam)
        .bind(&schema::ScraperType::ExamSide)
        .bind(&exam_key)
        .execute(db_pool)
        .await?;

    Ok(schema::Exam {
        id: exam_id.to_string(),
        title: exam_title,
        ..Default::default()
    })
}
