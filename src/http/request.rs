use crate::{error, http::method};
use tokio::io::{AsyncBufReadExt, AsyncReadExt};

// TODO: maybe we can somehow make sure that we do not use "Clone" here
#[derive(Debug, Clone)]
pub struct Request {
    pub method: method::HttpMethod,
    pub path: String,
    pub version: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Option<String>,
}

impl Default for Request {
    fn default() -> Self {
        Request {
            method: method::HttpMethod::GET,
            path: String::from("/"),
            version: String::from("HTTP/1.1"),
            headers: std::collections::HashMap::new(),
            body: None,
        }
    }
}

impl Request {
    pub async fn new(stream: &mut tokio::net::TcpStream) -> Result<Request, error::Error> {
        // ------ read the request ------
        let mut buf_reader = tokio::io::BufReader::new(stream);
        let mut request_lines_vector: Vec<String> = Vec::new();
        let mut content_length = 0;
        loop {
            let mut line = String::new();
            let bytes = buf_reader.read_line(&mut line).await?;
            if bytes == 0 {
                break;
            }
            let trimmed = line.trim_end().to_string();
            if let Some(content_length_str) = trimmed.strip_prefix("Content-Length: ") {
                content_length = content_length_str.parse()?;
            }
            if trimmed.is_empty() {
                request_lines_vector.push(String::from(""));
                break;
            }
            request_lines_vector.push(trimmed);
        }
        let mut body_buffer: Vec<u8> = Vec::new();
        if content_length > 0 {
            body_buffer.resize(content_length, 0);
            buf_reader
                .take(content_length as u64)
                .read_exact(&mut body_buffer)
                .await?;
            request_lines_vector.push(String::from_utf8_lossy(&body_buffer).to_string());
        }

        // ----- handle empty request ------
        if request_lines_vector.len() == 0 {
            return Err(error::Error::EmptyRequestError());
        }

        // ------ parse the request ------
        let mut parsed_request = Request::default();
        let mut request_lines_iter = request_lines_vector.iter();
        // parse request line
        match request_lines_iter.next() {
            Some(line) => {
                let parts = line.split_whitespace().collect::<Vec<&str>>();
                if parts.len() != 3 {
                    return Err(error::Error::InvalidRequestLine(line.to_string()));
                }
                parsed_request.method = method::HttpMethod::from(parts[0].to_string());
                parsed_request.path = String::from(parts[1]);
                parsed_request.version = String::from(parts[2]);
            }
            None => {
                return Err(error::Error::InvalidRequestLine(
                    "no request line found".to_string(),
                ));
            }
        }
        // parse headers
        while let Some(line) = request_lines_iter.next() {
            if line.trim().is_empty() {
                break;
            }
            match line.split_once(":") {
                Some((key, value)) => {
                    parsed_request
                        .headers
                        .insert(key.trim().to_string(), value.trim().to_string());
                }
                None => {
                    return Err(error::Error::InvalidRequestHeader(line.to_string()));
                }
            }
        }
        // parse body
        parsed_request.body = request_lines_iter.next().cloned();

        Ok(parsed_request)
    }
}
