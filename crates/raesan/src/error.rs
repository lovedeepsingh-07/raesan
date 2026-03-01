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

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOError(err_str) => write!(f, "IOError: {}", err_str),
            Error::FSError(err_str) => write!(f, "FSError: {}", err_str),
            Error::ParseError(err_str) => write!(f, "ParseError: {}", err_str),
            Error::ProtocolError(err_str) => write!(f, "ProtocolError: {}", err_str),
            Error::NotFoundError(err_str) => write!(f, "NotFoundError: {}", err_str),
            Error::ChannelSendError(err_str) => write!(f, "ChannelSendError: {}", err_str),
            Error::ChannelReceiveError(err_str) => write!(f, "ChannelReceiveError: {}", err_str),
            Error::RequestError(err_str) => write!(f, "RequestError: {}", err_str),
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
