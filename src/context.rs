use crate::http::{request, response, status};
use mlua::prelude::*;
use std::collections::HashMap;

// TODO: maybe we can somehow make sure that we do not use "Clone" here

// ------ context ------
#[derive(Debug, Clone)]
pub struct Context {
    pub req: request::Request,
    pub res: response::Response,
    pub path_params: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
}

impl LuaUserData for Context {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        let _ = fields;
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // ------ context:get_path_param(param_key)------
        methods.add_async_method("get_path_param", |lua, _, param_key: String| async move {
            let option_table = lua.create_table()?;
            option_table.set("none", false)?;
            option_table.set("value", param_key)?;
            Ok(option_table)
        });
        // ------ context:get_query_param_list()------
        methods.add_async_method("get_query_param_list", |lua, this, ()| async move {
            let param_list_table = lua.create_table()?;

            let temp_iter = this.query_params.clone();
            for curr_param in temp_iter {
                param_list_table.set(curr_param.0, curr_param.1)?;
            }

            Ok(param_list_table)
        });
        // ------ context:get_query_param(param_key)------
        methods.add_async_method(
            "get_query_param",
            |lua, this, param_key: String| async move {
                let option_table = lua.create_table()?;
                let param_value = match this.query_params.get(&param_key) {
                    Some(out) => out,
                    None => {
                        option_table.set("none", true)?;
                        return Ok(option_table);
                    }
                };
                option_table.set("none", false)?;
                option_table.set("value", param_value.clone())?;
                Ok(option_table)
            },
        );
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
            |_, mut this, (key, value): (String, String)| async move {
                this.res.headers.insert(key, value);
                Ok(())
            },
        );
        // ------ context:send_string(input) ------
        methods.add_async_method_mut("send_string", |_, mut this, input: String| async move {
            this.res.body = input;
            Ok(())
        });
    }
}
