use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                panic!(e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap();
    let mut parts = request_line.split_whitespace();

    // skip the http method
    parts.next().unwrap();

    let path = parts.next().unwrap().trim_start_matches('/');

    let response = format!("{}Hi there, {} !", "HTTP/1.1 200 OK \r\n\r\n", path);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
