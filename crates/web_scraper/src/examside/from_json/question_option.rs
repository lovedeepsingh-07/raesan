pub async fn from_json(
    db_pool: &sqlx::Pool<sqlx::Sqlite>,
    json: &serde_json::Value,
    question_id: &str,
    question_type: &schema::QuestionType,
) -> Result<Vec<schema::QuestionOption>, error::Error> {
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

        sqlx::query(schema::QuestionOption::INSERT_QUERY)
            .bind(&question_option_id)
            .bind(question_id)
            .bind(&option_key)
            .bind(&option_value)
            .execute(db_pool)
            .await?;

        output.push(schema::QuestionOption {
            id: question_option_id,
            question_id: question_id.to_string(),
            key: option_key,
            value: option_value,
        });
    }

    Ok(output)
}
