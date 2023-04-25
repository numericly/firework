use crate::WorldError;
use firework_data::{
    biomes::Biome,
    blocks::{Air, Block},
    Palette,
};
use firework_protocol::{client_bound::ChunkUpdateAndLightUpdate, data_types::BitSet};
use firework_protocol_core::{SerializeField, VarInt};
use nbt::{from_zlib_reader, Blob};
use serde::Deserialize;
use std::{cmp, fmt::Debug, hash::Hash};

#[derive(Deserialize, Debug)]
struct RawChunkData {
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    #[serde(rename = "xPos")]
    pub x: i32,
    #[serde(rename = "zPos")]
    pub z: i32,
    #[serde(rename = "InhabitedTime")]
    pub inhabited_time: i64,
    #[serde(rename = "LastUpdate")]
    pub last_update: i64,
    #[serde(rename = "PostProcessing")]
    pub _post_processing: Option<Vec<Vec<i16>>>,
    #[serde(rename = "Status")]
    pub _status: Option<String>,
    pub sections: Vec<RawChunkSection>,
    pub block_entities: Vec<BlockEntities>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "id")]
pub enum BlockEntities {
    #[serde(rename = "minecraft:chest")]
    Chest {
        x: i32,
        y: i32,
        z: i32,
        #[serde(rename = "Items")]
        #[serde(default = "Vec::new")]
        items: Vec<ContainerItemStack>,
    },
    #[serde(other)]
    Unknown,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ContainerItemStack {
    #[serde(rename = "id")]
    pub item: String,
    #[serde(rename = "Count")]
    pub count: u8,
    #[serde(rename = "Slot")]
    pub slot: u8,
}

#[derive(Debug, Deserialize)]
struct RawChunkSection {
    #[serde(rename = "Y")]
    y: i8,
    block_states: Option<CompactedPalettedContainer<Block, 4096>>,
    biomes: Option<CompactedPalettedContainer<Biome, 64>>,
    #[serde(rename = "SkyLight")]
    // #[serde(skip)]
    pub sky_light: Option<Vec<i8>>,
    #[serde(rename = "BlockLight")]
    // #[serde(skip)]
    pub block_light: Option<Vec<i8>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CompactedPalettedContainer<T, const CONTAINER_SIZE: usize> {
    palette: Vec<T>,
    data: Option<Vec<i64>>,
}

const BITS_PER_ENTRY: usize = 64;
impl<T, const CONTAINER_SIZE: usize, const MINIMUM_BITS: usize>
    Into<DirectPalettedContainer<T, CONTAINER_SIZE, MINIMUM_BITS>>
    for CompactedPalettedContainer<T, CONTAINER_SIZE>
where
    T: Debug,
{
    fn into(self) -> DirectPalettedContainer<T, CONTAINER_SIZE, MINIMUM_BITS> {
        let Some(data) = self.data else {
            if self.palette.len() != 1 {
                panic!("Palette length is not 1");
            }
            return DirectPalettedContainer {
                palette: self.palette,
                data: Data::None
            };
        };

        let bits_per_value = data.len() * BITS_PER_ENTRY / CONTAINER_SIZE;
        let values_per_long = BITS_PER_ENTRY / bits_per_value;

        if self.palette.len() < u8::MAX as usize {
            let mut new_data = Vec::with_capacity(CONTAINER_SIZE);
            for i in 0..CONTAINER_SIZE {
                let array_index = i / values_per_long;
                let long = data[array_index];
                let offset = (i % values_per_long) * bits_per_value;
                let mask = (1 << bits_per_value) - 1;
                let value_index = (long >> offset) & mask as i64;

                new_data.push(value_index as u8);
            }
            return DirectPalettedContainer {
                palette: self.palette,
                data: Data::Single(new_data),
            };
        } else {
            dbg!(&self.palette.len());
            let mut new_data = Vec::with_capacity(CONTAINER_SIZE);
            for i in 0..CONTAINER_SIZE {
                let array_index = i / values_per_long;
                let long = data[array_index];
                let offset = (i % values_per_long) * bits_per_value;
                let mask = (1 << bits_per_value) - 1;
                let value_index = (long >> offset) & mask as i64;

                new_data.push(value_index as u16);
            }
            return DirectPalettedContainer {
                palette: self.palette,
                data: Data::Double(new_data),
            };
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub data_version: i32,
    pub inhabited_time: i64,
    pub last_update: i64,
    pub x: i32,
    pub z: i32,
    section_offset: i8,
    sections: Vec<ChunkSection>,
}

#[derive(Debug)]
pub struct ChunkSection {
    block_states: Option<DirectPalettedContainer<Block, 4096, 4>>,
    biomes: Option<DirectPalettedContainer<Biome, 64, 3>>,
    sky_light: Option<Vec<i8>>,
    block_light: Option<Vec<i8>>,
    non_air_blocks: u16,
    y: i8,
}

trait Write {
    fn write(&self, packet_data: &mut Vec<u8>);
}

impl Chunk {
    pub fn from_zlib_reader(reader: &[u8]) -> Result<Chunk, WorldError> {
        let raw_chunk: RawChunkData = from_zlib_reader(reader)?;

        let mut section_offset = i8::MAX;
        let mut sections = Vec::new();

        for RawChunkSection {
            y,
            block_states,
            biomes,
            sky_light,
            block_light,
            ..
        } in raw_chunk.sections
        {
            if y < section_offset {
                section_offset = y
            }
            let block_states: Option<DirectPalettedContainer<Block, 4096, 4>> =
                if let Some(block_states) = block_states {
                    Some(block_states.into())
                } else {
                    None
                };

            let biomes = if let Some(biomes) = biomes {
                Some(biomes.into())
            } else {
                None
            };

            let non_air_blocks = if let Some(block_data) = &block_states {
                let mut non_air_blocks = 0u16;
                for i in 0..4096 {
                    let block = block_data.get(i);
                    if block != Some(&Block::Air(Air)) {
                        non_air_blocks += 1;
                    }
                }
                non_air_blocks
            } else {
                0
            };

            sections.push(ChunkSection {
                sky_light,
                block_light,
                block_states,
                biomes,
                non_air_blocks,
                y,
            });
        }

        Ok(Chunk {
            x: raw_chunk.x,
            z: raw_chunk.z,
            section_offset,
            sections,
            inhabited_time: raw_chunk.inhabited_time,
            last_update: raw_chunk.last_update,
            data_version: raw_chunk.data_version,
        })
    }
    pub fn into_packet(&self) -> ChunkUpdateAndLightUpdate {
        let mut packet_data = Vec::new();
        self.write(&mut packet_data);

        // in these masks, the least significant bit corresponds to the section underneath the bottom of the world
        // the most significant bit corresponds to the section above the max build height
        let mut sky_light_mask = BitSet::new();
        let mut block_light_mask = BitSet::new();
        let mut empty_sky_light_mask = BitSet::new();
        let mut empty_block_light_mask = BitSet::new();

        let mut sky_light = Vec::new();
        let mut block_light = Vec::new();

        for section in &self.sections {
            if section.sky_light.is_some() {
                sky_light_mask.set((section.y + 5) as usize, true);
                sky_light.push(section.sky_light.clone().unwrap());
            } else if section.sky_light == Some(vec![0; 2048]) {
                empty_sky_light_mask.set((section.y + 5) as usize, true);
            }
            if section.block_light.is_some() {
                block_light_mask.set((section.y + 5) as usize, true);
                block_light.push(section.block_light.clone().unwrap());
            } else if section.block_light == Some(vec![0; 2048]) {
                empty_block_light_mask.set((section.y + 5) as usize, true);
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
    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<&Block> {
        let section_index = (y >> 4) - self.section_offset as i32;
        if section_index < 0 || section_index >= self.sections.len() as i32 {
            return None;
        }
        let section = &self.sections[section_index as usize];
        let block_index = (x & 0xF) | ((z & 0xF) << 4) | ((y & 0xF) << 8);
        section
            .block_states
            .as_ref()
            .map(|x| x.get(block_index as usize))
            .unwrap()
    }
}

#[derive(Debug)]
enum Data {
    None,
    Single(Vec<u8>),
    Double(Vec<u16>),
}

#[derive(Debug)]
struct DirectPalettedContainer<T, const CONTAINER_SIZE: usize, const MINIMUM_BITS: usize> {
    palette: Vec<T>,
    data: Data,
}

impl<T, const CONTAINER_SIZE: usize, const MINIMUM_BITS: usize>
    DirectPalettedContainer<T, CONTAINER_SIZE, MINIMUM_BITS>
where
    T: std::fmt::Debug + Eq + Hash + Clone,
{
    pub fn get(&self, index: usize) -> Option<&T> {
        match self.data {
            Data::None => {
                if self.palette.len() != 1 {
                    panic!("Palette length is not 1");
                }
                return Some(&self.palette[0]);
            }
            Data::Single(ref data) => {
                let Some(palette_index) = data.get(index) else {
                    return None;
                };
                self.palette.get(*palette_index as usize)
            }
            Data::Double(ref data) => {
                let Some(paletted_index) = data.get(index) else {
                    return None;
                };
                self.palette.get(*paletted_index as usize)
            }
        }
    }
}

impl Write for Chunk {
    fn write(&self, packet_data: &mut Vec<u8>) {
        for section in &self.sections {
            section.write(packet_data);
        }
    }
}

impl Write for ChunkSection {
    fn write(&self, mut packet_data: &mut Vec<u8>) {
        if self.block_states.is_none() {
            return;
        }
        self.non_air_blocks.serialize(&mut packet_data);

        self.block_states.as_ref().unwrap().write(&mut packet_data);
        self.biomes.as_ref().unwrap().write(&mut packet_data);
    }
}

impl<T: Palette + Debug, const CONTAINER_SIZE: usize, const MINIMUM_BITS: usize> Write
    for DirectPalettedContainer<T, CONTAINER_SIZE, MINIMUM_BITS>
{
    fn write(&self, mut packet_data: &mut Vec<u8>) {
        match self.data {
            Data::None => {
                if self.palette.len() != 1 {
                    panic!("Palette length is not 1");
                }

                // Bits per value
                0u8.serialize(&mut packet_data);
                // We don't need to write the container size since the client now knows that there is only
                // one value in this paletted container

                // Paletted item
                let paletted_item = self.palette[0].get();
                VarInt(paletted_item).serialize(&mut packet_data);

                // Empty long array
                VarInt(0).serialize(&mut packet_data);
            }
            Data::Single(ref data) => {
                // Bits per value
                let bits_per_value = cmp::max(
                    MINIMUM_BITS as u8,
                    (self.palette.len() as f32).log2().ceil() as u8,
                );

                bits_per_value.serialize(&mut packet_data);

                // Palette size
                VarInt(self.palette.len() as i32).serialize(&mut packet_data);

                // Palette data
                for item in &self.palette {
                    VarInt(item.get()).serialize(&mut packet_data);
                }

                // Data array
                let values_per_long = BITS_PER_ENTRY as u8 / bits_per_value;
                let long_array_size =
                    (CONTAINER_SIZE as f32 / values_per_long as f32).ceil() as usize;
                let mask = 2u32.pow(bits_per_value as u32) - 1;
                let byte_mask = mask as u8;

                // Data array length
                VarInt(long_array_size as i32).serialize(&mut packet_data);

                for long_index in 0..long_array_size {
                    let mut long = 0u64;
                    for value_index in 0..values_per_long {
                        let index = long_index * values_per_long as usize + value_index as usize;
                        let Some(value) = data.get(index) else {
                            break;
                        };
                        let masked_value = value & byte_mask;
                        let shifted_value = (masked_value as u64) << (bits_per_value * value_index);
                        long |= shifted_value;
                    }
                    long.serialize(&mut packet_data);
                }
            }
            Data::Double(ref _data) => {
                unimplemented!(
                    "Double byte paletted container, size: {}",
                    self.palette.len()
                )
            }
        }
    }
}
