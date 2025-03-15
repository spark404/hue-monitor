use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serialization(String),
    Parsing(Box<dyn StdError + 'static>),
    Json(JsonError),
    Io(IoError),
    CertificateError(Box<dyn StdError + 'static>)
}
impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Self {
        Error::Reqwest(error)
    }
}
impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        match *self {
            Error::Reqwest(ref cause) => write!(
                formatter, "RING error: {}", cause
            ),
            Error::Serialization(ref message) => write!(
                formatter, "serialization error: {}", message
            ),
            Error::Parsing(ref cause) => write!(
                formatter, "parsing error: {}", cause
            ),
            Error::Json(ref cause) => write!(
                formatter, "json error: {}", cause
            ),
            Error::Io(ref cause) => write!(
                formatter, "I/O error: {}", cause
            ),
            Error::CertificateError(ref cause) => write!(
                formatter, "Error parsing certificate: {}", cause
            )
        }
    }
}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::Reqwest(ref cause) => Some(cause),
            Error::Serialization(_) => None,
            Error::Parsing(ref cause) => Some(&**cause),
            Error::Json(ref cause) => Some(cause),
            Error::Io(ref cause) => Some(cause),
            Error::CertificateError(ref cause) => Some(&**cause)
        }
    }
}
