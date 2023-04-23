use std::{io::Write, net::TcpStream};

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
                println!("Connected to server: {}", stream.peer_addr().unwrap());

                stream
                    .write(format!("GET {}\r\n", path).to_string().as_bytes())
                    .unwrap();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
