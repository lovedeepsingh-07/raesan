// JSON: { key: String, tite: String, examGroup: String }
impl crate::ExamFromJson for schema::Exam {
    fn from_json(json: &serde_json::Value) -> Result<Self, error::Error> {
        let exam_key = json
            .get("key")
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the exam[key] field".to_string())
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the exam[key] field as a string".to_string(),
                )
            })?
            .to_string();
        let exam_title = json
            .get("title")
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the exam[title] field".to_string())
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the exam[title] field as a string".to_string(),
                )
            })?
            .to_string();
        let exam_group = json
            .get("examGroup")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the exam[examGroup] field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the exam[examGroup] field as a string".to_string(),
                )
            })?
            .to_string();

        Ok(schema::Exam {
            id: uuid::Uuid::new_v4().to_string(),
            key: exam_key,
            title: exam_title,
            group: exam_group,
        })
    }
}
