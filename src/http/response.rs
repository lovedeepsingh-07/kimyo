use crate::http;

#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: http::HttpStatus,
    pub headers: std::collections::HashMap<String, String>,
    pub body: String,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            status_code: http::HttpStatus::OK,
            headers: std::collections::HashMap::new(),
            body: String::new(),
        }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let status = self.status_code.get();

        // status line
        let mut output_string = format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n",
            status.1,
            status.0,
            &self.body.len(),
        );

        // headers
        for (key, value) in &self.headers {
            output_string.push_str(&format! {"{}: {}\r\n",key,value});
        }
        output_string.push_str("\r\n");

        // body
        output_string.push_str(&self.body);

        output_string
    }
}
