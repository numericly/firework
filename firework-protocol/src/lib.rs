use crate::{
    client_bound::{ClientBoundPacket, SerializePacket},
    server_bound::ServerBoundPacket,
};
use aes::cipher::{inout::InOutBuf, BlockDecryptMut, BlockEncryptMut};
use aes::{cipher::KeyIvInit, Aes128};
use cfb8::{self, Decryptor, Encryptor};
use client_bound::SetCompression;
use firework_protocol_core::{DeserializeError, DeserializeField, SerializeField, VarInt};
use firework_protocol_derive::DeserializeField;
use miniz_oxide::{
    deflate::compress_to_vec_zlib,
    inflate::{decompress_to_vec_zlib, DecompressError},
};
use std::{
    fmt::Debug,
    io::{self, Cursor, ErrorKind},
    pin::Pin,
    task::{Context, Poll},
};
use thiserror::Error;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt, ReadBuf},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::{Mutex, RwLock},
};

pub mod client_bound;
pub mod data_types;
pub mod server_bound;

pub use firework_protocol_core as core;
pub use firework_protocol_derive as protocol_derive;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("client disconnected")]
    ClientDisconnect,
    #[error("client forcibly disconnected")]
    ClientForceDisconnect,
    #[error("io error read {0}")]
    ReadError(io::Error),
    #[error("io error write {0}")]
    WriteError(io::Error),
    #[error("failed to deserialize packet {0}")]
    DeserializeError(#[from] DeserializeError),
    #[error("failed to decompress packet")]
    DecompressError(DecompressError),
    #[error("unexpected packet, expected {expected}, got {got} in state {state:?}")]
    UnexpectedPacket {
        got: String,
        expected: &'static str,
        state: ConnectionState,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, DeserializeField)]
#[protocol(typ = "u8")]
pub enum ConnectionState {
    HandShaking,
    Status,
    Login,
    Play,
}

#[derive(Debug)]
pub struct Protocol {
    pub connection_state: RwLock<ConnectionState>,
    pub joined_world: RwLock<bool>,
    writer: Mutex<ProtocolWriter>,
    reader: Mutex<ProtocolReader>,
    compression: Option<usize>,
}

#[derive(Debug)]
pub struct ProtocolReader {
    reader: OwnedReadHalf,
    cipher: Option<Decryptor<Aes128>>,
}

#[derive(Debug)]
pub struct ProtocolWriter {
    writer: OwnedWriteHalf,
    cipher: Option<Encryptor<Aes128>>,
}

impl<'a> AsyncRead for ProtocolReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), io::Error>> {
        match OwnedReadHalf::poll_read(Pin::new(&mut self.reader), cx, buf) {
            Poll::Ready(Ok(_)) => {
                if let Some(cipher) = &mut self.cipher {
                    let buf = InOutBuf::from(buf.filled_mut()).into_chunks().0;
                    cipher.decrypt_blocks_inout_mut(buf);
                }
                Poll::Ready(Ok(()))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl ProtocolWriter {
    pub async fn write(&mut self, buf: &[u8]) -> Result<(), io::Error> {
        let mut buf = buf.to_vec();
        if let Some(cipher) = &mut self.cipher {
            let (buf, _) = InOutBuf::from(buf.as_mut_slice()).into_chunks();
            cipher.encrypt_blocks_inout_mut(buf);
        }
        self.writer.write_all(buf.as_slice()).await
    }
}

impl Protocol {
    pub fn new(stream: TcpStream) -> Protocol {
        let (reader, writer) = stream.into_split();
        Protocol {
            compression: None,
            writer: Mutex::new(ProtocolWriter {
                writer,
                cipher: None,
            }),
            reader: Mutex::new(ProtocolReader {
                reader,
                cipher: None,
            }),
            joined_world: RwLock::new(false),
            connection_state: RwLock::new(ConnectionState::HandShaking),
        }
    }
    pub async fn read_and_deserialize(&self) -> Result<ServerBoundPacket, ProtocolError> {
        let packet_data = self.read_packet().await?;
        let packet =
            ServerBoundPacket::deserialize(packet_data.as_slice(), &self.connection_state).await?;

        if cfg!(feature = "log_packets") {
            println!("C -> S {}", packet.name());
        }
        Ok(packet)
    }
    pub async fn read_packet(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut reader = self.reader.lock().await;
        let packet_length = read_packet_length(&mut reader).await?;

        let mut buffer = vec![0; packet_length];
        reader.read_exact(&mut buffer).await.map_err(|err| {
            if err.kind() == ErrorKind::UnexpectedEof {
                ProtocolError::ClientDisconnect
            } else {
                DeserializeError::IoError(err).into()
            }
        })?;

        if let Some(compression_threshold) = self.compression {
            if packet_length >= compression_threshold {
                let mut packet_data = Cursor::new(buffer.as_slice());
                VarInt::deserialize(&mut packet_data)?;

                let decompressed =
                    decompress_to_vec_zlib(&buffer[packet_data.position() as usize..])
                        .map_err(ProtocolError::DecompressError)?;

                return Ok(decompressed);
            }
        }
        Ok(buffer)
    }
    pub async fn write_packet<T>(&self, packet: T) -> Result<(), ProtocolError>
    where
        T: SerializePacket + ClientBoundPacket + Debug,
    {
        let packet_data = packet.serialize();

        let mut packet_data_len = Vec::new();
        VarInt(packet_data.len() as i32).serialize(&mut packet_data_len);

        let packet = if let Some(compression_threshold) = self.compression {
            if packet_data.len() >= compression_threshold {
                let compressed_data = compress_to_vec_zlib(&packet_data, 3); // Level 3 is optimal for net performance

                let mut compressed_packet_data =
                    Vec::with_capacity(compressed_data.len() + packet_data_len.len() + 5);
                VarInt(compressed_data.len() as i32 + packet_data_len.len() as i32)
                    .serialize(&mut compressed_packet_data);
                compressed_packet_data.extend(packet_data_len);
                compressed_packet_data.extend(compressed_data);
                compressed_packet_data
            } else {
                packet_data_len.extend(packet_data);
                packet_data_len
            }
        } else {
            packet_data_len.extend(packet_data);
            packet_data_len
        };

        let mut writer = self.writer.lock().await;

        if cfg!(feature = "log_packets") {
            println!("S -> C {}", T::name());
        }

        writer.write(&packet).await.map_err(|err| {
            if err.kind() == ErrorKind::BrokenPipe {
                ProtocolError::ClientDisconnect
            } else {
                ProtocolError::WriteError(err)
            }
        })
    }
    pub async fn enable_encryption(&mut self, key: &[u8], iv: &[u8]) {
        self.reader.lock().await.cipher = Some(Decryptor::new_from_slices(key, iv).unwrap());
        self.writer.lock().await.cipher = Some(Encryptor::new_from_slices(key, iv).unwrap());
    }
    pub async fn enable_compression(&mut self, threshold: usize) -> Result<(), ProtocolError> {
        let set_compression = SetCompression {
            threshold: VarInt(threshold as i32),
        };

        self.write_packet(set_compression).await?;
        self.compression = Some(threshold);
        Ok(())
    }
}

async fn read_packet_length(reader: &mut ProtocolReader) -> Result<usize, ProtocolError> {
    let mut value = 0;
    let mut bytes_read = 0;

    loop {
        if bytes_read > 5 {
            return Err(DeserializeError::VarIntTooLong.into());
        }

        let byte = reader.read_u8().await.map_err(|err| {
            if err.kind() == ErrorKind::UnexpectedEof {
                ProtocolError::ClientDisconnect
            } else {
                DeserializeError::IoError(err).into()
            }
        })?;

        value |= ((byte & 0x7F) as i32) << (7 * bytes_read);

        bytes_read += 1;

        if byte & 0x80 == 0 {
            break;
        }
    }

    Ok(value as usize)
}

#[macro_export]
macro_rules! read_specific_packet {
    ($protocol:expr, $packet:ident) => {
        async {
            let protocol: &Protocol = $protocol;
            let packet = protocol.read_and_deserialize().await?;

            if let ServerBoundPacket::$packet(packet) = packet {
                Ok(packet)
            } else {
                return Err(ProtocolError::UnexpectedPacket {
                    expected: stringify!($packet),
                    got: format!("{:?}", packet),
                    state: *protocol.connection_state.read().await,
                });
            }
        }
    };
}
