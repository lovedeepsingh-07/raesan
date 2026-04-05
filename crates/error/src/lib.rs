use serde::ser::SerializeStruct;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    NotFoundError(String),
    InvalidInputError(String),
    IOError(String),
    FSError(String),
    RequestError(String),
    ParseError(String),
    HtmlSelectorError(String),
    SerializeError(String),
    DeserializeError(String),
    BoaEngineError(String),
    TauriError(String),
    AlreadyRunningError(String),
    DatabaseError(String),
    ChannelError(String),
}
impl std::error::Error for Error {}

impl Error {
    pub fn kind(&self) -> &str {
        match self {
            Error::NotFoundError(_) => "NotFoundError",
            Error::InvalidInputError(_) => "InvalidInputError",
            Error::IOError(_) => "IOError",
            Error::FSError(_) => "FSError",
            Error::RequestError(_) => "RequestError",
            Error::ParseError(_) => "ParseError",
            Error::HtmlSelectorError(_) => "HtmlSelectorError",
            Error::SerializeError(_) => "SerializeError",
            Error::BoaEngineError(_) => "BoaEngineError",
            Error::DeserializeError(_) => "DeserializeError",
            Error::TauriError(_) => "TauriError",
            Error::AlreadyRunningError(_) => "AlreadyRunningError",
            Error::DatabaseError(_) => "DatabaseError",
            Error::ChannelError(_) => "ChannelError",
        }
    }
    pub fn message(&self) -> &str {
        match self {
            Error::NotFoundError(err_str)
            | Error::InvalidInputError(err_str)
            | Error::IOError(err_str)
            | Error::FSError(err_str)
            | Error::RequestError(err_str)
            | Error::ParseError(err_str)
            | Error::HtmlSelectorError(err_str)
            | Error::SerializeError(err_str)
            | Error::DeserializeError(err_str)
            | Error::BoaEngineError(err_str)
            | Error::TauriError(err_str)
            | Error::AlreadyRunningError(err_str)
            | Error::DatabaseError(err_str)
            | Error::ChannelError(err_str) => err_str.as_str(),
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFoundError(err_str) => write!(f, "NotFoundError: {}", err_str),
            Error::InvalidInputError(err_str) => write!(f, "InvalidInputError: {}", err_str),
            Error::IOError(err_str) => write!(f, "IOError: {}", err_str),
            Error::FSError(err_str) => write!(f, "FSError: {}", err_str),
            Error::RequestError(err_str) => write!(f, "RequestError: {}", err_str),
            Error::ParseError(err_str) => write!(f, "ParseError: {}", err_str),
            Error::HtmlSelectorError(err_str) => write!(f, "HtmlSelectorError: {}", err_str),
            Error::SerializeError(err_str) => write!(f, "SerializeError: {}", err_str),
            Error::DeserializeError(err_str) => write!(f, "DeserializeError: {}", err_str),
            Error::BoaEngineError(err_str) => write!(f, "BoaEngineError: {}", err_str),
            Error::TauriError(err_str) => write!(f, "TauriError: {}", err_str),
            Error::AlreadyRunningError(err_str) => write!(f, "AlreadyRunningError: {}", err_str),
            Error::DatabaseError(err_str) => write!(f, "DatabaseError: {}", err_str),
            Error::ChannelError(err_str) => write!(f, "ChannelError: {}", err_str),
        }
    }
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Error", 2)?;
        s.serialize_field("kind", self.kind())?;
        s.serialize_field("message", self.message())?;
        s.end()
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value.to_string())
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::DeserializeError(value.to_string())
    }
}
#[cfg(feature = "tauri")]
impl From<tauri::Error> for Error {
    fn from(value: tauri::Error) -> Self {
        Error::TauriError(value.to_string())
    }
}
#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::RequestError(value.to_string())
    }
}
#[cfg(feature = "scraper")]
impl From<scraper::error::SelectorErrorKind<'_>> for Error {
    fn from(value: scraper::error::SelectorErrorKind) -> Self {
        Error::HtmlSelectorError(value.to_string())
    }
}
#[cfg(feature = "boa_engine")]
impl From<boa_engine::error::JsError> for Error {
    fn from(value: boa_engine::error::JsError) -> Self {
        Error::BoaEngineError(value.to_string())
    }
}
#[cfg(feature = "sqlx")]
impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Error::DatabaseError(value.to_string())
    }
}
#[cfg(feature = "tokio")]
impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(value: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Error::ChannelError(value.to_string())
    }
}
