pub mod request;
pub mod response;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    Other(String),
}
impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        let return_value = match value {
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

#[derive(Debug)]
pub enum HttpStatus {
    OK,
    Created,
    Accepted,
    NoContent,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
}
impl HttpStatus {
    pub fn get(&self) -> (String, u16) {
        match self {
            HttpStatus::OK => ("OK".to_string(), 200),
            HttpStatus::Created => ("Created".to_string(), 201),
            HttpStatus::Accepted => ("Accepted".to_string(), 202),
            HttpStatus::NoContent => ("NoContent".to_string(), 204),
            HttpStatus::MovedPermanently => ("Moved Permanently".to_string(), 301),
            HttpStatus::Found => ("Found".to_string(), 302),
            HttpStatus::SeeOther => ("See Other".to_string(), 303),
            HttpStatus::NotModified => ("Not Modified".to_string(), 304),
            HttpStatus::BadRequest => ("Bad Request".to_string(), 400),
            HttpStatus::Unauthorized => ("Unauthorized".to_string(), 401),
            HttpStatus::Forbidden => ("Forbidden".to_string(), 403),
            HttpStatus::NotFound => ("Not Found".to_string(), 404),
            HttpStatus::MethodNotAllowed => ("Method Not Allowed".to_string(), 405),
            HttpStatus::InternalServerError => ("Internal Server Error".to_string(), 500),
            HttpStatus::NotImplemented => ("Not Implemented".to_string(), 501),
            HttpStatus::BadGateway => ("Bad Gateway".to_string(), 502),
            HttpStatus::ServiceUnavailable => ("Service Unavailable".to_string(), 503),
        }
    }
}
