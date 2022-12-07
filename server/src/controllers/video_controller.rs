use std::{
  fs::{self, File},
  io::{Read, Seek, SeekFrom},
};

use lib::{
  controller::{Controller, Endpoint, EndpointHandler},
  http_methods::Method,
  request::Request,
  response::Response,
};

pub struct VideoController {}

impl VideoController {
  pub fn get_video(req: Request) -> Response {
    let max_size = 1024 * 1024 * 3;
    let from = parse_range(&req);

    let mut video_file = File::open("server/public/bunny.mp4").unwrap();
    video_file.seek(SeekFrom::Start(from)).unwrap();
    let size = video_file.metadata().unwrap().len() as u64;

    let max_size = if size - from > max_size {size-from}else {max_size};
    let mut buffer = vec![0; max_size as usize];
    let readed = video_file.read(&mut buffer).unwrap();

    println!("{} bytes were readed", readed);

    let to = from + (buffer.len() - 1) as u64;

    Response::partial_content(&buffer, from, to, size)
  }
}

impl Controller for VideoController {
  fn get_route(&self) -> String {
    "/videos".to_string()
  }

  fn get_endpoints(&self) -> Vec<Endpoint> {
    let mut endpoints = Vec::new();
    let handler:EndpointHandler = Box::new(VideoController::get_video.clone());
    endpoints.push(Endpoint::new(Method::Get, "/get-video", handler));
    endpoints
  }
}

fn parse_range(req: &Request) -> u64 {
  let mut value = req.headers.get("Range").unwrap().split("=");
  value.next();
  let value = value.next().unwrap();
  let start = value
    .get(0..(value.len() - 1))
    .unwrap()
    .parse::<u64>()
    .unwrap();
  println!("{} bytes were requested", start);
  start
}