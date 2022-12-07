use std::{
  io::{Read, Write},
  net::{TcpListener, TcpStream},
};

use crate::{controller::Controller, request::Request, response::Response};

pub struct Server {
  controllers: Vec<Box<dyn Controller>>,
}

impl Server {
  pub fn new() -> Server {
    let controllers = Vec::new();

    Server { controllers }
  }

  pub fn add_controller(&mut self, controller: Box<dyn Controller>) -> &mut Self {
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
    let mut request = Request::new();
    let mut req_buffer = vec![0; 1024 * 1024 * 4];
    stream.read(&mut req_buffer).unwrap();
    request.read_from_buffer(&req_buffer);
    // println!("{}", String::from_utf8(req_buffer).unwrap().to_string());

    let controller = self
      .controllers
      .iter()
      .find(|c| request.query.path().starts_with(&c.get_route()));

    let endpoint = match controller {
      Some(c) => {
        let endpoints = c.get_endpoints();
        let response = endpoints.into_iter().find(|e| {
          let path = format!("{}{}", c.get_route(), e.route);
          path == request.query.path() && e.method == request.method
        });
        response
      }
      None => None,
    };

    let mut response = match endpoint {
      Some(e) => e.handler.as_ref()(request),
      None => Response::not_found(None),
    };

    // println!("{}", response.to_string());
    let mut res_buffer = Vec::new();
    response.write(&mut res_buffer);

    stream.write(&res_buffer).unwrap();
    stream.flush().unwrap();
  }
}
