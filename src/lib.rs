use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use request::Request;

pub mod config;
pub mod request;

pub fn serve(config: Config) {
    let localhost: [u8; 4] = [127, 0, 0, 1];
    let addrs = [
        SocketAddr::from((localhost, config.port)),
        SocketAddr::from((localhost, config.port + 1)),
        SocketAddr::from((localhost, config.port + 2)),
    ];
    let listener =
        TcpListener::bind(&addrs[..]).expect("Failed to bind to address");
    println!("Listening on address {:?}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &config.serve_dir);
    }
}

fn handle_connection(stream: TcpStream) {
    let request = Request::parse(stream);

    println!("{:#?}", request);
}
