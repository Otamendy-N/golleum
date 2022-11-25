use std::{
  fs,
  io::{Read, Write},
  net::{TcpListener, TcpStream},
};

use self::{controller::Controller, response::Response};

mod controller;
pub mod response;

pub type ControllerHandler = Box<(dyn Fn(Request) -> Response + 'static)>;

pub enum Method {
  GET,
}
pub struct Request {}


pub struct Server {
  controllers: Vec<Controller>,
}

impl Server {
  pub fn new() -> Server {
    let controllers: Vec<Controller> = Vec::new();

    Server { controllers }
  }

  pub fn add_get(&mut self, route: &str, handler: ControllerHandler) -> &mut Self {
    let controller = Controller::new(Method::GET, String::from(route), handler);
    self.controllers.push(controller);
    self
  }

  pub fn listen(&mut self, port: u16, _callback: &dyn Fn()) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    // let pool = ThreadPool::new(4);

    _callback();
    for stream in listener.incoming() {
      let stream = stream.unwrap();
      // pool.execute(|| handle_connection(stream));
      self.handle_connection(stream);
    }
  }

  fn handle_connection(&mut self, mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let mut response = Response::new();

    let controller = self
      .controllers
      .iter()
      .find(|c| buffer.starts_with(c.route.as_bytes()));

    let (status_code, content) = match controller {
      Some(c) => {
        let handler = c.handler.as_ref();
        response = handler(Request {});
        ("HTTP/1.1 200 OK", "public/index.html")
      }
      None => ("HTTP/1.1 404 NOT FOUND", "public/404.html"),
    };

    let page = fs::read_to_string(content).unwrap();

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
  }
}
