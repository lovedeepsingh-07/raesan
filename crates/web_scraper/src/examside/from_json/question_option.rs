use crate::examside;

pub async fn from_json(
    db_pool: &sqlx::Pool<sqlx::Sqlite>,
    json: &serde_json::Value,
    question_id: &str,
    question_type: &schema::QuestionType,
) -> Result<Vec<schema::QuestionOption>, examside::QuestionResult> {
    let mut output: Vec<schema::QuestionOption> = Vec::new();

    if *question_type != schema::QuestionType::MCQ {
        return Ok(output);
    }

    let options_array = json
        .get("options")
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question[question][en][option] field".to_string(),
            )
        })?
        .as_array()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question[question][en][options] field as an array".to_string(),
            )
        })?;

    for option in options_array {
        let question_option_id = uuid::Uuid::new_v4().to_string();
        let option_key = option
            .get("identifier")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[question][en][options][.][identifier] field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[question][en][options][.][identifier] field as a string".to_string(),
                )
            })?
            .to_string();
        let option_value = option
            .get("content")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[question][en][options][.][content] field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[question][en][options][.][content] field as a string".to_string(),
                )
            })?
            .to_string();

        // NOTE: I cannot host pictures right now, so it would be the best idea to only store
        // non-picture based questions for now, hence we filter all the picture based questions
        if option_value.contains("https")
            || option_value.contains("jpg")
            || option_value.contains("jpeg")
        {
            return Err(examside::QuestionResult::Filtered);
        }

        let cleaned_option_value = ammonia::clean(&option_value);

        sqlx::query(schema::QuestionOption::INSERT_QUERY)
            .bind(&question_option_id)
            .bind(question_id)
            .bind(&option_key)
            .bind(&cleaned_option_value)
            .execute(db_pool)
            .await
            .map_err(|e| examside::QuestionResult::Error(error::Error::from(e)))?;

        output.push(schema::QuestionOption {
            id: question_option_id,
            question_id: question_id.to_string(),
            key: option_key,
            value: cleaned_option_value,
        });
    }

    Ok(output)
}
