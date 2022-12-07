use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream}
};

use crate::{
    controller::{Controller, ControllerHandler},
    http_methods::Method,
    request::Request,
    response::Response,
};

pub struct Server {
    controllers: Vec<Controller>,
}

impl Server {
    pub fn new() -> Server {
        let controllers: Vec<Controller> = Vec::new();

        Server { controllers }
    }

    pub fn add_get(&mut self, route: &str, handler: ControllerHandler) -> &mut Self {
        let controller = Controller::new(Method::Get, String::from(route), handler);
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
        println!("{}", String::from_utf8(req_buffer).unwrap().to_string());

        let controller = self
            .controllers
            .iter()
            .find(|c| c.method == request.method && c.route == request.path);

        let mut response = if let Some(c) = controller {
            let handler = c.handler.as_ref();
            handler(request)
        } else {
            let mut res = Response::new();
            res.not_found(None);
            res
        };

        // println!("{}", response.to_string());
        let mut res_buffer = Vec::new();
        response.write(&mut res_buffer);

        stream.write(&res_buffer).unwrap();
        stream.flush().unwrap();
    }
}
