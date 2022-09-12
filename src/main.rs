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
}

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
