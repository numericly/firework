use packet_parser::parser;
use std::cell::Cell;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

use crate::packet_parser::parser::IndexedBuffer;

mod packet_parser;

fn handle_client(mut stream: TcpStream) {
    println!("Connection from {}", stream.peer_addr().unwrap());

    loop {
        process_packet(&mut stream).unwrap();
    }
}

fn process_packet(stream: &mut TcpStream) -> Result<(), std::io::Error> {
    let length = parser::parse_packet_length(stream)?;

    // Read the packet data and store it in a buffer
    let mut buffer = vec![0u8; length as usize];
    stream.read_exact(&mut buffer).unwrap();

    let indexed_buffer = IndexedBuffer(&buffer, Cell::new(0));

    let packet_type = parser::parse_var_int(&indexed_buffer);

    println!("Packet: (type: {}, content: {:x?})", packet_type, buffer);
    Ok(())
}

#[tokio::main]
async fn main() {
    println!(
        "{:b}",
        parser::parse_signed_int(&IndexedBuffer(&vec!(0xf2, 0x35, 0x12, 0xc3), Cell::new(0)))
    );
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();

    for stream in listener.incoming() {
        tokio::spawn(async move { handle_client(stream.unwrap()) });
    }
}

//length of the packet: 10 (16 bytes)
//packet ID?: 0
//protocol version: f8 5 (760)
//stuff?: 9
//server address: 6c 6f 63 61 6c 68 6f 73 74 (localhost)
//port: 63 dd (25565)
//next state: 2 (login)

//the first bit in each byte of a varint is whether or not the next byte is also part of the varint

// 1 1010011 0 1001010
//0010100111001010
