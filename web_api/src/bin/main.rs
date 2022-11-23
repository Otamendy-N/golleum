use std::fs;
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    let pool = ThreadPool::new(4);

    println!("[info] Server started at [localhost:5000]");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("a new thread was created!?");

        pool.execute(|| handle_connection(stream));
    }
}

fn get_video(fileName: &str) -> Vec<u8> {
    let full_path = format!(videos_path, fileName);
    fs::read(full_path).unwrap()
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let video = b"GET /video HTTP/1.1\r\n";

    let (status_code, content) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "public/index.html")
    } else if buffer.starts_with(video) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "public/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "public/404.html")
    };

    server.add_route("/", || {
        let video = getVideo("bunny.mp4");
    });

    let page = fs::read_to_string(content).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        page.len(),
        page
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
