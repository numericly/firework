use quartz_nbt::{
    io::{self, Flavor},
    NbtCompound, NbtList,
};
use std::{
    convert::TryInto,
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
    let start = std::time::Instant::now();
    let chunk_binary = fs::read(file_path.clone()).unwrap();
    println!("Read file in {}ms", start.elapsed().as_secs_f32() * 1000.0);

    let mut cursor = Cursor::new(chunk_binary);

    let mut chunk_positions = [0u8; 4096];
    cursor.read_exact(&mut chunk_positions).unwrap();

    let mut chunk_timestamps = [0u8; 4096];
    cursor.read_exact(&mut chunk_timestamps).unwrap();

    let mut buffer2 = [0u8; 5];
    let _chunk_header = cursor.read_exact(&mut buffer2).unwrap();

    //println!("Chunk header: {:?}", buffer2);

    println!("Cursor position: {}", cursor.position());

    let file_header_size = cursor.position();

    //for i in (0..32 * 32).step_by(4) {
    for i in 0..1 {
        let bytes = [
            chunk_positions[i + 2],
            chunk_positions[i + 1],
            chunk_positions[i],
            0,
        ];

        let chunk_position = u32::from_le_bytes(bytes);

        // println!(
        //     "Position: {}",
        //     file_header_size + (chunk_position * 4096) as u64,
        // );

        read_chunk_nbt(
            i,
            &mut cursor,
            file_header_size + (chunk_position * 4096) as u64,
        )
    }

    vec![]
}

fn read_chunk_nbt(i: usize, cursor: &mut Cursor<Vec<u8>>, pointer: u64) {
    cursor.set_position(pointer);

    let chunk_nbt = match io::read_nbt(cursor, Flavor::ZlibCompressed) {
        //nbt data for the current chunk
        Ok(chunk_nbt) => chunk_nbt.0,
        Err(e) => {
            println!("Error reading NBT: {}, inx {}", e, i);
            return;
        }
    };

    let sections: &NbtList = chunk_nbt.get::<_, _>("sections").unwrap();

    for i in 0..sections.len() {
        let section = sections.get::<&NbtCompound>(i).unwrap();

        println!("Section: {:?}", section);
        // let section = sections.get::<&NbtCompound>(i).unwrap();
        // let block_states = match section.get::<_, &NbtCompound>("block_states") {
        //     Ok(block_data) => block_data,
        //     Err(_) => {
        //         //println!("Section {} is empty", section);
        //         continue;
        //     }
        // };
        // let block_data = match block_states.get::<_, &[i64]>("data") {
        //     Ok(block_data) => block_data,
        //     Err(_) => {
        //         //println!("Section {} is empty", section);
        //         continue;
        //     }
        // };
        // let palette = block_states.get::<_, &NbtList>("palette").unwrap();
        // let bit_array = BitArray::new(block_data, 4096);

        // let bits_per_block = 4;

        // const CHUNK_WIDTH: usize = 16;
        // const CHUNK_HEIGHT: usize = 16;

        // //println!("Bits per block: {}", bits_per_block);
        // if palette.len() > 4 {
        //     println!("Bit array: {:?}", bit_array);
        //     println!("Palette: {:?}", palette);
        // }
    }
}

#[derive(Debug)]
struct BitArray<'a> {
    data: &'a [i64],
    bits_per_value: usize,
    individual_value_mask: i64,
}

impl BitArray<'_> {
    fn new<'a>(data: &'a [i64], size: usize) -> BitArray {
        let bits_per_value = ((data.len() * 64) + size - 1) / size;
        let individual_value_mask = (1 << bits_per_value) - 1;
        BitArray {
            data,
            bits_per_value,
            individual_value_mask,
        }
    }
    fn get_3d(&self, x: usize, y: usize, z: usize) -> usize {
        const SECTION_WIDTH: usize = 16;
        const SECTION_HEIGHT: usize = 16;
        let block_number: usize = ((((y * SECTION_HEIGHT) + z) * SECTION_WIDTH) + x) as usize;
        self.get(block_number)
    }
    fn get(&self, index: usize) -> usize {
        let values_per_long = 64 / self.bits_per_value;
        let long_index = index / values_per_long;
        let long_value = self.data[long_index];
        let value_index = index % values_per_long;
        ((long_value >> (value_index * self.bits_per_value)) & self.individual_value_mask) as usize
    }
}
