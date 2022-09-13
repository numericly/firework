
use std::{fs, io::{Cursor, BufRead, Read}};
use quartz_nbt::{io::{self, Flavor}, NbtTag};

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

    let chunk_binary = fs::read(file_path).unwrap();

    //println!("Chunk binary: {:?}", &mut chunk_binary);

    let mut cursor = Cursor::new(chunk_binary);

    // println!("Cursor has {} lines", &cursor.lines().count());


    let mut buffer = [0u8; 8192];
    let _chunk_tables = cursor.read_exact(&mut buffer);

    println!("Chunk tables: {:x?}", buffer.len());


    let mut buffer2 = [0u8; 5];
    let _chunk_header = cursor.read_exact(&mut buffer2);

    println!("Chunk header: {:x?}", buffer2);

    let nbt = match io::read_nbt(&mut cursor , Flavor::ZlibCompressed) {
        Ok(nbt) => nbt.0,
        Err(e) => panic!("Error reading NBT: {}", e),
    };

    println!{"NBT: {:?}", nbt};

    println!("nbt.1: {:?}", nbt.get::<_, &NbtTag>("sections").unwrap());

    let chunk = Chunk {
        x: nbt.get::<_, i32>("xPos").unwrap(),
        z: nbt.get::<_, i32>("zPos").unwrap(),
        sections: Vec::new(),
        biomes: Vec::new(),
        height_maps: Vec::new(),
        block_entities: Vec::new(),
        entities: Vec::new(),
        block_light: Vec::new(),
        sky_light: Vec::new(),
    };

    vec!(chunk)
}
