use byteorder::{BigEndian, ReadBytesExt};
use dashmap::DashMap;
use data::v1_19_2::blocks::state::BlockStates;
use data::v1_19_2::chunk::{Chunk, ChunkSection};
use data::v1_19_2::data_structure::{BitSet, PalettedContainer};
use data::v1_19_2::Palette;
use nbt::Blob;
use protocol::client_bound::{ChunkUpdateAndLightUpdate, SerializeField, SerializePacket};
use protocol::data_types::VarInt;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;

#[repr(u8)]
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
    sections: Mutex<[RegionChunk; 1024]>,
}

#[derive(Debug)]
pub enum RegionChunk {
    None,
    ChunkBytes(Box<Vec<u8>>),
    Chunk(Box<Arc<Chunk>>),
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
    pub fn get_chunk(&self, x: i32, z: i32) -> Result<Option<Arc<Chunk>>, String> {
        let region = self.get_region(x >> 5, z >> 5)?;

        if let Some(region) = region {
            Ok(region.get_chunk(x, z)?)
        } else {
            Ok(None)
        }
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
    pub fn get_chunk<'a>(&self, x: i32, z: i32) -> Result<Option<Arc<Chunk>>, String> {
        let index = x.rem_euclid(32) as usize + (z.rem_euclid(32) as usize * 32);

        let sections = self.sections.lock().unwrap();

        match &sections[index] {
            RegionChunk::None => Ok(None),
            RegionChunk::ChunkBytes(bytes) => {
                // Cut off the first 4 bits as they are the header and must be ignored
                // in order to deserialize the chunk.
                let reader = bytes[5..].to_vec();
                drop(sections);
                let chunk = Arc::new(Chunk::from_zlib_reader(reader.as_slice())?);
                let chunk_ref = Arc::clone(&chunk);
                self.sections.lock().unwrap()[index] = RegionChunk::Chunk(Box::new(chunk));
                Ok(Some(chunk_ref))
            }
            RegionChunk::Chunk(chunk) => Ok(Some(Arc::clone(&chunk))),
        }
    }
    pub fn get_chunk_timestamp(&self, x: u8, z: u8) -> u32 {
        let index = ((x % 32) as usize) + ((z % 32) as usize) * 32;
        self.chunk_timestamps[index]
    }
}

pub trait Write {
    fn write(&self, packet_data: &mut Vec<u8>);
}

impl Write for Chunk {
    fn write(&self, packet_data: &mut Vec<u8>) {
        //write section data
        for section in &self.sections {
            section.write(packet_data);
        }
    }
}

impl Write for ChunkSection {
    fn write(&self, mut packet_data: &mut Vec<u8>) {
        if self.block_states.is_none() {
            println!("No block states");
            return;
        }
        //FIXME: This code is here because I don't want to calculate the number of non-air blocks
        if let Some(block_states) = self.block_states.as_ref() {
            if let Some(block) = block_states.get(0) {
                if block == &BlockStates::Air && block_states.palette.len() == 1 {
                    0u16.serialize(&mut packet_data);
                } else {
                    4096u16.serialize(&mut packet_data);
                }
            } else {
                4096u16.serialize(&mut packet_data);
            }
        } else {
            0u16.serialize(&mut packet_data);
        }

        self.block_states.as_ref().unwrap().write(&mut packet_data);
        self.biomes.as_ref().unwrap().write(&mut packet_data);
    }
}

impl<T, const CONTAINER_SIZE: usize> Write for PalettedContainer<T, CONTAINER_SIZE>
where
    T: Hash + Palette + Eq + Debug,
{
    fn write(&self, mut packet_data: &mut Vec<u8>) {
        match &self.bits_per_value() {
            // If the container only contains one value send it to the client as a single value palette type
            0 => {
                // Bits per value
                0u8.serialize(&mut packet_data);
                // We don't need to write the container size since the client now knows that there is only
                // one value in this paletted container

                // Paletted item
                let paletted_item = T::get_palette(&self.palette[0]);
                VarInt(paletted_item).serialize(&mut packet_data);

                // Empty long array
                VarInt(0).serialize(&mut packet_data);
            }
            bits_per_value => {
                // Bits per value
                (*bits_per_value as u8).serialize(&mut packet_data);

                // Palette size
                VarInt(self.palette.len() as i32).serialize(&mut packet_data);
                // Palette data
                for item in &self.palette {
                    let paletted_item = T::get_palette(item);
                    VarInt(paletted_item).serialize(&mut packet_data);
                }

                // Container size
                VarInt(self.data.as_ref().unwrap().len() as i32).serialize(&mut packet_data);
                // Container data
                for item in self.data.as_ref().unwrap() {
                    item.serialize(&mut packet_data);
                }
            }
        }
    }
}

impl Write for BitSet {
    fn write(&self, buf: &mut Vec<u8>) {
        self.0.serialize(buf);
    }
}

impl ToPacket<ChunkUpdateAndLightUpdate> for Chunk {
    fn to_packet(&self) -> ChunkUpdateAndLightUpdate {
        let mut packet_data = Vec::new();
        self.write(&mut packet_data);
        let mut sky_light_refs = Vec::new();
        let mut block_light_refs = Vec::new();
        for i in 0..22 {
            sky_light_refs.push(&self.sections[i].sky_light);
            block_light_refs.push(&self.sections[i].block_light);
        }
        //create bitmasks
        let mut sky_light_mask = BitSet::new();
        let mut block_light_mask = BitSet::new();
        //height + 2
        for i in 0..22 {
            sky_light_mask.push(sky_light_refs[i].is_some());
            block_light_mask.push(block_light_refs[i].is_some());
        }
        let mut empty_sky_light_mask = BitSet::new();
        let mut empty_block_light_mask = BitSet::new();
        for i in 0..22 {
            empty_sky_light_mask.push(
                sky_light_refs[i].is_some()
                    && sky_light_refs[i].as_ref().unwrap().iter().all(|&x| x == 0),
            );
            empty_block_light_mask.push(
                block_light_refs[i].is_some()
                    && block_light_refs[i]
                        .as_ref()
                        .unwrap()
                        .iter()
                        .all(|&x| x == 0),
            );
        }
        // TODO: implement support for zeroed out chunks instead of sending empty light arrays
        //calculate outgoing lighting data
        let mut sky_light: Vec<Vec<i8>> = Vec::new();
        let mut block_light: Vec<Vec<i8>> = Vec::new();
        for i in 0..sky_light_refs.len() {
            if sky_light_mask.get(i) && !empty_sky_light_mask.get(i) {
                sky_light.push(sky_light_refs[i].as_ref().unwrap().clone());
            }
        }
        for i in 0..block_light_refs.len() {
            if block_light_mask.get(i) && !empty_block_light_mask.get(i) {
                block_light.push(block_light_refs[i].as_ref().unwrap().clone());
            }
        }
        ChunkUpdateAndLightUpdate {
            x: self.x,
            z: self.z,
            heightmaps: Blob::new(),
            data: packet_data,
            block_entities: Vec::new(),
            trust_edges: true,
            sky_light_mask,
            block_light_mask,
            empty_sky_light_mask,
            empty_block_light_mask,
            sky_light,
            block_light,
        }
    }
}
