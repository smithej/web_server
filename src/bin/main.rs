use std::fs;
use std::io::prelude::*;
use std::io;
use std::net::TcpListener;
use std::net::TcpStream;

use web_server::ThreadPool;

fn main() {
    let local_host = String::from("127.0.0.1");
    let port_number = String::from("7878");
    let host = format!("{}:{}", local_host, port_number);

    let listener = TcpListener::bind(host.clone());
    match listener {
        Ok(listener) => {
            let pool = ThreadPool::new(4);

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        pool.execute(|| {
                            handle_connection(stream);
                        });
                    },
                    Err(e) => println!("Failed to open TcpStream: '{}'", e)
                }
            }

            println!("Shutting down.");
        },
        Err(e) => println!("Failed to bind to address '{}': '{}'", host, e)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = vec![0; 2048];
    // TODO: Properly handle the error.
    stream.read(&mut buffer).unwrap();
    
    let get = "GET";

    let line = String::from_utf8(buffer).unwrap();
    let (status_line, filename) = if line.starts_with(get) {
        let mut line_iter = line.split_whitespace();
        line_iter.next();
        let full_path = line_iter.next().unwrap();
        let path = &full_path[1..];
        ("HTTP/1.1 200 OK\r\n\r\n", path)
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = match get_file("html", filename) {
        Ok(contents) => contents,
        Err(_) => match get_file("html", "404.html") {
            Ok(contents) => contents,
            Err(error) => {
                println!("404 html file not found. Default response returned instead. {}", error);
                String::from("404")
            }
        },
    };

    let response = format!("{}{}", status_line, contents);

    // TODO: Properly handle the error.
    stream.write(response.as_bytes()).unwrap();
    // TODO: Properly handle the error.
    stream.flush().unwrap();
}

fn get_file(html_root: &str, path: &str) -> io::Result<String> {
    let full_path = format!("{}/{}", html_root, path);
    fs::read_to_string(full_path)
}
