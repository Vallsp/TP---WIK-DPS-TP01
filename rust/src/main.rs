use std::env;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut headers = Vec::new();

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);

    for line in request.lines() {
        if line.is_empty() {
            break;
        }
        headers.push(line.to_string());
    }

    let request_line = headers[0].split_whitespace().collect::<Vec<&str>>();
    if request_line.len() != 3 {
        return;
    }
    let method = request_line[0];
    let path = request_line[1];

    if method == "GET" && path == "/ping" {
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{:?}", headers);
        stream.write(response.as_bytes()).unwrap();
    } else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
    }

    stream.flush().unwrap();
}

fn main() {
    let port = env::var("PING_LISTEN_PORT").unwrap_or("8080".to_string());
    let addr = format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(&addr).unwrap();

    println!("Listening on port {}...", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}