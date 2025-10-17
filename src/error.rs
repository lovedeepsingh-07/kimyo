use color_eyre::{self, eyre};
use httparse;
use mlua;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    HttpParseError(#[from] httparse::Error),

    #[error("WebServer error, {0}")]
    WebServerError(String),

    #[error(transparent)]
    Other(#[from] eyre::Report),
}

impl From<Error> for mlua::Error {
    fn from(value: Error) -> Self {
        return mlua::Error::external(value);
    }
}
