use std::collections::HashMap;

pub type HeaderMap = HashMap<String, String>;
pub trait RequestHeader {
  fn get(&self, key: &str) -> Option<String>;
  fn parse_header(s: &str) -> (String, String);
}

impl RequestHeader for HeaderMap {
  fn get(&self, key: &str) -> Option<String> {
    let result = self.get(key).unwrap();
    Some(result.to_owned())
  }

  fn parse_header(s: &str) -> (String, String) {
    let parts = s.split(": ").map(|s| s).collect::<Vec<&str>>();
    let key = parts.get(0).unwrap_or(&"no data").trim().to_string();
    let value = parts.get(1..).map(|s| s.join("")).unwrap_or_default();
    (key, value)
  }
}