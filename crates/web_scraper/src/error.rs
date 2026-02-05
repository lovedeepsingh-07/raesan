#[derive(Debug)]
pub enum Error {
    NotFoundError(String),
    RequestError(String),
    ParseError(String),
    HtmlSelectorError(String),
    SerializeError(String),
    DeserializeError(String),
    MissingAnswerError(String),
    BoaEngineError(String),
}

impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::NotFoundError(err_str) => format!("NotFoundError: {}", err_str),
            Error::RequestError(err_str) => format!("RequestError: {}", err_str),
            Error::ParseError(err_str) => format!("ParseError: {}", err_str),
            Error::HtmlSelectorError(err_str) => format!("HtmlSelectorError: {}", err_str),
            Error::SerializeError(err_str) => format!("SerializeError: {}", err_str),
            Error::DeserializeError(err_str) => format!("DeserializeError: {}", err_str),
            Error::MissingAnswerError(err_str) => format!("MissingAnswerError: {}", err_str),
            Error::BoaEngineError(err_str) => format!("BoaEngineError: {}", err_str),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::RequestError(value.to_string())
    }
}
impl From<scraper::error::SelectorErrorKind<'_>> for Error {
    fn from(value: scraper::error::SelectorErrorKind) -> Self {
        Error::HtmlSelectorError(value.to_string())
    }
}
impl From<boa_engine::error::JsError> for Error {
    fn from(value: boa_engine::error::JsError) -> Self {
        Error::BoaEngineError(value.to_string())
    }
}
