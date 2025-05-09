use core::panic;
use std::{
    collections::HashMap,
    io::{ BufReader, Read },
    net::{ TcpListener, TcpStream },
    pin::Pin,
    sync::Arc,
};

use crate::{ error::EspressoRequestError, request::EspressoRequest, response::EspressoResponse };

pub type RequestHandler = Pin<
    Box<dyn FnMut(EspressoRequest, &mut EspressoResponse) + Send + 'static>
>;

pub struct Espresso {
    addr: String,
    tcp_listener: TcpListener,
    request_handlers: Arc<HashMap<String, Box<[RequestHandler]>>>,
}

type EspressoMiddleware = Box<dyn FnMut(EspressoRequest) + Send + 'static>;

impl Espresso {
    pub fn new(addr: &str) -> Espresso {
        let tcp_listener: TcpListener = match TcpListener::bind(addr) {
            Ok(listener) => listener,
            Err(err) => {
                panic!("Error occurred while binding to {addr}");
            }
        };
        Espresso {
            addr: addr.to_string(),
            tcp_listener,
            request_handlers: Arc::new(HashMap::new()),
        }
    }
    fn handle_request(mut stream: TcpStream) {}

    pub fn get(
        &mut self,
        pattern: &str,
        request_handler: impl (FnMut(&EspressoRequest, &mut EspressoResponse) -> ()) +
            Send +
            'static
    ) -> () {}
    pub fn post(
        &self,
        pattern: &str,
        request_handler: impl (FnMut(&EspressoRequest, &mut EspressoResponse) -> ()) +
            Send +
            'static
    ) -> () {}
    pub fn put(
        &self,
        pattern: &str,
        request_handler: impl (FnMut(&EspressoRequest, &mut EspressoResponse) -> ()) +
            Send +
            'static
    ) -> () {}
    pub fn delete(
        &self,
        pattern: &str,
        request_handler: impl (FnMut(&EspressoRequest, &mut EspressoResponse) -> ()) +
            Send +
            'static
    ) -> () {
        self.common(request_handler);
    }

    fn common(
        &self,
        request_handler: impl (FnMut(&EspressoRequest, &mut EspressoResponse) -> ()) +
            Send +
            'static
    ) {}
    // Can't use use because it is a Rust language word.
    pub fn middleware(&mut self, middleware: EspressoMiddleware) -> () {}
}
