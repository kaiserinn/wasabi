use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub url: String,
}

impl Request {
    pub fn parse(stream: TcpStream) -> Self {
        let mut buf_reader = BufReader::new(&stream);
        let mut first_line = String::new();

        if buf_reader.read_line(&mut first_line).is_ok() {
            let status_line: Vec<&str> =
                first_line.split_whitespace().collect();

            if status_line.len() >= 2 {
                return Self {
                    method: status_line[0].to_owned(),
                    url: status_line[1].to_owned(),
                };
            }
        }

        panic!("Invalid request line");
    }
}
