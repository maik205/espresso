use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::format,
    io::{ BufRead, BufReader, Read },
    marker::PhantomData,
    net::TcpStream,
    sync::Arc,
};
use atoi::atoi;

use crate::{ error::EspressoRequestError, espresso::Espresso, response::ResponseWriter };
#[derive(Clone)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct EspressoStream {
    reader: BufReader<TcpStream>,
    pub writer: ResponseWriter,
    tcp: TcpStream,
}
impl EspressoStream {
    /// Creates a new [`EspressoStream`] wrapping the underlying [`TcpStream`] and provides a [`BufReader`] and [`ResponseWriter`] instance.
    pub fn new(tcp_stream: TcpStream) -> EspressoStream {
        // These references are essentially the same underlying TcpStream.
        let read_stream = tcp_stream.try_clone().expect("The TCP stream was unable to be cloned.");
        let write_stream = tcp_stream.try_clone().expect("The TCP stream was unable to be cloned.");
        EspressoStream {
            reader: BufReader::new(read_stream),
            writer: ResponseWriter::new(write_stream),
            tcp: tcp_stream.try_clone().expect("Unable to clone the TCP stream."),
        }
    }

    pub fn clone(&self) -> EspressoStream {
        EspressoStream {
            reader: BufReader::new(self.tcp.try_clone().unwrap()),
            writer: ResponseWriter::new(self.tcp.try_clone().unwrap()),
            tcp: self.tcp.try_clone().unwrap(),
        }
    }
}

pub struct EspressoStreamFrame {
    pub request: EspressoRequest,
}

impl EspressoStream {
    pub fn next(&mut self) -> Option<EspressoStreamFrame> {
        let body: String = String::new();
        let mut headers: HashMap<String, String> = HashMap::new();

        let mut buf: String = String::new();
        let (method, resource, protocol) = {
            self.reader.read_line(&mut buf).expect("Couldn't read request.");
            let items: Vec<&str> = buf.split(" ").take(3).collect();
            (
                match items[0].to_uppercase().as_str() {
                    "GET" => RequestMethod::GET,
                    "PUT" => RequestMethod::PUT,
                    "POST" => RequestMethod::POST,
                    "DELETE" => RequestMethod::DELETE,
                    _ => {
                        return None;
                    }
                },
                items[1],
                items[2],
            )
        };
        let mut buf: String = String::new();
        // Read headers
        while !buf.is_empty() {
            buf.clear();
            match self.reader.read_line(&mut buf) {
                Err(_) => {
                    return None;
                }
                _ => (),
            }

            let header_parts: Vec<&str> = buf.split(": ").collect();
            headers.insert(header_parts[0].to_uppercase().to_string(), header_parts[1].to_string());
        }
        // Reads body
        if let Some(len_str) = headers.get("CONTENT-LENGTH") {
            if let Some(len) = atoi::<u32>(len_str.as_bytes()) {
                let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
                self.reader.read_exact(&mut buf).unwrap();
            }
        } else {
        }
        let mut cloned_stream = self.clone();
        Some(EspressoStreamFrame {
            request: EspressoRequest {
                headers,
                method,
                resource: resource.to_string(),
                protocol_ver: protocol.to_string(),
                body: Some(body),
            },
        })
    }
}

// impl<'a> Drop for EspressoStream<'a> {
//     fn drop(&mut self) {
//         todo!()
//     }
// }
pub struct EspressoRequest {
    pub headers: HashMap<String, String>,
    pub method: RequestMethod,
    pub resource: String,
    pub protocol_ver: String,
    pub body: Option<String>,
}

impl EspressoRequest {
    pub fn get_header(&self) -> Option<String> {
        Some("".to_string())
    }
}

/// This parses a single request from a byte slice/buffer from a `TcpStream`
impl<'a> TryFrom<&'a [u8]> for EspressoRequest {
    type Error = EspressoRequestError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);
        let mut body: String = String::new();
        let mut headers: HashMap<String, String> = HashMap::new();

        let mut buf: String = String::new();
        let (method, resource, protocol) = {
            reader.read_line(&mut buf);
            let items: Vec<&str> = buf.split(" ").take(3).collect();
            (
                match items[0].to_uppercase().as_str() {
                    "GET" => RequestMethod::GET,
                    "PUT" => RequestMethod::PUT,
                    "POST" => RequestMethod::POST,
                    "DELETE" => RequestMethod::DELETE,
                    _ => {
                        return Err(
                            EspressoRequestError::MalformedRequest(
                                "Request method not supported".to_string()
                            )
                        );
                    }
                },
                items[1],
                items[2],
            )
        };
        let mut buf: String = String::new();
        // Read headers
        while !buf.is_empty() {
            buf.clear();
            let _ = reader.read_line(&mut buf);

            let header_parts: Vec<&str> = buf.split(": ").collect();
            headers.insert(
                header_parts[0].to_ascii_uppercase().to_string(),
                header_parts[1].to_string()
            );
        }
        // Reads body
        buf.clear();
        reader.read_to_string(&mut buf);
        body.push_str(buf.as_str());

        // for (ind, line) in reader.lines().enumerate() {
        //     match line {
        //         Ok(line) => {
        //             if line.is_empty() {
        //                 break;
        //             }
        //             match ind {
        //                 0 => {
        //                     let line = line.to_owned();
        //                     let line_parts: Vec<&str> = line.split(" ").collect();
        //                     method = match line_parts[0].to_uppercase().as_str() {
        //                         "GET" => Some(RequestMethod::GET),
        //                         "POST" => Some(RequestMethod::POST),
        //                         "PUT" => Some(RequestMethod::PUT),
        //                         "DELETE" => Some(RequestMethod::DELETE),
        //                         _ => {
        //                             return Err(
        //                                 EspressoRequestError::MalformedRequest(
        //                                     "Request method not supported.".to_string()
        //                                 )
        //                             );
        //                         }
        //                     };
        //                     resource.push_str(line_parts[1]);
        //                     protocol_ver.push_str(line_parts[2]);
        //                 }
        //                 1.. => {
        //                     if line.is_empty() {
        //                         if break_found {
        //                             break;
        //                         } else {
        //                             break_found = true;
        //                             continue;
        //                         }
        //                     }
        //                     if !break_found {
        //                         let header_parts: Vec<&str> = line.split(": ").collect();
        //                         headers.insert(
        //                             header_parts[0].to_ascii_uppercase().to_string(),
        //                             header_parts[1].to_string()
        //                         );
        //                     } else {
        //                         if let Some(content_length) = headers.get("CONTENT-LENGTH") {
        //                             if let Some(len) = atoi::atoi::<u32>(content_length.as_bytes()) {
        //                                 // Insert the body into the Request data fields
        //                                 line.
        //                             }
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //         Err(_) => {
        //             return Err(
        //                 EspressoRequestError::MalformedRequest(
        //                     "Request is misaligned or incomplete.".to_string()
        //                 )
        //             );
        //         }
        //     }
        // }
        Ok(EspressoRequest {
            headers,
            method,
            resource: resource.to_string(),
            protocol_ver: protocol.to_string(),
            body: Some(body),
        })
    }
}
