use crate::{error, schema, exam_page::chapters};

pub async fn extract(exam_data: &serde_json::Value, output: &mut schema::Exam) -> Result<(), error::Error> {
    let subjects_field = exam_data.get("subjects").ok_or_else(|| {
        error::Error::DeserializeError("Failed to get the 'subjects' field".to_string())
    })?;
    let subjects_array = subjects_field.as_array().ok_or_else(|| {
        error::Error::DeserializeError("Failed to get the 'subjects' field as an array".to_string())
    })?;

    for subject in subjects_array {
        let mut output_subject = schema::Subject::default();

        output_subject.key = subject
            .get("key")
            .ok_or_else(|| error::Error::DeserializeError("Failed to get the subject's 'key' field".to_string()))?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the subjects' 'key' field as a string".to_string())
            })?
            .to_string();
        output_subject.title = subject
            .get("title")
            .ok_or_else(|| error::Error::DeserializeError("Failed to get the subject's 'title' field".to_string()))?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the subjects' 'title' field as a string".to_string())
            })?
            .to_string();
        chapters::extract(&subject, &mut output_subject).await?;

        output.subjects.insert(output_subject.key.clone(), output_subject);
    }
    Ok(())
}
