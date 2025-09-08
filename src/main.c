#include <lauxlib.h>
#include <lua.h>
#include <lualib.h>
#include <stdio.h>

int main() {
    lua_State* L = luaL_newstate();
    luaL_openlibs(L);

    const char* lua_script = "print(\"hello,world!\")";

    if (luaL_dofile(L, "main.lua") != LUA_OK) {
        fprintf(stderr, "Lua error: %s\n", lua_tostring(L, -1));
        lua_close(L);
        return 1;
    }

    lua_getglobal(L, "SERVER_CONFIG");
    if (!lua_istable(L, -1)) {
        fprintf(stderr, "SERVER_CONFIG is not a table\n");
        lua_close(L);
        return 1;
    }

    lua_getfield(L, -1, "address");
    if (!lua_isstring(L, -1)) {
        fprintf(stderr, "address is not a string\n");
        lua_close(L);
        return 1;
    }
    const char* server_address = (const char*)lua_tostring(L, -1);
    lua_pop(L, 1);

    lua_getfield(L, -1, "port");
    if (!lua_isinteger(L, -1)) {
        fprintf(stderr, "port is not an integer\n");
        lua_close(L);
        return 1;
    }

    int server_port = (int)lua_tointeger(L, -1);

    printf("server running on http://%s:%d\n", server_address, server_port);

    lua_close(L);
    return 0;
}
