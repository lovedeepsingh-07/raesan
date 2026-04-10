// JSON: { key: String, exam: String, title: String }
pub async fn from_json(
    db_pool: &sqlx::Pool<sqlx::Sqlite>,
    json: &serde_json::Value,
    exam_id: &str,
) -> Result<schema::Subject, error::Error> {
    let subject_id = uuid::Uuid::new_v4().to_string();
    let subject_key = json
        .get("key")
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the subject[key] field".to_string())
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the subject[key] field as a string".to_string(),
            )
        })?
        .to_string();
    let subject_title = json
        .get("title")
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the subject[title] field".to_string())
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the subject[title] field as a string".to_string(),
            )
        })?
        .to_string();

    sqlx::query(schema::Subject::INSERT_QUERY)
        .bind(&subject_id)
        .bind(exam_id)
        .bind(&subject_title)
        .execute(db_pool)
        .await?;
    sqlx::query(schema::SourceRecord::INSERT_QUERY)
        .bind(&subject_id)
        .bind(&schema::EntityType::Subject)
        .bind(&schema::ScraperType::ExamSide)
        .bind(&subject_key)
        .execute(db_pool)
        .await?;

    Ok(schema::Subject {
        id: subject_id.to_string(),
        exam_id: exam_id.to_string(),
        title: subject_title,
        ..Default::default()
    })
}
