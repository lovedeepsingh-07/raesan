// JSON: { key: String, exam: String, title: String }
impl crate::examside::SubjectFromJson for schema::Subject {
    fn from_json(exam_id: String, json: &serde_json::Value) -> Result<Self, error::Error> {
        let subject_key = json
            .get("key")
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the subject[key] field".to_string())
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subject[key] field as a string".to_string(),
                )
            })?
            .to_string();
        let exam_key = json
            .get("exam")
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the subject[exam] field".to_string())
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subject[exam] field as a string".to_string(),
                )
            })?
            .to_string();
        let subject_title = json
            .get("title")
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the subject[title] field".to_string())
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the subject[title] field as a string".to_string(),
                )
            })?
            .to_string();

        Ok(schema::Subject {
            id: uuid::Uuid::new_v4().to_string(),
            key: subject_key,
            exam_key,
            exam_id,
            title: subject_title,
            ..Default::default()
        })
    }
}
