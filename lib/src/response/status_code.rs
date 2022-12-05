pub enum StatusCode {
  Ok = 200,
  NotFound = 404,
  BadRequest = 400,
  PartialContent = 206,
  Unauthorized = 401,
  ServerError = 500,
  Created = 201,
  NoContent = 204,
}

impl StatusCode {
  pub fn to_string(&self) -> String {
    match self {
      StatusCode::Ok => "200 Ok".to_string(),
      StatusCode::NotFound => "404 Not Found".to_string(),
      StatusCode::BadRequest => "400 Bad Request".to_string(),
      StatusCode::PartialContent => "206 Partial Content".to_string(),
      StatusCode::Unauthorized => "401 Unauthorized".to_string(),
      StatusCode::ServerError => "500 Internal Server Error".to_string(),
      StatusCode::Created => "201 Created".to_string(),
      StatusCode::NoContent => "204 No Content".to_string(),
    }
  }
}