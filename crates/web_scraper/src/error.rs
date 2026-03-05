#[derive(Debug)]
pub enum Error {
    NotFoundError(String),
    InvalidInputError(String),
    RequestError(String),
    ParseError(String),
    HtmlSelectorError(String),
    SerializeError(String),
    DeserializeError(String),
    MissingAnswerError(String),
    BoaEngineError(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFoundError(err_str) => write!(f, "NotFoundError: {}", err_str),
            Error::InvalidInputError(err_str) => write!(f, "InvalidInputError: {}", err_str),
            Error::RequestError(err_str) => write!(f, "RequestError: {}", err_str),
            Error::ParseError(err_str) => write!(f, "ParseError: {}", err_str),
            Error::HtmlSelectorError(err_str) => write!(f, "HtmlSelectorError: {}", err_str),
            Error::SerializeError(err_str) => write!(f, "SerializeError: {}", err_str),
            Error::DeserializeError(err_str) => write!(f, "DeserializeError: {}", err_str),
            Error::MissingAnswerError(err_str) => write!(f, "MissingAnswerError: {}", err_str),
            Error::BoaEngineError(err_str) => write!(f, "BoaEngineError: {}", err_str),
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
