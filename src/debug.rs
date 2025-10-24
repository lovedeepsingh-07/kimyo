use crate::error;
use mlua::prelude::*;

pub fn debug_table(lua: &Lua) -> Result<LuaTable, error::Error> {
    let debug_table = lua.create_table()?;
    debug_table.set(
        "info",
        lua.create_function(|_, input: LuaValue| {
            tracing::info!("{:#?}", input);
            Ok(())
        })?,
    )?;
    debug_table.set(
        "warn",
        lua.create_function(|_, input: LuaValue| {
            tracing::warn!("{:#?}", input);
            Ok(())
        })?,
    )?;
    debug_table.set(
        "error",
        lua.create_function(|_, input: LuaValue| {
            tracing::error!("{:#?}", input);
            Ok(())
        })?,
    )?;
    Ok(debug_table)
}
