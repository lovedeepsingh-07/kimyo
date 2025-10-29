use crate::error;

// TODO: maybe we can somehow make sure that we do not use "Clone" here
#[derive(Debug, Clone)]
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
impl TryFrom<u16> for HttpStatus {
    type Error = error::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let return_value = match value {
            200 => HttpStatus::OK,
            201 => HttpStatus::Created,
            202 => HttpStatus::Accepted,
            204 => HttpStatus::NoContent,
            301 => HttpStatus::MovedPermanently,
            302 => HttpStatus::Found,
            303 => HttpStatus::SeeOther,
            304 => HttpStatus::NotModified,
            400 => HttpStatus::BadRequest,
            401 => HttpStatus::Unauthorized,
            403 => HttpStatus::Forbidden,
            404 => HttpStatus::NotFound,
            405 => HttpStatus::MethodNotAllowed,
            500 => HttpStatus::InternalServerError,
            501 => HttpStatus::NotImplemented,
            502 => HttpStatus::BadGateway,
            503 => HttpStatus::ServiceUnavailable,
            _ => {
                return Err(error::Error::InvalidStatusCode(value.to_string()));
            }
        };
        Ok(return_value)
    }
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
