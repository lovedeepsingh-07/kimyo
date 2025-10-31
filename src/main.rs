use clap::Parser;
use mlua::prelude::*;
use tokio;

#[derive(Parser, Debug)]
#[command(about,long_about=None)]
#[command(next_line_help = true)]
pub struct CLIArgs {
    path: String,
}

pub mod context;
pub mod debug;
pub mod error;
pub mod http;
pub mod router;
pub mod server;

async fn main_code(lua: &Lua, script_input: &str) -> Result<(), error::Error> {
    let kimyo = lua.create_table()?;
    kimyo.set("debug", debug::debug_table(&lua)?)?;
    kimyo.set("server", server::server_table(&lua)?)?;
    lua.globals().set("kimyo", kimyo)?;

    lua.load(script_input)
        .set_name("main.lua")
        .exec_async()
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = CLIArgs::parse();
    let dir_path = std::path::Path::new(&args.path);

    if !dir_path.exists() {
        tracing::error!("provided directory does not exist, {:#?}", dir_path);
        return;
    }
    if !dir_path.is_dir() {
        tracing::error!("provided path is not a directory, {:#?}", dir_path);
        return;
    }

    let main_file: std::path::PathBuf = dir_path.join("main.lua");
    if !main_file.exists() | !main_file.is_file() {
        tracing::error!("no main.lua file found in provided folder, {:#?}", dir_path);
        return;
    }

    let script = match std::fs::read_to_string(main_file) {
        Ok(out) => out,
        Err(e) => {
            tracing::error!("(main.rs, 1): {}", e.to_string());
            return;
        }
    };

    let lua = Lua::new();
    match main_code(&lua, &script).await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("(main.rs, 2): {}", e.to_string());
            return;
        }
    }
}
