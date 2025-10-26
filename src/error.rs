use mlua;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    LuaError(#[from] mlua::Error),

    #[error("TCP error, {0}")]
    TcpError(String),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("empty request")]
    EmptyRequestError(),

    #[error("invalid request line, {0}")]
    InvalidRequestLine(String),

    #[error("invalid request header, {0:#?}")]
    InvalidRequestHeader(String),

    #[error("{0}")]
    Other(String),
}

macro_rules! lua_result {
    ($lua:expr,$input:expr) => {{
        let result_table = $lua.create_table()?;
        match $input {
            Ok(out) => {
                result_table.set("ok", true)?;
                result_table.set("value", out)?;
            }
            Err(e) => {
                result_table.set("ok", false)?;
                result_table.set("error", e.to_string())?;
            }
        }
        result_table
    }};
}
pub(crate) use lua_result;
