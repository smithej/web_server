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

    match stream.read(&mut buffer) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to load request buffer from stream: '{}'", e);
            return;
        }
    };
    
    let get = "GET";

    let line = match String::from_utf8(buffer) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to parse HTTP request line: '{}'", e);
            return;
        }
    };

    let (status_line, filename) = if line.starts_with(get) {
        let mut line_iter = line.split_whitespace();

        // Consume the method of the HTTP request line.
        line_iter.next();

        // Take the request URI of the HTTP request line; return root if none found.
        let full_path = match line_iter.next() {
            Some(item) => item,
            None => "/"
        };
        
        // Discard leading slash. "/my/uri/" -> "my/uri/"
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

    match stream.write(response.as_bytes()) {
        Ok(b) => b,
        Err(e) => 
        {
            println!("Failed to write to stream: '{}'", e);
            0
        }
    };

    match stream.flush() {
        Err(e) => println!("Failed to flush stream: '{}'", e),
        _ => ()
    };
}

fn get_file(html_root: &str, path: &str) -> io::Result<String> {
    let full_path = format!("{}/{}", html_root, path);
    fs::read_to_string(full_path)
}
