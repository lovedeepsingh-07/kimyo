use color_eyre;
use mlua::prelude::*;

pub(crate) mod error;
pub(crate) mod server;

#[mlua::lua_module]
fn kimyo(lua: &Lua) -> LuaResult<LuaTable> {
    color_eyre::install().unwrap();
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let exports = lua.create_table()?;

    let create_server = lua.create_function(|_, table: LuaTable| {
        let host: String = table.get("host")?;
        let port: u16 = table.get("port")?;
        let show_banner: bool = table.get("show_banner")?;

        let web_server: server::WebServer =
            server::WebServer::new(show_banner, host, port).unwrap();
        return Ok(web_server);
    })?;

    exports.set("create_server", create_server)?;
    Ok(exports)
}
