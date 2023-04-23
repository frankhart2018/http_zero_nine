use std::process::exit;

use colored::Colorize;
use sargparse::{ArgumentParser, ArgumentType, InnerData};

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
    parser.add_argument(
        "-p",
        "--path",
        "Path for GET",
        false,
        None,
        ArgumentType::STR,
    );
    parser.add_argument(
        "-h",
        "--host",
        "Host IP",
        false,
        Some(InnerData::STR("127.0.0.1".to_string())),
        ArgumentType::STR,
    );
    parser.add_argument(
        "-o",
        "--port",
        "Port number",
        false,
        Some(InnerData::INT(8080)),
        ArgumentType::INT,
    );

    let args = parser.parse_args().unwrap();

    let server = args.get("server").unwrap().get_bool();
    let client = args.get("client").unwrap().get_bool();
    let path = args.get("path").unwrap().get_str();
    let host = args.get("host").unwrap().get_str();
    let port = args.get("port").unwrap().get_int();

    if !server && !client {
        eprintln!(
            "{} You must specify either server (-s/--server) or client (-c/--client)",
            "Error:".red().bold()
        );
        exit(1);
    } else if server && !client {
        println!("Running server");
        let server = Server::new(host, port as u16);
        server.start();
    } else if !server && client {
        println!("Running client");

        if path == "" {
            eprintln!(
                "{} You must specify a path for GET (-p/--path)",
                "Error:".red().bold()
            );
            exit(1);
        }

        let client = Client::new(host, port as u16);
        client.start(path);
    } else {
        eprintln!(
            "{} You cannot specify both server (-s/--server) and client (-c/--client)",
            "Error:".red().bold()
        );
        exit(1);
    }
}
