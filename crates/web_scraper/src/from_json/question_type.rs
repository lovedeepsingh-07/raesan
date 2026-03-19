use std::collections::HashMap;

impl crate::QuestionTypeFromJson for schema::QuestionType {
    fn get_answer<'a>(&'a self, json: &'a serde_json::Value) -> Result<String, error::Error> {
        match self {
            schema::QuestionType::MCQ => {
                let answer = json
                    .get("correct_options")
                    .ok_or_else(|| {
                        error::Error::MissingAnswerError(
                            "Failed to get the question[question][en][correct_options] field".to_string(),
                        )
                    })?
                    .get(0)
                    .ok_or_else(|| {
                        error::Error::MissingAnswerError(
                            "Failed to get the question[question][en][correct_options][0] element".to_string(),
                        )
                    })?
                    .as_str()
                    .ok_or_else(|| {
                        error::Error::MissingAnswerError(
                            "Failed to get the question[question][en][correct_options][0] as a string"
                                .to_string(),
                        )
                    })?
                    .to_string();
                return Ok(answer);
            }
            schema::QuestionType::INTEGER => {
                let answer = json
                    .get("answer")
                    .ok_or_else(|| {
                        error::Error::DeserializeError(
                            "Failed to get the question[question][en][answer] field".to_string(),
                        )
                    })?
                    .as_str()
                    .ok_or_else(|| {
                        error::Error::DeserializeError(
                            "Failed to get the question[question][en][answer] field as a string"
                                .to_string(),
                        )
                    })?
                    .to_string();
                return Ok(answer);
            }
        }
    }
    fn get_options<'a>(
        &'a self,
        question_id: String,
        json: &'a serde_json::Value,
    ) -> Result<HashMap<String, schema::QuestionOption>, error::Error> {
        if *self != schema::QuestionType::MCQ {
            return Ok(HashMap::new());
        }

        let mut output: HashMap<String, schema::QuestionOption> = HashMap::new();

        let options_array = json
            .get("options")
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[question][en][option] field".to_string(),
                )
            })?
            .as_array()
            .ok_or_else(|| {
                error::Error::DeserializeError(
                    "Failed to get the question[question][en][options] field as an array"
                        .to_string(),
                )
            })?;

        for option in options_array {
            let option_key = option
                .get("identifier")
                .ok_or_else(|| {
                    error::Error::DeserializeError(
                        "Failed to get the question[question][en][options][.][identifier] field".to_string(),
                    )
                })?
                .as_str()
                .ok_or_else(|| {
                    error::Error::DeserializeError(
                        "Failed to get the question[question][en][options][.][identifier] field as a string".to_string(),
                    )
                })?
                .to_string();
            let option_value = option
                .get("content")
                .ok_or_else(|| {
                    error::Error::DeserializeError(
                        "Failed to get the question[question][en][options][.][content] field".to_string(),
                    )
                })?
                .as_str()
                .ok_or_else(|| {
                    error::Error::DeserializeError(
                        "Failed to get the question[question][en][options][.][content] field as a string".to_string(),
                    )
                })?
                .to_string();
            output.insert(
                option_key.clone(),
                schema::QuestionOption {
                    id: uuid::Uuid::new_v4().to_string(),
                    question_id: question_id.clone(),
                    key: option_key,
                    value: option_value,
                },
            );
        }

        return Ok(output);
    }
}
