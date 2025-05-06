use std::fs;
use std::io::BufReader;
use std::net::{ TcpListener, TcpStream };
use std::io::prelude::{ *, BufRead };
mod threads;

fn main() {
    let addr = "127.0.0.1:6969";
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_read = BufReader::new(&stream);
    let request: Vec<String> = buf_read
        .lines()
        .map(|x: Result<String, std::io::Error>| x.unwrap())
        .take_while(|x| !x.is_empty())
        .collect();
    let status_line = "HTTP/1.1 200 NOK";
    let content = fs::read_to_string("index.html").unwrap();
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests;
