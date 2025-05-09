use std::{ collections::HashMap, io::{ BufRead, BufReader } };

use crate::error::EspressoRequestError;

pub enum RequestMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct EspressoRequest<'a> {
    headers: HashMap<String, String>,
    method: RequestMethod,
    resource: String,
    protocol_ver: String,
    raw_bytes: &'a [u8],
    body: String,
}

impl EspressoRequest<'_> {
    pub fn bytes(&self) -> &[u8] {
        &self.raw_bytes
    }
    pub fn get_header(&self) -> Option<String> {
        Some("".to_string())
    }
}

impl<'a> TryFrom<&'a [u8]> for EspressoRequest<'a> {
    type Error = EspressoRequestError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let reader = BufReader::new(value);
        let mut method: Option<RequestMethod> = None;
        let mut protocol_ver: String = String::new();
        let mut resource: String = String::new();
        let mut body: String = String::new();
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut break_found: bool = false;
        for (ind, line) in reader.lines().enumerate() {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        break;
                    }
                    match ind {
                        0 => {
                            let line = line.to_owned();
                            let line_parts: Vec<&str> = line.split(" ").collect();
                            method = match line_parts[0].to_uppercase().as_str() {
                                "GET" => Some(RequestMethod::GET),
                                "POST" => Some(RequestMethod::POST),
                                "PUT" => Some(RequestMethod::PUT),
                                "DELETE" => Some(RequestMethod::DELETE),
                                _ => {
                                    return Err(
                                        EspressoRequestError::MalformedRequest(
                                            "Request method not supported.".to_string()
                                        )
                                    );
                                }
                            };
                            resource.push_str(line_parts[1]);
                            protocol_ver.push_str(line_parts[2]);
                        }
                        1.. => {
                            if line.is_empty() {
                                if break_found {
                                    break;
                                } else {
                                    break_found = true;
                                }
                            }
                            if !break_found {
                                let header_parts: Vec<&str> = line.split(": ").collect();
                                headers.insert(
                                    header_parts[0].to_string(),
                                    header_parts[1].to_string()
                                );
                            } else {
                                body.push_str(line.as_str());
                            }
                        }
                    }
                }
                Err(_) => {
                    return Err(
                        EspressoRequestError::MalformedRequest(
                            "Request is misaligned or incomplete.".to_string()
                        )
                    );
                }
            }
        }
        Ok(EspressoRequest {
            headers,
            method: method.unwrap(),
            resource,
            protocol_ver,
            raw_bytes: value,
            body,
        })
    }
}
