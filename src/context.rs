use crate::http::{request, response};
use mlua::prelude::*;

// ------ context ------
#[derive(Debug)]
pub struct Context {
    pub req: request::Request,
    pub res: response::Response,
}

impl LuaUserData for Context {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        let _ = fields;
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
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
