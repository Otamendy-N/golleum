use std::collections::HashMap;


pub enum StatusCode {
  Ok = 200,
  NotFound = 404,
  BadRequest = 400,
  Partial = 206,
  Unauthorized = 401,
  ServerError = 500,
  Created = 201,
  NoContent = 204,
}

impl StatusCode {
    pub fn to_string(&self)-> String {
        match self {
            StatusCode::Ok => "200 Ok".to_string(),
            StatusCode::NotFound => "404 Not Found".to_string(),
            StatusCode::BadRequest => "400 Bad Request".to_string(),
            StatusCode::Partial => "206 Partial".to_string(),
            StatusCode::Unauthorized => "401 Unauthorized".to_string(),
            StatusCode::ServerError => "500 Internal Server Error".to_string(),
            StatusCode::Created => "201 Created".to_string(),
            StatusCode::NoContent => "204 No Content".to_string(),
        }
    }
}

pub struct Response {
  headers: HashMap<String, String>,
  body: String,
  status: StatusCode,
}

impl Response {
  pub fn new() -> Response {
    let headers: HashMap<String, String> = HashMap::new();
    let body = String::new();
    Response { headers, body, status: StatusCode::NotFound }
  }
  pub fn Ok(&mut self, body: String) -> &Self {
    self.status = StatusCode::Ok;
    self.body = body;
    self
  }
  pub fn as_bytes(&self) -> &[u8] {
    let response = format!(
      "{}\r\nContent-Length: {}{}\r\n\r\n{}",
      self.status.to_string(),
      self.body.len(),
      "\r\nServer: Dark Horse",
      self.body
    );
    response.as_bytes()
  }
}