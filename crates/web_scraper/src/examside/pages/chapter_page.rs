use crate::examside;
use tokio::sync::mpsc;

// the reason why we provide the chapter_id to this function, is that we already have the chapter
// in the database, we just need the questions for that chapter and to make a parent-child
// relationship between the chapter and it's questions
//
// in this process, if anything goes wrong while fetching the questions, we don't retry, we just
// skip that question, send a log, and just move on
//
// on the chapter page in the website, the actual questions are a little nested, so from the
// fetched data, I firstly have to get the "questions" array, which is a misleading name because it
// is question types array as it contains 3 sub arrays that themselves contain the questions of
// different types such as one array contains MCQs, the other contains Integer questions and the
// other contains MCQMs
pub async fn extract(
    db_pool: &sqlx::Pool<sqlx::Sqlite>,
    log_tx: mpsc::Sender<crate::ScraperLog>,
    chapter_page_metadata: &serde_json::Value,
    chapter_id: &str,
) -> Result<(), error::Error> {
    let mut curr_chapter: schema::Chapter = sqlx::query_as("SELECT * FROM chapter WHERE id = $1")
        .bind(chapter_id)
        .fetch_one(db_pool)
        .await?;
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
                Ok(_) => {
                    curr_chapter.total_questions += 1;
                }
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

    log_tx
        .send(crate::ScraperLog::Info(format!(
            "Extracted Chapter: {:#?}",
            curr_chapter
        )))
        .await?;

    sqlx::query("UPDATE chapter SET total_questions = $1 WHERE id = $2")
        .bind(curr_chapter.total_questions)
        .bind(chapter_id)
        .execute(db_pool)
        .await?;

    Ok(())
}
