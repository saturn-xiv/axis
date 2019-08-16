use std::result::Result as StdResult;

use failure::{Error as FailureError, Fail};

pub type Result<T> = StdResult<T, FailureError>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Io(#[fail(cause)] std::io::Error),
    #[fail(display = "{}", _0)]
    Utf8(#[fail(cause)] std::str::Utf8Error),
    #[fail(display = "{}", _0)]
    NetAddrParse(#[fail(cause)] std::net::AddrParseError),

    #[fail(display = "{}", _0)]
    SerdeJson(#[fail(cause)] serde_json::Error),

    #[fail(display = "{}", _0)]
    Tera(String),
}

impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Error {
        Error::Tera(e.to_string())
    }
}
