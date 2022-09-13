use quartz_nbt::{
    io::{self, Flavor},
    NbtCompound, NbtList,
};
use std::{
    fs,
    io::{Cursor, Read},
};

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

#[derive(Debug)]
pub struct ChunkSection {
    pub block_count: i16,
    pub block_states: PalettedContainer,
    pub biomes: PalettedContainer,
}

#[derive(Debug)]
pub struct PalettedContainer {
    pub bits_per_entry: u8,
    //pub palette: todo(),
    pub data_length: i32, //varint
    pub data: Vec<i64>,
}

pub struct BlockEntity {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    //pub nbt: todo(),
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

pub fn read_region_file(file_path: String) -> Vec<Chunk> {
    let chunk_binary = fs::read(file_path.clone()).unwrap();

    //println!("Chunk binary: {:?}", &mut chunk_binary);

    let mut cursor = Cursor::new(chunk_binary);

    // println!("Cursor has {} lines", &cursor.lines().count());

    let mut chunk_positions = [0u8; 4096];
    cursor.read_exact(&mut chunk_positions).unwrap();

    let mut chunk_timestamps = [0u8; 4096];
    cursor.read_exact(&mut chunk_timestamps).unwrap();

    //println!("Chunk tables: {:x?}", buffer.len());

    let mut buffer2 = [0u8; 5];
    let _chunk_header = cursor.read_exact(&mut buffer2);

    //println!("Chunk header: {:x?}", buffer2);

    let chunks = vec![];

    let mut been_accessed = [0u8; 2000];

    //for i in 0..chunk_positions.len() / 4 {
    for i in 0..1 {
        // print!("{} ", chunk_positions[i*4] as u64 * 256 * 256 + chunk_positions[i*4+1] as u64 * 256 + chunk_positions[i*4+2] as u64);
        let chunk_binary = fs::read(file_path.clone()).unwrap();
        let cursor = Cursor::new(chunk_binary);
        read_chunk_nbt(
            cursor,
            8197 + 4096
                * (chunk_positions[i * 4] as u64 * 256 * 256
                    + chunk_positions[i * 4 + 1] as u64 * 256
                    + chunk_positions[i * 4 + 2] as u64),
        );
        been_accessed[(chunk_positions[i * 4] as u64 * 256 * 256
            + chunk_positions[i * 4 + 1] as u64 * 256
            + chunk_positions[i * 4 + 2] as u64) as usize] = 1;
    }

    // println!("");

    // println!("Been accessed: {:x?}", been_accessed);

    //println!("Chunk positions: {:?}", chunk_positions);

    chunks
}

fn read_chunk_nbt(mut cursor: Cursor<Vec<u8>>, pointer: u64) {
    cursor.set_position(pointer);
    let chunk_nbt = match io::read_nbt(&mut cursor, Flavor::ZlibCompressed) {
        //nbt data for the current chunk
        Ok(chunk_nbt) => chunk_nbt.0,
        Err(e) => panic!("Error reading NBT: {}", e),
    };

    let sections: &NbtList = chunk_nbt.get::<_, _>("sections").unwrap();

    for i in 0..1 {
        let section = sections.get::<&NbtCompound>(i).unwrap();
        let block_states = section.get::<_, &NbtCompound>("block_states").unwrap();
        let block_data = block_states.get::<_, &[i64]>("data").unwrap();
        let palette = block_states.get::<_, &NbtList>("palette").unwrap();

        const BITS_PER_BYTE: usize = 8;

        let bits_per_block =
            BITS_PER_BYTE * std::mem::size_of::<usize>() - (4usize.leading_zeros() as usize) - 1;

        println!("Bits per block: {}", bits_per_block);
        println!("Palette: {:?}", palette);

        const CHUNK_WIDTH: usize = 16;
        const CHUNK_HEIGHT: usize = 16;

        let individual_value_mask = (1 << bits_per_block) - 1;

        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_WIDTH {
                for x in 0..CHUNK_WIDTH {
                    let block_number: usize =
                        ((((y * CHUNK_HEIGHT) + z) * CHUNK_WIDTH) + x) as usize;
                    let start_long = 64 / bits_per_block;
                }
            }
        }
    }
}
