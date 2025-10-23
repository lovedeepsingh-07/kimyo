use crate::error;
use color_eyre::eyre::{self, Context};
use mlua::prelude::*;
// use std::io::Write;
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
    ) -> eyre::Result<WebServer, error::Error> {
        let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
            .await
            .wrap_err(error::Error::TcpError(
                "Failed to create listener".to_string(),
            ))?;

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
        // fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
    }
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("listen", |_, this, ()| async move {
            if this.show_banner == true {
                println!("running on {}:{}", this.host, this.port);
            }

            // for stream in this.listener.incoming() {
            //     let mut stream = stream.unwrap();
            //     let req = http::request::Request::try_from(&mut stream)?;
            //     tracing::info!("{:#?}", req);
            //     let res = http::response::Response::default();
            //     stream.write(res.to_string().as_bytes())?;
            // }

            Ok(())
        });
    }
}
