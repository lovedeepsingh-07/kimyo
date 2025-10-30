use crate::http::{request, response, status};
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
        // ------ context:set_res_status(status_code)------
        methods.add_async_method_mut(
            "set_res_status",
            |_, mut this, status_code: u16| async move {
                this.res.status_code =
                    status::HttpStatus::try_from(status_code).map_err(LuaError::external)?;
                Ok(())
            },
        );
        // ------ context:set_res_header(key,value) ------
        methods.add_async_method_mut(
            "set_res_header",
            |_, mut this, (key, value): (LuaString, LuaString)| async move {
                let header_key = key.to_str()?;
                let header_value = value.to_str()?;
                this.res
                    .headers
                    .insert(header_key.to_string(), header_value.to_string());
                Ok(())
            },
        );
        // ------ context:send_string(input) ------
        methods.add_async_method_mut(
            "send_string",
            |_, mut this, input: Option<LuaString>| async move {
                if let Some(input_str) = input {
                    this.res
                        .headers
                        .insert("Content-Type".to_string(), "text/plain".to_string());
                    this.res.body = input_str.to_str()?.to_string();
                } else {
                    this.res.status_code = status::HttpStatus::InternalServerError;
                    this.res
                        .headers
                        .insert("Content-Type".to_string(), "text/plain".to_string());
                    this.res.body = String::from("Internal Server Error");
                }
                Ok(())
            },
        );
    }
}
