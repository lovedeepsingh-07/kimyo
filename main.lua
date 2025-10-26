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

-- routes setup
server.router:add_route("GET", "/", function()
	kimyo.debug.info("some handler")
end)
server.router:add_route("POST", "/shit", function()
	kimyo.debug.info("some other handler")
end)

-- listen
local listen_result = server:listen()
if not listen_result.ok then
	kimyo.debug.error(listen_result.error)
	return
end
