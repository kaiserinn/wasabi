use std::net::TcpListener;

use clap::Parser;
use wasabi::config::Config;

fn main() {
    let config = Config::parse();

    let listener = TcpListener::bind(("localhost", config.port)).unwrap();
    println!("Listening on port {}", config.port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("{:#?}", stream);
        println!("Connection established!");
    }
}
