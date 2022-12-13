use byteorder::{BigEndian, ReadBytesExt};
use dashmap::DashMap;
use minecraft_data::blocks::Block;
use protocol::client_bound::{SerializeField, SerializePacket};
use protocol::data_types::{BitSet};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::time::Duration;
use crate::chunk::Chunk;
use std::sync::{Arc, RwLock, Mutex};
use queues::{Queue, IsQueue};

// For lighting engine
const DIRECTIONS: [(i32, i32, i32); 6] = [
    (0, 0, -1),
    (0, 0, 1),
    (0, -1, 0),
    (0, 1, 0),
    (-1, 0, 0),
    (1, 0, 0),
];

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
    block_lighting_increases: Queue<LightUpdate>,
    block_lighting_decreases: Queue<LightUpdate>,
    sky_lighting_increases: Queue<LightUpdate>,
    sky_lighting_decreases: Queue<LightUpdate>,
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
            block_lighting_increases: Queue::new(),
            block_lighting_decreases: Queue::new(),
            sky_lighting_increases: Queue::new(),
            sky_lighting_decreases: Queue::new(),
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

    pub async fn increase_block_light(&mut self, x: i32, y: i32, z: i32, value: u8) {
        println!("block light increase called at {} {} {} with value {}", x, y, z, value);
        let start_time = std::time::Instant::now();
        self.get_chunk_from_pos(x, z).await.unwrap().unwrap().write().unwrap().set_block_light(x, y, z, value);
        self.block_lighting_increases.add(LightUpdate {x, y, z, value}).unwrap();
        println!("block light increase started in {:?}", start_time.elapsed());
        println!("asdfasdfasdf print test in {:?}", start_time.elapsed());
        self.propagate_block_light().await;
        println!("block light increase finished in {:?}", start_time.elapsed());
    }

    pub async fn decrease_block_light(&mut self, x: i32, y: i32, z: i32, value: u8) {
        self.block_lighting_decreases.add(LightUpdate {x, y, z, value}).unwrap();
        self.propagate_block_light().await;
    }

    pub async fn propagate_block_light(&mut self) {
        let mut tally = Duration::new(0, 0);
        let start = std::time::Instant::now();
        while !(self.block_lighting_decreases.size() == 0) {
            let light_decrease = self.block_lighting_decreases.remove().unwrap();
            todo!();
        }
        while !(self.block_lighting_increases.size() == 0) {
            let mut chunk_map: HashMap<(i32, i32), Arc<RwLock<Chunk>>> = HashMap::new();
            let light_increase = self.block_lighting_increases.remove().unwrap();
            for direction in DIRECTIONS {
                let (x, y, z) = direction;
                let (x, y, z) = (
                    light_increase.x + x,
                    light_increase.y + y,
                    light_increase.z + z,
                );
                let start = std::time::Instant::now();
                let chunk = match chunk_map.get(&(x >> 4, z >> 4)) {
                    Some(chunk) => {println!("it works????");chunk.clone()},
                    None => {
                        let chunk = match self.get_chunk_from_pos(x, z).await {
                            Ok(Some(chunk)) => chunk,
                            _ => continue,
                        };
                        chunk_map.insert((x >> 4, z >> 4), chunk.clone());
                        chunk
                    }
                };
                tally += start.elapsed();
                let mut chunk_lock = chunk.write().unwrap();
                let block = chunk_lock.get_block(x, y, z);
                let Some(block) = block else { continue; };
                let light = chunk_lock.get_block_light(x, y, z);
                let light = match light {
                    Some(light) => light,
                    None => 0,
                };
                if light != 1 && light < light_increase.value - 1 && block.get_transparency() {
                    chunk_lock.set_block_light(x, y, z, light_increase.value - 1);
                    self.block_lighting_increases.add(LightUpdate {
                        x,
                        y,
                        z,
                        value: light_increase.value - 1,
                    }).unwrap();
                }
            }
        }
        println!("block light propagation finished in {:?} with tally {:?}", start.elapsed(), tally);
    }
}

// An item in either the lighting increase or decrease queue for block or sky light, depending on which queue it is in
#[derive(Clone, Debug)]
struct LightUpdate {
    x: i32,
    y: i32,
    z: i32,
    value: u8,
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