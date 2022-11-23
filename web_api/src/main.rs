use std::fs;

const PORT: u16 = 5000;

mod server;
use server::ControllerHandler;
use server::Request;
use server::Response;
use server::Server;

fn main() {
  let mut server = Server::new();

  let main_route_handler: ControllerHandler = Box::new(|_req: Request, res: Response| {
    println!("someone enter");
    res
  });

  server.add_get("/", main_route_handler);
  server.listen(PORT, &|| {
    println!("[info]: Server listening on port: {}", PORT)
  })
}

fn _get_video(_file_name: &str) -> Vec<u8> {
  let videos_path = "./public/videos/";
  let full_path = format!("{}{}", videos_path, _file_name);
  fs::read(full_path).unwrap()
}