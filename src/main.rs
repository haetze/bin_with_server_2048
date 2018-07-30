#![allow(unused_must_use)]

extern crate ncurses;

use ncurses::*;

use std::net::TcpStream;
use std::net::SocketAddr;
use std::io::prelude::*;
use std::env;


const DEFAULT_PORT: u16 = 4343;
const DEFAULT_SIZE: u8  = 4;


fn main() {
    initscr();
    
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


fn start_game(socket: TcpStream, size: u8) {
    use std::io::BufReader;
    
    let mut socket = BufReader::new(socket);
    let create_command = format!("new {}\n", size);
    socket.get_mut().write(create_command.as_bytes());
    noecho();
    loop {
        clear();
        let mut res = String::new();
        socket.read_line(&mut res);
        print_response(res);
        let mut break_out = false;
        loop {
            let c = getch() as u8 as char;
            let result = match c {
                'r' => socket.get_mut().write(b"right\n"),
                'l' => socket.get_mut().write(b"left\n"),
                'u' => socket.get_mut().write(b"up\n"),
                'd' => socket.get_mut().write(b"down\n"),
                'q' => {
                    break_out = true;
                    socket.get_mut().write(b"exit\n")
                },
                _  => Ok(0),
                
            };
            match result {
                Ok(0) => continue,
                Ok(_) => break,
                _     => {},
            }
         
        }
        if break_out {
            break;
        }
        
    }
    
}

fn print_response(response: String) {
    printw(response.as_str());
    let lines = response.split(';');
    for l in lines {
        if l == "\n" {
            continue;
        }
        let l = l.to_string();
        
        let fields: Vec<&str> =
            l
            .trim_matches(|c| c == ']' || c == '[')
            .split(',')
            .collect();
        printw("| ");
        for field in fields {
            let mut field_string = String::new();
            for c in field.chars() {
                if c.is_digit(10) {
                    field_string.push(c);
                }
            }
            while field_string.len() < 4 {
                field_string.push(' ');
            }
            printw(&field_string);
            printw(" | ");
            
        }   
        printw("\n");
    }
}
