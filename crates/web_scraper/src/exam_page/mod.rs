pub mod exam_metadata;
pub mod subjects;
pub mod chapters;

use crate::{error, schema};

pub async fn extract(page_metadata: serde_json::Value) -> Result<schema::Exam, error::Error> {
    let root_array = page_metadata.as_array().ok_or_else(|| {
        error::Error::DeserializeError("Failed to get the root node as an array".to_string())
    })?;
    let root_element = root_array.get(1).ok_or_else(|| {
        error::Error::DeserializeError("Failed to get the second root element".to_string())
    })?;
    let data = root_element.get("data").ok_or_else(|| {
        error::Error::DeserializeError("Failed to get the 'data' field".to_string())
    })?;

    let mut output = schema::Exam::default();

    exam_metadata::extract(&data, &mut output).await?;
    subjects::extract(&data, &mut output).await?;

    println!("{:#?}", output);
    Ok(output)
}
