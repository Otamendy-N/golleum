use crate::{http_methods::Method, request::Request, response::Response};

pub type ControllerHandler = Box<(dyn Fn(Request) -> Response + 'static)>;

pub struct Controller {
    pub route: String,
    pub method: Method,
    pub handler: ControllerHandler,
}

impl Controller {
    pub fn new(method: Method, route: String, handler: ControllerHandler) -> Controller {
        let route = format!("{} {} HTTP/1.1\r\n", "GET", route);
        Controller {
            route,
            method,
            handler,
        }
    }
}
