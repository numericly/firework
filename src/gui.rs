use std::sync::Arc;

use minecraft_data::items::{DiamondShovel, Elytra, IronSword, Item};
use nbt::Blob;
use protocol::{
    client_bound::{OpenScreen, SetContainerContent},
    data_types::{ItemNbt, ItemNbtDisplay, Slot},
};
use protocol_core::VarInt;

use crate::{
    client::Client,
    server::{ConnectionError, Server, ServerHandler},
};

// these structs are blank for now, but if you want to add data to them, you can
#[derive(Debug)]
pub struct TestGui {}
#[derive(Debug)]
pub struct GameQueueMenuGui {}

pub trait GuiPackets {
    fn draw(&self) -> SetContainerContent;
    fn open(&self) -> OpenScreen;
    fn handle_click<T: ServerHandler>(&self, slot: i16, client: &Client, server: &Server<T>) {
        println!("Clicked on slot: {:?}", slot);
    }
}

#[derive(Debug)]
pub enum Gui {
    TestGui(TestGui),
    GameQueueMenuGui(GameQueueMenuGui),
}

impl Gui {
    pub async fn handle_click<T: ServerHandler>(
        &self,
        slot: i16,
        client: &Client,
        server: &Server<T>,
    ) -> Result<(), ConnectionError> {
        // this is stupid looking code but I'm not sure of any better ways to do this
        match self {
            Gui::TestGui(gui) => {
                gui.handle_click(slot, client, server);
                client.set_container_content(gui.draw()).await?;
            }
            Gui::GameQueueMenuGui(gui) => {
                gui.handle_click(slot, client, server);
                client.set_container_content(gui.draw()).await?;
            }
        }
        Ok(())
    }
}

enum WindowType {
    Generic9x1,
    Generic9x2,
    Generic9x3,
    Generic9x4,
    Generic9x5,
    Generic9x6,
    Generic3x3,
    Anvil,
    Beacon,
    BlastFurnace,
    BrewingStand,
    Crafting,
    Enchantment,
    Furnace,
    Grindstone,
    Hopper,
    Lectern,
    Loom,
    Merchant,
    ShulkerBox,
    Smithing,
    Smoker,
    CartographyTable,
    Stonecutter,
}

impl GuiPackets for TestGui {
    fn draw(&self) -> SetContainerContent {
        SetContainerContent {
            window_id: 42, // arbitrary number
            state_id: VarInt(0),
            items: vec![],
            held_item: None,
        }
    }

    fn open(&self) -> OpenScreen {
        OpenScreen {
            window_id: VarInt(42), // arbitrary number
            window_type: VarInt(WindowType::Generic9x2 as i32),
            title: r#"{"text":"foo","bold":true,"extra":[{"text":"bar"},{"text":"baz","bold":false},{"text":"qux","bold":true}]}"#.to_string(),
        }
    }
}

impl GuiPackets for GameQueueMenuGui {
    fn draw(&self) -> SetContainerContent {
        SetContainerContent {
            window_id: 42, // arbitrary number
            state_id: VarInt(0),
            // add the three items for each minigame with their respective names
            // elytra for glide
            // iron sword for combat
            // diamond shovel for tumble
            items: vec![
                None,
                None,
                Some(Slot {
                    item_id: VarInt(Elytra::ID), // elytra
                    item_count: 1,
                    nbt: ItemNbt {
                        display: Some(ItemNbtDisplay {
                            name: Some(r#"{"text":"Glide Minigame","italic":"false"}"#.to_string()),
                            lore: Some(vec![
                                r#"{"text":"Race other players through a course","italic":"false"}"#.to_string(),
                                r#"{"text":"with an elytra.","italic":"false"}"#.to_string()
                            ]),
                        }),
                    },
                }),
                None,
                Some(Slot {
                    item_id: VarInt(IronSword::ID), // iron sword
                    item_count: 1,
                    nbt: ItemNbt {
                        display: Some(ItemNbtDisplay {
                            name: Some(r#"{"text":"Battle Minigame","italic":"false"}"#.to_string()),
                            lore: Some(vec![
                                r#"{"text":"Battle your friends in an arena, getting","italic":"false"}"#.to_string(),
                                r#"{"text":"items to help you in the fight.","italic":"false"}"#.to_string()
                            ]),
                        }),
                    },
                }),
                None,
                Some(Slot {
                    item_id: VarInt(DiamondShovel::ID), // diamond shovel
                    item_count: 1,
                    nbt: ItemNbt {
                        display: Some(ItemNbtDisplay {
                            name: Some(r#"{"text":"Tumble Minigame","italic":"false"}"#.to_string()),
                            lore: Some(vec![
                                r#"{"text":"Throw snowballs to break the blocks underneath","italic":"false"}"#.to_string(),
                                r#"{"text":"other players' feet. Last one alive wins.","italic":"false"}"#.to_string()

                            ]),
                        }),
                    },
                }),
                None,
                None,
            ],
            held_item: None,
        }
    }

    fn open(&self) -> OpenScreen {
        OpenScreen {
            window_id: VarInt(42), // arbitrary number
            window_type: VarInt(WindowType::Generic9x1 as i32),
            title: r#"{"text":"      Minigame Selector","bold":true}"#.to_string(),
        }
    }

    fn handle_click<T: ServerHandler>(&self, slot: i16, client: &Client, server: &Server<T>) {
        match slot {
            2 => {
                println!("definitely joining glide game");
                //TODO send packet to server to join glide game
            }
            4 => {
                println!("definitely joining battle game");

                //TODO send packet to server to join battle game
            }
            6 => {
                println!("definitely joining tumble game");
                //TODO send packet to server to join tumble game
            }
            _ => {}
        }
    }
}
