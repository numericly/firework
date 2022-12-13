use byteorder::{BigEndian, ReadBytesExt};
use dashmap::DashMap;
use protocol::client_bound::{SerializePacket};
use protocol_core::{BitSet, SerializeField};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use crate::chunk::Chunk;
use std::sync::{Arc, RwLock, Mutex};

pub mod chunk;

pub enum Difficulty {
    Peaceful = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

pub struct World {
    pub path: &'static str,
    regions: DashMap<(i32, i32), Arc<Region>>,
    pub difficulty: RwLock<u8>,
    pub difficulty_locked: RwLock<bool>,
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
    pub fn new(path: &'static str) -> World {
        World {
            path,
            difficulty: RwLock::new(1),
            difficulty_locked: RwLock::new(false),
            regions: DashMap::new(),
        }
    }
    pub async fn get_chunk(&self, x: i32, z: i32) -> Result<Option<Arc<RwLock<Chunk>>>, String> {
        let region = self.get_region(x >> 5, z >> 5)?;

        if let Some(region) = region {
            Ok(region.get_chunk(x, z)?)
        } else {
            Ok(None)
        }
    }
    pub async fn get_chunk_from_pos(&self, x: i32, z: i32) -> Result<Option<Arc<RwLock<Chunk>>>, String> {
        self.get_chunk(x >> 4, z >> 4).await
    }
    pub fn get_region(&self, x: i32, z: i32) -> Result<Option<Arc<Region>>, String> {
        Ok(match self.regions.get(&(x, z)) {
            Some(region) => Some(region.clone()),
            None => {
                let file = match File::open(format!("{}/r.{}.{}.mca", self.path, x, z)) {
                    Ok(file) => file,
                    Err(err) => match err.kind() {
                        std::io::ErrorKind::NotFound => return Ok(None),
                        _ => return Err(format!("Error opening region file: {}", err)),
                    },
                };
                let region = Arc::new(Region::deserialize(file)?);
                let region_ref = region.clone();
                self.regions.insert((x, z), region);
                Some(region_ref)
            }
        })
    }
}
    

impl Region {
    pub fn deserialize(mut reader: File) -> Result<Region, String> {
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
                        _ => return Err(format!("Error reading unused data: {}", err)),
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
    pub fn get_chunk(& self, x: i32, z: i32) -> Result<Option<Arc<RwLock<Chunk>>>, String> {
        let index = x.rem_euclid(32) as usize + (z.rem_euclid(32) as usize * 32);

        let sections = self.sections.lock().unwrap();

        match &sections[index] {
            RegionChunk::None => Ok(None),
            RegionChunk::ChunkBytes(bytes) => {
                // Cut off the first 4 bits as they are the header and must be ignored
                // in order to deserialize the chunk.
                let reader = bytes[5..].to_vec();
                drop(sections);
                let chunk_data = crate::chunk::Chunk::from_zlib_reader(reader.as_slice()).unwrap();
                let chunk = Arc::new(RwLock::new(chunk_data));
                let cloned = chunk.clone();
                self.sections.lock().unwrap()[index] = RegionChunk::Chunk(chunk);
                Ok(Some(cloned))
            },
            RegionChunk::Chunk(chunk) => Ok(Some(chunk.clone())),
        }
    }
    pub fn get_chunk_timestamp(&self, x: u8, z: u8) -> u32 {
        let index = ((x % 32) as usize) + ((z % 32) as usize) * 32;
        self.chunk_timestamps[index]
    }
}

trait Write {
    fn write(&self, packet_data: &mut Vec<u8>);
}

impl Write for BitSet {
    fn write(&self, buf: &mut Vec<u8>) {
        self.0.serialize(buf);
    }
}

pub mod test {


    #[tokio::test]
    async fn test() {
        println!("Test");

        let world = World::new("./region");

        let lock1 = world.get_region(0, 0).unwrap().unwrap();
        let lock = lock1.sections.lock().unwrap();
        let value = lock.get(0);
        let section = value.unwrap();

        if let RegionChunk::ChunkBytes(bytes) = section {
            // Cut off the first 4 bits as they are the header and must be ignored  
            // in order to deserialize the chunk.
            let reader = bytes[5..].to_vec();
            let chunk_data = crate::chunk::Chunk::from_zlib_reader(reader.as_slice()).unwrap();
            let chunk_data_old = ChunkNew::from(Chunk::from_zlib_reader(reader.as_slice()).unwrap());

            let mut packet_data = Vec::new();
            let start = std::time::Instant::now();

            chunk_data.write(&mut packet_data);
            println!("Chunk serialization took {:?}", start.elapsed());

            let new_end = start.elapsed();
            let start = std::time::Instant::now();
            let mut packet_data_old = Vec::new();
            chunk_data_old.write(&mut packet_data_old);
            println!("Chunk serialization took {:?}", start.elapsed());
            println!("Performance improvement: {:?}", start.elapsed().as_secs_f64() / new_end.as_secs_f64());

            assert_eq!(packet_data, packet_data_old);

            assert_eq!(ChunkUpdateAndLightUpdate::from(chunk_data), chunk_data_old.to_packet());
            // println!("{:?}",ChunkUpdateAndLightUpdate::from(chunk_data));
            // println!("{:?}", chunk_data_old.to_packet());
        }

        panic!("yo");
    }
}

