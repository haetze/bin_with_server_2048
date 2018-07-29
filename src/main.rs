use std::net::TcpStream;
use std::net::SocketAddr;
use std::io::prelude::*;
use std::env;

const default_port: u16 = 4343;

fn main() {
    let port_requested: u16 = match env::args().skip(1).next() {
        Some(p) => match p.parse() {
            Ok(port) => port,
            Err(_)   => default_port,
        },
        None    => default_port,
    };
    let mut stream = TcpStream::connect(SocketAddr::from(([127, 0, 0, 1], port_requested))).unwrap();

    
}
