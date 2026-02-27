use crate::error;
use std::collections::HashMap;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum QuestionType {
    #[default]
    MCQ,
    INTEGER,
}

impl From<&str> for QuestionType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "mcq" => QuestionType::MCQ,
            _ => QuestionType::INTEGER,
        }
    }
}

impl QuestionType {
    pub fn get_answer<'a>(&'a self, json: &'a serde_json::Value) -> Result<String, error::Error> {
        match self {
            QuestionType::MCQ => {
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
            QuestionType::INTEGER => {
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
                            "Failed to get the question[question][en][answer] field as a string".to_string(),
                        )
                    })?
                    .to_string();
                return Ok(answer);
            }
        }
    }
    pub fn get_options<'a>(
        &'a self,
        json: &'a serde_json::Value,
    ) -> Result<HashMap<String, String>, error::Error> {
        if *self != QuestionType::MCQ {
            return Ok(HashMap::new());
        }

        let mut output: HashMap<String, String> = HashMap::new();

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
                    "Failed to get the question[question][en][options] field as an array".to_string(),
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
            output.insert(option_key, option_value);
        }

        return Ok(output);
    }
}
