#[derive(Debug)]
pub enum Error {
    RequestError(String),
    ParseError(String),
    HtmlSelectorError(String),
    SerializeError(String),
    DeserializeError(String),
}

impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::RequestError(err_str) => format!("RequestError: {}", err_str),
            Error::ParseError(err_str) => format!("ParseError: {}", err_str),
            Error::HtmlSelectorError(err_str) => format!("HtmlSelectorError: {}", err_str),
            Error::SerializeError(err_str) => format!("SerializeError: {}", err_str),
            Error::DeserializeError(err_str) => format!("DeserializeError: {}", err_str),
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
