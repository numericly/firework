use convert_case::{Case, Casing};
use serde::Deserialize;
use std::str::FromStr;
use std::{
    collections::HashMap,
    fs,
    hash::{Hash, Hasher},
};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Block {
    id: i32,
    name: String,
    #[serde(rename = "displayName")]
    display_name: String,
    hardness: f32,
    resistance: f32,
    #[serde(rename = "stackSize")]
    stack_size: u8,
    diggable: bool,
    material: String,
    transparent: bool,
    #[serde(rename = "emitLight")]
    emit_light: u8,
    #[serde(rename = "filterLight")]
    filter_light: u8,
    #[serde(rename = "defaultState")]
    default_state: i32,
    #[serde(rename = "minStateId")]
    min_state_id: i32,
    #[serde(rename = "maxStateId")]
    max_state_id: i32,
    states: Vec<StateProperty>,
    drops: Vec<i32>,
    #[serde(rename = "boundingBox")]
    bounding_box: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct StateProperty {
    name: String,
    r#type: StatePropertyType,
    num_values: u32,
    values: Option<Vec<String>>,
}

impl Hash for StateProperty {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.r#type.hash(state);
        self.num_values.hash(state);

        if let Some(mut values) = self.values.clone() {
            let sorted_values = values.sort();
            sorted_values.hash(state);
        }
    }
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum StatePropertyType {
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "enum")]
    Enum,
}
pub fn build_blocks() {
    let debug = true;
    let file = fs::read_to_string("./data/1.19/blocks.json").expect("Unable to open blocks file");
    let blocks: Vec<Block> =
        serde_json::from_str(file.as_str()).expect("Failed to deserialize blocks");

    let mut block_rs = String::new();

    block_rs += "// This code was generated using data provided by PrismarineJS/minecraft-data\n\n";
    block_rs += "use crate::ConstrainedInt;\n";
    block_rs += "use crate::Values;\n";
    block_rs += "use crate::BlockProperties;\n";
    block_rs += "use serde::de::MapAccess;\n";
    block_rs += "use std::collections::HashMap;\n\n";

    let mut enums = HashMap::new();
    let mut properties = HashMap::new();

    block_rs += "#[derive(Debug, Hash, PartialEq, Eq)]\n";
    block_rs += "pub enum Block {";
    for block in &blocks {
        let name = block.name.to_case(Case::Pascal);
        block_rs += &format!("\n\t{}({}),", name, name);
    }

    // Map Block Enum to Block Properties
    {
        block_rs += "\n}\n\n";
        block_rs += "impl Block {\n";
        block_rs += "\tpub fn get_display_name(&self) -> &'static str {\n";
        block_rs += "\t\tmatch self {\n";
        for block in &blocks {
            let name = block.name.to_case(Case::Pascal);
            block_rs += &format!("\t\t\tBlock::{}(_) => {}::DISPLAY_NAME,\n", name, name);
        }
        block_rs += "\t\t}\n";
        block_rs += "\t}\n";
        block_rs += "\tpub fn get_transparency(&self) -> bool {\n";
        block_rs += "\t\tmatch self {\n";
        for block in &blocks {
            let name = block.name.to_case(Case::Pascal);
            block_rs += &format!("\t\t\tBlock::{}(_) => {}::TRANSPARENT,\n", name, name);
        }
        block_rs += "\t\t}\n";
        block_rs += "\t}\n";
        block_rs += "\tpub fn get_emit_light(&self) -> u8 {\n";
        block_rs += "\t\tmatch self {\n";
        for block in &blocks {
            let name = block.name.to_case(Case::Pascal);
            block_rs += &format!("\t\t\tBlock::{}(_) => {}::EMIT_LIGHT,\n", name, name);
        }
        block_rs += "\t\t}\n";
        block_rs += "\t}\n";
        block_rs += "\tpub fn get_filter_light(&self) -> u8 {\n";
        block_rs += "\t\tmatch self {\n";
        for block in &blocks {
            let name = block.name.to_case(Case::Pascal);
            block_rs += &format!("\t\t\tBlock::{}(_) => {}::FILTER_LIGHT,\n", name, name);
        }
        block_rs += "\t\t}\n";
        block_rs += "\t}\n";
        block_rs += "}\n\n";
    }

    for block in &blocks {
        for state in &block.states {
            match state.r#type {
                StatePropertyType::Bool => {
                    properties.insert(state, "bool".to_string());
                }
                StatePropertyType::Int => {
                    let numbers = state
                        .values
                        .as_ref()
                        .unwrap()
                        .iter()
                        .map(|v| i32::from_str(v).expect("Unable to parse number"))
                        .collect::<Vec<i32>>();
                    properties.insert(
                        state,
                        format!(
                            "ConstrainedInt::<{}, {}>",
                            numbers.iter().min().unwrap(),
                            numbers.iter().max().unwrap()
                        ),
                    );
                }
                StatePropertyType::Enum => {
                    let mut count: Option<u32> = None;
                    let enum_name = loop {
                        let name = if let Some(count) = count {
                            format!("{}{}", state.name, count).to_case(Case::Pascal)
                        } else {
                            state.name.clone().to_case(Case::Pascal)
                        };
                        let result = enums.get(&name);
                        let Some(values) = result else {
                            break name
                        };

                        if values == state.values.as_ref().unwrap() {
                            break name;
                        }

                        count = Some(count.unwrap_or(0) + 1);
                    };
                    enums.insert(enum_name.clone(), state.values.clone().unwrap());
                    properties.insert(state, format!("property_enums::{}", enum_name));
                }
            }
        }
    }

    block_rs += "pub fn deserialize_content<'de, T: MapAccess<'de>>(\n";
    block_rs += "\ttag: &str,\n";
    block_rs += "\tmap: Option<&HashMap<String, String>>,\n";
    block_rs += ") -> Result<Block, String> {\n";
    block_rs += "\tOk(match tag {\n";
    for block in &blocks {
        block_rs += &format!("\t\t\"minecraft:{}\" => ", block.name);
        if block.states.is_empty() {
            let name = block.name.to_case(Case::Pascal);
            block_rs += &format!("Block::{}({} {{}})", name, name);
        } else {
            block_rs += "{\n\t\t\t";
            block_rs +=
                "if map.is_none() { return Err(\"Missing block state\".to_owned()); }\n\t\t\t";
            block_rs += "let map = map.unwrap();\n";
            let name = block.name.to_case(Case::Pascal);
            block_rs += &format!("\t\t\tBlock::{}({} {{\n", name, name);
            for state in &block.states {
                block_rs += &format!("\t\t\t\tr#{}: ", state.name);
                // block_rs += &format!(
                //     "crate::get_prop(map, \"{}\")?.parse().map_err(|_| \"unable to parse\".to_string())?,\n",
                //     state.name
                // );
                block_rs += &format!("map.get(\"{}\").unwrap().parse().unwrap(),\n", state.name);
            }
            block_rs += "\t\t\t})\n";
            block_rs += "\t\t}";
        }
        block_rs += ",\n";
    }
    block_rs += "\t\t_ => return Err(format!(\"unknown block: {}\", tag)),\n";
    block_rs += "\t})\n";
    block_rs += "}\n";

    block_rs += "\npub mod property_enums {\n\t";
    block_rs += "use std::str::FromStr;\n\t";
    for (name, values) in enums {
        if debug {
            block_rs += "\n\t#[derive(Debug, Hash, PartialEq, Eq, Clone)]";
        }
        block_rs += &format!("\n\tpub enum {} {{", name);
        for value in &values {
            block_rs += &format!("\n\t\t{},", value);
        }
        block_rs += "\n\t}";

        block_rs += &format!("\n\timpl FromStr for {} {{", name);
        block_rs += &format!("\n\t\ttype Err = String;\n");
        block_rs += &format!("\n\t\tfn from_str(s: &str) -> Result<Self, Self::Err> {{");
        block_rs += &format!("\n\t\t\tmatch s {{");
        for value in &values {
            block_rs += &format!("\n\t\t\t\t\"{}\" => Ok({}::{}),", value, name, value);
        }
        block_rs += &format!(
            "\n\t\t\t\t_ => Err(format!(\"Invalid {} value: {{}}\", s)),",
            name
        );
        block_rs += "\n\t\t\t}\n";
        block_rs += "\t\t}\n";
        block_rs += "\t}\n";

        block_rs += &format!("\timpl crate::Values for {} {{", name);
        block_rs += "\n\t\ttype ValueIterator = std::vec::IntoIter<Self>;\n";

        block_rs += "\n\t\tfn possible_values() -> Self::ValueIterator {\n";

        block_rs += "\t\t\tvec![\n";
        for value in &values {
            block_rs += &format!("\t\t\t\tSelf::{},\n", value);
        }
        block_rs += "\t\t\t].into_iter()\n";
        block_rs += "\t\t}\n";
        block_rs += "\t}\n";
    }
    block_rs += "\n}\n";

    block_rs += "pub mod blocks_props {\n\n\t";
    block_rs += "use crate::BlockProperties;\n\t";
    block_rs += "use super::*;\t";
    for block in &blocks {
        block_rs += &format!(
            "\n\n\timpl BlockProperties for {} {{\n\t\t",
            block.name.to_case(Case::Pascal)
        );
        block_rs += &format!(
            "const DISPLAY_NAME: &'static str = \"{}\";\n\t\t",
            block.display_name
        );
        block_rs += &format!("const HARDNESS: f32 = {:?};\n\t\t", block.hardness);
        block_rs += &format!("const RESISTANCE: f32 = {:?};\n\t\t", block.resistance);
        block_rs += &format!("const STACK_SIZE: u8 = {};\n\t\t", block.stack_size);
        block_rs += &format!("const DIGGABLE: bool = {};\n\t\t", block.diggable);
        block_rs += &format!("const TRANSPARENT: bool = {};\n\t\t", block.transparent);
        block_rs += &format!("const EMIT_LIGHT: u8 = {};\n\t\t", block.emit_light);
        block_rs += &format!("const FILTER_LIGHT: u8 = {};\n\t", block.filter_light);
        block_rs += "}";
    }
    block_rs += "\n}\n";

    for block in &blocks {
        if debug {
            block_rs += "\n#[derive(Debug, Hash, PartialEq, Eq)]\n";
        }
        if block.states.is_empty() {
            block_rs += &format!("pub struct {};\n", block.name.to_case(Case::Pascal));
        } else {
            block_rs += &format!("pub struct {} {{\n", block.name.to_case(Case::Pascal));
            for state in &block.states {
                block_rs += &format!(
                    "\tpub r#{}: {},\n",
                    state.name,
                    properties.get(state).unwrap()
                );
            }
            block_rs += "}\n";
        }
    }

    block_rs += "\n\n";
    block_rs += "pub fn create_global_palette() -> HashMap<Block, usize> {\n";
    block_rs += "\tlet mut map = HashMap::new();\n";
    for block in &blocks {
        let mut nested = 1;
        for state in &block.states {
            for _ in 0..nested {
                block_rs += "\t";
            }

            let enum_name = properties.get(state).unwrap();

            block_rs += &format!(
                "for r#{} in {}::possible_values() {{\n",
                state.name, enum_name
            );

            nested += 1;
        }

        for _ in 0..nested {
            block_rs += "\t";
        }
        block_rs += &format!(
            "map.insert(Block::{} ( {} {{",
            block.name.to_case(Case::Pascal),
            block.name.to_case(Case::Pascal)
        );

        for state in &block.states {
            block_rs += &format!("r#{}: r#{}.clone(), ", state.name, state.name);
        }

        block_rs += "} ), map.len());\n";

        for i in (1..nested).rev() {
            for _ in 0..i {
                block_rs += "\t";
            }
            block_rs += "}\n";
        }
    }
    block_rs += "\tmap\n";
    block_rs += "}\n";

    fs::write("./src/blocks.rs", block_rs).unwrap();
}
