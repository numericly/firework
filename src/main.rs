use packet_parser::parser;
use std::cell::Cell;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

use crate::client::client_data::{Client, State};
use crate::packet::packet::create_packet;
use crate::packet_parser::parser::IndexedBuffer;

mod client;
mod packet;
mod packet_parser;

fn handle_client(mut stream: TcpStream) {
    println!("Connection from {}", stream.peer_addr().unwrap());
    let client = Client {
        state: State::HandShaking,
    };

    loop {
        process_packet(&mut stream, &client.state).unwrap();
    }
}

fn process_packet(stream: &mut TcpStream, state: &State) -> Result<(), std::io::Error> {
    let length = parser::parse_packet_length(stream)?;

    // Read the packet data and store it in a buffer
    let mut buffer = vec![0u8; length as usize];
    stream.read_exact(&mut buffer).unwrap();

    let indexed_buffer = IndexedBuffer(&buffer, Cell::new(0));

    let packet_type = parser::parse_var_int(&indexed_buffer);

    let packet = create_packet(&indexed_buffer, &packet_type, state).unwrap();

    println!("Packet: {:?}", packet);

    Ok(())
}

#[tokio::main]
async fn main() {
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
