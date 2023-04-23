use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    path::PathBuf,
    thread,
};

pub struct Server {
    host: String,
    port: u16,
}

const WWW_DIR: &str = "~/.www";

impl Server {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    fn handle_client(stream: &mut TcpStream) {
        println!("New client: {}", stream.peer_addr().unwrap());

        let mut buf = String::new();
        stream.read_to_string(&mut buf).unwrap();
        println!("Request: {}", buf);

        let buf_split = buf.split_whitespace().collect::<Vec<&str>>();
        let method = buf_split[0];
        let path = buf_split[1];

        match method {
            "GET" => {
                let path = PathBuf::from(WWW_DIR).join(path);
                println!("Path: {}", path.display());
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
