pub mod world {
    use std::fs;

    pub struct Chunk {
        pub x: i32,
        pub z: i32,
        pub sections: Vec<ChunkSection>,
        pub biomes: Vec<i32>,
        pub height_maps: Vec<i32>,
        pub block_entities: Vec<BlockEntity>,
        pub entities: Vec<Entity>,
        pub block_light: Vec<i32>,
        pub sky_light: Vec<i32>,
    }

    pub struct ChunkSection {
        pub block_count: i16,
        pub block_states: PalettedContainer,
        pub biomes: PalettedContainer,
    }

    pub struct PalettedContainer {
        pub bits_per_entry: u8,
        pub palette: Vec<u8>, // FIX THIS XAVIER
        pub data_length: i32, //varint
        pub data: Vec<i64>,
    }

    pub struct BlockEntity {
        pub x: i32,
        pub y: i32,
        pub z: i32,
        pub nbt: bool, // FIX THIS XAVIER
    }

    pub struct Entity {
        pub uuid: u128,
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub yaw: f32,
        pub pitch: f32,
        pub head_pitch: f32,
        pub velocity_x: f32,
        pub velocity_y: f32,
        pub velocity_z: f32,
        pub on_ground: bool,
        pub entity_id: i32,
        pub entity_type: i32,
        pub nbt: bool, // FIX THIS XAVIER
    }

    // fn read_chunk_file(file_path: String) -> Chunk {
    //     let chunk_binary = fs::read(file_path);
    // }
}
