use std::collections::HashMap;

pub trait Header {
  fn to_string(&self) -> String;
}

pub type HeaderMap = HashMap<String, String>;

impl Header for HeaderMap {
  fn to_string(&self) -> String {
    let result = self
      .into_iter()
      .map(|(key, value)| format!("{}: {}", key, value))
      .reduce(|prev, current| format!("{}\r\n{}", prev, current));
    match result {
      Some(s) => s,
      None => "".to_string(),
    }
  }
}
