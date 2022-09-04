pub mod packet {
    use crate::{
        client::client_data::State,
        packet_parser::parser::{self, IndexedBuffer},
    };

    pub fn create_packet(
        indexed_buffer: &IndexedBuffer,
        packet_id: &i32,
        state: &State,
    ) -> Result<C2S, ()> {
        match state {
            State::HandShaking => get_handshake_packet(indexed_buffer, packet_id),
            _ => Err(()),
        }
    }

    fn get_handshake_packet(buf: &IndexedBuffer, packet_id: &i32) -> Result<C2S, ()> {
        match packet_id {
            0 => Ok(C2S::Handshake(Handshake::parse(buf))),
            _ => Err(()),
        }
    }

    pub trait Packet<T> {
        fn parse(indexed_buffer: &IndexedBuffer) -> T;
    }

    #[derive(Debug)]
    pub enum C2S {
        Handshake(Handshake),
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
}
