pub mod request;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PATCH,
    DELETE,
}
