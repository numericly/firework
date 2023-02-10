use std::env;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use minecraft_data::blocks::Block;
use nbt::from_zlib_reader;
use serde::Deserialize;
use std::hash::Hash;
use world::{RegionChunk, World};

fn criterion_benchmark(c: &mut Criterion) {
    env::set_var("RUST_BACKTRACE", "1");
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
        pub block_states: Option<PalettedContainer<Block, 4096>>,
    }

    #[derive(Debug, Deserialize, Clone)]
    pub struct PalettedContainer<T, const CONTAINER_SIZE: usize> {
        pub palette: Vec<T>,
        pub data: Option<Vec<i64>>,
    }

    const BITS_PER_ENTRY: usize = 64;

    impl<T, const CONTAINER_SIZE: usize> PalettedContainer<T, CONTAINER_SIZE>
    where
        T: std::fmt::Debug + Eq + Hash + Clone,
    {
        pub fn get(&self, index: usize) -> Option<&T> {
            let Some(data) = self.data.as_ref() else {
                if self.palette.len() != 1 {
                    panic!("Palette length is not 1");
                }
                return Some(&self.palette[0]);
            };
            let bits_per_value = data.len() * BITS_PER_ENTRY / CONTAINER_SIZE;
            let values_per_long = BITS_PER_ENTRY / bits_per_value;
            let array_index = index / values_per_long;
            let long = data[array_index];
            let offset = (index % values_per_long) * bits_per_value;
            let mask = (1 << bits_per_value) - 1;
            let value_index = (long >> offset) & mask as i64;
            Some(&self.palette[value_index as usize])
        }
        pub fn bits_per_value(&self) -> usize {
            if let Some(data) = self.data.as_ref() {
                data.len() * BITS_PER_ENTRY / CONTAINER_SIZE
            } else {
                0
            }
        }
    }

    let world = World::new("./world/region/", false);

    println!("Size of block {}", std::mem::size_of::<Block>());

    enum Data {
        None,
        Single(Vec<u8>),
        Double(Vec<u16>),
    }

    struct FastPalettedContainer<T, const CONTAINER_SIZE: usize> {
        pub palette: Vec<T>,
        pub data: Data,
    }

    impl<T, const CONTAINER_SIZE: usize> FastPalettedContainer<T, CONTAINER_SIZE>
    where
        T: std::fmt::Debug + Eq + Hash + Clone,
    {
        pub fn get(&self, index: usize) -> Option<&T> {
            match self.data {
                Data::None => {
                    if self.palette.len() != 1 {
                        panic!("Palette length is not 1");
                    }
                    return Some(&self.palette[0]);
                }
                Data::Single(ref data) => {
                    let Some(palette_index) = data.get(index) else {
                        return None;
                    };
                    self.palette.get(*palette_index as usize)
                }
                Data::Double(ref data) => {
                    let Some(paletted_index) = data.get(index) else {
                        return None;
                    };
                    self.palette.get(*paletted_index as usize)
                }
            }
        }
    }

    let region = world.get_region(0, 0).unwrap().unwrap();

    // let lock = region.sections.lock();
    // if let RegionChunk::ChunkBytes(bytes) = &lock.await[32] {
    //     let reader = bytes[5..].to_vec();
    //     let chunk_data: Chunk = from_zlib_reader(reader.as_slice()).unwrap();
    //     let blocks: &PalettedContainer<Block, 4096> =
    //         chunk_data.sections[1].block_states.as_ref().unwrap();
    // };

    // c.bench_function("deserialize chunk", |a| {
    //     a.iter(|| {
    //         let lock = region.sections.lock();
    //         if let RegionChunk::ChunkBytes(bytes) = &lock.unwrap()[32] {
    //             let reader = bytes[5..].to_vec();
    //             let chunk_data: Chunk = from_zlib_reader(reader.as_slice()).unwrap();
    //             let block = chunk_data.sections[1]
    //                 .block_states
    //                 .as_ref()
    //                 .unwrap()
    //                 .get(3337);
    //             println!("{:?}", block);
    //             black_box(chunk_data);
    //         };
    //     });
    // });
    c.bench_function("read from vec", |a| {
        let vec = {
            let mut vec = Vec::new();
            vec.push(rand::random::<u8>());
            vec
        };

        a.iter(|| {
            let mut blocks = 0u32;
            for i in &vec {
                if *i != 0 {
                    blocks += 1;
                }
            }
            black_box(blocks);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
