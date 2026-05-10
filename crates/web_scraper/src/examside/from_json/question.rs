use crate::examside;

#[derive(Debug)]
pub enum QuestionResult {
    MissingAnswer,
    Filtered,
    Error(error::Error),
}
impl From<error::Error> for QuestionResult {
    fn from(value: error::Error) -> Self {
        QuestionResult::Error(value)
    }
}

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
) -> Result<schema::Question, examside::QuestionResult> {
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

    // I cannot host pictures right now, so it would be the best idea to only store
    // non-picture based questions for now, hence we filter all the picture based questions
    if question_content.contains("https")
        || question_content.contains("jpg")
        || question_content.contains("jpeg")
    {
        return Err(examside::QuestionResult::Filtered);
    }

    let cleaned_question_content = examside::content_cleaner::clean(&question_content).await?;
    let question_answer = get_answer(&question_type, question_body_data).await?;
    sqlx::query(schema::Question::INSERT_QUERY)
        .bind(&question_id)
        .bind(chapter_id)
        .bind(&question_type)
        .bind(&cleaned_question_content)
        .bind(&question_answer)
        .execute(db_pool)
        .await
        .map_err(|e| examside::QuestionResult::Error(error::Error::from(e)))?;

    let question_options = examside::question_option_from_json(
        db_pool,
        question_body_data,
        &question_id,
        &question_type,
    )
    .await?;
    Ok(schema::Question {
        id: question_id,
        chapter_id: chapter_id.to_string(),
        question_type,
        content: cleaned_question_content,
        options: question_options,
        answer: question_answer,
    })
}

pub async fn get_answer(
    question_type: &schema::QuestionType,
    json: &serde_json::Value,
) -> Result<String, examside::QuestionResult> {
    match question_type {
        schema::QuestionType::MCQ => {
            let answer = json
                .get("correct_options")
                .ok_or(examside::QuestionResult::MissingAnswer)?
                .get(0)
                .ok_or(examside::QuestionResult::MissingAnswer)?
                .as_str()
                .ok_or(examside::QuestionResult::MissingAnswer)?
                .to_string();
            return Ok(answer);
        }
        schema::QuestionType::INTEGER => {
            let answer = json
                .get("answer")
                .ok_or(examside::QuestionResult::MissingAnswer)?
                .as_str()
                .ok_or(examside::QuestionResult::MissingAnswer)?
                .to_string();
            return Ok(answer);
        }
    }
}
