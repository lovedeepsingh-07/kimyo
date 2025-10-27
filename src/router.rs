use crate::{
    context, error,
    http::{self, request, response},
};
use mlua::prelude::*;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RouteKey {
    pub path: String,
    pub method: http::HttpMethod,
}

// ------ router ------
// TODO: we can take inspiration from `axum` and `tower` in order to find a better way or a better
// data-structure to store/define these routes and middleware
pub struct Router {
    pub routes: RwLock<HashMap<RouteKey, LuaRegistryKey>>,
    pub middleware_list: RwLock<Vec<LuaRegistryKey>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: RwLock::new(HashMap::new()),
            middleware_list: RwLock::new(Vec::new()),
        }
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
            let out_ctx: LuaAnyUserData = handler.call_async(ctx.clone()).await?;
            ctx = out_ctx.borrow::<context::Context>()?.clone();
        }

        let routes = self.routes.read().await;
        let route_key = RouteKey {
            path: ctx.req.path.clone(),
            method: ctx.req.method.clone(),
        };
        match routes.get(&route_key) {
            Some(route) => {
                let route_handler: LuaFunction = lua.registry_value(route)?;
                let out_ctx: LuaAnyUserData = route_handler.call_async(ctx).await?;
                let out = out_ctx.borrow::<context::Context>()?;
                return Ok(out.res.clone());
            }
            None => {
                tracing::warn!("no route found, {:#?}", &route_key);
            }
        }

        Ok(response::Response::default())
    }
}

impl LuaUserData for Router {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        let _ = fields;
    }
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // ------ router:route(method,path,handler) ------
        methods.add_async_method(
            "route",
            |lua, this, (method, path, handler): (LuaString, LuaString, LuaFunction)| async move {
                let route_path = path.to_str()?;
                let route_method = http::HttpMethod::from(method.to_str()?.to_string());
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
        // ------ router:middleware(handler) ------
        methods.add_async_method("middleware", |lua, this, handler: LuaFunction| async move {
            let mut middleware_list = this.middleware_list.write().await;
            middleware_list.push(lua.create_registry_value(handler)?);
            Ok(())
        });
    }
}
