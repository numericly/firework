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
    pub z: i32,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct RegionPos {
    pub x: i32,
    pub z: i32,
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
                x: (chunk_pos.x as f32 / 32.0).floor() as i32,
                z: (chunk_pos.z as f32 / 32.0).floor() as i32,
            };
            println!("region_pos: {:?}", region_pos);
            // Check if the region is cached and if not, load it
            let region = cached_regions.entry(region_pos).or_insert_with(|| {
                Region::new(format!(
                    "{}/r.{}.{}.mca",
                    self.path.clone(),
                    region_pos.x,
                    region_pos.z
                ))
            });

            //add the chunk to the return vector
            let chunk = region.get_chunk(chunk_pos.x, chunk_pos.z, self.registry);
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

            Region {
                data: data_cursor,
                chunk_positions,
                _chunk_timestamps: chunk_timestamps,
            }
        }
        pub fn get_chunk<'a>(
            &mut self,
            x: i32,
            z: i32,
            registry: &'a Registry,
        ) -> Result<Chunk<'a>, String> {
            let chunk_pos = (x.rem_euclid(32) as usize + (z.rem_euclid(32) as usize * 32)) * 4;

            let mut bytes = [0u8; 4];
            bytes[1..].clone_from_slice(&self.chunk_positions[chunk_pos..chunk_pos + 3]);

            let chunk_pos = u32::from_be_bytes(bytes);

            let file_position = 5 + (chunk_pos * 4096) as u64;

            self.data.set_position(file_position);

            let chunk_nbt = match io::read_nbt(&mut self.data, Flavor::ZlibCompressed) {
                //nbt data for the current chunk
                Ok(chunk_nbt) => chunk_nbt.0,
                Err(e) => {
                    return Err(format!("Error reading NBT: {e}"));
                }
            };
            Chunk::from_nbt(chunk_nbt, registry)
        }
    }

    pub mod chunk {
        use std::{
            collections::{hash_map::DefaultHasher, HashMap},
            convert::TryInto,
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
                //println!("{:?}", &chunk_nbt);
                drop(chunk_nbt);

                let mut chunk_sections: Vec<ChunkSection> = Vec::new();

                for i in 0..24 {
                    let section = sections.get::<&NbtCompound>(i).unwrap();

                    println!();
                    //println!("section: {:?}", section);

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

                    println!("at y level {:?}, ", section.get::<_, u8>("Y").unwrap());

                    let sky_light = match section.get::<_, &NbtTag>("SkyLight") {
                        Ok(sky_light) => Some(
                            (Vec::<i8>::try_from(sky_light.clone()).unwrap())
                                .try_into()
                                .unwrap(),
                        ),

                        Err(e) => {
                            println!("Probably no sky light in section: {:?}", e);
                            None
                        }
                    };
                    let block_light = match section.get::<_, &NbtTag>("BlockLight") {
                        Ok(block_light) => Some(
                            (Vec::<i8>::try_from(block_light.clone()).unwrap())
                                .try_into()
                                .unwrap(),
                        ),

                        Err(e) => {
                            println!("Probably no block light in section: {:?}", e);
                            None
                        }
                    };

                    if (block_light.is_some()) {
                        println!("block light: {:?}", block_light.unwrap());
                    }
                    if (sky_light.is_some()) {
                        println!("sky light: {:?}", sky_light.unwrap());
                    }

                    let chunk_section = ChunkSection {
                        block_states,
                        biomes,
                        sky_light,
                        block_light,
                    };
                    chunk_sections.push(chunk_section);
                }
                Ok(Chunk {
                    sections: chunk_sections,
                })
            }
            pub fn write(&self, packet: &mut OutboundPacketData) {
                for section in &self.sections {
                    section.write(packet);
                }
                packet.write_bytes(&vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ]);
            }

            pub fn get_sections(&self) -> &Vec<ChunkSection> {
                &self.sections
            }
        }

        #[derive(Debug)]
        pub struct ChunkSection<'a> {
            pub block_states: PalettedContainer<'a>,
            pub biomes: PalettedContainer<'a>,
            pub sky_light: Option<[i8; 2048]>,
            pub block_light: Option<[i8; 2048]>,
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
                                        properties.sort_by(|a, b| a.name.cmp(&b.name));
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
                    if self.palette.len() > 16 {
                        println!("Bit array {}", bit_array.bits_per_value)
                    }
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
