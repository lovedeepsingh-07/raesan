use crate::{error, schema};

pub fn handle<'a>(output: &'a mut schema::Question, question_body_data: &serde_json::Value) -> Result<(), error::Error> {
    output.answer = question_body_data
        .get("answer")
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'answer' field".to_string(),
            )
        })?
    .as_str()
        .ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the question's 'answer' field as a string".to_string(),
            )
        })?
        .to_string();
    Ok(())
}
