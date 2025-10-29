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

-- middleware
server.router:middleware(function(ctx)
	ctx:set_res_header("X-Custom-Header", "Some-Value")
	return ctx
end)

-- routes
server.router:route("GET", "/", function(ctx)
	ctx:set_res_status(500)
	ctx:send_string("users API GET handler")
	return ctx
end)
server.router:route("POST", "/users", function(ctx)
	ctx:send_string("users API GET handler")
	return ctx
end)

-- listen
local listen_result = server:listen()
if not listen_result.ok then
	kimyo.debug.error(listen_result.error)
	return
end
