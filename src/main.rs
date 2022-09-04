use packet::packet::C2S;
use packet_parser::parser;
// use packet_serializer::serializer;
use std::cell::Cell;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::client::client_data::{Client, State};
use crate::packet::packet::create_packet;
use crate::packet_parser::parser::IndexedBuffer;

mod client;
mod packet;
mod packet_parser;
//mod packet_serializer;

fn handle_client(mut stream: TcpStream) {
    println!("Connection from {}", stream.peer_addr().unwrap());
    let mut client = Client {
        state: State::HandShaking,
    };

    loop {
        let packet = process_packet(&mut stream, &client.state);

        if let Err(_) = packet {
            println!("Stream closed");
            return;
        }
        println!("Packet {:?}", &packet);

        match packet.unwrap() {
            C2S::Handshake(handshake) => match handshake.next_state {
                1 => {
                    client.state = State::Status;
                }
                2 => client.state = State::Login,
                _ => {
                    println!("Error handling client state")
                }
            },
            C2S::StatusRequest(status_request) => {
                println!("received status request: {:?}", status_request)
            }
            C2S::PingRequest(ping_request) => {
                println!("received ping request: {:?}", ping_request)
            }
            _ => {
                println!("Packet not handled")
            }
        }
    }
}

fn process_packet(stream: &mut TcpStream, state: &State) -> Result<C2S, ()> {
    let length = parser::parse_packet_length(stream)?;

    // Read the packet data and store it in a buffer
    let mut buffer = vec![0u8; length as usize];
    stream.read_exact(&mut buffer).unwrap();

    let indexed_buffer = IndexedBuffer(&buffer, Cell::new(0));

    let packet_type = parser::parse_var_int(&indexed_buffer);

    // println!("Packet type: {}, data: {:?}", &packet_type, &buffer);

    create_packet(&indexed_buffer, &packet_type, state)
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
