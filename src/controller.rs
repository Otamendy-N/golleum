use crate::{http_methods::Method, request::Request, response::Response};

pub type EndpointHandler = Box<(dyn Fn(Request) -> Response + 'static)>;

pub struct Endpoint {
  pub route: String,
  pub method: Method,
  pub handler: EndpointHandler,
}

impl Endpoint {
  pub fn new(method: Method, route: &str, handler: EndpointHandler) -> Self {
    Endpoint {
      route: route.to_string(),
      method,
      handler,
    }
  }
}

pub struct ControllerBase {
  pub route: String,
  pub endpoints: Vec<Endpoint>,
}

pub trait Controller {
  fn get_route(&self) -> String;
  fn get_endpoints(&self) -> Vec<Endpoint>;
}
