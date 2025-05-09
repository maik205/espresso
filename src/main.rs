use std::{ fs, thread };
use std::io::BufReader;
use std::net::TcpStream;
use std::io::prelude::{ *, BufRead };
use std::sync::{ Arc, Mutex };
use std::time::Duration;

use espresso::Espresso;
use threads::stream_threads::ThreadPool;
use threads::TPool;
mod espresso;
mod threads;

fn main() {
    let app = Espresso::new("localhost:4200");
    app.get("/", |request, response| {
        response.write("hello world");
    });



    let pool = ThreadPool::new(2);
    let result = Arc::new(Mutex::new(0));
    let t1 = Arc::clone(&result);
    let t2 = Arc::clone(&result);
    pool.exec(move || {
        thread::sleep(Duration::from_millis(2000));
        *t1.lock().unwrap() += 1;
    });
    pool.exec(move || {
        thread::sleep(Duration::from_millis(1000));
        *t2.lock().unwrap() += 1;
    });
    thread::sleep(Duration::from_millis(2110));
    assert!(*result.lock().unwrap() == 2);
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
