use crate::error;
use color_eyre::eyre::{self, Context};
use httparse;
use mlua::prelude::*;
use std::io::Read;

pub(crate) struct WebServer {
    pub(crate) show_banner: bool,
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) listener: std::net::TcpListener,
}

impl WebServer {
    pub(crate) fn new(
        show_banner: bool,
        host: String,
        port: u16,
    ) -> eyre::Result<WebServer, error::Error> {
        let listener = std::net::TcpListener::bind(format!("{}:{}", host, port)).wrap_err(
            error::Error::WebServerError("Failed to create listener".to_string()),
        )?;
        return Ok(WebServer {
            show_banner,
            host,
            port,
            listener,
        });
    }
}

impl LuaUserData for WebServer {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        let _ = fields;
        // fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
    }
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("listen", |_, this, ()| -> LuaResult<()> {
            if this.show_banner == true {
                tracing::info!("running on {}:{}", this.host, this.port);
            }

            // for stream in this.listener.incoming() {
            //     let mut stream = stream.unwrap();
            //
            //     let mut request_data: Vec<u8> = Vec::new();
            //
            //     let mut read_buffer: [u8; 1024] = [0; 1024];
            //     loop {
            //         let bytes_read = stream.read(&mut read_buffer)?;
            //         if bytes_read == 0 {
            //             break;
            //         }
            //         request_data.extend_from_slice(&read_buffer[..bytes_read]);
            //
            //         let mut request_headers = [httparse::EMPTY_HEADER; 64];
            //         let mut request = httparse::Request::new(&mut request_headers);
            //         match request
            //             .parse(&request_data)
            //             .map_err(|e| error::Error::HttpParseError(e))?
            //         {
            //             httparse::Status::Partial => {}
            //             httparse::Status::Complete(_) => {
            //                 tracing::info!("{:#?}", request);
            //                 break;
            //             }
            //         };
            //     }
            // }

            return Err(mlua::Error::external(error::Error::WebServerError(
                "shit don't work".to_string(),
            )));
            // return Ok(());
        });
    }
}
