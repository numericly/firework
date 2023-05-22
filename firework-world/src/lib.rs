use crate::chunk::Chunk;
use byteorder::{BigEndian, ReadBytesExt};
use dashmap::DashMap;
use firework_protocol::client_bound::SerializePacket;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::Read;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{Mutex, RwLock};

pub mod chunk;

pub use firework_world_loader::world;

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("NBT error: {0}")]
    NbtError(#[from] nbt::Error),
}

pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

pub struct World {
    pub flat: bool,
    regions: DashMap<(i32, i32), Arc<Region>>,
}

#[derive(Debug)]
pub struct Region {
    chunk_timestamps: [u32; 1024],
    pub sections: Mutex<[RegionChunk; 1024]>,
}

#[derive(Debug)]
pub enum RegionChunk {
    None,
    ChunkBytes(Box<Vec<u8>>),
    Chunk(Arc<RwLock<Chunk>>),
}

#[derive(Clone, Copy, Debug)]
pub struct ChunkOffset {
    pub offset: u32,
    pub size: u8,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ChunkPos {
    pub x: u8,
    pub z: u8,
}

pub trait ToPacket<T: SerializePacket> {
    fn to_packet(&self) -> T;
}

impl World {
    pub fn new(flat: bool) -> World {
        World {
            flat,
            regions: DashMap::new(),
        }
    }
    pub async fn get_chunk(
        &self,
        x: i32,
        z: i32,
    ) -> Result<Option<Arc<RwLock<Chunk>>>, WorldError> {
        let region = self.get_region(x >> 5, z >> 5)?;

        if let Some(region) = region {
            Ok(region.get_chunk(x, z).await?)
        } else {
            Ok(None)
        }
    }
    pub async fn get_chunk_from_pos(
        &self,
        x: i32,
        z: i32,
    ) -> Result<Option<Arc<RwLock<Chunk>>>, WorldError> {
        self.get_chunk(x >> 4, z >> 4).await
    }
    pub fn get_region(&self, x: i32, z: i32) -> Result<Option<Arc<Region>>, WorldError> {
        Ok(self.regions.get(&(x, z)).map(|region| region.clone()))
    }
    pub fn add_region<R: Read>(&self, x: i32, z: i32, reader: R) {
        let region = Arc::new(Region::deserialize(reader).expect("Failed to deserialize region"));
        self.regions.insert((x, z), region);
    }
}

impl Region {
    pub fn deserialize<R: Read>(mut reader: R) -> Result<Region, WorldError> {
        #[derive(Debug)]
        struct ChunkInfo {
            size: u8,
            index: usize,
        }

        fn read_bytes_exact<R: Read>(reader: R, bytes: u64) -> Vec<u8> {
            let mut buf = Vec::with_capacity(bytes as usize);
            let mut section = reader.take(bytes);
            section.read_to_end(&mut buf).unwrap();
            buf
        }

        let mut info_map = HashMap::new();

        for i in 0..1024 {
            let offset = reader.read_u24::<BigEndian>().unwrap();
            let size = reader.read_u8().unwrap();
            if size != 0 {
                info_map.insert(offset, ChunkInfo { size, index: i });
            }
        }

        let mut chunk_times = [0; 1024];

        for i in 0..1024 {
            let timestamp = reader.read_u32::<BigEndian>().unwrap();
            chunk_times[i] = timestamp;
        }

        const INIT: RegionChunk = RegionChunk::None;
        let mut sections = [INIT; 1024];

        let mut pos = 2;
        loop {
            if let Some(info) = info_map.get(&pos) {
                let data = read_bytes_exact(&mut reader, info.size as u64 * 4096);

                sections[info.index] = RegionChunk::ChunkBytes(Box::new(data));

                pos += info.size as u32;
            } else {
                let mut unused_data = [0; 4096];
                if let Err(err) = reader.read_exact(&mut unused_data) {
                    match err.kind() {
                        std::io::ErrorKind::UnexpectedEof => break,
                        _ => return Err(err.into()),
                    }
                }
                pos += 1;
            }
        }

        Ok(Region {
            sections: Mutex::new(sections),
            chunk_timestamps: chunk_times.try_into().unwrap(),
        })
    }
    /// Gets and caches a chunk from the region at the given position.
    pub async fn get_chunk(
        &self,
        x: i32,
        z: i32,
    ) -> Result<Option<Arc<RwLock<Chunk>>>, WorldError> {
        let index = x.rem_euclid(32) as usize + (z.rem_euclid(32) as usize * 32);

        let sections = self.sections.lock().await;

        match &sections[index] {
            RegionChunk::None => Ok(None),
            RegionChunk::ChunkBytes(bytes) => {
                // Cut off the first 4 bits as they are the header and must be ignored
                // in order to deserialize the chunk.
                let reader = bytes[5..].to_vec();
                drop(sections);
                let chunk_data = Chunk::from_zlib_reader(reader.as_slice())?;
                let chunk = Arc::new(RwLock::new(chunk_data));
                let cloned = chunk.clone();
                self.sections.lock().await[index] = RegionChunk::Chunk(chunk);
                Ok(Some(cloned))
            }
            RegionChunk::Chunk(chunk) => Ok(Some(chunk.clone())),
        }
    }
    pub fn get_chunk_timestamp(&self, x: u8, z: u8) -> u32 {
        let index = ((x % 32) as usize) + ((z % 32) as usize) * 32;
        self.chunk_timestamps[index]
    }
}
