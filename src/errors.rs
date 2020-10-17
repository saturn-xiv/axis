use std::fmt;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    StdStrUtf8(std::str::Utf8Error),
    StdIo(std::io::Error),
    StdNetAddrParse(std::net::AddrParseError),

    HandlebarsRender(handlebars::RenderError),
    HandlebarsTemplate(handlebars::TemplateError),
    HandlebarsTemplateRender(handlebars::TemplateRenderError),
    TomlDe(toml::de::Error),

    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StdStrUtf8(v) => v.fmt(f),
            Self::StdIo(v) => v.fmt(f),
            Self::StdNetAddrParse(v) => v.fmt(f),

            Self::HandlebarsRender(v) => v.fmt(f),
            Self::HandlebarsTemplate(v) => v.fmt(f),
            Self::HandlebarsTemplateRender(v) => v.fmt(f),
            Self::TomlDe(v) => v.fmt(f),

            Self::Custom(v) => v.fmt(f),
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::StdStrUtf8(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::StdIo(err)
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Self::StdNetAddrParse(err)
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(err: handlebars::RenderError) -> Self {
        Self::HandlebarsRender(err)
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(err: handlebars::TemplateError) -> Self {
        Self::HandlebarsTemplate(err)
    }
}

impl From<handlebars::TemplateRenderError> for Error {
    fn from(err: handlebars::TemplateRenderError) -> Self {
        Self::HandlebarsTemplateRender(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::TomlDe(err)
    }
}
