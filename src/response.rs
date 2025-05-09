pub struct EspressoResponse {}

pub struct ResponseWriter {
    bytes: Vec<u8>,
}

pub trait Write {
    fn write_bytes(&mut self, bytes: &[u8]);
    fn write_str(&mut self, str: &str);
    fn write_string(&mut self, string: String);
}

pub trait Serialize {
    fn get(&self) -> &[u8];
}

impl Write for ResponseWriter {
    fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes.to_owned() {
            self.bytes.push(byte);
        }
    }

    fn write_str(&mut self, str: &str) {
        let bytes = str.bytes();
        for byte in bytes {
            self.bytes.push(byte);
        }
    }

    fn write_string(&mut self, string: String) {
        let bytes = string.bytes();
        for byte in bytes {
            self.bytes.push(byte);
        }
    }
}

impl Serialize for ResponseWriter {
    fn get(&self) -> &[u8] {
        return &self.bytes;
    }
}
