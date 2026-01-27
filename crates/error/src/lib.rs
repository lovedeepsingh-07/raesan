#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    IOError(String),
    FSError(String),
    ParseError(String),
    ProtocolError(String),
    NotFoundError(String),
    ChannelSendError(String),
    ChannelReceiveError(String),
    RequestError(String),
}

impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::IOError(err_str) => format!("IOError: {}", err_str),
            Error::FSError(err_str) => format!("FSError: {}", err_str),
            Error::ParseError(err_str) => format!("ParseError: {}", err_str),
            Error::ProtocolError(err_str) => format!("ProtocolError: {}", err_str),
            Error::NotFoundError(err_str) => format!("NotFoundError: {}", err_str),
            Error::ChannelSendError(err_str) => format!("ChannelSendError: {}", err_str),
            Error::ChannelReceiveError(err_str) => format!("ChannelReceiveError: {}", err_str),
            Error::RequestError(err_str) => format!("RequestError: {}", err_str),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value.to_string())
    }
}
impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(value: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Error::ChannelSendError(value.to_string())
    }
}
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::RequestError(value.to_string())
    }
}
impl From<scraper::error::SelectorErrorKind<'_>> for Error {
    fn from(value: scraper::error::SelectorErrorKind) -> Self {
        Error::ParseError(value.to_string())
    }
}
