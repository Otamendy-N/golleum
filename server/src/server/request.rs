use std::{collections::HashMap, fmt::format, str::FromStr};

use super::Method;

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

pub struct Request {
  pub route: String,
  pub method: Method,
  pub headers: HeaderMap,
}

impl Request {
  pub fn new() -> Self {
    Request {
      headers: HashMap::new(),
      route: "/".to_owned(),
      method: Method::Get,
    }
  }
  pub fn read_from_buffer(&mut self, buffer: &[u8]) {
    let request = String::from_utf8_lossy(buffer).to_string();

    request.split("\r\n").for_each(|s| {
      if s.contains("HTTP/1.1") {
        println!("{s}");
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
        return;
      }
      let parts = s.split(": ").map(|s| s).collect::<Vec<&str>>();
      let key = parts.get(0).unwrap_or(&"no data").trim().to_string();
      let value = parts.get(1..).map(|s| s.join("")).unwrap_or_default();
      self.headers.insert(key, value);
    });
    let headers = self.headers.clone();
    let headers: Vec<String> = headers
      .iter()
      .map(|(k, v)| format!("({k} === {v})\r\n"))
      .collect();
    println!("{}", headers.join(""));
  }
}
