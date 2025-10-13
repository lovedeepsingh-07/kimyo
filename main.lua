package.cpath = package.cpath .. ";./build/?.so"
local kimyo = require("kimyo")

local server_config = { host = "localhost", port = 8080, show_banner = true }
local server = kimyo.create_server(server_config)

server:listen()
