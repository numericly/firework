use packet_parser::Parser;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

mod packet_parser;

fn handle_client(mut stream: TcpStream) {
    println!("Connection from {}", stream.peer_addr().unwrap());

    loop {
        process_packet(&mut stream);
    }
}

fn process_packet(stream: &mut TcpStream) -> Result<(), std::io::Error> {
    let length = Parser::parse_var_int(stream)?;

    let mut buffer = vec![0u8; length as usize];
    stream.read_exact(&mut buffer).unwrap();

    let packet_type = Parser::parse_var_int(stream)?;

    println!("Packet: {:?}", buffer);
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
