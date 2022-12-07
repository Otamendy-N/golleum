use std::fs;

use lib::{
  controller::{Controller, Endpoint, EndpointHandler},
  http_methods::Method, response::Response, request::Request,
};

pub struct MainController {}

impl MainController {
  pub fn get(_req: Request) -> Response {
    let file = fs::read_to_string("server/public/index.html").unwrap();
    Response::ok(file)
  }
}

impl Controller for MainController {
  fn get_route(&self) -> String {
    "/".to_string()
  }

  fn get_endpoints(&self) -> Vec<Endpoint> {
    let mut endpoints = Vec::new();
    let handler: EndpointHandler = Box::new(MainController::get.clone());
    endpoints.push(Endpoint::new(Method::Get, "", handler));
    endpoints
  }
}
