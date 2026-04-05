use crate::examside;
use tokio::sync::mpsc;

pub async fn extract(
    db_pool: &sqlx::Pool<sqlx::Sqlite>,
    log_tx: mpsc::Sender<crate::ScraperLog>,
    chapter_page_metadata: &serde_json::Value,
    chapter_id: &str,
) -> Result<(), error::Error> {
    let question_types_array = chapter_page_metadata
        .get("questions")
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the chapter[questions] (question_type) field".to_string(),
            )
        })?
        .as_array()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the chapter[questions] (question_type) field as an array"
                    .to_string(),
            )
        })?;

    for question_data in question_types_array {
        let type_key = question_data
            .get("key")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[key] (question_type) field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[key] (question_type) field as a string".to_string(),
                )
            })?;
        if type_key == "mcqm" {
            continue;
        }
        let question_array = question_data
            .get("questions")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[questions] (question_type) field".to_string(),
                )
            })?
            .as_array()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[questions] (question_type) field as an array"
                        .to_string(),
                )
            })?;

        for question_json in question_array {
            match examside::question_from_json(
                db_pool,
                question_json,
                chapter_id,
                schema::QuestionType::from(type_key),
            )
            .await
            {
                Ok(_) => {}
                Err(e) => {
                    match e {
                        examside::QuestionResult::Error(e) => {
                            return Err(e);
                        }
                        examside::QuestionResult::MissingAnswer => {
                            log_tx.send(crate::ScraperLog::Warn("Failed to deserialize question because of a missing answer, skipping entirely".to_string())).await?;
                        }
                        examside::QuestionResult::Filtered => {}
                    };
                    continue;
                }
            };
        }
    }
    Ok(())
}
