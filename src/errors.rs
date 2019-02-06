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

    #[fail(display = "bad media type {}", _0)]
    BadMediaType(String),
    #[fail(display = "bad gender {}", _0)]
    BadGender(String),
    #[fail(display = "sodium init failed")]
    SodiumInit,
}
