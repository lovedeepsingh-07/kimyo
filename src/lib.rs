use mlua::prelude::*;

#[mlua::lua_module]
fn kimyo(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    let hello_world = lua.create_function(|_, ()| {
        println!("hello from rust!");
        return Ok(());
    })?;

    exports.set("hello_world", hello_world)?;
    Ok(exports)
}
