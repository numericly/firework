use std::fs;

use convert_case::{Case, Casing};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Biome {
    id: i32,
    name: String,
    category: String,
    temperature: f32,
    has_precipitation: bool,
    dimension: String,
    #[serde(rename = "displayName")]
    display_name: String,
}

pub fn build_biomes() {
    let file = fs::read_to_string("./data/1.19.4/biomes.json").expect("Unable to open biomes file");
    let biomes: Vec<Biome> = serde_json::from_str(file.as_str()).unwrap();

    let mut biomes_rs = String::new();
    println!("cargo:warning={:#?}", biomes);

    biomes_rs += "use std::collections::HashMap;\n\n";

    biomes_rs += "#[derive(Debug, Clone, PartialEq, Eq, Hash)]\n";
    biomes_rs += "pub enum Biome {\n";
    for biome in &biomes {
        biomes_rs += &format!("\t{},\n", biome.name.to_case(Case::Pascal));
    }
    biomes_rs += "}\n\n";

    biomes_rs += "pub fn create_biome_map() -> HashMap<Biome, usize> {\n";
    biomes_rs += "\tlet mut map = HashMap::new();\n";
    for biome in &biomes {
        biomes_rs += &format!(
            "\tmap.insert(Biome::{}, {});\n",
            biome.name.to_case(Case::Pascal),
            biome.id
        );
    }
    biomes_rs += "\tmap\n";
    biomes_rs += "}\n";

    biomes_rs += "pub fn visit_str<Err>(__value: &str) -> serde::__private::Result<Biome, Err>
    where
        Err: serde::de::Error,
    {
        const FIELDS: &'static [&'static str] = &[];
        match __value {\n";
    for biome in &biomes {
        biomes_rs += &format!(
            "\t\t\t\"minecraft:{}\" => Ok(Biome::{}),\n",
            biome.name,
            biome.name.to_case(Case::Pascal)
        );
    }
    biomes_rs += "\t\t\tf => Err(serde::de::Error::unknown_field(f, FIELDS)),
        }
    }";

    fs::write("./src/biomes.rs", biomes_rs).expect("Unable to write biomes.rs");
}
