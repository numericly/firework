use minecraft_data::blocks::Block;

fn main() {
    let string = r#"
        [
            {
                "Name": "minecraft:grass_block",
                "Properties": {
                    "snowy": "true"
                }
            },
            {
                "Name": "minecraft:grass_block",
                "Properties": {
                    "snowy": "false"
                }
            },
            {
                "Name": "minecraft:grass_block",
                "Properties": {
                    "snowy": "false"
                }
            },
            {
                "Name": "minecraft:stone"
            },
            {
                "Name": "minecraft:stone"
            }
        ]
    "#;
    let block: Vec<Block> = serde_json::from_str(string).unwrap();

    println!("{:?}", block);
    println!("{:?}", block[0]);
}
