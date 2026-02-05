use crate::{error, schema};

pub fn extract<'a>(
    exam_page_metadata: &'a serde_json::Value,
) -> Result<(Vec<schema::Subject>, Vec<&'a serde_json::Value>), error::Error> {
    let mut output: Vec<schema::Subject> = Vec::new();

    let subjects_field = exam_page_metadata.get("subjects").ok_or_else(|| {
        error::Error::DeserializeError("Failed to get the 'subjects' field".to_string())
    })?;
    let subjects_array = subjects_field.as_array().ok_or_else(|| {
        error::Error::DeserializeError("Failed to get the 'subjects' field as an array".to_string())
    })?;

    let mut chapter_json_array: Vec<&'a serde_json::Value> = Vec::new();

    for subject_json in subjects_array {
        let mut output_subject = schema::Subject::default();

        output_subject.key = subject_json
            .get("key")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subject's 'key' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subjects' 'key' field as a string".to_string(),
                )
            })?
            .to_string();
        output_subject.exam_key = subject_json
            .get("exam")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subject's 'exam' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subjects' 'exam' field as a string".to_string(),
                )
            })?
            .to_string();
        output_subject.title = subject_json
            .get("title")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subject's 'title' field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subjects' 'title' field as a string".to_string(),
                )
            })?
            .to_string();

        output.push(output_subject);

        let chapters_field = subject_json.get("chapters").ok_or_else(|| {
            error::Error::DeserializeError("Failed to get the 'chapters' field".to_string())
        })?;
        let chapters_array = chapters_field.as_array().ok_or_else(|| {
            error::Error::DeserializeError(
                "Failed to get the 'chapters' field as an array".to_string(),
            )
        })?;
        chapter_json_array.extend(chapters_array);
    }

    Ok((output, chapter_json_array))
}
