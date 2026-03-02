use crate::{QuestionTypeFromJson, error};

// JSON: {
//   exam: String,
//   subject: String,
//   chapter: String,
//   chapterGroup: String
//   question: {
//     en: {
//       content: String,
//       correct_options: [String],
//       answer: String,
//       options: [ { identifier: String, content: String } ]
//     }
//   }
// }
impl crate::QuestionFromJson for schema::Question {
    fn from_json(
        chapter_id: String,
        question_type: schema::QuestionType,
        json: &serde_json::Value,
    ) -> Result<Self, error::Error> {
        let exam_key = json
            .get("exam")
            .ok_or_else(|| {
                error::Error::DeserializeError("Failed to get the question[exam] field".to_string())
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[exam] field as a string".to_string(),
                )
            })?
            .to_string();
        let subject_key = json
            .get("subject")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[subject] field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[subject] field as a string".to_string(),
                )
            })?
            .to_string();
        let chapter_key = json
            .get("chapter")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[chapter] field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[chapter] field as a string".to_string(),
                )
            })?
            .to_string();
        let chapter_group = json
            .get("chapterGroup")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[chapterGroup] field".to_string(),
                )
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[chapterGroup] field as a string".to_string(),
                )
            })?
            .to_string();

        let question_body_data = json
            .get("question")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[question] field".to_string(),
                )
            })?
            .get("en")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[question][en] field".to_string(),
                )
            })?;
        let question_content = question_body_data
            .get("content")
            .ok_or_else(|| {
                error::Error::DeserializeError(format!(
                    "Failed to get the question[question][en][content] field, {:#?}",
                    question_body_data
                ))
            })?
            .as_str()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[question][en][content] field as a string"
                        .to_string(),
                )
            })?
            .to_string();

        let question_id = uuid::Uuid::new_v4().to_string();
        let question_answer = question_type.get_answer(question_body_data)?;
        let question_options =
            question_type.get_options(question_id.clone(), question_body_data)?;
        Ok(schema::Question {
            id: question_id,
            exam_key,
            subject_key,
            chapter_key,
            chapter_id,
            chapter_group,
            question_type,
            content: question_content,
            options: question_options,
            answer: question_answer,
        })
    }
}
