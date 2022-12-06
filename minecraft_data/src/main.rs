use std::{mem, time::Instant};

use minecraft_data::{blocks::Block, Palette};

fn main() {
    let string = r#"
    [
        {
          "Name": "minecraft:bedrock"
        },
        {
          "Name": "minecraft:deepslate",
          "Properties": {
            "axis": "y"
          }
        },
        {
          "Name": "minecraft:deepslate_redstone_ore",
          "Properties": {
            "lit": "false"
          }
        },
        {
          "Name": "minecraft:deepslate_gold_ore"
        },
        {
          "Name": "minecraft:deepslate_diamond_ore"
        },
        {
          "Name": "minecraft:tuff"
        },
        {
          "Name": "minecraft:deepslate_iron_ore"
        },
        {
          "Name": "minecraft:deepslate_lapis_ore"
        }
      ]
    "#;
    let start = Instant::now();
    let block: Vec<Block> = serde_json::from_str(string).unwrap();
    println!("Time taken: {:?}", start.elapsed());

    for block in &block {
        let start = Instant::now();
        print!("Id: {} for {:?}", block.get(), block);
        println!(" in: {:?}", start.elapsed());
        println!("Datasize: {}", mem::size_of::<Block>());
    }
}
