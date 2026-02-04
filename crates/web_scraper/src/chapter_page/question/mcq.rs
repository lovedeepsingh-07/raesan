use crate::{error, schema};
use std::collections::HashMap;

pub fn handle<'a>(output: &'a mut schema::Question, question_body_data: &serde_json::Value) -> Result<(), error::Error> {
    output.answer = question_body_data
        .get("correct_options")
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'correct_options' field".to_string(),
            )
        })?
        .get(0)
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'correct_options' field's first element".to_string(),
            )
        })?.as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'correct_options' field's first element as a string".to_string(),
            )
        })?
        .to_string();
    let options_array = question_body_data
        .get("options")
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'option' field".to_string(),
            )
        })?
        .as_array()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'option' field as an array".to_string(),
            )
        })?;
    output.options = HashMap::new();
    for option in options_array {
        let option_key = option.get("identifier")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the option's 'identifier' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the option's 'identifier' field as a string".to_string(),
                )
            })?.to_string();
        let option_value = option.get("content")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the option's 'content' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the option's 'content' field as a string".to_string(),
                )
            })?.to_string();
        output.options.insert(option_key, option_value);
    }
    Ok(())
}
