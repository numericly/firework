use std::fmt::Debug;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{
    cfb8::CarrotCakeCipher,
    deserializer::IncomingPacketData,
    packets::{client_bound::Serialize, server_bound::ServerBoundPacket},
    serializer::OutboundPacketData,
};

pub struct Protocol<'a> {
    stream: &'a mut TcpStream,
    pub connection_state: ConnectionState,
    pub encryption: Option<CarrotCakeCipher>,
    pub decryption: Option<CarrotCakeCipher>,
}

#[derive(Debug)]
pub enum ConnectionState {
    HandShaking,
    Status,
    Login,
    Play,
}

impl Protocol<'_> {
    pub fn new(stream: &mut TcpStream) -> Protocol {
        Protocol {
            stream: stream,
            connection_state: ConnectionState::HandShaking,
            encryption: None,
            decryption: None,
        }
    }
    pub async fn read_and_serialize(&mut self) -> Result<ServerBoundPacket, String> {
        let packet_data = self.read_packet().await?;

        let packet = ServerBoundPacket::from(&self.connection_state, packet_data);

        println!("Received packet: {:?}", packet);

        packet
    }
    pub async fn read_packet(&mut self) -> Result<IncomingPacketData, String> {
        let packet_length = self.read_packet_length().await? as usize;

        let mut buffer = vec![0; packet_length];

        self.read_bytes_exact(&mut buffer).await?;

        // ZLIB decompression (Not implemented)

        Ok(IncomingPacketData::new(buffer))
    }
    pub async fn write_packet(&mut self, packet: impl Serialize + Debug) -> Result<(), String> {
        //println!("Sending packet: {:#?}", packet);

        let packet_data = packet.serialize();

        // ZLIB compression (Not implemented)

        let data_length = OutboundPacketData::write_length(packet_data.data.len());

        let mut full_packet = [&data_length[..], &packet_data.data[..]].concat();

        if let Some(cipher) = &mut self.encryption {
            cipher.encrypt(&mut full_packet);
        }

        self.stream
            .write_all(&full_packet)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
    pub fn set_encryption(&mut self, key: &[u8], iv: &[u8]) {
        self.encryption = Some(CarrotCakeCipher::new(key, iv).unwrap());
        self.decryption = Some(CarrotCakeCipher::new(key, iv).unwrap());
    }
    async fn read_packet_length(&mut self) -> Result<i32, String> {
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;

        let mut val = 0i32;
        for i in 0..5 {
            let position = i * 7;
            let current_byte = &self.read_byte().await?;
            val |= ((current_byte & SEGMENT_BITS) as i32) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            } else if i == 4 {
                return Err("var int cannot exceed 5 bytes".to_string());
            }
        }
        Ok(val)
    }
    async fn read_bytes_exact(&mut self, buf: &mut [u8]) -> Result<(), String> {
        self.stream
            .read_exact(buf)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(cipher) = &mut self.decryption {
            cipher.decrypt(buf);
        }
        Ok(())
    }
    async fn read_byte(&mut self) -> Result<u8, String> {
        let mut buf = [0u8; 1];

        if let Err(err) = &self.stream.read_exact(&mut buf).await {
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

        if let Some(cipher) = &mut self.decryption {
            cipher.decrypt(&mut buf);
        }

        Ok(buf[0])
    }
}
