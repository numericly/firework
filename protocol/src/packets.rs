pub mod server_bound {

    use crate::{deserializer::IncomingPacketData, protocol::ConnectionState};

    #[derive(Debug)]
    pub enum ServerBoundPacket {
        Handshake(Handshake),
        LoginStart(LoginStart),

        StatusRequest(StatusRequest),
        PingRequest(PingRequest),
        EncryptionResponse(EncryptionResponse),
    }

    impl ServerBoundPacket {
        pub fn from(
            state: &ConnectionState,
            mut packet_data: IncomingPacketData,
        ) -> Result<ServerBoundPacket, String> {
            let packet_id = packet_data.read_var_int()?;

            match state {
                ConnectionState::HandShaking => match packet_id {
                    0 => Handshake::deserialize(packet_data),
                    _ => Err(format!(
                        "Unknown packet id {} for state {:?}",
                        packet_id, state
                    )),
                },
                ConnectionState::Status => match packet_id {
                    0 => StatusRequest::deserialize(packet_data),
                    1 => PingRequest::deserialize(packet_data),
                    _ => Err(format!(
                        "Unknown packet id {} for state {:?}",
                        packet_id, state
                    )),
                },
                ConnectionState::Login => match packet_id {
                    0 => LoginStart::deserialize(packet_data),
                    1 => EncryptionResponse::deserialize(packet_data),
                    _ => Err(format!(
                        "Unknown packet id {} for state {:?}",
                        packet_id, state
                    )),
                },
                ConnectionState::Play => match packet_id {
                    _ => Err(format!(
                        "Unknown packet id {} for state {:?}",
                        packet_id, state
                    )),
                },
            }
        }
    }

    pub trait Deserialize {
        fn deserialize(packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String>;
    }

    #[derive(Debug)]
    pub struct Handshake {
        pub protocol_version: i32,
        pub server_address: String,
        pub server_port: u16,
        pub next_state: ConnectionState,
    }

    impl Deserialize for Handshake {
        fn deserialize(mut packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            let protocol_version = packet_data.read_var_int()?;
            let server_address = packet_data.read_string()?;
            let server_port = packet_data.read_u16()?;
            let next_state = match packet_data.read_var_int()? {
                1 => ConnectionState::Status,
                2 => ConnectionState::Login,
                _ => return Err("Invalid next state".to_string()),
            };

            Ok(ServerBoundPacket::Handshake(Handshake {
                protocol_version: protocol_version,
                server_address: server_address,
                server_port: server_port,
                next_state: next_state,
            }))
        }
    }

    #[derive(Debug)]
    pub struct LoginStart {
        pub username: String,
    }

    impl Deserialize for LoginStart {
        fn deserialize(mut packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            let username = packet_data.read_string()?;

            Ok(ServerBoundPacket::LoginStart(LoginStart { username }))
        }
    }

    #[derive(Debug)]
    pub struct StatusRequest {}

    impl Deserialize for StatusRequest {
        fn deserialize(_packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            Ok(ServerBoundPacket::StatusRequest(StatusRequest {}))
        }
    }

    #[derive(Debug)]
    pub struct PingRequest {
        pub payload: i64,
    }

    impl Deserialize for PingRequest {
        fn deserialize(mut packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            let payload = packet_data.read_long()?;

            Ok(ServerBoundPacket::PingRequest(PingRequest { payload }))
        }
    }

    #[derive(Debug)]
    pub struct EncryptionResponse {
        pub shared_secret: Vec<u8>,
        pub verify_token: Option<Vec<u8>>,
    }

    impl Deserialize for EncryptionResponse {
        fn deserialize(mut packet_data: IncomingPacketData) -> Result<ServerBoundPacket, String> {
            let shared_secret_length = packet_data.read_var_int()?;
            let shared_secret = packet_data.read_bytes(shared_secret_length as usize)?;
            let verify_token_length = packet_data.read_var_int()?;
            let verify_token = packet_data.read_bytes(verify_token_length as usize)?;

            Ok(ServerBoundPacket::EncryptionResponse(EncryptionResponse {
                shared_secret: shared_secret,
                verify_token: Some(verify_token),
            }))
        }
    }
}
