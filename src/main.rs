use openssl::rsa::{Padding, Rsa};
use openssl::symm::Cipher;
use packet::c2s_packet::C2S;
use packet_parser::parser::{self};
use rand::RngCore;
use rand::rngs::OsRng;
use std::net::{TcpListener, TcpStream};
use aes::cipher::{AsyncStreamCipher, KeyIvInit};

type Aes128Cfb8Enc = cfb8::Encryptor<aes::Aes128>;
type Aes128Cfb8Dec = cfb8::Decryptor<aes::Aes128>;

use crate::client::client_data::{Client, State};
use crate::packet::c2s_packet::{get_packet, LoginStart};
use crate::packet::s2c_packet::{PingResponse, S2CPacket, ServerStatus, EncryptionRequest};
use crate::packet_parser::parser::{IndexedBuffer, ReadUncompressed};

mod client;
mod packet;
mod packet_parser;
mod packet_serializer;

fn handle_client(mut stream: TcpStream) {
    println!("Connection from {}", stream.peer_addr().unwrap());
    let mut client = Client {
        state: State::HandShaking,
    };

    let rsa = Rsa::generate(1024).unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_der().unwrap();

    let mut encryptor = None;
    let mut decryptor = None;

    loop {
        let packet = process_packet(&mut stream, &client.state);

        if let Err(e) = packet {
            println!("Stream Closed by Client err {}", e);
            return;
        }
        println!("Packet {:?}", &packet);

        match packet.unwrap() {
            C2S::Handshake(handshake) => match handshake.next_state {
                1 => {
                    client.state = State::Status;
                }
                2 => {
                    client.state = State::Login;
                }
                _ => {
                    println!("Error handling client state")
                }
            },
            C2S::StatusRequest(_status_request) => {
                let mut server_status = ServerStatus {
                    server_data: r#"{"previewsChat":false,"enforcesSecureChat":true,"description":{"text":"\u00a7a<rust-minecraft-server>\u00a7r"},"players":{"max":10,"online":0},"version":{"name":"1.19.2","protocol":760}}
                    "#.to_string()
                };

                server_status.write_packet(&mut stream).unwrap();
            }
            C2S::PingRequest(ping_request) => {
                let mut ping_response = PingResponse {
                    payload: ping_request.payload,
                };
                ping_response.write_packet(&mut stream).unwrap();
            }
            C2S::LoginStart(_login_start) => {
                let mut rng = OsRng {};
                let mut bytes = [0u8; 4];
                rng.fill_bytes(&mut bytes);

                let mut start_encryption = EncryptionRequest {
                    server_id: "".to_string(), // depracated after 1.7
                    public_key_length: (&public_key.len()).clone() as i32,
                    public_key: (&public_key).clone(),
                    verify_token_length: bytes.len() as i32,
                    verify_token: bytes.to_vec()
                };

                start_encryption.write_packet(&mut stream).unwrap();
            }
            C2S::EncryptionResponse(encryption_reponse) => {
                let mut buf = vec![0u8; encryption_reponse.shared_secret.len()];
                let shared_secret_length = rsa.private_decrypt(&encryption_reponse.shared_secret, &mut buf, Padding::PKCS1).unwrap();
                if shared_secret_length != 16usize {
                    return;
                }

                encryptor = Some(Aes128Cfb8Enc::new(buf[0..16].into(), buf[0..16].into()));
                decryptor = Some(Aes128Cfb8Dec::new(buf[0..16].into(), buf[0..16].into()));
            }
            _ => {
                println!("Packet not handled");
            }
        }
    }
}

fn process_packet(stream: &mut TcpStream, state: &State) -> Result<C2S, String> {
    let packet_buffer = match stream.read_packet() {
        Ok(packet_buffer) => packet_buffer,
        Err(e) => return Err(e),
    };

    let packet_type = packet_buffer.parse_var_int();

    get_packet(packet_buffer, &packet_type, state)
}

#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("127.0.0.1:25566").unwrap();

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
