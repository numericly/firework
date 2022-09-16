use protocol::serializer::OutboundPacketData;
use quartz_nbt::{
    io::{self, Flavor},
    NbtCompound, NbtList, NbtTag,
};
use serde::Deserialize;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    convert::TryInto,
    fs,
    hash::{Hash, Hasher},
    io::{Cursor, Read},
    time::Instant,
};

#[derive(Debug, Deserialize)]
struct JsonPaletteElement {
    #[serde(rename = "block")]
    pub name: String,
    pub key: i32,
    pub properties: Vec<JsonPaletteProperty>,
}

#[derive(Debug, Hash, Deserialize)]
struct JsonPaletteProperty {
    pub name: String,
    pub value: String,
}

impl Hash for JsonPaletteElement {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.properties.hash(state);
    }
}

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

    let _chunk_sections: Vec<String> = Vec::new();

    let block_registry = {
        let file_content = fs::read_to_string("./blocks-palette.json").unwrap();

        let mut hashmap = HashMap::new();
        let json: Vec<JsonPaletteElement> = serde_json::from_str(&file_content).unwrap();

        for element in json.iter() {
            let mut hasher = DefaultHasher::new();
            element.hash(&mut hasher);
            hashmap.insert(hasher.finish(), element.key);
        }
        hashmap
    };

    for i in 0..sections.len() {
        let section = sections.get::<&NbtCompound>(i).unwrap();

        let biomes = match section.get::<_, &NbtCompound>("biomes") {
            Ok(block_data) => PalettedContainer::from_nbt(block_data, 64, &block_registry),
            Err(_) => {
                println!("An error occurred while reading biomes");
                continue;
            }
        };
        let block_states = match section.get::<_, &NbtCompound>("block_states") {
            Ok(block_data) => PalettedContainer::from_nbt(block_data, 4096, &block_registry),
            Err(_) => {
                println!("An error occurred while reading block states");
                continue;
            }
        };

        let mut data = OutboundPacketData::new();

        block_states.write_packet(&mut data);

        println!("Data {:?}", data.data);

        //println!("Block states: {:?}", block_states);
    }
}

#[derive(Debug)]
struct PalettedContainer<'a> {
    pub palette: Vec<PaletteElement>,
    pub data: Option<BitArray<'a>>,
    pub size: usize,
    pub registry: &'a HashMap<u64, i32>,
}

#[derive(Debug, Hash)]
struct PaletteElement {
    pub name: String,
    pub properties: Vec<PaletteProperty>,
}

#[derive(Debug, Hash)]
struct PaletteProperty {
    pub name: String,
    pub value: String,
}

impl PalettedContainer<'_> {
    fn from_nbt<'a>(
        nbt: &'a NbtCompound,
        size: usize,
        registry: &'a HashMap<u64, i32>,
    ) -> PalettedContainer<'a> {
        let data = match nbt.get::<_, &[i64]>("data") {
            Ok(data) => Some(BitArray::new(data, size.clone())),
            Err(_) => None,
        };
        let palette = {
            let palette_nbt = nbt.get::<_, &NbtList>("palette").unwrap();

            let mut palette = Vec::new();
            for i in 0..palette_nbt.len() {
                let palette_element_nbt = palette_nbt.get::<&NbtTag>(i).unwrap();

                let palette_element_deserialized = match palette_element_nbt {
                    NbtTag::Compound(element_data) => {
                        let name = element_data.get::<_, &String>("Name").unwrap();
                        let nbt_properties = element_data.get::<_, &NbtCompound>("Properties");
                        let mut properties = Vec::new();
                        if let Ok(properties_nbt) = nbt_properties {
                            for (key, value) in properties_nbt {
                                properties.push(PaletteProperty {
                                    name: key.clone(),
                                    value: value.to_string(),
                                });
                            }
                        }
                        PaletteElement {
                            name: name.clone(),
                            properties: properties,
                        }
                    }
                    NbtTag::String(name) => PaletteElement {
                        name: name.clone(),
                        properties: Vec::new(),
                    },
                    tag_type => {
                        panic!("Unknown Type {}", tag_type);
                    }
                };

                palette.push(palette_element_deserialized);
            }
            palette
        };

        PalettedContainer::new(data, palette, size, registry)
    }
    fn new<'a>(
        data: Option<BitArray<'a>>,
        palette: Vec<PaletteElement>,
        size: usize,
        registry: &'a HashMap<u64, i32>,
    ) -> PalettedContainer<'a> {
        PalettedContainer {
            palette,
            data,
            size,
            registry,
        }
    }
    fn write_packet(&self, packet_data: &mut OutboundPacketData) {
        // This is temporary, I'm just using this to test the packet writing
        packet_data.write_short(1000);

        match self.data.is_some() {
            true => {
                let bit_array = self.data.as_ref().unwrap();
                packet_data.write_unsigned_byte(bit_array.bits_per_value as u8);

                packet_data.write_var_int(self.palette.len() as i32);
                for i in 0..self.palette.len() {
                    let block_data = &self.palette[i];

                    let mut hasher = DefaultHasher::new();
                    block_data.hash(&mut hasher);
                    let hash = hasher.finish();
                    println!("Writing {}", self.registry.get(&hash).unwrap().clone());
                    packet_data.write_var_int(self.registry.get(&hash).unwrap().clone());
                }

                packet_data.write_var_int(bit_array.data.len() as i32);
                for i in 0..bit_array.data.len() {
                    packet_data.write_signed_long(bit_array.data[i]);
                    println!("Bits {:64b}", bit_array.data[i]);
                }
            }
            false => {
                packet_data.write_unsigned_byte(0);
                // Get from the registry IMPORTANT
                //packet_data.write_var_int(0);

                // Write empty long array
                packet_data.write_var_int(0);
            }
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
    fn get(&self, index: usize) -> usize {
        let values_per_long = 64 / self.bits_per_value;
        let long_index = index / values_per_long;
        let long_value = self.data[long_index];
        let value_index = index % values_per_long;
        ((long_value >> (value_index * self.bits_per_value)) & self.individual_value_mask) as usize
    }
}
