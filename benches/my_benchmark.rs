use criterion::{black_box, criterion_group, criterion_main, Criterion};
use world::{self, tesr::Region};

fn criterion_benchmark(c: &mut Criterion) {
    let mut data = std::fs::File::open("./world/region/r.0.0.mca").unwrap();
    let region = world::tesr::Region::deserialize(black_box(&mut data)).unwrap();

    let chunk = &region.get_chunk(0, 0).unwrap().unwrap().sections[0];

    let start = std::time::Instant::now();
    for x in 0..1 {
        for z in 0..1 {
            for section in &region.get_chunk(x, z).unwrap().unwrap().sections {
                println!("{:?}", section.block_states);
                black_box(section);
            }
        }
    }
    println!("Time: {:?}", start.elapsed());
    // c.bench_function("read container", |a| {
    //     a.iter(|| {
    //         for i in 0..4096 {
    //             //black_box(chunk.block_states.get(i, 4096));
    //         }
    //     });
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
