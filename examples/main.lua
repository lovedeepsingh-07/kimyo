-- server
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

-- internal error handler
server.router:internal_error_handler(function(err, ctx)
	kimyo.debug.error(err.message)
	ctx:set_res_status(500)
	ctx:send_string("Something went so wrong, that I can't even tell you about it!")
	return ctx
end)

-- middleware
server.router:middleware(function(ctx)
	ctx:set_res_header("X-Custom-Header", "Some-Value")
	return ctx
end)

-- routes
server.router:route("GET", "/", function(ctx)
	ctx:set_res_status(200)
	ctx:set_res_header("Content-Type", "text/plain")
	ctx:send_string("hello, world!")
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
