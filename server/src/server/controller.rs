use super::{Method, ControllerHandler};


pub struct Controller {
  pub route: String,
  pub method: Method,
  pub handler: ControllerHandler,
}

impl Controller {
  pub fn new(method: Method, route: String, handler: ControllerHandler) -> Controller {
    let route = format!("{} {} HTTP/1.1\r\n", "GET", route);
    Controller { route, method, handler }
  }
}