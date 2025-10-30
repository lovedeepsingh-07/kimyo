use crate::{
    context, error,
    http::{method, request, response, status},
};
use mlua::prelude::*;
use std::collections::HashMap;
use tokio::sync::RwLock;

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
        let handler_option = self.internal_error_handler.read().await;
        // let handler_opt = self.error_handler.read().await.clone();
        // if let Some(key) = handler_opt {
        //     let handler: LuaFunction = lua.registry_value(&key)?;
        //     // Call Lua error handler with (err, ctx)
        //     match handler.call_async::<_, LuaAnyUserData>((err.to_string(), ctx.clone())).await {
        //         Ok(out_ctx) => {
        //             let ctx_ref = out_ctx.borrow::<context::Context>()?;
        //             return Ok(ctx_ref.res.clone());
        //         }
        //         Err(lua_err) => {
        //             // Even the error handler failed; fall through to fallback
        //             tracing::error!("Error in error handler: {}", lua_err);
        //         }
        //     }
        // }
        //
        // // Default fallback: generic 500 response
        // let mut error_res = response::Response::default();
        // error_res.status_code = status::HttpStatus::InternalServerError;
        // error_res.body = "Internal Server Error".to_string();
        // tracing::error!("Unhandled Lua error: {}", err);
        // Ok(error_res)
        let mut err_res = response::Response::default();
        err_res.status_code = status::HttpStatus::InternalServerError;
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
        };

        let middleware_list = self.middleware_list.read().await;
        for curr_middleware in middleware_list.iter() {
            let handler: LuaFunction = lua.registry_value(curr_middleware)?;
            let out_ctx: LuaAnyUserData = match handler.call_async(ctx.clone()).await {
                Ok(out) => out,
                Err(e) => {
                    // tracing::error!("(router.rs, 1): {}", e.to_string());
                    // let mut error_res = response::Response::default();
                    // error_res.status_code = status::HttpStatus::InternalServerError;
                    // error_res.body = e.to_string();
                    return Ok(self.handle_internal_error(&lua, e, &ctx).await);
                }
            };
            ctx = out_ctx.borrow::<context::Context>()?.clone();
        }

        let routes = self.routes.read().await;
        let route_key = RouteKey {
            path: ctx.req.path.clone(),
            method: ctx.req.method.clone(),
        };
        if let Some(route) = routes.get(&route_key) {
            let route_handler: LuaFunction = lua.registry_value(route)?;
            let lua_out: LuaAnyUserData = match route_handler.call_async(ctx.clone()).await {
                Ok(out) => out,
                Err(e) => {
                    // tracing::error!("(router.rs, 2): {}", e.to_string());
                    // let mut error_res = response::Response::default();
                    // error_res.status_code = status::HttpStatus::InternalServerError;
                    // error_res.body = e.to_string();
                    return Ok(self.handle_internal_error(&lua, e, &ctx).await);
                }
            };
            let out = lua_out.borrow::<context::Context>()?;
            return Ok(out.res.clone());
        }

        ctx.res.status_code = status::HttpStatus::NotFound;
        Ok(ctx.res)
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
