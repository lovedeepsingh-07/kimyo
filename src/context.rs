use crate::http::{request, response};
use mlua::prelude::*;

// TODO: maybe we can somehow make sure that we do not use "Clone" here

// ------ context ------
#[derive(Debug, Clone)]
pub struct Context {
    pub req: request::Request,
    pub res: response::Response,
}

impl LuaUserData for Context {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        let _ = fields;
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // ------ context:set_header ------
        methods.add_async_method_mut(
            "set_header",
            |_, mut this, (header_key, header_value): (LuaString, LuaString)| async move {
                let header_key_str = header_key.to_str()?;
                let header_value_str = header_value.to_str()?;
                this.res
                    .headers
                    .insert(header_key_str.to_string(), header_value_str.to_string());
                Ok(())
            },
        );
        methods.add_async_method_mut("set_body", |_, mut this, input: LuaString| async move {
            let input_str = input.to_str()?.to_string();
            this.res
                .headers
                .insert("Content-Type".to_string(), "text/plain".to_string());
            this.res
                .headers
                .insert("Content-Length".to_string(), input_str.len().to_string());
            this.res.body = input_str;
            Ok(())
        });
    }
}
