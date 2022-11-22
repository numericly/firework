use std::env;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use minecraft_data::blocks::Block;
use nbt::{from_zlib_reader, Blob, Map};
use serde::Deserialize;
use std::hash::Hash;
use world::world::{Region, RegionChunk};

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

    #[derive(Debug, Deserialize)]
    pub struct PalettedContainer<T, const CONTAINER_SIZE: usize> {
        pub palette: Vec<T>,
        pub data: Option<Vec<i64>>,
    }

    const BITS_PER_ENTRY: usize = 64;

    impl<T, const CONTAINER_SIZE: usize> PalettedContainer<T, CONTAINER_SIZE>
    where
        T: std::fmt::Debug + Eq + Hash,
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

    let world = world::world::World::new("./world/region/");

    let region = world.get_region(0, 0).unwrap().unwrap();

    c.bench_function("deserialize chunk", |a| {
        a.iter(|| {
            let lock = region.sections.lock();
            if let RegionChunk::ChunkBytes(bytes) = &lock.unwrap()[32] {
                let reader = bytes[5..].to_vec();
                let chunk_data: Chunk = from_zlib_reader(reader.as_slice()).unwrap();
                let block = chunk_data.sections[1]
                    .block_states
                    .as_ref()
                    .unwrap()
                    .get(3337);
                println!("{:?}", block);
                black_box(chunk_data);
            };
        });
    });

    // let data = std::fs::File::open("./world/region/r.0.0.mca").unwrap();
    // let region = Region::deserialize(data).unwrap();

    // for x in 0..1 {
    //     for z in 0..1 {
    //         let chunk = &region.get_chunk(x, z).unwrap();
    //         let mut data = Vec::new();
    //         chunk.as_ref().unwrap().write(&mut data);
    //         // for data in data.data {
    //         //     print!("{:08b} ", data);
    //         // }
    //         println!("{:?}", data.len());
    //         black_box(chunk);
    //     }
    // }

    // c.bench_function("deserialize chunk", |a| {
    //     a.iter(|| {
    //         for x in 0..32 {
    //             for z in 0..32 {
    //                 let chunk = &region.get_chunk(x, z).unwrap();
    //                 black_box(chunk);
    //             }
    //         }
    //     });
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
