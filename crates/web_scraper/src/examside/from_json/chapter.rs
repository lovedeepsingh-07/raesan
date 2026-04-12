// JSON: {
//   key: String,
//   exam: String,
//   subject: String,
//   title: String,
//   chapterGroup: String
// }
pub async fn from_json(
    db_pool: &sqlx::Pool<sqlx::Sqlite>,
    json: &serde_json::Value,
    subject_id: &str,
) -> Result<schema::Chapter, error::Error> {
    let chapter_id = uuid::Uuid::new_v4().to_string();
    let chapter_key = json
        .get("key")
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the chapter[key] field".to_string())
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the chapters[key] field as a string".to_string(),
            )
        })?
        .to_string();
    let chapter_title = json
        .get("title")
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the chapter[title] field".to_string())
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the chapters[title] field as a string".to_string(),
            )
        })?
        .to_string();

    sqlx::query(schema::Chapter::INSERT_QUERY)
        .bind(&chapter_id)
        .bind(subject_id)
        .bind(&chapter_title)
        .execute(db_pool)
        .await?;
    sqlx::query(schema::SourceRecord::INSERT_QUERY)
        .bind(&chapter_id)
        .bind(&schema::EntityType::Chapter)
        .bind(&schema::ScraperType::ExamSide)
        .bind(&chapter_key)
        .execute(db_pool)
        .await?;

    Ok(schema::Chapter {
        id: chapter_id,
        subject_id: subject_id.to_string(),
        title: chapter_title.to_string(),
        ..Default::default()
    })
}
