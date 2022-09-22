use byteorder::{BigEndian, ByteOrder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Region {
    chunk_pos: [ChunkOffset; 4096],
}

pub struct ChunkOffset {
    pub offset: u32,
    pub size: u8,
}

pub struct ChunkHeader {}

impl<'de> Deserialize<'de> for ChunkOffset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: [u8; 4] = Deserialize::deserialize(deserializer)?;
        let offset = BigEndian::read_u32(&bytes[0..4]);
        let size = bytes[3];

        Ok(ChunkOffset { offset, size })
    }
}
