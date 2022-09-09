use std::{io::Read, net::TcpStream};

use crate::{deserializer::IncomingPacketData, packets::server_bound::ServerBoundPacket};

pub struct Protocol<'a> {
    stream: &'a TcpStream,
    pub connection_state: ConnectionState,
}

#[derive(Debug)]
pub enum ConnectionState {
    HandShaking,
    Status,
    Login,
    Play,
}

impl Protocol<'_> {
    pub fn new(stream: &TcpStream) -> Protocol {
        Protocol {
            stream: &stream,
            connection_state: ConnectionState::HandShaking,
        }
    }
    pub fn read_and_serialize(
        &mut self,
        state: &ConnectionState,
    ) -> Result<ServerBoundPacket, String> {
        let packet_data = self.read_packet()?;

        Ok(ServerBoundPacket::from(state, packet_data)?)
    }
    pub fn read_packet(&mut self) -> Result<IncomingPacketData, String> {
        let packet_length = self.read_packet_length()? as usize;

        let mut buffer = vec![0; packet_length];

        self.read_bytes_exact(&mut buffer)?;

        // ZLIB decompression (Not implemented)

        Ok(IncomingPacketData::new(buffer))
    }
    fn read_packet_length(&mut self) -> Result<i32, String> {
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;

        let mut val = 0i32;
        for i in 0..5 {
            let position = i * 7;
            let current_byte = &self.read_byte()?;
            val |= ((current_byte & SEGMENT_BITS) as i32) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            } else if i == 4 {
                return Err("var int cannot exceed 5 bytes".to_string());
            }
        }
        Ok(val)
    }
    fn read_bytes_exact(&mut self, buf: &mut [u8]) -> Result<(), String> {
        self.stream.read_exact(buf).map_err(|e| e.to_string())?;

        // decrypt the the read data (Not implemented)
        Ok(())
    }
    fn read_byte(&mut self) -> Result<u8, String> {
        let mut buf = [0u8; 1];

        if let Err(err) = &self.stream.read_exact(&mut buf) {
            return match err.kind() {
                std::io::ErrorKind::UnexpectedEof => Err("stream closed by client".to_string()),
                std::io::ErrorKind::Interrupted => {
                    Err("connection forcibly closed by the client".to_string())
                }
                _ => {
                    println!("error: {}", err);
                    Err("unknown error".to_string())
                }
            };
        };

        // decrypt the read bytes (Not implemented)

        Ok(buf[0])
    }
}
