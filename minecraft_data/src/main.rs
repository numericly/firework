use minecraft_data::blocks::{create_global_palette, Block};

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
    let start = std::time::Instant::now();
    let block: Vec<Block> = serde_json::from_str(string).unwrap();
    println!("Time taken: {:?}", start.elapsed());

    println!("{:?}", block);

    let global_palette = create_global_palette();

    println!("{:?}", global_palette.len());
}
