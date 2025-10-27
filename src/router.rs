use crate::{
    context, error,
    http::{self, request, response},
};
use mlua::prelude::*;
use std::collections::HashMap;
use tokio::sync::Mutex;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct RouteKey {
    pub path: String,
    pub method: http::HttpMethod,
}

// ------ router ------
pub struct Router {
    pub routes: Mutex<HashMap<RouteKey, LuaRegistryKey>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: Mutex::new(HashMap::new()),
        }
    }
    pub async fn handle_request(
        &self,
        lua: &Lua,
        stream: &mut tokio::net::TcpStream,
    ) -> Result<response::Response, error::Error> {
        let request = request::Request::new(stream).await?;
        let ctx = context::Context {
            req: request,
            res: response::Response::default(),
        };

        let routes = self.routes.lock().await;
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
        // ------ router:add_route() ------
        methods.add_async_method(
            "add_route",
            |lua, this, (method, path, handler): (LuaString, LuaString, LuaFunction)| async move {
                let route_path = path.to_str()?;
                let route_method = http::HttpMethod::from(method.to_str()?.to_string());
                let route_handler = lua.create_registry_value(handler)?;

                let route_key = RouteKey {
                    path: route_path.to_string(),
                    method: route_method.clone(),
                };

                let mut routes = this.routes.lock().await;
                routes.insert(route_key, route_handler);
                Ok(())
            },
        );
    }
}
