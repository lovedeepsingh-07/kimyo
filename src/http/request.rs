use crate::{error, http};
use color_eyre::eyre;
use std::io::{BufRead, Read};

#[derive(Debug)]
pub struct Request {
    pub method: http::HttpMethod,
    pub path: String,
    pub version: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Option<String>,
}

impl Default for Request {
    fn default() -> Self {
        Request {
            method: http::HttpMethod::GET,
            path: String::from("/"),
            version: String::from("HTTP/1.1"),
            headers: std::collections::HashMap::new(),
            body: None,
        }
    }
}

impl TryFrom<&mut std::net::TcpStream> for Request {
    type Error = error::Error;
    fn try_from(value: &mut std::net::TcpStream) -> eyre::Result<Self, Self::Error> {
        // ------ read the request ------
        let mut buf_reader = std::io::BufReader::new(value);
        let mut request_lines_vector: Vec<String> = Vec::new();
        let mut content_length = 0;
        for line in buf_reader.by_ref().lines() {
            let line = line?;
            if let Some(content_length_str) = line.strip_prefix("Content-Length: ") {
                content_length = content_length_str.parse()?;
            }
            if line.is_empty() {
                request_lines_vector.push(String::from(""));
                break;
            }
            request_lines_vector.push(line);
        }
        let mut body_buffer: Vec<u8> = Vec::new();
        if content_length > 0 {
            body_buffer.resize(content_length, 0);
            buf_reader
                .take(content_length as u64)
                .read_exact(&mut body_buffer)?;
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
                    return Err(error::Error::InvalidRequestLine());
                }
                parsed_request.method = http::HttpMethod::from(parts[0]);
                parsed_request.path = String::from(parts[1]);
                parsed_request.version = String::from(parts[2]);
            }
            None => {
                return Err(error::Error::InvalidRequestLine());
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
