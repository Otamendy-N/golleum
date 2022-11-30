use std::collections::HashMap;

use self::header_map::{Header, HeaderMap};
use self::status_code::StatusCode;

mod header_map;
mod status_code;

pub struct Response {
  headers: HeaderMap,
  body: Vec<u8>,
  status: StatusCode,
}

impl Response {
  pub fn new() -> Response {
    let headers: HeaderMap = HashMap::new();
    let body = Vec::new();
    Response {
      headers,
      body,
      status: StatusCode::NotFound,
    }
  }

  pub fn ok(&self, body: String) -> Response {
    Response {
      headers: self.headers.clone(),
      body: body.as_bytes().to_vec(),
      status: StatusCode::Ok,
    }
  }

  pub fn ok_as_bytes(&self, body: &[u8]) -> Response {
    Response {
      headers: self.headers.clone(),
      body: body.to_vec(),
      status: StatusCode::Ok,
    }
  }

  pub fn _to_string(&mut self) -> String {
    self
      .headers
      .insert("Content-Length".to_string(), self.body.len().to_string());
    self
      .headers
      .insert("Server".to_string(), "Dark Horse".to_string());
    let formatted_body =
      String::from_utf8(self.body.clone()).expect("couldn't parse the body to a string");
    format!(
      "HTTP/1.1 {}\r\n{}\r\n\r\n{}",
      self.status.to_string(),
      self.headers.to_string(),
      formatted_body
    )
  }

  pub fn write(&mut self, buffer: &mut Vec<u8>) {
    self
      .headers
      .insert("Content-Length".to_string(), self.body.len().to_string());
    self
      .headers
      .insert("Server".to_string(), "Dark Horse".to_string());

    let head = format!(
      "HTTP/1.1 {}\r\n{}\r\n\r\n",
      self.status.to_string(),
      self.headers.to_string(),
    );

    let mut body_clone = self.body.clone();
    let mut head = head.as_bytes().to_vec();
    head.append(&mut body_clone);
    let response = head;

    buffer.append(&mut response.to_vec());
  }

  pub fn not_found(&mut self, _body: Option<String>) {
    self.status = StatusCode::NotFound;
    self.body = if let Some(b) = _body {
      b.as_bytes().to_vec()
    } else {
      "The specified resource does not exist.".as_bytes().to_vec()
    }
  }

  pub fn partial_content(&mut self, buffer: &[u8], from: u64, to: u64, size: u64) -> Response {
    self.headers.insert(
      "Content-Range".to_string(),
      format!("bytes {}-{}/{}", from, to, size),
    );
    self
      .headers
      .insert("Accept-Ranges".to_string(), "bytes".to_string());
    self
      .headers
      .insert("Content-Type".to_string(), "video/mp4".to_string());
    Response {
      headers: self.headers.clone(),
      body: buffer.to_vec(),
      status: StatusCode::Ok,
    }
  }
}
