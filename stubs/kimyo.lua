---@meta

---Debug utilities for logging information, warnings, and errors.
---@class kimyo_debug_module
---@field info fun(message: string): nil @Logs an informational message.
---@field warn fun(message: string): nil @Logs a warning message.
---@field error fun(message: string): nil @Logs an error message.

---Module for creating and managing HTTP servers.
---@class kimyo_server_module
---@field create fun(options: { host: string, port: integer, show_banner: boolean }): { ok: boolean, value: kimyo_server, error: string } @Creates a new server instance with the given options.

---Represents an active HTTP server instance.
---@class kimyo_server
---@field router kimyo_router @Router associated with this server.
---@field listen fun(self: kimyo_server): { ok: boolean, error: string } @Starts listening for incoming HTTP connections.

---Handles routing and middleware for an HTTP server.
---@class kimyo_router
---@field middleware fun(self: kimyo_router, handler: fun(ctx: kimyo_context): kimyo_context): nil @Registers a middleware function that runs before routes.
---@field route fun(self: kimyo_router, method: string, path: string, handler: fun(ctx: kimyo_context): kimyo_context): nil @Registers a route handler for a given HTTP method and path.

---Represents a request/response context used inside route and middleware handlers.
---@class kimyo_context
---@field set_res_header fun(self: kimyo_context, key: string, value: string): nil @Sets a response header.
---@field set_res_status fun(self: kimyo_context, status: integer): nil @Sets the HTTP status code for the response.
---@field send_string fun(self: kimyo_context, body: string): nil @Sends a plain text response to the client.

---Global entry point for the Kimyo API.
---@class kimyo
---@field debug kimyo_debug_module @Debug module for logging.
---@field server kimyo_server_module @Server module for creating HTTP servers.

---@type kimyo
kimyo = {}
