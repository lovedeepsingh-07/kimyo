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
	ctx:set_res_header("Content-Type", "text/html")
	ctx:send_string([[<html>
<head><title>kimyo</title></head>
<body>
<h1>hello, world!</h1>
</body>
</html>]])
	return ctx
end)
server.router:route("GET", "/api/users", function(ctx)
	kimyo.debug.info(ctx:get_query_param_list())
	local query_param_opt = ctx:get_query_param("name")
	if not query_param_opt.none then
		kimyo.debug.info(query_param_opt.value)
	else
		kimyo.debug.warn("no query param named 'name'")
	end

	ctx:send_string("users API GET handler")
	return ctx
end)
server.router:route("GET", "/api/users/{user_id}", function(ctx)
	local path_param_opt = ctx:get_path_param("user_id")
	if not path_param_opt.none then
		kimyo.debug.info(path_param_opt.value)
	else
		kimyo.debug.info("no path param named 'user_id'")
	end

	local query_param_opt = ctx:get_query_param("user_name")
	if not query_param_opt.none then
		kimyo.debug.info(query_param_opt.value)
	else
		kimyo.debug.info("no query param named 'user_name'")
	end

	ctx:send_string("users API GET handler")
	return ctx
end)

-- listen
local listen_result = server:listen()
if not listen_result.ok then
	kimyo.debug.error(listen_result.error)
	return
end
