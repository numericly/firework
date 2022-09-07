pub mod c2s_packet {
    use crate::{
        client::client_data::State,
        packet_parser::parser::{self, IndexedBuffer, PacketBuffer},
    };

    use super::s2c_packet::{PingResponse, S2CPacket};

    pub fn get_packet(
        packet_buffer: PacketBuffer,
        packet_id: &i32,
        state: &State,
    ) -> Result<C2S, String> {
        match state {
            State::HandShaking => get_handshake_packet(packet_buffer, packet_id),
            State::Login => get_login_packet(packet_buffer, packet_id),
            State::Status => get_status_packet(packet_buffer, packet_id),
            State::Play => get_play_packet(packet_buffer, packet_id),
        }
    }

    fn get_handshake_packet(buf: PacketBuffer, packet_id: &i32) -> Result<C2S, String> {
        match packet_id {
            0 => Ok(C2S::Handshake(Handshake::parse(buf))),
            //0xFE: legacy server list ping
            _ => Err(format!(
                "Could not find packet with ID: {}, state: Handshake",
                packet_id
            )),
        }
    }

    fn get_login_packet(buf: PacketBuffer, packet_id: &i32) -> Result<C2S, String> {
        match packet_id {
            0 => Ok(C2S::LoginStart(LoginStart::parse(buf))), //TODO two more login packets
            1 => Ok(C2S::EncryptionResponse(EncryptionReponse::parse(buf))),
            //2 => Login Plugin Response
            _ => Err(format!(
                "Could not find packet with ID: {}, state: Login",
                packet_id
            )),
        }
    }

    fn get_status_packet(buf: PacketBuffer, packet_id: &i32) -> Result<C2S, String> {
        match packet_id {
            0 => Ok(C2S::StatusRequest(StatusRequest::parse(buf))),
            1 => Ok(C2S::PingRequest(PingRequest::parse(buf))),
            _ => Err(format!(
                "Could not find packet with ID: {}, state: Status",
                packet_id
            )),
        }
    }

    fn get_play_packet(_buf: PacketBuffer, packet_id: &i32) -> Result<C2S, String> {
        match packet_id {
            _ => Err(format!(
                "Could not find packet with ID: {}, state: Handshake",
                packet_id
            )),
        }
    }

    pub trait Packet<T> {
        fn parse(indexed_buffer: PacketBuffer) -> T;
    }

    #[derive(Debug)]
    pub enum C2S {
        //Handshaking
        Handshake(Handshake),
        LoginStart(LoginStart),
        //Status
        StatusRequest(StatusRequest),
        PingRequest(PingRequest),
        EncryptionResponse(EncryptionReponse)
    }

    #[derive(Debug)]
    pub struct Handshake {
        pub protocol_version: i32,
        pub server_address: String,
        pub server_port: u16,
        pub next_state: i32,
    }

    impl Packet<Handshake> for Handshake {
        fn parse(buf: PacketBuffer) -> Handshake {
            Handshake {
                protocol_version: buf.parse_var_int(),
                server_address: buf.parse_string(),
                server_port: buf.parse_unsigned_short(),
                next_state: buf.parse_var_int(),
            }
        }
    }

    #[derive(Debug)]
    pub struct LoginStart {
        pub player_name: String,
    }

    impl Packet<LoginStart> for LoginStart {
        fn parse(buf: PacketBuffer) -> LoginStart {
            LoginStart {
                player_name: buf.parse_string(),
            }
        }
    }
    #[derive(Debug)]
    pub struct StatusRequest {}

    impl Packet<StatusRequest> for StatusRequest {
        fn parse(_buf: PacketBuffer) -> StatusRequest {
            StatusRequest {}
        }
    }
    #[derive(Debug)]
    pub struct PingRequest {
        pub payload: i64,
    }

    impl Packet<PingRequest> for PingRequest {
        fn parse(buf: PacketBuffer) -> PingRequest {
            PingRequest {
                payload: buf.parse_signed_long(),
            }
        }
    }

    #[derive(Debug)]
    pub struct EncryptionReponse {
        pub shared_secret: Vec<u8>,
        pub verify_token: Option<Vec<u8>>,
        pub salt: Option<i64>
    }

    impl Packet<EncryptionReponse> for EncryptionReponse {
        fn parse(buf: PacketBuffer) -> EncryptionReponse {
            EncryptionReponse {
                shared_secret: buf.parse_byte_array(),
                verify_token: None,
                salt: None
            }
        }
    }

}

pub mod s2c_packet {
    use std::{io::Write, net::TcpStream};

    use crate::packet_serializer::serializer::{
        serialize_byte_array, serialize_signed_long, serialize_string, serialize_var_int,
    };

    pub trait S2CPacket {
        fn write(&mut self) -> Vec<u8>;
        fn write_packet(&mut self, stream: &mut TcpStream) -> Result<(), std::io::Error> {
            let packet_data = &self.write();

            stream.write_all(&serialize_var_int(Vec::new(), packet_data.len() as i32))?;
            stream.write_all(&packet_data)?;

            Ok(())
        }
    }

    #[derive(Debug)]
    pub struct ServerStatus {
        pub server_data: String,
    }

    impl S2CPacket for ServerStatus {
        fn write(&mut self) -> Vec<u8> {
            let mut data = Vec::new();
            data = serialize_var_int(data, 0);
            data = serialize_string(data, &mut self.server_data);
            data
        }
    }

    #[derive(Debug)]
    pub struct PingResponse {
        pub payload: i64,
    }

    impl S2CPacket for PingResponse {
        fn write(&mut self) -> Vec<u8> {
            let mut data = Vec::new();
            data = serialize_var_int(data, 1);
            data = serialize_signed_long(data, self.payload);
            data
        }
    }

    #[derive(Debug)]
    pub struct EncryptionRequest {
        pub server_id: String,
        pub public_key_length: i32,
        pub public_key: Vec<u8>,
        pub verify_token_length: i32,
        pub verify_token: Vec<u8>,
    }

    impl S2CPacket for EncryptionRequest {
        fn write(&mut self) -> Vec<u8> {
            let mut data = Vec::new();
            data = serialize_var_int(data, 1);
            data = serialize_string(data, &self.server_id);
            data = serialize_var_int(data, self.public_key_length);
            data = serialize_byte_array(data, &mut self.public_key);
            data = serialize_var_int(data, self.verify_token_length);
            data = serialize_byte_array(data, &mut self.verify_token);
            data
        }
    }
}
