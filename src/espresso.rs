use core::panic;
use std::{
    collections::HashMap,
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use crate::{
    error::EspressoProcessingError,
    request::{EspressoRequest, EspressoStream, RequestMethod},
    response::EspressoResponse,
    threads::{stream_threads::ThreadPool, TPool},
};

pub type RequestHandler =
    Box<dyn Fn(&EspressoRequest, &mut EspressoResponse) + Send + Sync + 'static>;
pub type MethodHandlers = HashMap<String, Arc<RequestHandler>>;
pub struct Espresso {
    tcp_listener: TcpListener,
    /// HM of Request Type => Pattern => Route handler
    method_handlers: HashMap<RequestMethod, MethodHandlers>,
    thread_pool: ThreadPool,
    global_handlers: MethodHandlers,
    internal: Option<Arc<EspressoInternal>>,
}

/// Internal struct to hold ownership of the methods available to be after a `listen()` call.
/// This is for cross-thread access purposes. We do not need mutability of the variables after `listen()`
struct EspressoInternal {
    all: Box<[(String, Arc<RequestHandler>)]>,
    methods: HashMap<RequestMethod, HashMap<String, Arc<RequestHandler>>>,
}

type EspressoMiddleware = Box<dyn FnMut(EspressoRequest) + Send + 'static>;

impl Espresso {
    pub fn new(addr: &str) -> Espresso {
        let tcp_listener: TcpListener = match TcpListener::bind(addr) {
            Ok(listener) => listener,
            Err(_) => {
                panic!("Error occurred while binding to {addr}");
            }
        };
        Espresso {
            tcp_listener,
            method_handlers: HashMap::new(),
            thread_pool: ThreadPool::new(100),
            global_handlers: HashMap::new(),
            internal: None,
        }
    }
    pub fn all(
        &mut self,
        pattern: &str,
        request_handler: impl Fn(&EspressoRequest, &mut EspressoResponse) + Send + Sync + 'static,
    ) {
        self.global_handlers
            .insert(pattern.to_string(), Arc::new(Box::new(request_handler)));
    }

    fn register_fn_handler(
        &mut self,
        pattern: &str,
        method: RequestMethod,
        handle_func: impl (FnMut(&EspressoRequest, &mut EspressoResponse) -> ()) + Send + 'static,
    ) {
    }

    pub fn listen(&mut self) {
        self.internal = Some(Arc::new(EspressoInternal {
            all: {
                let mut global_handles: Vec<(String, Arc<RequestHandler>)> = Vec::new();
                let global_handler_map = self.global_handlers.clone();
                for (key, value) in global_handler_map {
                    global_handles.push((key, value));
                }
                global_handles.as_slice().into()
            },
            methods: self.method_handlers.clone(),
        }));
        for stream in self.tcp_listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_stream(stream);
                }
                Err(_) => {
                    println!("Error during handshake.");
                }
            }
        }
    }

    pub fn handle_stream(&self, tcp_stream: TcpStream) -> Result<(), EspressoProcessingError> {
        let i = Arc::clone(match &self.internal {
            Some(reference) => reference,
            None => {
                return Err(EspressoProcessingError::HandleBeforeListen);
            }
        });

        self.thread_pool.exec(move || {
            let mut stream = EspressoStream::new(tcp_stream);
            // Cook up a new response in the thread
            let global_handlers = &i.all;
            loop {
                if let Some(frame) = stream.next() {
                    let request = frame.request;
                    let mut response = EspressoResponse::new();

                    for (l, handle_fn) in global_handlers {
                        if request.resource.eq(l) {
                            handle_fn(&request, &mut response);
                        }
                    }
                    match request.method {
                        RequestMethod::GET => {
                            let rwrite = &mut stream.writer;
                            rwrite.write_response(response);
                        }
                        RequestMethod::POST => todo!(),
                        RequestMethod::PUT => todo!(),
                        RequestMethod::DELETE => todo!(),
                        RequestMethod::PATCH => todo!(),
                        RequestMethod::OPTIONS => todo!(),
                        RequestMethod::HEAD => todo!(),
                    }
                } else {
                    break;
                }
            }
        });
        Ok(())
    }

    // Can't use `use` because it is a Rust language word.
    // pub fn middleware(&mut self, middleware: EspressoMiddleware) -> () {}
}
