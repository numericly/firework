use byteorder::{BigEndian, ReadBytesExt};
use nbt::{from_zlib_reader, Blob, Map, Value};
use serde::Deserialize;
use std::{
    io::{Cursor, Read},
    time::Instant,
};

use crate::materials::Materials;
use serde;

#[derive(Debug)]
pub struct Region {
    chunk_positions: [ChunkOffset; 1024],
    chunk_timestamps: [u32; 1024],
    data: Vec<u8>,
}

#[derive(Clone, Copy, Debug)]
pub struct ChunkOffset {
    pub offset: u32,
    pub size: u8,
}

#[derive(Debug, Deserialize)]
pub struct ChunkNBT {
    pub sections: Vec<SectionNBT>,
}

#[derive(Debug, Deserialize)]
pub struct SectionNBT {
    pub block_states: PalettedContainer<BlockPaletteElement>,
    biomes: PalettedContainer<String>,
    #[serde(rename = "SkyLight")]
    sky_light: Option<Vec<i8>>,
    #[serde(rename = "BlockLight")]
    block_light: Option<Vec<i8>>,
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
#[serde(tag = "Name", content = "Properties")]
pub enum Blocks {}

#[derive(Debug, Deserialize)]
pub struct BlockPaletteElement {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Properties")]
    pub properties: Option<Vec<Props>>,
    //pub properties: Option<Map<String, String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Props {
    Attached(#[serde(deserialize_with = "deserialize_str")] bool),
    Axis(#[serde(deserialize_with = "deserialize_str")] String),
    Lit(#[serde(deserialize_with = "deserialize_str")] bool),
    Snowy(#[serde(deserialize_with = "deserialize_str")] bool),
    Half(#[serde(deserialize_with = "deserialize_str")] String),
}

#[derive(Debug, Deserialize)]
pub struct PropVal(#[serde(deserialize_with = "deserialize_str")] pub String);

fn deserialize_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr + serde::de::Deserialize<'de> + std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.parse().unwrap())
}

#[derive(Debug, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: Value,
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
        })
    }
    pub fn get_chunk<'a>(&self, x: u8, z: u8) -> Result<Option<ChunkNBT>, String> {
        let index = ((x % 32) as usize) + ((z % 32) as usize) * 32;

        let offset = self.chunk_positions[index];

        if offset.size == 0 {
            return Ok(None);
        }

        let offset_bytes = ((offset.offset as usize - 2) * 4096) + 5;

        let mut cursor = Cursor::new(&self.data[offset_bytes..]);

        let nbt: ChunkNBT = from_zlib_reader(&mut cursor).map_err(|e| e.to_string())?;
        Ok(Some(nbt))
    }
}

impl ChunkNBT {
    pub fn get_block<'a>(
        &'a self,
        x: usize,
        y: usize,
        z: usize,
    ) -> Option<&'a BlockPaletteElement> {
        let section = (y / 16) - SECTION_OFFSET;
        let section = &self.sections[section];

        //FIXME: This might not work
        let index = (x * 16 + z) * 16 + (y % 16);

        section.block_states.get(index, 4096)
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
