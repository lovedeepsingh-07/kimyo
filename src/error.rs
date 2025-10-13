use color_eyre::{self, eyre};
use thiserror;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("WebServer error, {0}")]
    WebServerError(String),

    #[error(transparent)]
    Other(#[from] eyre::Report),
}
