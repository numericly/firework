use serde::Deserialize;

use super::{biomes::Biomes, blocks::state::BlockStates, data_structure::PalettedContainer};
use nbt::from_zlib_reader;

#[derive(Debug, Deserialize)]
pub struct Chunk {
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
    pub post_processing: Option<Vec<Vec<i16>>>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    pub sections: Vec<ChunkSection>,
}

#[derive(Debug, Deserialize)]
pub struct ChunkSection {
    pub block_states: Option<PalettedContainer<BlockStates, 4096>>,
    pub biomes: Option<PalettedContainer<Biomes, 64>>,
    #[serde(rename = "SkyLight")]
    pub sky_light: Option<Vec<i8>>,
    #[serde(rename = "BlockLight")]
    pub block_light: Option<Vec<i8>>,
}

impl Chunk {
    pub fn from_zlib_reader(reader: &[u8]) -> Result<Chunk, String> {
        from_zlib_reader(reader).map_err(|e| e.to_string())
    }
}
