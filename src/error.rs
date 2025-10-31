use mlua;
use thiserror;

#[allow(non_camel_case_types)]
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

    #[error("invalid status code, {0:#?}")]
    InvalidStatusCode(String),

    #[error("{0}")]
    Other(String),
}

impl Error {
    pub fn into_lua_table(&self, lua: &mlua::Lua) -> mlua::Table {
        let table = match lua.create_table() {
            Ok(out) => out,
            Err(e) => {
                return Error::from(e).into_lua_table(lua);
            }
        };
        match table.set("message", self.to_string()) {
            Ok(_) => {}
            Err(e) => {
                return Error::from(e).into_lua_table(lua);
            }
        };
        return table;
    }
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
