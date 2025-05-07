use core::panic;
use std::net::TcpListener;

struct Espresso {
    addr: String,
}

impl Espresso {
    pub fn new(addr: &str) {
        let listener: TcpListener = match TcpListener::bind(addr) {
            Ok(listener) => listener,
            Err(err) => {
                panic!("Error occurred while binding to {addr}");
            }
        };
        
        for request in listener.incoming() {
        }
    }
}
