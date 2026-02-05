pub mod integer;
pub mod mcq;

use crate::{error, schema};

fn extract_question<'a>(
    type_key: &'a str,
    question_json: &'a serde_json::Value,
) -> Result<schema::Question, error::Error> {
    let mut output_question = schema::Question::default();

    output_question.exam_key = question_json
        .get("exam")
        .ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the question's 'exam' field".to_string())
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'exam' field as a string".to_string(),
            )
        })?
        .to_string();
    output_question.subject_key = question_json
        .get("subject")
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'subject' field".to_string(),
            )
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'subject' field as a string".to_string(),
            )
        })?
        .to_string();
    output_question.chapter_key = question_json
        .get("chapter")
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'chapter' field".to_string(),
            )
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'chapter' field as a string".to_string(),
            )
        })?
        .to_string();
    output_question.chapter_group = question_json
        .get("chapterGroup")
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'chapterGroup' field".to_string(),
            )
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'chapterGroup' field as a string".to_string(),
            )
        })?
        .to_string();

    let question_body_data = question_json
        .get("question")
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'question' field".to_string(),
            )
        })?
        .get("en")
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'question' field".to_string(),
            )
        })?;
    output_question.content = question_body_data
        .get("content")
        .ok_or_else(|| {
            error::Error::DeserializeError(
                // "Failed to get the question's 'content' field".to_string(),
                format!(
                    "Failed to get the question's 'content' field, {:#?}",
                    question_body_data
                ),
            )
        })?
        .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'content' field as a string".to_string(),
            )
        })?
        .to_string();

    match type_key {
        "mcq" => {
            mcq::handle(&mut output_question, question_body_data)?;
        }
        "integer" => {
            integer::handle(&mut output_question, question_body_data)?;
        }
        _ => {
            return Err(error::Error::DeserializeError(format!(
                "Invalid question type key: {:#?}",
                type_key
            )));
        }
    }
    Ok(output_question)
}

pub fn extract(
    chapter_page_metadata: &serde_json::Value,
) -> Result<Vec<schema::Question>, error::Error> {
    let mut output: Vec<schema::Question> = Vec::new();

    let question_types_array = chapter_page_metadata
        .get("questions")
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the chapter's 'question' field".to_string(),
            )
        })?
        .as_array()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the chapter's 'question' field as an array".to_string(),
            )
        })?;

    for question_data in question_types_array {
        let type_key = question_data
            .get("key")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question type's 'key' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question type's 'key' field as a string".to_string(),
                )
            })?;
        if type_key == "mcqm" {
            continue;
        }
        let question_array = question_data
            .get("questions")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question type's 'question' field".to_string(),
                )
            })?
            .as_array()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question type's 'question' field as an array".to_string(),
                )
            })?;
        for question_json in question_array {
            let output_question = match extract_question(type_key, question_json) {
                Ok(out) => out,
                Err(e) => {
                    log::warn!("Failed to deserialize question, {}", e.to_string());
                    continue;
                }
            };
            output.push(output_question);
        }
    }

    Ok(output)
}
