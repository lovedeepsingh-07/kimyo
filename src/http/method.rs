// TODO: maybe we can somehow make sure that we do not use "Clone" here
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    Other(String),
}
impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        let return_value = match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::Other(other) => other,
        }
        .to_string();
        return_value
    }
}
impl From<String> for HttpMethod {
    fn from(value: String) -> Self {
        let return_value = match value.as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "PATCH" => HttpMethod::PATCH,
            "DELETE" => HttpMethod::DELETE,
            other => HttpMethod::Other(other.to_string()),
        };
        return_value
    }
}
