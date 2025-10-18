use crate::error;
use crate::http;
use color_eyre::eyre;
use std::io::{BufRead, Read};

#[derive(Debug)]
pub struct Request {
    pub method: http::HttpMethod,
}

impl Request {
    pub fn new(stream: &mut std::net::TcpStream) -> eyre::Result<Request, error::Error> {
        let mut buf_reader = std::io::BufReader::new(stream);
        let mut request_lines_vector: Vec<String> = Vec::new();
        let mut content_length = 0;
        for line in buf_reader.by_ref().lines() {
            let line = line?;
            if let Some(content_length_str) = line.strip_prefix("Content-Length: ") {
                content_length = content_length_str.parse()?;
            }
            if line.is_empty() {
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
        }
        request_lines_vector.push(String::from_utf8_lossy(&body_buffer).to_string());

        match request_lines_vector.get(0) {
            Some(_) => {}
            None => {}
        }

        return Ok(Request {
            method: http::HttpMethod::GET,
        });
    }
}
