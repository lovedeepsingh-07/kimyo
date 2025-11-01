use crate::{
    context, error,
    http::{method, request, response, status},
};
use mlua::prelude::*;
use std::collections::HashMap;
use tokio::sync::RwLock;
use urlencoding;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RouteKey {
    pub path: String,
    pub method: method::HttpMethod,
}

// ------ router ------
// TODO: we can take inspiration from `axum` and `tower` in order to find a better way or a better
// data-structure to store/define these routes and middleware
pub struct Router {
    pub routes: RwLock<HashMap<RouteKey, LuaRegistryKey>>,
    pub middleware_list: RwLock<Vec<LuaRegistryKey>>,
    pub internal_error_handler: RwLock<Option<LuaRegistryKey>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: RwLock::new(HashMap::new()),
            middleware_list: RwLock::new(Vec::new()),
            internal_error_handler: RwLock::new(None),
        }
    }
    async fn handle_internal_error(
        &self,
        lua: &Lua,
        err: LuaError,
        ctx: &context::Context,
    ) -> response::Response {
        let mut err_res = response::Response::default();
        err_res.status_code = status::HttpStatus::InternalServerError;
        err_res.body = String::from("Something went wrong!");

        let handler_option = self.internal_error_handler.read().await;
        if let Some(key) = handler_option.as_ref() {
            let handler: LuaFunction = match lua.registry_value(&key) {
                Ok(out) => out,
                Err(e) => {
                    tracing::error!("(router.rs, 1): {}", e.to_string());
                    return err_res;
                }
            };
            match handler
                .call_async::<LuaAnyUserData>((
                    error::Error::from(err).into_lua_table(lua),
                    ctx.clone(),
                ))
                .await
            {
                Ok(out_ctx) => {
                    let ctx_ref = match out_ctx.borrow::<context::Context>() {
                        Ok(out) => out,
                        Err(e) => {
                            tracing::error!("(router.rs, 2): {}", e.to_string());
                            return err_res;
                        }
                    };
                    return ctx_ref.res.clone();
                }
                Err(e) => {
                    tracing::error!("(router.rs, 3): {}", e.to_string());
                    return err_res;
                }
            };
        }

        err_res
    }
    // TODO: improve space efficiency here, alot of cloning is happening
    pub async fn handle_request(
        &self,
        lua: &Lua,
        stream: &mut tokio::net::TcpStream,
    ) -> Result<response::Response, error::Error> {
        let request = request::Request::new(stream).await?;
        let mut ctx = context::Context {
            req: request,
            res: response::Response::default(),
            path_params: HashMap::new(),
            query_params: HashMap::new(),
        };

        let middleware_list = self.middleware_list.read().await;
        for curr_middleware in middleware_list.iter() {
            let handler: LuaFunction = lua.registry_value(curr_middleware)?;
            let out_ctx: LuaAnyUserData = match handler.call_async(ctx.clone()).await {
                Ok(out) => out,
                Err(e) => {
                    return Ok(self.handle_internal_error(&lua, e, &ctx).await);
                }
            };
            ctx = out_ctx.borrow::<context::Context>()?.clone();
        }

        // extract query parameters (if any)
        self.extract_query_parameters(&mut ctx);

        let routes = self.routes.read().await;
        let route_key = RouteKey {
            path: ctx.req.path.clone(),
            method: ctx.req.method.clone(),
        };
        // matching for exact route definitions
        if let Some(route) = routes.get(&route_key) {
            let route_handler: LuaFunction = lua.registry_value(route)?;
            let lua_out: LuaAnyUserData = match route_handler.call_async(ctx.clone()).await {
                Ok(out) => out,
                Err(e) => {
                    return Ok(self.handle_internal_error(&lua, e, &ctx).await);
                }
            };
            let out = lua_out.borrow::<context::Context>()?;
            return Ok(out.res.clone());
        }
        // matching for dynamic route definitions
        return Ok(self.match_dynamic_route(&mut ctx));
    }
    pub fn extract_query_parameters(&self, ctx: &mut context::Context) {
        let req_path = match urlencoding::decode(&ctx.req.path) {
            Ok(out) => out,
            Err(e) => {
                tracing::warn!("(router.rs, 4): {}", e.to_string());
                return;
            }
        }
        .to_string();
        let path_parts = match req_path.split_once("?") {
            Some(out) => out,
            None => {
                return;
            }
        };
        ctx.req.path = path_parts.0.to_string();
        let param_list = path_parts.1.split("&").collect::<Vec<&str>>();
        for curr_param in param_list {
            let param_parts = match curr_param.split_once("=") {
                Some(out) => out,
                None => {
                    return;
                }
            };
            ctx.query_params
                .insert(param_parts.0.to_string(), param_parts.1.to_string());
        }
    }
    pub fn match_dynamic_route(&self, ctx: &mut context::Context) -> response::Response {
        ctx.res.status_code = status::HttpStatus::NotFound;
        ctx.res.clone()
    }
}

impl LuaUserData for Router {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        let _ = fields;
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // ------ router:internal_error_handler(handler) ------
        methods.add_async_method(
            "internal_error_handler",
            |lua, this, handler: LuaFunction| async move {
                let handler = lua.create_registry_value(handler)?;
                let mut internal_error_handler = this.internal_error_handler.write().await;
                *internal_error_handler = Some(handler);
                Ok(())
            },
        );
        // ------ router:middleware(handler) ------
        methods.add_async_method("middleware", |lua, this, handler: LuaFunction| async move {
            let mut middleware_list = this.middleware_list.write().await;
            middleware_list.push(lua.create_registry_value(handler)?);
            Ok(())
        });
        // ------ router:route(method,path,handler) ------
        methods.add_async_method(
            "route",
            |lua, this, (method, path, handler): (LuaString, LuaString, LuaFunction)| async move {
                let route_path = path.to_str()?;
                let route_method = method::HttpMethod::from(method.to_str()?.to_string());
                let route_handler = lua.create_registry_value(handler)?;

                let route_key = RouteKey {
                    path: route_path.to_string(),
                    method: route_method.clone(),
                };

                let mut routes = this.routes.write().await;
                routes.insert(route_key, route_handler);
                Ok(())
            },
        );
    }
}
