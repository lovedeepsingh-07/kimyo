-- server setup
local server_result = kimyo.server.create({
	host = "localhost",
	port = 3000,
	show_banner = true,
})
if not server_result.ok then
	kimyo.debug.error(server_result.error)
	return
end
local server = server_result.value

-- middleware setup
-- server.router:add_middleware(function(ctx)
-- 	kimyo.debug.info(ctx)
-- end)

-- routes setup
server.router:add_route("GET", "/", function(ctx)
	ctx:set_body("home page GET handler")
	return ctx
end)
server.router:add_route("POST", "/users", function(ctx)
	ctx:set_body("users API POST handler")
	return ctx
end)

-- listen
local listen_result = server:listen()
if not listen_result.ok then
	kimyo.debug.error(listen_result.error)
	return
end
