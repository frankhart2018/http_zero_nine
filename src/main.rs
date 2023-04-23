use std::process::exit;

use colored::Colorize;
use sargparse::{ArgumentParser, ArgumentType};

use http_zero_nine::{client::Client, server::Server};

fn main() {
    let mut parser = ArgumentParser::new(Some("HTTP/0.9 Client/Server"));
    parser.add_argument(
        "-s",
        "--server",
        "Run a server",
        false,
        None,
        ArgumentType::BOOL,
    );
    parser.add_argument(
        "-c",
        "--client",
        "Run a client",
        false,
        None,
        ArgumentType::BOOL,
    );

    let args = parser.parse_args().unwrap();

    let server = args.get("server").unwrap().get_bool();
    let client = args.get("client").unwrap().get_bool();

    if !server && !client {
        eprintln!(
            "{} You must specify either server (-s/--server) or client (-c/--client)",
            "Error:".red().bold()
        );
        exit(1);
    } else if server && !client {
        println!("Running server");
        let server = Server::new("127.0.0.1".to_string(), 8080);
        server.start();
    } else if !server && client {
        println!("Running client");
        let client = Client::new("127.0.0.1".to_string(), 8080);
        client.start("index.html".to_string());
    } else {
        eprintln!(
            "{} You cannot specify both server (-s/--server) and client (-c/--client)",
            "Error:".red().bold()
        );
        exit(1);
    }
}
