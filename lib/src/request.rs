use std::collections::HashMap;

use crate::http_methods::Method;

use self::query::Query;
use self::header::{HeaderMap, RequestHeader};

mod header;
mod query;

pub struct Request {
  pub query: Query,
  pub method: Method,
  pub headers: HeaderMap,
  pub body: String,
}

impl Request {
  pub fn new() -> Self {
    Request {
      query: Query::new(),
      method: Method::Get,
      headers: HashMap::new(),
      body: String::new(),
    }
  }
  pub fn read_from_buffer(&mut self, buffer: &[u8]) {
    let request = String::from_utf8_lossy(buffer).to_string();

    request.split("\r\n").for_each(|s| {
      if s.contains("HTTP/1.1") {
        self.method = Method::parse(s);
        self.query = Query::parse(s);
        return;
      }
      let (key, value) = HeaderMap::parse_header(s);
      self.headers.insert(key, value);
    });
  }
}
