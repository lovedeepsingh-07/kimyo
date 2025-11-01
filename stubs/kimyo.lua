---@meta

---debug utilities for logging information, warnings, and errors.
---@class kimyo_debug_module
---@field info fun(message: any): nil @logs an informational message.
---@field warn fun(message: any): nil @logs a warning message.
---@field error fun(message: any): nil @logs an error message.

---module for creating and managing http servers.
---@class kimyo_server_module
---@field create fun(options: { host: string, port: integer, show_banner: boolean }): { ok: boolean, value: kimyo_server, error: string } @creates a new server instance with the given options.

---represents an active http server instance.
---@class kimyo_server
---@field router kimyo_router @router associated with this server.
---@field listen fun(self: kimyo_server): { ok: boolean, error: string } @starts listening for incoming http connections.

---handles routing and middleware for an http server.
---@class kimyo_router
---@field internal_error_handler fun(self: kimyo_router, handler: fun(err: { message: string }, ctx: kimyo_context): kimyo_context): nil @registers a handler that allows you to handle errors before generating responses.
---@field middleware fun(self: kimyo_router, handler: fun(ctx: kimyo_context): kimyo_context): nil @registers a middleware function that runs before routes.
---@field route fun(self: kimyo_router, method: string, path: string, handler: fun(ctx: kimyo_context): kimyo_context): nil @registers a route handler for a given http method and path.

---represents a request/response context used inside route and middleware handlers.
---@class kimyo_context
---@field get_path_param fun(self: kimyo_context, key: string): { none: boolean, value: string } @gets the path parameter from request URL
---@field get_query_param_list fun(self: kimyo_context): {} @gets all the query parameters from request URL
---@field get_query_param fun(self: kimyo_context, key: string):{ none: boolean, value: string } @gets the query parameter from request URL
---@field set_res_header fun(self: kimyo_context, key: string, value: string): nil @sets a response header.
---@field set_res_status fun(self: kimyo_context, status: integer): nil @sets the http status code for the response.
---@field send_string fun(self: kimyo_context, body: string): nil @sends a plain text response to the client.

---global entry point for the kimyo API.
---@class kimyo
---@field debug kimyo_debug_module @debug module for logging.
---@field server kimyo_server_module @server module for creating http servers.

---@type kimyo
kimyo = {}
