use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

fn main() -> io::Result<()> {
    task::block_on(async {
        let listener = TcpListener::bind("0.0.0.0:8080").await?;
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            task::spawn(async {
                handle_client(stream).await.unwrap();
            });
        }
        Ok(())
    })
}

async fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).await?;

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap();
    let mut parts = request_line.split_whitespace();

    // skip the http method
    parts.next().unwrap();

    let path = parts.next().unwrap().trim_start_matches('/');

    let response = format!("{}Hi there, {} !", "HTTP/1.1 200 OK \r\n\r\n", path);

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}
