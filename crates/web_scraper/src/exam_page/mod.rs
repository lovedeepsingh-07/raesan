pub mod chapters;
pub mod exam_metadata;
pub mod subjects;

use crate::{error, schema};

pub async fn extract(page_metadata: serde_json::Value) -> Result<schema::Exam, error::Error> {
    let mut output = schema::Exam::default();

    exam_metadata::extract(&page_metadata, &mut output).await?;
    subjects::extract(&page_metadata, &mut output).await?;

    Ok(output)
}
