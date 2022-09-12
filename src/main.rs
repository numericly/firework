use protocol::packets::client_bound::{
    EncryptionRequest, LoginSuccess, PingResponse, ServerStatus,
};
use protocol::packets::server_bound::ServerBoundPacket;
use protocol::protocol::Protocol;
use rand::rngs::OsRng;
use rand::RngCore;
use server::server_data::Server;
use std::env;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use crate::authentication::authenticate;

mod authentication;
mod server;
//mod client;
//mod world;

async fn handle_client(stream: TcpStream, server: Arc<Server>) {
    println!("Connection from {}", stream.peer_addr().unwrap());

    let mut protocol = Protocol::new(&stream);
    let mut temp_username = None;

    loop {
        let packet = match protocol.read_and_serialize() {
            Ok(packet) => packet,
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        };

        println!("Packet: {:?}", packet);

        match packet {
            ServerBoundPacket::Handshake(handshake) => {
                println!("Changing state: {:?}", handshake.next_state);
                protocol.connection_state = handshake.next_state;
            }
            ServerBoundPacket::StatusRequest(_status_request) => {
                let server_status = ServerStatus {
                    server_data: r#"{"previewsChat":false,"enforcesSecureChat":true,"description":{"text":"\u00a7a<rust-minecraft-server>\u00a7r"},"players":{"max":10,"online":0},"version":{"name":"1.19.2","protocol":760}}
                    "#.to_string()
                };

                protocol.write_packet(server_status).unwrap();
            }
            ServerBoundPacket::PingRequest(ping_request) => {
                let ping_response = PingResponse {
                    payload: ping_request.payload,
                };
                protocol.write_packet(ping_response).unwrap();
            }
            ServerBoundPacket::LoginStart(login_start) => {
                let mut rng = OsRng {};
                let mut bytes = [0u8; 4];

                rng.fill_bytes(&mut bytes);

                temp_username = Some(login_start.username);

                let encryption_request = EncryptionRequest {
                    server_id: "".to_string(), // deprecated after 1.7
                    public_key: server.encryption.encoded_pub.clone(),
                    verify_token: bytes.to_vec(),
                };

                protocol.write_packet(encryption_request).unwrap();
            }
            ServerBoundPacket::EncryptionResponse(encryption_response) => {
                let shared_secret = server
                    .encryption
                    .priv_key
                    .decrypt(
                        rsa::PaddingScheme::PKCS1v15Encrypt,
                        encryption_response.shared_secret.as_slice(),
                    )
                    .unwrap();
                if shared_secret.len() != 16usize {
                    return;
                }

                let profile = authenticate(
                    &shared_secret[0..16],
                    &server.encryption.encoded_pub,
                    &temp_username.as_ref().unwrap(),
                    &stream.peer_addr().unwrap().ip().to_string(),
                )
                .await;

                protocol.set_encryption(shared_secret.as_slice(), shared_secret.as_slice());

                let login_success = LoginSuccess {
                    id: u128::from_str_radix(&profile.id, 16).unwrap(),
                    username: profile.name,
                    properties: Vec::new(),
                };

                protocol.write_packet(login_success).unwrap();
            }
        }
    }
    /*
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
                    println!("Error handling next client state")
                }
            },
            C2S::StatusRequest(_status_request) => {
                let mut server_status = ServerStatus {
                    server_data: r#"{"previewsChat":false,"enforcesSecureChat":true,"description":{"text":"\u00a7a<rust-minecraft-server>\u00a7r"},"players":{"max":10,"online":0},"version":{"name":"1.20.3","protocol":760}}
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
            C2S::LoginStart(login_start) => {
                let mut rng = OsRng {};
                let mut bytes = [0u8; 4];

                rng.fill_bytes(&mut bytes);

                client.username = Some(login_start.player_name);

                let mut start_encryption = EncryptionRequest {
                    server_id: "".to_string(), // deprecated after 1.7
                    public_key_length: server.encryption.encoded_pub.len().clone() as i32,
                    public_key: server.encryption.encoded_pub.clone(),
                    verify_token_length: bytes.len() as i32,
                    verify_token: bytes.to_vec(),
                };

                start_encryption.write_packet(&mut stream).unwrap();
            }
            C2S::EncryptionResponse(encryption_response) => {
                let shared_secret = server
                    .encryption
                    .priv_key
                    .decrypt(
                        rsa::PaddingScheme::PKCS1v15Encrypt,
                        encryption_response.shared_secret.as_slice(),
                    )
                    .unwrap();
                if shared_secret.len() != 16usize {
                    return;
                }

                let profile = authenticate(
                    &shared_secret[0..16],
                    &server.encryption.encoded_pub,
                    client.username.as_ref().unwrap(),
                    &stream.peer_addr().unwrap().ip().to_string(),
                )
                .await;

                let mut cipher = CarrotCakeCipher::new(
                    shared_secret.as_slice().into(),
                    shared_secret.as_slice().into(),
                )
                .unwrap();

                let mut packet = LoginSuccess {
                    id: u128::from_str_radix(&profile.id, 16).unwrap(),
                    username: profile.name,
                    properties: {
                        let mut properties = Vec::new();
                        for prop in profile.properties {
                            properties.push(LoginSuccessProperty {
                                name: prop.name,
                                value: prop.value,
                                signature: Some(prop.signature),
                            });
                        }
                        properties
                    },
                };

                //save the uuid to the client for later use
                client.uuid = Some(profile.id);

                println!("Sending Login Success");
                println!("{:?}", packet);

                packet
                    .write_encrypted_packet(&mut stream, &mut cipher)
                    .unwrap();
            }
            _ => {
                println!("Packet not handled");
            }
        }
    }
    */
}

// fn process_packet(stream: &mut TcpStream, state: &State) -> Result<C2S, String> {
//     let packet_buffer = match stream.read_packet() {
//         Ok(packet_buffer) => packet_buffer,
//         Err(e) => return Err(e),
//     };

//     let packet_type = packet_buffer.parse_var_int();

//     get_packet(packet_buffer, &packet_type, state)
// }

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let listener = TcpListener::bind("127.0.0.1:25566").unwrap();
    let server = Arc::new(Server::new());

    for stream in listener.incoming() {
        let server = Arc::clone(&server);
        tokio::spawn(async move { handle_client(stream.unwrap(), server).await });
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
