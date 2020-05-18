use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::thread;
use std::fmt;
use std::time::Instant;

use chrono;


pub struct Request {
    pub method: String,
    pub path: String
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "METHOD: `{}` PATH: `{}`", self.method, self.path)
    }
}


fn handle_read(mut stream: &TcpStream) -> Option<Request> {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(0) => {
            // println!("Connection closed...?");
            None
        },
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            match req_str.split("\n").next() {
                Some(first_line) => {
                    let request_params : Vec<&str> = first_line.split(" ").collect();
                    match request_params.as_slice() {
                        [] => {
                            println!("request empty...");
                            None
                        },
                        [method] => {
                            println!("METHOD {} but route empty...", method);
                            None
                        },
                        [method, path, ..] => Some(Request { method: method.to_string(), path: path.to_string() })
                    }
                },
                None => {
                    println!("error reading first line of request");
                    None
                }
            }
        },
        Err(e) => {
            println!("Unable to read stream: {}", e);
            None
        },
    }
}

fn handle_write(mut stream: TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/json; charset=UTF-8\r\n\r\n{\"value\":\"pong\"}\r\n";
    // match
    stream.write(response).ok();
    // {
    //     Ok(n) =>  println!("Response sent: {} bytes", n),
    //     Err(e) =>  println!("Failed sending response: {}", e),
    // }
}

fn handle_client(stream: TcpStream) {
    let start = Instant::now();
    match handle_read(&stream) {
        Some(req) => {
            handle_write(stream);
            println!("[INFO] {:?} | 200 | {:3}µs | {} {} ", chrono::Utc::now(), start.elapsed().as_nanos() / 1000, req.method, req.path);
        },
        None => {
            // println!("[WARN] {:?} empty stream...", chrono::Utc::now());
        }
    }

}

fn main() {
    let port = "8080";
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    println!("\nlistening for connections on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // handle_client(stream);
                thread::spawn(|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
