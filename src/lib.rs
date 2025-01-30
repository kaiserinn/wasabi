use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use request::Request;

pub mod config;
pub mod request;

pub fn listen<T: ToSocketAddrs>(socket_addr: T) {
    let listener = TcpListener::bind(socket_addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    let request = Request::parse(stream);

    println!("{:#?}", request);
}
