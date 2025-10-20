pub mod request;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PATCH,
    DELETE,
    Other(String),
}
impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        let return_value = match value {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PATCH" => HttpMethod::PATCH,
            "DELETE" => HttpMethod::DELETE,
            other => HttpMethod::Other(other.to_string()),
        };
        return_value
    }
}
