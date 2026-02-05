pub mod mcq;
pub mod integer;

use crate::{error, schema};

pub fn extract<'a>(
    question_data: &'a serde_json::Value,
    chapter_schema: &'a mut schema::Chapter,
) -> Result<(), error::Error> {
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
    for question in question_array {
        let mut output = schema::Question::default();

        output.exam_key = question
            .get("exam")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question's 'exam' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question's 'exam' field as a string".to_string(),
                )
            })?
            .to_string();
        output.subject_key = question
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
        output.chapter_key = question
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
        output.chapter_group = question
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

        let question_body_data = question
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
        output.content = question_body_data
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
                mcq::handle(&mut output,question_body_data)?;
            },
            "integer" => {
                integer::handle(&mut output,question_body_data)?;
            },
            "mcqm" => {
                break;
            },
            _ => {
                return Err(error::Error::DeserializeError(
                        format!("Invalid question type key: {:#?}", type_key)
                ));
            }
        }

        chapter_schema.questions.push(output);
    }
    Ok(())
}
