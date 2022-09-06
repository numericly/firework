use packet::c2s_packet::C2S;
use packet_parser::parser::{self, parse_var_int};
use packet_serializer::serializer;
use std::cell::Cell;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::client::client_data::{Client, State};
use crate::packet::c2s_packet::{get_packet, StatusRequest};
use crate::packet::s2c_packet::{PingResponse, S2CPacket, ServerStatus};
use crate::packet_parser::parser::IndexedBuffer;
use crate::packet_serializer::serializer::serialize_var_int;

mod client;
mod packet;
mod packet_parser;
mod packet_serializer;

fn handle_client(mut stream: TcpStream) {
    println!("Connection from {}", stream.peer_addr().unwrap());
    let mut client = Client {
        state: State::HandShaking,
    };

    loop {
        let packet = process_packet(&mut stream, &client.state);

        if let Err(_) = packet {
            println!("Stream Closed by Client");
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
                println!("received status request: {:?}", status_request);

                let mut server_status = ServerStatus {
                    server_data: r#"{"previewsChat":false,"enforcesSecureChat":true,"description":{"text":"\u00a7a<rust-minecraft-server>\u00a7r"},"players":{"max":1000000,"online":0},"version":{"name":"1.19.2","protocol":760}}
                    "#.to_string()
                };

                let data = server_status.write_packet(&mut stream);
            }
            C2S::PingRequest(ping_request) => {
                println!("received ping request: {:?}", ping_request);
                let mut ping_response = PingResponse {
                    payload: ping_request.payload,
                };
                ping_response.write_packet(&mut stream);
            }
            _ => {
                println!("Packet not handled");
            }
        }
    }
}

fn process_packet(stream: &mut TcpStream, state: &State) -> Result<C2S, ()> {
    println!("process_packet called with state: {:?}", state);
    let length = parser::parse_packet_length(stream)?;
    println!("length: {}", length);

    // Read the packet data and store it in a buffer
    let mut buffer = vec![0u8; length as usize];
    stream.read_exact(&mut buffer).unwrap();
    println!("buffer: {:?}", buffer);

    let indexed_buffer = IndexedBuffer(&buffer, Cell::new(0));

    let packet_type = parser::parse_var_int(&indexed_buffer);

    println!("Packet type: {}, data: {:?}", &packet_type, &buffer);

    get_packet(&indexed_buffer, &packet_type, state)
}

#[tokio::main]
async fn main() {
    // let data = vec![
    //     145, 5, 0, 142, 5, 123, 34, 118, 101, 114, 115, 105, 111, 110, 34, 58, 123, 34, 110, 97,
    //     109, 101, 34, 58, 34, 49, 46, 49, 57, 46, 50, 34, 44, 34, 112, 114, 111, 116, 111, 99, 111,
    //     108, 34, 58, 55, 53, 57, 125, 44, 34, 112, 108, 97, 121, 101, 114, 115, 34, 58, 123, 34,
    //     109, 97, 120, 34, 58, 54, 57, 44, 34, 111, 110, 108, 105, 110, 101, 34, 58, 48, 44, 34,
    //     115, 97, 109, 112, 108, 101, 34, 58, 91, 123, 125, 93, 125, 44, 34, 100, 101, 115, 99, 114,
    //     105, 112, 116, 105, 111, 110, 34, 58, 123, 34, 116, 101, 120, 116, 34, 58, 34, 72, 101,
    //     108, 108, 111, 32, 119, 111, 114, 108, 100, 34, 125, 44, 34, 102, 97, 118, 105, 99, 111,
    //     110, 34, 58, 34, 100, 97, 116, 97, 58, 105, 109, 97, 103, 101, 47, 112, 110, 103, 59, 98,
    //     97, 115, 101, 54, 52, 44, 100, 97, 116, 97, 58, 105, 109, 97, 103, 101, 47, 112, 110, 103,
    //     59, 98, 97, 115, 101, 54, 52, 44, 105, 86, 66, 79, 82, 119, 48, 75, 71, 103, 111, 65, 65,
    //     65, 65, 78, 83, 85, 104, 69, 85, 103, 65, 65, 65, 69, 65, 65, 65, 65, 66, 65, 67, 65, 77,
    //     65, 65, 65, 67, 100, 116, 52, 72, 115, 65, 65, 65, 65, 65, 88, 78, 83, 82, 48, 73, 65, 114,
    //     115, 52, 99, 54, 81, 65, 65, 65, 69, 104, 81, 84, 70, 82, 70, 122, 98, 54, 115, 101, 51,
    //     49, 113, 84, 108, 86, 79, 80, 107, 107, 52, 100, 53, 100, 89, 89, 110, 121, 65, 81, 87,
    //     112, 100, 77, 85, 82, 75, 109, 112, 79, 51, 102, 86, 57, 118, 80, 68, 116, 75, 73, 83, 49,
    //     65, 105, 48, 104, 75, 97, 106, 81, 108, 82, 83, 56, 112, 120, 97, 82, 114, 110, 50, 99, 43,
    //     100, 86, 69, 112, 83, 85, 69, 112, 118, 52, 49, 51, 101, 108, 120, 85, 77, 67, 115, 108,
    //     73, 104, 111, 77, 67, 65, 81, 65, 120, 56, 104, 119, 76, 103, 65, 65, 65, 66, 104, 48, 85,
    //     107, 53, 84, 65, 80, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47,
    //     47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 47, 54, 100, 120, 69, 78, 103, 65, 65, 65,
    //     72, 57, 74, 82, 69, 70, 85, 87, 73, 88, 116, 49, 76, 69, 83, 119, 67, 65, 73, 65, 49, 67,
    //     71, 55, 118, 122, 47, 51, 51, 97, 49, 83, 70, 74, 81, 66, 52, 99, 119, 108, 118, 67, 113,
    //     100, 53, 122, 50, 98, 74, 89, 74, 69, 67, 66, 65, 103, 73, 66, 98, 65, 102, 99, 57, 119,
    //     74, 48, 74, 111, 70, 99, 71, 85, 76, 78, 56, 104, 82, 74, 65, 113, 51, 67, 70, 112, 82, 73,
    //     103, 81, 77, 67, 70, 119, 77, 113, 76, 79, 65, 76, 56, 82, 89, 84, 47, 43, 65, 73, 47, 81,
    //     112, 89, 73, 65, 66, 88, 83, 103, 73, 85, 43, 102, 53, 89, 53, 103, 67, 75, 56, 97, 85,
    //     108, 111, 121, 113, 72, 118, 69, 122, 65, 107, 56, 48, 112, 79, 70, 104, 101, 112, 79, 90,
    //     53, 116, 89, 109, 115, 99, 114, 88, 74, 49, 71, 103, 75, 78, 69, 105, 68, 103, 67, 80, 65,
    //     67, 90, 88, 104, 73, 109, 90, 97, 122, 68, 80, 77, 65, 65, 65, 65, 65, 83, 85, 86, 79, 82,
    //     75, 53, 67, 89, 73, 73, 61, 34, 44, 34, 112, 114, 101, 118, 105, 101, 119, 115, 67, 104,
    //     97, 116, 34, 58, 116, 114, 117, 101, 44, 34, 101, 110, 102, 111, 114, 99, 101, 115, 83,
    //     101, 99, 117, 114, 101, 67, 104, 97, 116, 34, 58, 116, 114, 117, 101, 125,
    // ];
    // let indexed_buffer = IndexedBuffer(&data, Cell::new(0));
    // let length = parser::parse_var_int(&indexed_buffer);
    // let string = parser::parse_string(&indexed_buffer);

    // println!("Length: {}, string: {}", length, string);

    // println!(
    //     "{:x?}",
    //     serializer::serialize_signed_short(Vec::new(), -23176)
    // );

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
