use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub struct Client {
    host: String,
    port: u16,
}

impl Client {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub fn start(&self, path: String) {
        match TcpStream::connect(format!("{}:{}", self.host, self.port)) {
            Ok(mut stream) => {
                stream
                    .write_all(format!("GET {}\r\n", path).as_bytes())
                    .unwrap();

                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                let response = String::from_utf8_lossy(&buffer[..]).to_string();
                println!("Response: {}", response);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
