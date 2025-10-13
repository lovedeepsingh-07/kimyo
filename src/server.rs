use crate::error;
use color_eyre::eyre::{self, Context};
use mlua::prelude::*;
use std::io::{BufRead, BufReader, Read};

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
        methods.add_method("listen", |_, this, ()| {
            if this.show_banner == true {
                tracing::info!("running on {}:{}", this.host, this.port);
            }

            for stream in this.listener.incoming() {
                let mut stream = stream.unwrap();

                // parse the request string into a `Request` struct by first parsing the string to a string
                // vector containling the lines of requests as elements by following cases:-
                //
                // - if the headers contain the `Content-Length` header and it's value is more than 0, then
                //   we properly parse the body too
                // - if the headers do not contain the `Content-Length` then we stop after parsing
                let mut request_vector = Vec::new();
                let mut content_length = 0;
                let mut buf_reader = BufReader::new(&mut stream);
                for line in buf_reader.by_ref().lines() {
                    let line = line?;
                    match line.strip_prefix("Content-Length: ") {
                        Some(c_l) => {
                            content_length = c_l.trim().parse().unwrap();
                        }
                        None => {}
                    }
                    if line.is_empty() {
                        request_vector.push(line);
                        break;
                    }
                    request_vector.push(line);
                }
                let mut body = Vec::new();
                if content_length > 0 {
                    body.resize(content_length, 0);
                    buf_reader
                        .take(content_length as u64)
                        .read_exact(&mut body)?;
                    request_vector.push(String::from_utf8_lossy(&body).to_string());
                }
                tracing::info!("request: {:#?}", request_vector);
            }

            return Ok(());
        });
    }
}
