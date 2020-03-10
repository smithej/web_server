use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use web_server::ThreadPool;

fn main() {
    // TODO: Properly handle the case in which we cannot bind to the address & port.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        // TODO: Properly handle the error.
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = vec![0; 2048];
    // TODO: Properly handle the error.
    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    
    // TODO: Properly handle the error.
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    // TODO: Properly handle the error.
    stream.write(response.as_bytes()).unwrap();
    // TODO: Properly handle the error.
    stream.flush().unwrap();
}
