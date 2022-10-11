use byteorder::{BigEndian, ReadBytesExt};
use data::data_1_19_2::BlockState;
use nbt::{from_zlib_reader, Blob, Value};
use serde;
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use std::sync::Arc;

#[derive(Debug)]
pub struct Region {
    chunk_positions: [ChunkOffset; 1024],
    chunk_timestamps: [u32; 1024],
    data: Vec<u8>,
    chunk_cache: RefCell<HashMap<ChunkPos, Arc<Chunk>>>,
}

#[derive(Clone, Copy, Debug)]
pub struct ChunkOffset {
    pub offset: u32,
    pub size: u8,
}

#[derive(Debug, Deserialize)]
pub struct Chunk {
    pub sections: Vec<ChunkSection>,
}

#[derive(Debug, Deserialize)]
pub struct ChunkSection {
    //pub block_states: HashMap<String, nbt::Value>,
    pub block_states: PalettedContainer<BlockState>,
    pub biomes: PalettedContainer<String>,
    #[serde(rename = "SkyLight")]
    pub sky_light: Option<Vec<i8>>,
    #[serde(rename = "BlockLight")]
    pub block_light: Option<Vec<i8>>,
}

#[derive(Debug, Deserialize)]
pub struct PalettedContainer<T> {
    pub palette: Palette<T>,
    data: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Palette<T> {
    SingleValue(T),
    LinearValue(Vec<T>),
}

#[derive(Debug, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: Value,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ChunkPos {
    pub x: u8,
    pub z: u8,
}

impl Region {
    pub fn deserialize<R>(mut reader: R) -> Result<Region, String>
    where
        R: Read,
    {
        let mut chunk_poses = [ChunkOffset { offset: 0, size: 0 }; 1024];

        for i in 0..1024 {
            let offset = reader.read_u24::<BigEndian>().map_err(|e| e.to_string())?;
            let size = reader.read_u8().map_err(|e| e.to_string())?;

            chunk_poses[i] = ChunkOffset { offset, size };
        }

        let mut chunk_times = [0; 1024];

        for i in 0..1024 {
            let timestamp = reader.read_u32::<BigEndian>().map_err(|e| e.to_string())?;
            chunk_times[i] = timestamp;
        }
        let mut data = Vec::new();
        reader.read_to_end(&mut data).map_err(|e| e.to_string())?;

        Ok(Region {
            chunk_positions: chunk_poses.as_slice().try_into().unwrap(),
            chunk_timestamps: chunk_times.as_slice().try_into().unwrap(),
            data,
            chunk_cache: RefCell::new(HashMap::new()),
        })
    }
    /// Gets and caches a chunk from the region at the given position.
    pub fn get_chunk<'a>(&'a self, x: u8, z: u8) -> Result<Option<Arc<Chunk>>, String> {
        let index = ((x % 32) as usize) + ((z % 32) as usize) * 32;

        let offset = self.chunk_positions[index];

        if offset.size == 0 {
            return Ok(None);
        }

        let pos = ChunkPos { x, z };

        let mut binding = self.chunk_cache.borrow_mut();
        let entry = match binding.entry(pos) {
            Entry::Vacant(e) => Arc::clone({
                let offset_bytes = ((offset.offset as usize - 2) * 4096) + 5;
                let mut reader = Cursor::new(&self.data[offset_bytes..]);
                let chunk = from_zlib_reader(&mut reader).map_err(|e| e.to_string())?;
                e.insert(Arc::new(chunk))
            }),
            Entry::Occupied(e) => Arc::clone(e.get()),
        };
        Ok(Some(entry))
    }
    pub fn get_chunk_timestamp(&self, x: u8, z: u8) -> u32 {
        let index = ((x % 32) as usize) + ((z % 32) as usize) * 32;
        self.chunk_timestamps[index]
    }
}

impl Chunk {
    pub fn get_block<'a>(&'a self, x: usize, y: usize, z: usize) -> Option<&'a BlockState> {
        let section = (y / 16) - SECTION_OFFSET;
        let section = &self.sections[section];

        //FIXME: This might not work
        let index = (x * 16 + z) * 16 + (y % 16);
        None
        //section.block_states.get(index, 4096)
    }
}

const SECTION_OFFSET: usize = 4;
const BITS_PER_ENTRY: usize = 64;

impl<T> PalettedContainer<T>
where
    T: std::fmt::Debug,
{
    pub fn get(&self, index: usize, container_size: usize) -> Option<&T> {
        match &self.palette {
            Palette::SingleValue(value) => Some(value),
            Palette::LinearValue(values) => {
                let bits_per_value = &self.bits_per_value(container_size);
                if bits_per_value == &0 && values.len() == 1 {
                    return Some(&values[0]);
                }
                let values_per_long = BITS_PER_ENTRY / bits_per_value;
                let array_index = index / values_per_long;
                let long = self.data.as_ref().unwrap()[array_index];
                let offset = (index % values_per_long) * bits_per_value;
                let mask = (1 << bits_per_value) - 1;
                let value_index = (long >> offset) & mask as i64;
                Some(&values[value_index as usize])
            }
        }
    }
    pub fn bits_per_value(&self, container_size: usize) -> usize {
        match &self.palette {
            Palette::SingleValue(_) => 0,
            Palette::LinearValue(_) => {
                if let Some(data) = self.data.as_ref() {
                    data.len() * BITS_PER_ENTRY / container_size
                } else {
                    0
                }
            }
        }
    }
}
