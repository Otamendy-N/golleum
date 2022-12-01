use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

const PORT: u16 = 5000;

mod server;
use server::ControllerHandler;
use server::{request::Request, response::Response, Server};

fn main() {
  let mut server = Server::new();

  let main_route_handler: ControllerHandler = Box::new(|_req: Request| {
    let file = fs::read_to_string("public/index.html").unwrap();
    Response::new().ok(file)
  });

  let get_video_handler: ControllerHandler = Box::new(|req: Request| {
    let max_size = 1024 * 1024 * 3;
    let from = parse_range(&req);
    
    let mut video_file = File::open("public/bunny.mp4").unwrap();
    video_file.seek(SeekFrom::Start(from)).unwrap();

    let mut buffer = vec![0; max_size];
    let readed = video_file.read(&mut buffer).unwrap();

    println!("{} bytes were readed", readed);

    let mut res = Response::new();

    let to = from + (buffer.len() - 1) as u64;
    let size = video_file.metadata().unwrap().len() as u64;

    res.partial_content(&buffer, from, to, size)
  });

  server
    .add_get("/", main_route_handler)
    .add_get("/video", get_video_handler);
  server.listen(PORT, &|| {
    println!("[info]: Server listening on port: {}", PORT)
  })
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

fn _get_video(_file_name: &str) -> Vec<u8> {
  let videos_path = "./public/videos/";
  let full_path = format!("{}{}", videos_path, _file_name);
  fs::read(full_path).unwrap()
}
