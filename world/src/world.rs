use self::region::{chunk::Chunk, Region};
use server_state::registry::Registry;
use std::collections::HashMap;

use std::hash::Hash;

pub struct World<'a> {
    pub path: String,
    pub loaded_chunks: HashMap<ChunkPos, Chunk<'a>>,
    registry: &'a Registry,
}

#[derive(Hash, Debug)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct RegionPos {
    pub x: i32,
    pub y: i32,
}

impl World<'_> {
    pub fn new<'a>(path: String, registry: &'a Registry) -> World<'a> {
        World {
            path,
            loaded_chunks: HashMap::new(),
            registry,
        }
    }
    pub fn get_chunks(&mut self, chunk_positions: Vec<ChunkPos>) -> Vec<Chunk> {
        let mut cached_regions: HashMap<RegionPos, Region> = HashMap::new();
        let mut return_chunks: Vec<Chunk> = Vec::new();
        for chunk_pos in chunk_positions {
            let region_pos = RegionPos {
                x: chunk_pos.x / 32,
                y: chunk_pos.y / 32,
            };

            let region = cached_regions.entry(region_pos).or_insert_with(|| {
                Region::new(format!(
                    "{}/r.{}.{}.mca",
                    self.path.clone(),
                    region_pos.x,
                    region_pos.y
                ))
            });

            let chunk = region.get_chunk(
                (chunk_pos.x % 32) as u8,
                (chunk_pos.y % 32) as u8,
                self.registry,
            );
            return_chunks.push(chunk.unwrap());
        }
        return_chunks
    }
}

pub mod region {
    use std::{
        fs,
        io::{Cursor, Read},
    };

    use quartz_nbt::io::{self, Flavor};
    use server_state::registry::Registry;

    use self::chunk::Chunk;

    #[derive(Debug)]
    pub struct Region {
        pub data: Cursor<Vec<u8>>,
        file_header_size: u64,
        chunk_positions: [u8; 4096],
        _chunk_timestamps: [u8; 4096],
    }

    impl Region {
        pub fn new(path: String) -> Region {
            let mut data_cursor = Cursor::new(fs::read(path.clone()).unwrap());

            let mut chunk_positions = [0u8; 4096];
            data_cursor.read_exact(&mut chunk_positions).unwrap();

            let mut chunk_timestamps = [0u8; 4096];
            data_cursor.read_exact(&mut chunk_timestamps).unwrap();

            let mut _header_buffer = [0u8; 5];
            data_cursor.read_exact(&mut _header_buffer).unwrap();

            Region {
                file_header_size: data_cursor.position(),
                data: data_cursor,
                chunk_positions,
                _chunk_timestamps: chunk_timestamps,
            }
        }
        pub fn get_chunk<'a>(
            &mut self,
            x: u8,
            y: u8,
            registry: &'a Registry,
        ) -> Result<Chunk<'a>, String> {
            let chunk_pos = (x as usize + y as usize * 32) * 4;

            let bytes = [
                self.chunk_positions[chunk_pos + 2],
                self.chunk_positions[chunk_pos + 1],
                self.chunk_positions[chunk_pos],
                0,
            ];

            let chunk_position = u32::from_le_bytes(bytes);

            let file_position = self.file_header_size + (chunk_position * 4096) as u64;

            self.data.set_position(file_position);

            let chunk_nbt = match io::read_nbt(&mut self.data, Flavor::ZlibCompressed) {
                //nbt data for the current chunk
                Ok(chunk_nbt) => chunk_nbt.0,
                Err(e) => {
                    return Err(format!("Error reading NBT {e}"));
                }
            };
            Chunk::from_nbt(chunk_nbt, registry)
        }
    }

    pub mod chunk {
        use std::{
            collections::{hash_map::DefaultHasher, HashMap},
            hash::{Hash, Hasher},
        };

        use protocol::serializer::OutboundPacketData;
        use quartz_nbt::{NbtCompound, NbtList, NbtTag};
        use server_state::registry::Registry;

        #[derive(Debug)]
        pub struct Chunk<'a> {
            pub sections: Vec<ChunkSection<'a>>,
        }

        impl Chunk<'_> {
            pub fn from_nbt<'a>(
                chunk_nbt: NbtCompound,
                registry: &'a Registry,
            ) -> Result<Chunk<'a>, String> {
                let sections = chunk_nbt.get::<_, &NbtList>("sections").unwrap().clone();
                drop(chunk_nbt);

                let mut chunk: Vec<ChunkSection> = Vec::new();

                for i in 0..24 {
                    let section = sections.get::<&NbtCompound>(i).unwrap();

                    let biomes = match section.get::<_, &NbtCompound>("biomes") {
                        Ok(block_data) => PalettedContainer::from_nbt(
                            block_data,
                            64,
                            &registry.global_biome_palette,
                        ),
                        Err(_) => {
                            println!("An error occurred while reading biomes");
                            continue;
                        }
                    };
                    let block_states = match section.get::<_, &NbtCompound>("block_states") {
                        Ok(block_data) => PalettedContainer::from_nbt(
                            block_data,
                            4096,
                            &registry.global_block_palette,
                        ),
                        Err(_) => {
                            println!("An error occurred while reading block states");
                            continue;
                        }
                    };

                    let chunk_section = ChunkSection {
                        block_states,
                        biomes,
                    };
                    chunk.push(chunk_section);
                }
                Ok(Chunk { sections: chunk })
            }
            pub fn write(&self, packet: &mut OutboundPacketData) {
                for section in &self.sections {
                    section.write(packet);
                }
                packet.write_bytes(&vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ]);
            }
        }

        #[derive(Debug)]
        pub struct ChunkSection<'a> {
            pub block_states: PalettedContainer<'a>,
            pub biomes: PalettedContainer<'a>,
        }

        impl ChunkSection<'_> {
            pub fn write(&self, packet: &mut OutboundPacketData) {
                if self.block_states.palette.len() > 1 {
                    packet.write_short(1024);
                } else {
                    packet.write_short(0);
                }

                self.block_states.write_packet(packet);
                self.biomes.write_packet(packet);
            }
        }

        #[derive(Debug)]
        pub struct PalettedContainer<'a> {
            pub palette: Vec<PaletteElement>,
            pub data: Option<BitArray>,
            pub size: usize,
            pub registry: &'a HashMap<u64, i32>,
        }

        #[derive(Debug)]
        pub struct PaletteElement {
            pub name: String,
            pub properties: Option<Vec<PaletteProperty>>,
        }

        #[derive(Debug, Hash)]
        pub struct PaletteProperty {
            pub name: String,
            pub value: String,
        }

        impl Hash for PaletteElement {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.name.hash(state);
                if let Some(properties) = &self.properties {
                    properties.hash(state);
                }
            }
        }

        impl PalettedContainer<'_> {
            fn from_nbt<'a>(
                nbt: &NbtCompound,
                size: usize,
                registry: &'a HashMap<u64, i32>,
            ) -> PalettedContainer<'a> {
                let data = match nbt.get::<_, &[i64]>("data") {
                    Ok(data) => Some(BitArray::new(data.to_vec(), size.clone())),
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
                                let nbt_properties =
                                    element_data.get::<_, &NbtCompound>("Properties");
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
                                    properties: if properties.len() > 0 {
                                        Some(properties)
                                    } else {
                                        None
                                    },
                                }
                            }
                            NbtTag::String(name) => PaletteElement {
                                name: name.clone(),
                                properties: None,
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
                data: Option<BitArray>,
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
                if let Some(bit_array) = self.data.as_ref() {
                    packet_data.write_unsigned_byte(bit_array.bits_per_value as u8);

                    packet_data.write_var_int(self.palette.len() as i32);
                    for i in 0..self.palette.len() {
                        let block_data = &self.palette[i];

                        let mut hasher = DefaultHasher::new();
                        block_data.hash(&mut hasher);
                        let hash = hasher.finish();
                        packet_data.write_var_int(self.registry.get(&hash).unwrap().clone());
                    }

                    packet_data.write_var_int(bit_array.data.len() as i32);
                    for i in 0..bit_array.data.len() {
                        packet_data.write_signed_long(bit_array.data[i]);
                    }
                } else {
                    packet_data.write_unsigned_byte(0);

                    let block_data = &self.palette[0];

                    let mut hasher = DefaultHasher::new();
                    block_data.hash(&mut hasher);
                    let hash = hasher.finish();

                    packet_data.write_var_int(self.registry.get(&hash).unwrap().clone());

                    // Write empty long array
                    packet_data.write_var_int(0);
                }
            }
        }

        #[derive(Debug)]
        pub struct BitArray {
            pub data: Vec<i64>,
            pub bits_per_value: usize,
            individual_value_mask: i64,
        }

        impl BitArray {
            fn new(data: Vec<i64>, size: usize) -> BitArray {
                let bits_per_value = ((data.len() * 64) + size - 1) / size;
                let individual_value_mask = (1 << bits_per_value) - 1;
                BitArray {
                    data,
                    bits_per_value,
                    individual_value_mask,
                }
            }
            pub fn get(&self, index: usize) -> usize {
                let values_per_long = 64 / self.bits_per_value;
                let long_index = index / values_per_long;
                let long_value = self.data[long_index];
                let value_index = index % values_per_long;
                ((long_value >> (value_index * self.bits_per_value)) & self.individual_value_mask)
                    as usize
            }
        }
    }
}
/*
#[derive(Debug, Deserialize)]
struct JsonPaletteElement {
    pub value: String,
    pub key: i32,
    pub properties: Option<Vec<JsonPaletteProperty>>,
}

#[derive(Debug, Hash, Deserialize)]
struct JsonPaletteProperty {
    pub name: String,
    pub value: String,
}

impl Hash for JsonPaletteElement {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        if let Some(properties) = &self.properties {
            properties.hash(state);
        }
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
        let file_content = fs::read_to_string("./block-palette.json").unwrap();

        let mut hashmap = HashMap::new();
        let json: Vec<JsonPaletteElement> = serde_json::from_str(&file_content).unwrap();

        for element in json.iter() {
            let mut hasher = DefaultHasher::new();
            element.hash(&mut hasher);
            hashmap.insert(hasher.finish(), element.key);
        }
        hashmap
    };

    let biome_registry = {
        let file_content = fs::read_to_string("./biome-palette.json").unwrap();

        let mut hashmap = HashMap::new();
        let json: Vec<JsonPaletteElement> = serde_json::from_str(&file_content).unwrap();

        for element in json.iter() {
            let mut hasher = DefaultHasher::new();
            element.hash(&mut hasher);
            hashmap.insert(hasher.finish(), element.key);
        }
        hashmap
    };

    let mut data = OutboundPacketData::new();

    for i in 0..24 {
        let section = sections.get::<&NbtCompound>(i).unwrap();

        let biomes = match section.get::<_, &NbtCompound>("biomes") {
            Ok(block_data) => PalettedContainer::from_nbt(block_data, 64, &biome_registry),
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

        //println!("Block states: {:?}", block_states);
    }

    // This is a stupid hack fix this later
    let end_buf = [0u8; 23];

    data.write_bytes(&end_buf);

    println!("Data {:?}", data.data);
}

#[derive(Debug)]
struct PalettedContainer<'a> {
    pub palette: Vec<PaletteElement>,
    pub data: Option<BitArray<'a>>,
    pub size: usize,
    pub registry: &'a HashMap<u64, i32>,
}

#[derive(Debug)]
struct PaletteElement {
    pub name: String,
    pub properties: Option<Vec<PaletteProperty>>,
}

#[derive(Debug, Hash)]
struct PaletteProperty {
    pub name: String,
    pub value: String,
}

impl Hash for PaletteElement {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        if let Some(properties) = &self.properties {
            properties.hash(state);
        }
    }
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
                            properties: if properties.len() > 0 {
                                Some(properties)
                            } else {
                                None
                            },
                        }
                    }
                    NbtTag::String(name) => PaletteElement {
                        name: name.clone(),
                        properties: None,
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
                    packet_data.write_var_int(self.registry.get(&hash).unwrap().clone());
                }

                packet_data.write_var_int(bit_array.data.len() as i32);
                for i in 0..bit_array.data.len() {
                    packet_data.write_signed_long(bit_array.data[i]);
                }
            }
            false => {
                packet_data.write_unsigned_byte(0);

                let block_data = &self.palette[0];

                let mut hasher = DefaultHasher::new();
                block_data.hash(&mut hasher);
                let hash = hasher.finish();

                packet_data.write_var_int(self.registry.get(&hash).unwrap().clone());

                // Write empty long array
                packet_data.write_var_int(0);
            }
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
*/
