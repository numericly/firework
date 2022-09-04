pub mod packet {
    use crate::{
        client::client_data::State,
        packet_parser::parser::{self, IndexedBuffer},
    };

    pub fn create_packet(
        indexed_buffer: &IndexedBuffer,
        packet_id: &i32,
        state: &State,
    ) -> Result<c2s, ()> {
        match state {
            State::HandShaking => get_handshake_packet(indexed_buffer, packet_id),
            _ => Err(()),
        }
    }

    fn get_handshake_packet(indexed_buffer: &IndexedBuffer, packet_id: &i32) -> Result<c2s, ()> {
        match packet_id {
            0 => Ok(c2s::Handshake(Handshake::parse(indexed_buffer))),
            _ => Err(()),
        }
    }

    pub trait Packet<T> {
        fn parse(indexed_buffer: &IndexedBuffer) -> T;
    }

    #[derive(Debug)]
    #[warn(non_camel_case_types)]
    pub enum c2s {
        Handshake(Handshake),
    }

    #[derive(Debug)]
    pub struct Handshake {
        pub protcol_version: i32,
        pub server_address: String,
        pub server_port: i16,
        pub next_state: i32,
    }

    impl Packet<Handshake> for Handshake {
        fn parse(indexed_buffer: &IndexedBuffer) -> Handshake {
            Handshake {
                protcol_version: parser::parse_var_int(&indexed_buffer),
                server_address: "".to_string(),
                server_port: 0,
                next_state: 0,
            }
        }
    }
}
