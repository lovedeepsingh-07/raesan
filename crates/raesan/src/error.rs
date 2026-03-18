use serde::ser::SerializeStruct;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputError(String),
    NotFoundError(String),
    IOError(String),
    FSError(String),
    DeserializeError(String),
    TauriError(String),
    AlreadyRunningError(String),
}

impl Error {
    pub fn kind(&self) -> &str {
        match self {
            Error::InvalidInputError(_) => "InvalidInputError",
            Error::NotFoundError(_) => "NotFoundError",
            Error::IOError(_) => "IOError",
            Error::FSError(_) => "FSError",
            Error::DeserializeError(_) => "DeserializeError",
            Error::TauriError(_) => "TauriError",
            Error::AlreadyRunningError(_) => "AlreadyRunningError",
        }
    }
    pub fn message(&self) -> &str {
        match self {
            Error::InvalidInputError(err_str)
            | Error::NotFoundError(err_str)
            | Error::IOError(err_str)
            | Error::FSError(err_str)
            | Error::DeserializeError(err_str)
            | Error::TauriError(err_str)
            | Error::AlreadyRunningError(err_str) => err_str.as_str(),
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

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidInputError(err_str) => write!(f, "InvalidInputError: {}", err_str),
            Error::NotFoundError(err_str) => write!(f, "NotFoundError: {}", err_str),
            Error::IOError(err_str) => write!(f, "IOError: {}", err_str),
            Error::FSError(err_str) => write!(f, "FSError: {}", err_str),
            Error::DeserializeError(err_str) => write!(f, "DeserializeError: {}", err_str),
            Error::TauriError(err_str) => write!(f, "TauriError: {}", err_str),
            Error::AlreadyRunningError(err_str) => {
                write!(f, "AlreadyRunningError: {}", err_str)
            }
        }
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
impl From<tauri::Error> for Error {
    fn from(value: tauri::Error) -> Self {
        Error::TauriError(value.to_string())
    }
}
