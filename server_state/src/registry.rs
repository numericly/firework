use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fs,
    hash::{Hash, Hasher},
};

use serde::Deserialize;

pub struct Registry {
    pub global_block_palette: HashMap<u64, i32>,
    pub global_biome_palette: HashMap<u64, i32>,
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            global_block_palette: create_global_block_palette(),
            global_biome_palette: create_global_biome_palette(),
        }
    }
}

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

fn create_global_block_palette() -> HashMap<u64, i32> {
    let file_content = fs::read_to_string("./block-palette.json").unwrap();

    let mut hashmap = HashMap::new();
    let json: Vec<JsonPaletteElement> = serde_json::from_str(&file_content).unwrap();

    for element in json.iter() {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        hashmap.insert(hasher.finish(), element.key);
    }
    hashmap
}

fn create_global_biome_palette() -> HashMap<u64, i32> {
    let file_content = fs::read_to_string("./biome-palette.json").unwrap();

    let mut hashmap = HashMap::new();
    let json: Vec<JsonPaletteElement> = serde_json::from_str(&file_content).unwrap();

    for element in json.iter() {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        hashmap.insert(hasher.finish(), element.key);
    }
    hashmap
}
