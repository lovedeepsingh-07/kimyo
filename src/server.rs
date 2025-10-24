use crate::error;
use mlua::prelude::*;
use tokio;

pub struct WebServer {
    pub show_banner: bool,
    pub host: String,
    pub port: u16,
    pub listener: tokio::net::TcpListener,
}

impl WebServer {
    pub async fn new(
        show_banner: bool,
        host: String,
        port: u16,
    ) -> Result<WebServer, error::Error> {
        let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
            .await
            .map_err(|e| {
                error::Error::TcpError(format!("Failed to create listener: {}", e.to_string()))
            })?;
        Ok(WebServer {
            show_banner,
            host,
            port,
            listener,
        })
    }
}

impl LuaUserData for WebServer {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        let _ = fields;
    }
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        // ------ server:listen ------
        methods.add_async_method("listen", |lua, this, ()| async move {
            let result: Result<LuaValue, error::Error> = async move {
                if this.show_banner == true {
                    tracing::info!("running on {}:{}", this.host, this.port);
                }
                Ok(LuaValue::Nil)
            }
            .await;
            Ok(error::lua_result!(lua, result))

            // for stream in this.listener.incoming() {
            //     let mut stream = stream.unwrap();
            //     let req = http::request::Request::try_from(&mut stream)?;
            //     tracing::info!("{:#?}", req);
            //     let res = http::response::Response::default();
            //     stream.write(res.to_string().as_bytes())?;
            // }
        });
    }
}
// ------ server.create ------
async fn server_create<'a>(
    _: &'a Lua,
    server_config: &LuaTable<'a>,
) -> Result<WebServer, error::Error> {
    let host: String = server_config.get("host")?;
    let port: u16 = server_config.get("port")?;
    let show_banner: bool = server_config.get("show_banner")?;

    let web_server = WebServer::new(show_banner, host, port).await?;
    Ok(web_server)
}

// ------ kimyo.server ------
pub fn server_table(lua: &Lua) -> Result<LuaTable, error::Error> {
    let server_table = lua.create_table()?;
    server_table.set(
        "create",
        lua.create_async_function(|lua, server_config: LuaTable| async move {
            let result = server_create(&lua, &server_config).await;
            Ok(error::lua_result!(lua, result))
        })?,
    )?;
    Ok(server_table)
}
