use std::sync::Arc;

use minecraft_data::items::{DiamondShovel, Elytra, IronSword, Item};
use nbt::Blob;
use protocol::{
    client_bound::{OpenScreen, SetContainerContent},
    data_types::Slot,
};
use protocol_core::VarInt;

use crate::{
    client::Client,
    server::{Server, ServerHandler},
};

// these structs are blank for now, but if you want to add data to them, you can
// insert buzzwords to justify this being a good idea
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
    pub fn handle_click<T: ServerHandler>(&self, slot: i16, client: &Client, server: &Server<T>) {
        match self {
            Gui::TestGui(gui) => gui.handle_click(slot, client, server),
            Gui::GameQueueMenuGui(gui) => gui.handle_click(slot, client, server),
        }
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
                    nbt: Blob::new(), //TODO name the item and lore
                                      /*
                                        {display:{Name:'[{"text":"Glide","italic":false}]',Lore:['[{"text":"Race against other players using elytras.","italic":false,"color":"gray"}]']},Enchantments:[{}]}
                                      */
                }),
                None,
                Some(Slot {
                    item_id: VarInt(IronSword::ID), // iron sword
                    item_count: 1,
                    nbt: Blob::new(),
                }),
                None,
                Some(Slot {
                    item_id: VarInt(DiamondShovel::ID), // diamond shovel
                    item_count: 1,
                    nbt: Blob::new(),
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
