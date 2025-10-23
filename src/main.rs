use color_eyre::{self, eyre};
use mlua::prelude::*;
use tokio;

pub mod debug;
pub mod error;
pub mod http;
pub mod server;

#[tokio::main]
async fn main() -> eyre::Result<(), error::Error> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let lua = Lua::new();

    let kimyo = lua.create_table()?;
    kimyo.set("debug", debug::debug_table(&lua)?)?;
    lua.globals().set("kimyo", kimyo)?;

    let script = std::fs::read_to_string("main.lua")?;
    lua.load(&script).set_name("main.lua").exec_async().await?;
    Ok(())
}
