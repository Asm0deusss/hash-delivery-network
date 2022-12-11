#![forbid(unsafe_code)]

pub mod tools;

use core::fmt;
pub enum ErrorType {
    BadReading,
    IoError(std::io::Error),
    JsonErr(serde_json::Error),
}

impl fmt::Display for ErrorType {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl From<std::io::Error> for ErrorType {
    fn from(err: std::io::Error) -> Self {
        ErrorType::IoError(err)
    }
}

impl From<serde_json::Error> for ErrorType {
    fn from(err: serde_json::Error) -> Self {
        ErrorType::JsonErr(err)
    }
}
