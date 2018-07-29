use std::net::TcpStream;
use std::net::SocketAddr;
use std::io::prelude::*;
use std::env;

const DEFAULT_PORT: u16 = 4343;
const DEFAULT_SIZE: u8  = 4;

fn main() {
    let port_requested: u16 = match env::args().skip(2).next() {
        Some(p) => match p.parse() {
            Ok(port) => port,
            Err(_)   => DEFAULT_PORT,
        },
        None    => DEFAULT_PORT,
    };

    let size: u8 = match env::args().skip(1).next() {
        Some(s) => match s.parse() {
            Ok(size) => size,
            Err(_)   => DEFAULT_SIZE,
        },
        None    => DEFAULT_SIZE,
    };
    
    
    let stream = TcpStream::connect(SocketAddr::from(([127, 0, 0, 1], port_requested)));

    match stream {
        Ok(s) => start_game(s, size),
        Err(_) => println!("Game failed to start"),
    }
    
}


fn start_game(mut socket: TcpStream, size: u8) {
    use std::io::BufReader;
    
    let mut socket = BufReader::new(socket);
    let create_command = format!("new {}", size);

    socket.get_mut().write(create_command.as_bytes());
    
}
