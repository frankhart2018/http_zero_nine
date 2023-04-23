use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    thread,
};

use crate::utils::{get_www_dir, ERR_HTML};

pub struct Server {
    host: String,
    port: u16,
}

impl Server {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    fn handle_client(stream: &mut TcpStream) {
        println!("New client: {}", stream.peer_addr().unwrap());

        // Receive GET PATH
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer[..]).to_string();
        println!("Request: {}", request);

        let request_split = request.split_whitespace().collect::<Vec<&str>>();
        let method = request_split[0];
        let path = request_split[1];

        match method {
            "GET" => {
                let full_path = PathBuf::from(get_www_dir()).join(path);

                if !full_path.exists() {
                    stream
                        .write_all(
                            ERR_HTML
                                .replace("{}", format!("{} not found", path).as_str())
                                .as_bytes(),
                        )
                        .unwrap();
                } else {
                    let mut file = std::fs::File::open(full_path).unwrap();
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    stream.write_all(contents.as_bytes()).unwrap();
                }
            }
            _ => {
                eprintln!("Error: Method not supported");
            }
        }

        println!("Client disconnected: {}", stream.peer_addr().unwrap());
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).unwrap();
        println!("Listening on: {}", listener.local_addr().unwrap());

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    thread::spawn(move || {
                        Server::handle_client(&mut stream);
                    });
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}
