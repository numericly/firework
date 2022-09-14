use quartz_nbt::{
    io::{self, Flavor},
    NbtCompound, NbtList,
};
use std::{
    fs,
    io::{Cursor, Read},
    time::Instant,
};

pub fn read_region_file(file_path: String) -> Vec<String> {
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
            &mut cursor,
            file_header_size + (chunk_position * 4096) as u64,
        )
    }

    vec![]
}

fn read_chunk_nbt(cursor: &mut Cursor<Vec<u8>>, pointer: u64) {
    cursor.set_position(pointer);

    let chunk_nbt = match io::read_nbt(cursor, Flavor::ZlibCompressed) {
        //nbt data for the current chunk
        Ok(chunk_nbt) => chunk_nbt.0,
        Err(e) => {
            println!("Error reading NBT: {}", e);
            return;
        }
    };

    let sections: &NbtList = chunk_nbt.get::<_, _>("sections").unwrap();

    let mut chunk_sections: Vec<String> = Vec::new();

    for i in 0..sections.len() {
        let section = sections.get::<&NbtCompound>(i).unwrap();

        //println!("Section: {:?}", section);
        // let section = sections.get::<&NbtCompound>(i).unwrap();
        let biomes = match section.get::<_, &NbtCompound>("biomes") {
            Ok(block_data) => PalettedContainer::from_nbt(block_data, 4096),
            Err(_) => {
                println!("An error occurred while reading biomes");
                continue;
            }
        };
        let block_states = match section.get::<_, &NbtCompound>("block_states") {
            Ok(block_data) => PalettedContainer::from_nbt(block_data, 64),
            Err(_) => {
                println!("An error occurred while reading block states");
                continue;
            }
        };

        println!("Block states: {:?}", block_states);
        println!("Biomes: {:?}", biomes);

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
struct PalettedContainer<'a> {
    pub palette: &'a NbtList,
    pub data: Option<BitArray<'a>>,
    pub size: usize,
}

impl PalettedContainer<'_> {
    fn from_nbt<'a>(nbt: &NbtCompound, size: usize) -> PalettedContainer {
        let data = match nbt.get::<_, &[i64]>("data") {
            Ok(data) => Some(BitArray::new(data, size.clone())),
            Err(_) => None,
        };
        let palette = nbt.get::<_, &NbtList>("palette").unwrap();

        PalettedContainer::new(data, palette, size)
    }
    fn new<'a>(
        data: Option<BitArray<'a>>,
        palette: &'a NbtList,
        size: usize,
    ) -> PalettedContainer<'a> {
        PalettedContainer {
            palette,
            data,
            size,
        }
    }
    // fn get_3d(&self, x: usize, y: usize, z: usize, width: usize) -> usize {
    //     const SECTION_WIDTH: usize = 16;
    //     const SECTION_HEIGHT: usize = 16;
    //     let block_number: usize = ((((y * SECTION_HEIGHT) + z) * SECTION_WIDTH) + x) as usize;
    //     self.get(block_number)
    // }

    // BROKEN
    fn get(&self, index: usize) -> usize {
        match self.data {
            Some(ref data) => {
                // let bits_per_block = data.bits_per_value;
                // let block_id = data.data.get(index).unwrap();
                // let block_state = self
                //     .palette
                //     .get::<&NbtCompound>(block_id.clone() as usize)
                //     .unwrap();
                // let block_name = block_state.get::<_, &str>("Name").unwrap();
                // let block_name = block_name.replace("minecraft:", "");
                // println!("Block name: {}", block_name);
                // block_name.parse::<usize>().unwrap()
                0
            }
            None => 0,
        }
    }
}

#[derive(Debug)]
struct BitArray<'a> {
    pub data: &'a [i64],
    pub bits_per_value: usize,
    individual_value_mask: i64,
}

impl BitArray<'_> {
    fn new<'a>(data: &'a [i64], size: usize) -> BitArray<'a> {
        let bits_per_value = ((data.len() * 64) + size - 1) / size;
        let individual_value_mask = (1 << bits_per_value) - 1;
        BitArray {
            data,
            bits_per_value,
            individual_value_mask,
        }
    }
}
