package.cpath = package.cpath .. ";./build/?.so"
local kimyo = require("kimyo")

local server_config = { host = "localhost", port = 3000, show_banner = true }
local server = kimyo.create_server(server_config)

local ok, err = pcall(function()
	server:listen()
end)
if not ok then
	print("listen error: ", err)
end
