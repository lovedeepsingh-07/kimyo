use crate::{error, router};
use mlua::prelude::*;
use std::sync::Arc;
use tokio::{self, io::AsyncWriteExt};

// ------ server ------
pub struct WebServer {
    pub show_banner: bool,
    pub host: String,
    pub port: u16,
    pub listener: tokio::net::TcpListener,
    pub router: Arc<router::Router>,
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
        let router = Arc::new(router::Router::new());
        Ok(WebServer {
            show_banner,
            host,
            port,
            listener,
            router,
        })
    }
}

impl LuaUserData for WebServer {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        // ------ server.router ------
        fields.add_field_method_get("router", |_, this| {
            let router = this.router.clone();
            return Ok(router);
        });
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // ------ server:listen ------
        methods.add_async_method("listen", |lua, this, ()| async move {
            let result_lua = lua.clone();
            let result: Result<LuaValue, error::Error> = async move {
                if this.show_banner == true {
                    tracing::info!("running on {}:{}", this.host, this.port);
                }
                let mut tasks: tokio::task::JoinSet<Result<(), error::Error>> =
                    tokio::task::JoinSet::new();

                loop {
                    tokio::select! {
                        conn = this.listener.accept() => {
                            let (mut stream,_) = conn?;
                            let router = this.router.clone();
                            let task_lua = result_lua.clone();
                            tasks.spawn(async move {
                                let res = router.handle_request(&task_lua, &mut stream).await?;
                                stream.write(res.to_string().as_bytes()).await?;
                                Ok(())
                            });
                        }
                        Some(join_result) = tasks.join_next() => {
                            let result = join_result.map_err(|e| error::Error::Other(e.to_string()))?;
                            match result {
                                Ok(_) => {},
                                Err(e) => {
                                    tracing::error!("{:#?}",e);
                                }
                            }
                        }
                        exit_signal = tokio::signal::ctrl_c() => {
                            match exit_signal{
                                Ok(()) => {
                                    break;
                                },
                                Err(e) => {
                                    tracing::error!("{:#?}",e);
                                }
                            }
                        }
                    }
                };
                Ok(LuaValue::Nil)
            }
            .await;
            Ok(error::lua_result!(lua, result))
        });
    }
}

// ------ server.create ------
async fn server_create(_: &Lua, server_config: &LuaTable) -> Result<WebServer, error::Error> {
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
