use crate::examside::QuestionFromJson;

pub fn extract(
    chapter_id: String,
    chapter_page_metadata: &serde_json::Value,
) -> Result<Vec<schema::Question>, error::Error> {
    let mut output: Vec<schema::Question> = Vec::new();

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
            let question = match schema::Question::from_json(
                chapter_id.clone(),
                schema::QuestionType::from(type_key),
                question_json,
            ) {
                Ok(out) => out,
                Err(e) => {
                    log::warn!("Failed to deserialize question, {}", e);
                    continue;
                }
            };
            output.push(question);
        }
    }

    Ok(output)
}
