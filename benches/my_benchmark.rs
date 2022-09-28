use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use world::tesr::Palette;

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("serialize region", |a| {
    //     a.iter(|| {
    //         let mut data = std::fs::File::open("./world/region/r.0.0.mca").unwrap();
    //         world::tesr::Region::deserialize(black_box(&mut data)).unwrap();
    //     })
    // });

    let mut data = std::fs::File::open("./world/region/r.0.0.mca").unwrap();
    let region = world::tesr::Region::deserialize(black_box(&mut data)).unwrap();

    // c.bench_function("read chunks", |a| {
    //     a.iter(|| {
    //         for z in 0..32 {
    //             for x in 0..32 {
    //                 region.get_chunk(x, z).unwrap();
    //             }
    //         }
    //     });
    // });

    let mut chunk = &region.get_chunk(0, 0).unwrap().unwrap().sections[0];

    let start = std::time::Instant::now();
    for x in 0..1 {
        for z in 0..1 {
            for section in region.get_chunk(x, z).unwrap().unwrap().sections {
                if let Palette::LinearValue(palette) = &section.block_states.palette {
                    for block in palette {
                        println!("{:?}", block);
                    }
                } else {
                    println!("{:?}", section.block_states.palette);
                }
                //black_box(section);
                // for i in 0..4096 {
                //     let block = section.block_states.get(i, 4096).unwrap();
                //     black_box(block);
                //     // if let Some(prop) = block.properties.as_ref() {
                //     //     if prop.len() > 1 {
                //     //         println!("{:?}", block);
                //     //     }
                //     // }
                // }
            }
        }
    }
    println!("Time: {:?}", start.elapsed());

    c.bench_function("read container", |a| {
        a.iter(|| {
            for i in 0..4096 {
                chunk.block_states.get(i, 4096);
            }
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
