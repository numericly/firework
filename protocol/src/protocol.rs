use std::{
    error::Error,
    fmt::{Debug, Display},
    io::{self, Cursor, ErrorKind, Read},
    string::FromUtf8Error,
};

use crate::{
    deserializer::IncomingPacketData,
    tesr::{
        client_bound::{SerializeField, SerializePacket},
        data_types::VarInt,
        server_bound::{DeserializeError, ServerBoundPacket},
    },
};
use aes::cipher::{inout::InOutBuf, BlockDecryptMut, BlockEncryptMut};
use aes::{cipher::KeyIvInit, Aes128};
use byteorder::ReadBytesExt;
use cfb8::{self, Decryptor, Encryptor};
use miniz_oxide::{deflate::compress_to_vec_zlib, inflate::decompress_to_vec_zlib};
use thiserror::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Client disconnected")]
    ClientDisconnect,
    #[error("Client forcibly disconnected")]
    ClientForceDisconnect,
    #[error("io error read: {0}")]
    ReadError(io::Error),
    #[error("io error write: {0}")]
    WriteError(io::Error),
    #[error("Failed to deserialize packet {0}")]
    DeserializeError(#[from] DeserializeError),
}

pub struct Protocol<'a> {
    pub connection_state: ConnectionState,

    stream: &'a mut TcpStream,
    compression_enabled: bool,
    cipher: Option<ProtocolCipher>,
}

pub struct ProtocolCipher {
    encryption: Encryptor<Aes128>,
    decryption: Decryptor<Aes128>,
}

impl<'a> Read for Protocol<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf);
        Ok(0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnectionState {
    HandShaking,
    Status,
    Login,
    Play,
}

impl Protocol<'_> {
    pub fn new(stream: &mut TcpStream) -> Protocol {
        Protocol {
            compression_enabled: false,
            stream: stream,
            connection_state: ConnectionState::HandShaking,
            cipher: None,
        }
    }
    pub async fn read_and_serialize(
        &mut self,
    ) -> Result<crate::packets::server_bound::ServerBoundPacket, ProtocolError> {
        let packet_data = self.read_packet().await?;
        let packet = crate::packets::server_bound::ServerBoundPacket::from(
            &self.connection_state,
            packet_data,
        )?;
        Ok(packet)
    }
    // pub async fn read_and_serialize(&mut self) -> Result<ServerBoundPacket, ProtocolError> {
    //     let packet_data = self.read_packet().await?;

    //     // let r = Cursor::new(Vec::new());

    //     let packet = ServerBoundPacket::deserialize(&packet_data.data[..], &self.connection_state)?;

    //     Ok(packet)
    // }
    pub async fn read_packet(&mut self) -> Result<IncomingPacketData, ProtocolError> {
        let packet_length = self.read_packet_length().await? as usize;

        let mut buffer = vec![0; packet_length];
        self.read_bytes_exact(&mut buffer).await?;

        if self.compression_enabled {
            let mut packet_data = IncomingPacketData::new(buffer);

            packet_data.read_var_int().unwrap();

            let decoded = decompress_to_vec_zlib(
                &packet_data.data[packet_data.index..packet_data.data.len()].to_vec(),
            )
            .unwrap();

            Ok(IncomingPacketData::new(decoded))
        } else {
            Ok(IncomingPacketData::new(buffer))
        }
    }
    // pub async fn write_packet(
    //     &mut self,
    //     packet: impl Serialize + Debug,
    // ) -> Result<(), ProtocolError> {
    //     let packet_data = packet.serialize();
    //     let data_length = OutboundPacketData::write_length(packet_data.data.len());
    //     let mut full_packet = if !self.compression_enabled {
    //         [&data_length[..], &packet_data.data[..]].concat()
    //     } else {
    //         let compressed_data = compress_to_vec_zlib(&packet_data.data, 10);
    //         let full_data_length =
    //             OutboundPacketData::write_length(compressed_data.len() + data_length.len());
    //         [
    //             &full_data_length[..],
    //             &data_length[..],
    //             &compressed_data[..],
    //         ]
    //         .concat()
    //     };
    //     if let Some(cipher) = &mut self.encryption {
    //         let buf = InOutBuf::from(full_packet.as_mut_slice()).into_chunks();
    //         cipher.encrypt_blocks_inout_mut(buf.0);
    //     }
    //     self.stream
    //         .write_all(&full_packet)
    //         .await
    //         .map_err(|err| ProtocolError::IoErrorWrite(err))?;
    //     Ok(())
    // }
    pub async fn write_packet(
        &mut self,
        packet: impl SerializePacket + Debug,
    ) -> Result<(), ProtocolError> {
        let packet_data = packet.serialize();

        let mut data_length_buf = Vec::new();
        VarInt(packet_data.len() as i32).serialize(&mut data_length_buf);

        let mut packet = match self.compression_enabled {
            true => {
                let compressed_data = compress_to_vec_zlib(&packet_data, 10);
                let mut full_data_length_buf = Vec::new();
                VarInt(compressed_data.len() as i32 + data_length_buf.len() as i32)
                    .serialize(&mut full_data_length_buf);
                full_data_length_buf.extend(data_length_buf);
                full_data_length_buf.extend(compressed_data);
                full_data_length_buf
            }
            false => {
                data_length_buf.extend(packet_data);
                data_length_buf
            }
        };

        if let Some(cipher) = &mut self.cipher {
            let buf = InOutBuf::from(packet.as_mut_slice()).into_chunks();
            cipher.encryption.encrypt_blocks_inout_mut(buf.0);
        }

        self.stream
            .write_all(&packet)
            .await
            .map_err(|err| ProtocolError::WriteError(err))?;

        Ok(())
    }
    pub fn enable_encryption(&mut self, key: &[u8], iv: &[u8]) {
        self.cipher = Some(ProtocolCipher {
            encryption: Encryptor::new_from_slices(key, iv).unwrap(),
            decryption: Decryptor::new_from_slices(key, iv).unwrap(),
        });
    }
    pub fn enable_compression(&mut self) {
        self.compression_enabled = true;
    }
    async fn read_packet_length(&mut self) -> Result<i32, ProtocolError> {
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;

        let mut val = 0i32;
        for i in 0..=4 {
            let position = i * 7;
            let current_byte = &self.read_byte().await?;

            val |= ((current_byte & SEGMENT_BITS) as i32) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            } else if i == 4 {
                return Err(ProtocolError::DeserializeError(
                    DeserializeError::VarIntTooLong,
                ));
            }
        }
        Ok(val)
    }
    async fn read_bytes_exact(&mut self, buf: &mut [u8]) -> Result<(), ProtocolError> {
        if let Err(err) = self.stream.read_exact(buf).await {
            return match err.kind() {
                ErrorKind::UnexpectedEof => Err(ProtocolError::ClientDisconnect),
                ErrorKind::Interrupted => Err(ProtocolError::ClientDisconnect),
                _ => Err(ProtocolError::ReadError(err)),
            };
        }

        if let Some(cipher) = &mut self.cipher {
            let buf = InOutBuf::from(buf).into_chunks();
            cipher.decryption.decrypt_blocks_inout_mut(buf.0);
        }
        Ok(())
    }
    async fn read_byte(&mut self) -> Result<u8, ProtocolError> {
        let mut buf = [0u8; 1];

        if let Err(err) = self.stream.read_exact(&mut buf).await {
            return match err.kind() {
                std::io::ErrorKind::UnexpectedEof => Err(ProtocolError::ClientDisconnect),
                std::io::ErrorKind::Interrupted => Err(ProtocolError::ClientForceDisconnect),
                _ => Err(ProtocolError::ReadError(err)),
            };
        };

        if let Some(cipher) = &mut self.cipher {
            let buf = InOutBuf::from(buf.as_mut_slice()).into_chunks();
            cipher.decryption.decrypt_blocks_inout_mut(buf.0);
        }

        Ok(buf[0])
    }
}
