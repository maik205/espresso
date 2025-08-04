use std::{ collections::HashMap, net::TcpStream };
pub struct EspressoResponse {
    pub status: usize,
    pub message: String,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl EspressoResponse {
    pub fn status(&mut self, status: usize) {
        self.status = status;

        match status {
            200 => {
                self.message = "OK".to_string();
            }
            400 => {
                self.message = "BAD REQUEST".to_string();
            }
            _ => {}
        }
    }

    pub fn send(&mut self, message: &str) {
        self.body.push_str(message);
    }

    pub fn set_header(&mut self, header_name: &str, header_value: &str) {
        self.headers.insert(header_name.to_string(), header_value.to_string());
    }
}
pub struct ResponseWriter {
    buffer: Vec<u8>,
    tcp_stream: TcpStream,
}

pub trait Write {
    fn write_bytes(&mut self, bytes: &[u8]);
    fn write_str(&mut self, str: &str);
    fn write_string(&mut self, string: String);
}

impl<'a> ResponseWriter {
    pub fn new(stream: TcpStream) -> ResponseWriter {
        ResponseWriter { buffer: Vec::new(), tcp_stream: stream }
    }

    pub fn flush(&mut self) -> Result<usize, EspressoResponseError> {
        match std::io::Write::write_all(&mut self.tcp_stream, self.buffer.as_ref()) {
            Ok(()) => {
                let wrote = self.buffer.len();
                self.buffer.clear();
                Ok(wrote)
            }
            Err(_) => Err(EspressoResponseError::BufferError),
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn write_response(&mut self, response: EspressoResponse) {
        self.write_string(format!("HTTP/2 {} {}\r\n", response.status, response.message));
        if let None = response.headers.get("CONTENT-LENGTH") {
            self.write_string(format!("Content-Length: {}", response.body.len()));
        }
        for (head_name, head_content) in response.headers {
            self.write_string(format!("{}: {}\r\n", head_name, head_content));
        }
        self.write_str("\r\n");
        self.write_str(&response.body);
        self.flush();
        self.clear();
    }
}

impl EspressoResponse {
    pub fn new() -> EspressoResponse {
        EspressoResponse {
            status: 200,
            message: "OK".to_string(),
            body: "".to_string(),
            headers: HashMap::new(),
        }
    }
}

pub trait Serialize {
    fn get(&self) -> &[u8];
}

impl Write for ResponseWriter {
    fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes.to_owned() {
            self.buffer.push(byte);
        }
    }

    fn write_str(&mut self, str: &str) {
        let bytes = str.bytes();
        for byte in bytes {
            self.buffer.push(byte);
        }
    }

    fn write_string(&mut self, string: String) {
        let bytes = string.bytes();
        for byte in bytes {
            self.buffer.push(byte);
        }
    }
}

impl Serialize for ResponseWriter {
    fn get(&self) -> &[u8] {
        return &self.buffer;
    }
}

pub enum EspressoResponseError {
    BufferError,
    Unknown,
}
