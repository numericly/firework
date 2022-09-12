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

pub mod client_bound {
    use crate::serializer::OutboundPacketData;

    pub trait Serialize {
        fn serialize(&self) -> OutboundPacketData {
            let mut packet_data = OutboundPacketData::new();

            // Write packet with packet id
            packet_data.write_var_int(self.packet_id());
            self.serialize_into(&mut packet_data);

            packet_data
        }
        fn serialize_into(&self, packet_data: &mut OutboundPacketData);
        fn packet_id(&self) -> i32;
    }

    #[derive(Debug)]
    pub struct ServerStatus {
        pub server_data: String,
    }

    impl Serialize for ServerStatus {
        fn serialize_into(&self, packet_data: &mut OutboundPacketData) {
            packet_data.write_string(&self.server_data);
        }
        fn packet_id(&self) -> i32 {
            0
        }
    }

    #[derive(Debug)]
    pub struct PingResponse {
        pub payload: i64,
    }

    impl Serialize for PingResponse {
        fn serialize_into(&self, packet_data: &mut OutboundPacketData) {
            packet_data.write_signed_long(self.payload);
        }
        fn packet_id(&self) -> i32 {
            1
        }
    }

    #[derive(Debug)]
    pub struct EncryptionRequest {
        pub server_id: String,
        pub public_key: Vec<u8>,
        pub verify_token: Vec<u8>,
    }

    impl Serialize for EncryptionRequest {
        fn serialize_into(&self, packet_data: &mut OutboundPacketData) {
            packet_data.write_string(&self.server_id);
            packet_data.write_var_int(self.public_key.len() as i32);
            packet_data.write_bytes(&self.public_key);
            packet_data.write_var_int(self.verify_token.len() as i32);
            packet_data.write_bytes(&self.verify_token);
        }
        fn packet_id(&self) -> i32 {
            1
        }
    }

    #[derive(Debug)]
    pub struct LoginSuccess {
        pub id: u128,
        pub username: String,
        pub properties: Vec<LoginSuccessProperty>,
    }

    #[derive(Debug)]
    pub struct LoginSuccessProperty {
        pub name: String,
        pub value: String,
        pub signature: Option<String>,
    }

    impl Serialize for LoginSuccess {
        fn serialize_into(&self, packet_data: &mut OutboundPacketData) {
            packet_data.write_uuid(self.id);
            packet_data.write_string(&self.username);
            packet_data.write_var_int(self.properties.len() as i32);
            for property in &self.properties {
                packet_data.write_string(&property.name);
                packet_data.write_string(&property.value);
                match &property.signature {
                    Some(signature) => {
                        packet_data.write_bool(true);
                        packet_data.write_string(signature);
                    }
                    None => packet_data.write_bool(false),
                }
            }
        }
        fn packet_id(&self) -> i32 {
            2
        }
    }
}
