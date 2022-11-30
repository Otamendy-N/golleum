use std::fs;

const PORT: u16 = 5000;

mod server;
use server::ControllerHandler;
use server::Request;
use server::{response::Response, Server};

fn main() {
  let mut server = Server::new();

  let main_route_handler: ControllerHandler = Box::new(|_req: Request| {
    let file = fs::read_to_string("public/index.html").unwrap();
    Response::new().ok(file)
  });

  let get_video_handler: ControllerHandler = Box::new(|_req: Request|{
    let video = fs::read("public/bunny.mp4").expect("couldn't read the file");
    let res = Response::new();

    let from = 0;
    let to = (video.len() - 1) as u64;

    res.partial_content(&video, from, to, video.len())
  });


  server
    .add_get("/", main_route_handler)
    .add_get("/video", get_video_handler);
  server.listen(PORT, &|| {
    println!("[info]: Server listening on port: {}", PORT)
  })
}

fn _get_video(_file_name: &str) -> Vec<u8> {
  let videos_path = "./public/videos/";
  let full_path = format!("{}{}", videos_path, _file_name);
  fs::read(full_path).unwrap()
}