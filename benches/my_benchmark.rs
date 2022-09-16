use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fs,
    hash::{Hash, Hasher},
    mem::size_of_val,
    sync::Arc,
};

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use dashmap::DashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct JsonPaletteElement {
    #[serde(rename = "block")]
    pub name: String,
    pub key: usize,
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

fn criterion_benchmark(c: &mut Criterion) {
    //c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));

    let file_content = fs::read_to_string("./blocks-palette.json").unwrap();

    let mut hashmap = HashMap::new();
    let json: Arc<Vec<JsonPaletteElement>> = Arc::new(serde_json::from_str(&file_content).unwrap());

    for element in json.iter() {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        hashmap.insert(hasher.finish(), element.key);
    }

    c.bench_with_input(BenchmarkId::new("read file", "h"), &json, |b, json| {
        b.iter(|| {
            for element in json.iter() {
                let mut hasher = DefaultHasher::new();
                element.hash(&mut hasher);
                hashmap.insert(hasher.finish(), element.key);
            }
        })
    });

    println!("Dash map {:?}", hashmap);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
