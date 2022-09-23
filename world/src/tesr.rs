use byteorder::{BigEndian, ReadBytesExt};
use nbt::{from_zlib_reader, Blob, Map, Value};
use serde::Deserialize;
use std::{
    io::{Cursor, Read},
    time::Instant,
};

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
    sections: Vec<SectionNBT>,
}

#[derive(Debug, Deserialize)]
pub struct SectionNBT {
    block_states: PalettedContainer<BlockPaletteElement>,
    biomes: PalettedContainer<String>,
    #[serde(rename = "SkyLight")]
    sky_light: Option<Vec<i8>>,
    #[serde(rename = "BlockLight")]
    block_light: Option<Vec<i8>>,
}

#[derive(Debug, Deserialize)]
pub struct PalettedContainer<T> {
    palette: Palette<T>,
    data: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Palette<T> {
    SingleValue(T),
    LinearValue(Vec<T>),
}

#[derive(Debug, Deserialize)]
pub struct BlockPaletteElement {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Properties")]
    properties: Option<Map<String, Value>>,
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
    pub fn get_chunk(&self, x: u8, z: u8) -> Result<Option<ChunkNBT>, String> {
        let index = ((x % 32) as usize) + ((z % 32) as usize) * 32;

        let offset = self.chunk_positions[index];

        if offset.size == 0 {
            return Ok(None);
        }

        let offset_bytes = ((offset.offset as usize - 2) * 4096) + 5;

        let mut cursor = Cursor::new(&self.data[offset_bytes..]);

        let nbt: ChunkNBT = from_zlib_reader(&mut cursor).map_err(|e| e.to_string())?;

        if let Palette::LinearValue(data) = &nbt.sections[0].block_states.palette {
            if data.len() > 16 {
                println!(
                    "Palette: {:?}",
                    &nbt.sections[0].block_states.data.as_ref().unwrap().len()
                );
                println!("Palette size: {:?}", data.len());
            }
        }

        Ok(Some(nbt))
    }
}

impl<T> PalettedContainer<T> {
    // pub fn get(&self, index: usize, container_size: usize) -> Option<&T> {
    //     match &self.palette {
    //         Palette::SingleValue(value) => Some(value),
    //         Palette::LinearValue(values) => {
    //             let data_index =
    //             values.get(index);
    //         }
    //     }
    // }
    fn bits_per_value(&self, container_size: usize) -> usize {
        match &self.palette {
            Palette::SingleValue(_) => 0,
            Palette::LinearValue(values) => values.len() * 64 / container_size,
        }
    }
}
