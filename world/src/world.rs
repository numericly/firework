use byteorder::{BigEndian, ReadBytesExt};
use data::v1_19_2::chunk::{Chunk, ChunkSection};
use data::v1_19_2::data_structure::PalettedContainer;
use data::v1_19_2::Palette;
use protocol::serializer::OutboundPacketData;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::sync::{Arc, Mutex};

pub struct World {
    pub path: &'static str,
    regions: RefCell<HashMap<(i32, i32), Arc<Region>>>,
}

#[derive(Debug)]
pub struct Region {
    chunk_timestamps: [u32; 1024],
    sections: Mutex<[RegionChunk; 1024]>,
}

#[derive(Debug)]
pub enum RegionChunk {
    None,
    ChunkBytes(Box<Vec<u8>>),
    Chunk(Box<Arc<Chunk>>),
}

#[derive(Clone, Copy, Debug)]
pub struct ChunkOffset {
    pub offset: u32,
    pub size: u8,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ChunkPos {
    pub x: u8,
    pub z: u8,
}

impl World {
    pub fn new(path: &'static str) -> World {
        World {
            path,
            regions: RefCell::new(HashMap::new()),
        }
    }
    pub fn get_chunk(&self, x: i32, z: i32) -> Result<Option<Arc<Chunk>>, String> {
        let region = self.get_region(x >> 5, z >> 5)?;

        if let Some(region) = region {
            Ok(region.get_chunk(x, z)?)
        } else {
            Ok(None)
        }
    }
    pub fn get_region(&self, x: i32, z: i32) -> Result<Option<Arc<Region>>, String> {
        let mut region_cache = self.regions.borrow_mut();

        Ok(Some(match region_cache.entry((x, z)) {
            Entry::Occupied(region) => Arc::clone(region.get()),
            Entry::Vacant(entry) => Arc::clone({
                let file = match File::open(format!("{}/r.{}.{}.mca", self.path, x, z)) {
                    Ok(file) => file,
                    Err(err) => match err.kind() {
                        std::io::ErrorKind::NotFound => return Ok(None),
                        _ => return Err(format!("Error opening region file: {}", err)),
                    },
                };
                let region = Region::deserialize(file)?;
                entry.insert(Arc::new(region))
            }),
        }))
    }
}

impl Region {
    pub fn deserialize(mut reader: File) -> Result<Region, String> {
        #[derive(Debug)]
        struct ChunkInfo {
            size: u8,
            index: usize,
        }

        fn read_bytes_exact<R: Read>(reader: R, bytes: u64) -> Vec<u8> {
            let mut buf = Vec::with_capacity(bytes as usize);
            let mut section = reader.take(bytes);
            section.read_to_end(&mut buf).unwrap();
            buf
        }

        let mut info_map = HashMap::new();

        for i in 0..1024 {
            let offset = reader.read_u24::<BigEndian>().unwrap();
            let size = reader.read_u8().unwrap();
            if size != 0 {
                info_map.insert(offset, ChunkInfo { size, index: i });
            }
        }

        let mut chunk_times = [0; 1024];

        for i in 0..1024 {
            let timestamp = reader.read_u32::<BigEndian>().unwrap();
            chunk_times[i] = timestamp;
        }

        const INIT: RegionChunk = RegionChunk::None;
        let mut sections = [INIT; 1024];

        let mut pos = 2;
        loop {
            if let Some(info) = info_map.get(&pos) {
                let data = read_bytes_exact(&mut reader, info.size as u64 * 4096);

                sections[info.index] = RegionChunk::ChunkBytes(Box::new(data));

                pos += info.size as u32;
            } else {
                let mut unused_data = [0; 4096];
                if let Err(err) = reader.read_exact(&mut unused_data) {
                    match err.kind() {
                        std::io::ErrorKind::UnexpectedEof => break,
                        _ => return Err(format!("Error reading unused data: {}", err)),
                    }
                }
                pos += 1;
            }
        }

        Ok(Region {
            sections: Mutex::new(sections),
            chunk_timestamps: chunk_times.try_into().unwrap(),
        })
    }
    /// Gets and caches a chunk from the region at the given position.
    pub fn get_chunk<'a>(&self, x: i32, z: i32) -> Result<Option<Arc<Chunk>>, String> {
        let index = x.rem_euclid(32) as usize + (z.rem_euclid(32) as usize * 32);

        let sections = self.sections.lock().unwrap();

        match &sections[index] {
            RegionChunk::None => Ok(None),
            RegionChunk::ChunkBytes(bytes) => {
                // Cut off the first 4 bits as they are the header and must be ignored
                // in order to deserialize the chunk.
                let reader = bytes[5..].to_vec();
                drop(sections);
                let chunk = Arc::new(Chunk::from_zlib_reader(reader.as_slice())?);
                let chunk_ref = Arc::clone(&chunk);
                self.sections.lock().unwrap()[index] = RegionChunk::Chunk(Box::new(chunk));
                Ok(Some(chunk_ref))
            }
            RegionChunk::Chunk(chunk) => Ok(Some(Arc::clone(&chunk))),
        }
    }
    pub fn get_chunk_timestamp(&self, x: u8, z: u8) -> u32 {
        let index = ((x % 32) as usize) + ((z % 32) as usize) * 32;
        self.chunk_timestamps[index]
    }
}
pub trait Write {
    fn write(&self, packet_data: &mut OutboundPacketData);
}

impl Write for Chunk {
    fn write(&self, packet_data: &mut OutboundPacketData) {
        for section in &self.sections {
            section.write(packet_data);
        }
    }
}

impl Write for ChunkSection {
    fn write(&self, packet_data: &mut OutboundPacketData) {
        //FIXME: This code is here because I don't want to calculate the number of non-air blocks
        if self.block_states.as_ref().unwrap().palette.len() > 1 {
            packet_data.write_short(4096);
        } else {
            packet_data.write_short(0);
        }

        self.block_states.as_ref().unwrap().write(packet_data);
        self.biomes.as_ref().unwrap().write(packet_data);
    }
}

impl<T, const CONTAINER_SIZE: usize> Write for PalettedContainer<T, CONTAINER_SIZE>
where
    T: Hash + Palette + Eq + Debug,
{
    fn write(&self, packet_data: &mut OutboundPacketData) {
        match &self.bits_per_value() {
            // If the container only contains one value send it to the client as a single value palette type
            0 => {
                // Bits per value
                packet_data.write_unsigned_byte(0);
                // We don't need to write the container size since the client now knows that there is only
                // one value in this paletted container

                // Paletted item
                let paletted_item = T::get_palette(&self.palette[0]);
                packet_data.write_var_int(paletted_item);

                // Empty long array
                packet_data.write_var_int(0);
            }
            bits_per_value => {
                // Bits per value
                packet_data.write_unsigned_byte(*bits_per_value as u8);

                // Palette size
                packet_data.write_var_int(self.palette.len() as i32);
                // Palette data
                for item in &self.palette {
                    let paletted_item = T::get_palette(item);
                    packet_data.write_var_int(paletted_item);
                }

                // Container size
                packet_data.write_var_int(self.data.as_ref().unwrap().len() as i32);
                // Container data
                for item in self.data.as_ref().unwrap() {
                    packet_data.write_signed_long(item.clone());
                }
            }
        }
    }
}
