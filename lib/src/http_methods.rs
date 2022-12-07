#[derive(PartialEq)]
pub enum Method {
  Get,
  Post,
  Patch,
  Delete,
  Put
}

impl Method {
  pub fn parse(s: &str) -> Self {
    let method = if s.starts_with("DELETE") {
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
    method
  }
}