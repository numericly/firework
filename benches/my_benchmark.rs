use criterion::{black_box, criterion_group, criterion_main, Criterion};
use world::world::Region;

fn criterion_benchmark(c: &mut Criterion) {
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
