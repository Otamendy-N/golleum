use controllers::{video_controller::VideoController, main_controller::MainController};
use lib::server::Server;

const PORT: u16 = 5000;

mod controllers;

fn main() {
  let mut server = Server::new();

  server
    .add_controller(Box::new(VideoController{}))
    .add_controller(Box::new(MainController{}));

  server.listen(PORT, &|| {
    println!("[info]: Server listening on port: {}", PORT)
  })
}
