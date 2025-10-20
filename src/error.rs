use color_eyre::{self, eyre};
use mlua;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("TCP error, {0}")]
    TcpError(String),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("empty request")]
    EmptyRequestError(),

    #[error("invalid request line")]
    InvalidRequestLine(),

    #[error("invalid request header, {0:#?}")]
    InvalidRequestHeader(String),

    #[error(transparent)]
    Other(#[from] eyre::Report),
}

impl From<Error> for mlua::Error {
    fn from(value: Error) -> Self {
        mlua::Error::external(value)
    }
}
