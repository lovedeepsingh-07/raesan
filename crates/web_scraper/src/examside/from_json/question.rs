use crate::examside;

// JSON: {
//   exam: String,
//   subject: String,
//   chapter: String,
//   chapterGroup: String
//   question: {
//     en: {
//       content: String,
//       correct_options: [String],
//       answer: String,
//       options: [ { identifier: String, content: String } ]
//     }
//   }
// }
pub async fn from_json(
    db_pool: &sqlx::Pool<sqlx::Sqlite>,
    json: &serde_json::Value,
    chapter_id: &str,
    question_type: schema::QuestionType,
) -> Result<(), error::Error> {
    let question_id = uuid::Uuid::new_v4().to_string();
    let question_body_data = json
        .get("question")
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the question[question] field".to_string())
        })?
        .get("en")
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question[question][en] field".to_string(),
            )
        })?;
    let question_content = question_body_data
        .get("content")
        .ok_or_else(|| {
            error::Error::DeserializeError(format!(
                "Failed to get the question[question][en][content] field, {:#?}",
                question_body_data
            ))
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question[question][en][content] field as a string".to_string(),
            )
        })?
        .to_string();

    let question_answer = get_answer(&question_type, question_body_data).await?;
    sqlx::query(schema::Question::INSERT_QUERY)
        .bind(&question_id)
        .bind(&chapter_id)
        .bind(&question_type)
        .bind(&question_content)
        .bind(&question_answer)
        .execute(db_pool)
        .await?;

    examside::question_option_from_json(db_pool, question_body_data, &question_id, &question_type)
        .await?;
    Ok(())
}

pub async fn get_answer(
    question_type: &schema::QuestionType,
    json: &serde_json::Value,
) -> Result<String, error::Error> {
    match question_type {
        schema::QuestionType::MCQ => {
            let answer = json
                .get("correct_options")
                .ok_or_else(|| {
                    error::Error::MissingAnswerError(
                        "Failed to get the question[question][en][correct_options] field"
                            .to_string(),
                    )
                })?
                .get(0)
                .ok_or_else(|| {
                    error::Error::MissingAnswerError(
                        "Failed to get the question[question][en][correct_options][0] element"
                            .to_string(),
                    )
                })?
                .as_str()
                .ok_or_else(|| {
                    error::Error::MissingAnswerError(
                        "Failed to get the question[question][en][correct_options][0] as a string"
                            .to_string(),
                    )
                })?
                .to_string();
            return Ok(answer);
        }
        schema::QuestionType::INTEGER => {
            let answer = json
                .get("answer")
                .ok_or_else(|| {
                    error::Error::DeserializeError(
                        "Failed to get the question[question][en][answer] field".to_string(),
                    )
                })?
                .as_str()
                .ok_or_else(|| {
                    error::Error::DeserializeError(
                        "Failed to get the question[question][en][answer] field as a string"
                            .to_string(),
                    )
                })?
                .to_string();
            return Ok(answer);
        }
    }
}
