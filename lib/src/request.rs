use std::collections::HashMap;

use crate::http_methods::Method;

type HeaderMap = HashMap<String, String>;
trait RequestHeader {
  fn get(&self, key: &str) -> Option<String>;
}

impl RequestHeader for HeaderMap {
  fn get(&self, key: &str) -> Option<String> {
    let result = self.get(key).unwrap();
    Some(result.to_owned())
  }
}

type QueryParamsMap = HashMap<String, String>;

pub struct Request {
  pub path: String,
  pub query_params: QueryParamsMap,
  pub method: Method,
  pub headers: HeaderMap,
  pub body: String,
}

impl Request {
  pub fn new() -> Self {
    Request {
      headers: HashMap::new(),
      path: "/".to_owned(),
      query_params: HashMap::new(),
      method: Method::Get,
      body: String::new(),
    }
  }
  pub fn read_from_buffer(&mut self, buffer: &[u8]) {
    let request = String::from_utf8_lossy(buffer).to_string();

    request.split("\r\n").for_each(|s| {
      if s.contains("HTTP/1.1") {
        self.parse_method(s);
        self.parse_path(s);
        self.parse_query_params(s);
        return;
      }
      let parts = s.split(": ").map(|s| s).collect::<Vec<&str>>();
      let key = parts.get(0).unwrap_or(&"no data").trim().to_string();
      let value = parts.get(1..).map(|s| s.join("")).unwrap_or_default();
      self.headers.insert(key, value);
    });
  }

  pub(crate) fn parse_query(s: &str) -> String {
    let q = s
      .split(" ")
      .filter(|part| part.starts_with("/"))
      .map(|str| urlencoding::decode(str).unwrap().to_string())
      .collect::<Vec<String>>()
      .first()
      .unwrap()
      .to_owned();
    q
  }
  fn parse_query_params(&mut self, s: &str) {
    let q = Request::parse_query(s);
    let mut query = q.split("?");
    query.next();
    if let Some(params) = query.next() {
      let params = params
        .split("&")
        .filter_map(parse_param)
        .collect::<HashMap<String, String>>();

      self.query_params = params;
    }
  }

  fn parse_path(&mut self, s: &str) {
    let q = Request::parse_query(s);
    let mut query = q.split("?");
    self.path = query.next().unwrap().to_string();
  }

  fn parse_method(&mut self, s: &str) {
    self.method = if s.starts_with("DELETE") {
      Method::Delete
    } else if s.starts_with("POST") {
      Method::Post
    } else if s.starts_with("PUT") {
      Method::Put
    } else if s.starts_with("PATCH") {
      Method::Patch
    } else {
      Method::Get
    };
  }
}

fn parse_param(param: &str) -> Option<(String, String)> {
  let mut param = param.split("=");
  let key = param.next().unwrap().to_owned();
  if let Some(value) = param.next() {
    return Some((key, value.to_owned()));
  }
  return None;
}
