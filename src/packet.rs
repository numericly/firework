pub mod c2s_packet {
    use crate::{
        client::client_data::State,
        packet_parser::parser::{self, IndexedBuffer},
    };

    pub fn get_packet(
        indexed_buffer: &IndexedBuffer,
        packet_id: &i32,
        state: &State,
    ) -> Result<C2S, ()> {
        match state {
            State::HandShaking => get_handshake_packet(indexed_buffer, packet_id),
            State::Login => get_login_packet(indexed_buffer, packet_id),
            State::Status => get_status_packet(indexed_buffer, packet_id),
            _ => Err(()),
        }
    }

    fn get_handshake_packet(buf: &IndexedBuffer, packet_id: &i32) -> Result<C2S, ()> {
        match packet_id {
            0 => Ok(C2S::Handshake(Handshake::parse(buf))),
            _ => Err(()),
        }
    }

    fn get_login_packet(buf: &IndexedBuffer, packet_id: &i32) -> Result<C2S, ()> {
        match packet_id {
            0 => Ok(C2S::LoginStart(LoginStart::parse(buf))),
            _ => Err(()),
        }
    }

    fn get_status_packet(buf: &IndexedBuffer, packet_id: &i32) -> Result<C2S, ()> {
        match packet_id {
            0 => Ok(C2S::StatusRequest(StatusRequest::parse(buf))),
            1 => Ok(C2S::PingRequest(PingRequest::parse(buf))),
            _ => Err(()),
        }
    }

    pub trait Packet<T> {
        fn parse(indexed_buffer: &IndexedBuffer) -> T;
    }

    #[derive(Debug)]
    pub enum C2S {
        //Handshaking
        Handshake(Handshake),
        LoginStart(LoginStart),
        //Status
        StatusRequest(StatusRequest),
        PingRequest(PingRequest),
    }

    #[derive(Debug)]
    pub struct Handshake {
        pub protocol_version: i32,
        pub server_address: String,
        pub server_port: u16,
        pub next_state: i32,
    }

    impl Packet<Handshake> for Handshake {
        fn parse(buf: &IndexedBuffer) -> Handshake {
            Handshake {
                protocol_version: parser::parse_var_int(&buf),
                server_address: parser::parse_string(&buf),
                server_port: parser::parse_unsigned_short(&buf),
                next_state: parser::parse_var_int(&buf),
            }
        }
    }

    #[derive(Debug)]
    pub struct LoginStart {
        pub player_name: String,
    }

    impl Packet<LoginStart> for LoginStart {
        fn parse(buf: &IndexedBuffer) -> LoginStart {
            LoginStart {
                player_name: parser::parse_string(&buf),
            }
        }
    }
    #[derive(Debug)]
    pub struct StatusRequest {}

    impl Packet<StatusRequest> for StatusRequest {
        fn parse(_buf: &IndexedBuffer) -> StatusRequest {
            StatusRequest {}
        }
    }
    #[derive(Debug)]
    pub struct PingRequest {
        _payload: i64,
    }

    impl Packet<PingRequest> for PingRequest {
        fn parse(buf: &IndexedBuffer) -> PingRequest {
            PingRequest {
                _payload: parser::parse_signed_long(&buf),
            }
        }
    }
}

pub mod s2c_packet {
    use std::{io::Write, net::TcpStream};

    use crate::packet_serializer::serializer::{serialize_string, serialize_var_int};

    pub trait S2CPacket {
        fn write(&mut self) -> Vec<u8>;
        fn write_packet(&mut self, stream: &mut TcpStream) -> Result<(), std::io::Error> {
            let packet_data = &self.write();

            // stream.write_all(&serialize_var_int(Vec::new(), packet_data.len() as i32))?;
            // stream.write_all(&packet_data)?;
            let full_packet = [
                serialize_var_int(Vec::new(), packet_data.len() as i32).to_vec(),
                packet_data.to_owned(),
            ]
            .concat();
            stream.write_all(&full_packet)?;
            println!("{:?}", &full_packet);

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
}
