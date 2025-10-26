use crate::http::{self, request, response};
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
    pub async fn handle_request(&self, request: &request::Request) -> response::Response {
        let routes = self.routes.lock().await;
        tracing::info!("{:#?}", routes);
        tracing::info!("{:#?}", request);
        response::Response::default()
    }
}

impl LuaUserData for Router {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        let _ = fields;
    }
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        // ------ router:add_route() ------
        methods.add_async_method(
            "add_route",
            async move |lua, this, (method, path, handler): (LuaString, LuaString, LuaFunction)| {
                let route_path = path.to_str()?;
                let route_method = http::HttpMethod::from(method.to_str()?);
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
