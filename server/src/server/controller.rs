use super::{Method, ControllerHandler};


pub struct Controller {
  pub route: String,
  pub handler: ControllerHandler,
}
impl Controller {
  pub fn new(method: Method, route: String, handler: ControllerHandler) -> Controller {
    let mut _method_str = "";
    match method {
      Method::GET => _method_str = "GET",
    }
    let route = format!("{} {} HTTP/1.1\r\n", _method_str, route);
    Controller { route, handler }
  }
}