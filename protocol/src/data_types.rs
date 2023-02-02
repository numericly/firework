use authentication::ProfileProperty;
use modular_bitfield::bitfield;
use nbt::Blob;
use protocol_core::{Position, SerializeField, VarInt};
use protocol_derive::{DeserializeField, SerializeField};
use std::io::Write;

#[derive(Debug, PartialEq)]
pub struct DeathLocation {
    pub dimension_name: String,
    pub position: Position,
}

#[bitfield(bits = 4)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PlayerAbilityFlags {
    pub invulnerable: bool,
    pub flying: bool,
    pub allow_flying: bool,
    pub creative_mode: bool,
}

#[bitfield(bits = 5)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PlayerPositionFlags {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub pitch: bool,
    pub yaw: bool,
}

#[bitfield(bits = 7)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DisplaySkinParts {
    pub cape: bool,
    pub jacket: bool,
    pub left_sleeve: bool,
    pub right_sleeve: bool,
    pub left_pants_leg: bool,
    pub right_pants_leg: bool,
    pub hat: bool,
}

#[derive(Debug, PartialEq)]
pub struct Recipe {}

#[derive(Debug, PartialEq)]
pub struct SignatureData {
    pub timestamp: i64,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "protocol_core::VarInt")]
pub enum PlayerActionStatus {
    StartDigging,
    CancelDigging,
    FinishDigging,
    DropItemStack,
    DropItem,
    ShootArrowOrFinishEating,
    SwapItemInHand,
}

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "protocol_core::VarInt")]
pub enum BlockFace {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
}

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "protocol_core::VarInt")]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "protocol_core::VarInt")]
pub enum MainHand {
    Left,
    Right,
}

#[derive(Debug, PartialEq, DeserializeField, Clone)]
#[protocol(typ = "protocol_core::VarInt")]
pub enum Arm {
    Main,
    Off,
}

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "protocol_core::VarInt")]
pub enum PlayerCommandAction {
    StartSneaking,
    StopSneaking,
    LeaveBed,
    StartSprinting,
    StopSprinting,
    StartJumpWithHorse,
    StopJumpWithHorse,
    OpenHorseInventory,
    StartFlyingWithElytra,
}

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "protocol_core::VarInt")]
pub enum RecipeBookType {
    Crafting,
    Furnace,
    BlastFurnace,
    Smoker,
}

#[derive(Debug, PartialEq)]
pub struct BlockEntity {}

#[bitfield(bits = 6)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PlayerInfoActions {
    pub add_player: bool,
    pub init_chat: bool,
    pub update_gamemode: bool,
    pub update_listed: bool,
    pub update_latency: bool,
    pub update_display_name: bool,
}

#[derive(Debug, PartialEq)]
pub enum PlayerInfoAction {
    AddAllPlayers(Vec<(u128, AddPlayer, UpdateGameMode, UpdateListed, UpdateLatency)>),
    AddSinglePlayer(u128, AddPlayer, UpdateGameMode, UpdateListed, UpdateLatency),
}

impl SerializeField for PlayerInfoAction {
    fn serialize<W: Write>(&self, mut writer: W) {
        match self {
            PlayerInfoAction::AddAllPlayers(players) => {
                PlayerInfoActions::new()
                    .with_add_player(true)
                    .with_update_gamemode(true)
                    .with_update_listed(true)
                    .with_update_latency(true)
                    .bytes[0]
                    .serialize(&mut writer);

                VarInt::from(players.len() as i32).serialize(&mut writer);

                for (uuid, add_player, update_gamemode, update_listed, update_latency) in players {
                    uuid.serialize(&mut writer);
                    add_player.serialize(&mut writer);
                    update_gamemode.serialize(&mut writer);
                    update_listed.serialize(&mut writer);
                    update_latency.serialize(&mut writer);
                }
            }
            PlayerInfoAction::AddSinglePlayer(
                uuid,
                add_player,
                update_gamemode,
                update_listed,
                update_latency,
            ) => {
                PlayerInfoActions::new()
                    .with_add_player(true)
                    .with_update_gamemode(true)
                    .with_update_listed(true)
                    .with_update_latency(true)
                    .bytes[0]
                    .serialize(&mut writer);

                VarInt::from(1).serialize(&mut writer);

                uuid.serialize(&mut writer);
                add_player.serialize(&mut writer);
                update_gamemode.serialize(&mut writer);
                update_listed.serialize(&mut writer);
                update_latency.serialize(&mut writer);
            }
        }
    }
}

#[derive(Debug, PartialEq, SerializeField)]
pub struct AddPlayer {
    pub name: String,
    pub properties: Vec<ProfileProperty>,
}

#[derive(Debug, PartialEq, SerializeField)]
pub struct UpdateGameMode {
    pub gamemode: VarInt,
}

#[derive(Debug, PartialEq, SerializeField)]
pub struct UpdateListed {
    pub listed: bool,
}

#[derive(Debug, PartialEq, SerializeField)]
pub struct UpdateLatency {
    pub latency: VarInt,
}

#[derive(Debug, PartialEq, SerializeField, Clone)]
pub struct Slot {
    pub item_id: VarInt,
    pub item_count: u8,
    pub nbt: Blob,
}

impl SerializeField for DeathLocation {
    fn serialize<W: Write>(&self, mut writer: W) {
        self.dimension_name.serialize(&mut writer);
        self.position.serialize(writer);
    }
}

impl SerializeField for PlayerAbilityFlags {
    fn serialize<W: Write>(&self, writer: W) {
        self.into_bytes().serialize(writer);
    }
}

impl SerializeField for PlayerPositionFlags {
    fn serialize<W: Write>(&self, writer: W) {
        self.into_bytes().serialize(writer);
    }
}

impl SerializeField for Recipe {
    fn serialize<W: Write>(&self, _writer: W) {
        todo!()
    }
}

use std::{cell::Cell, fmt::Debug};

#[derive(Debug, PartialEq)]
pub struct CommandNode {
    pub node_type: NodeType,
    pub redirect: Option<Box<CommandNode>>,
    pub is_executable: bool,
    pub children: Vec<CommandNode>,

    node_index: Cell<Option<i32>>,
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Root,
    Literal {
        name: String,
    },
    Argument {
        name: String,
        parser: Parser,
        suggestions_type: Option<SuggestionsType>,
    },
}

#[derive(Debug, PartialEq, SerializeField)]
#[protocol(typ = "protocol_core::VarInt")]
pub enum Parser {
    Bool,
    Float(FloatProps),
}

#[derive(Debug, PartialEq)]
pub struct FloatProps {
    pub min: Option<f32>,
    pub max: Option<f32>,
}

impl SerializeField for FloatProps {
    fn serialize<W: std::io::Write>(&self, mut writer: W) {
        let flags = {
            let mut flags = 0x00u8;
            if self.min.is_some() {
                flags |= 0x01;
            }
            if self.max.is_some() {
                flags |= 0x02;
            }
            flags
        };
        flags.serialize(&mut writer);
        if let Some(min) = self.min {
            min.serialize(&mut writer);
        }
        if let Some(max) = self.max {
            max.serialize(&mut writer);
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SuggestionsType {
    AskServer,
    AllRecipes,
    AvailableSounds,
    AvailableBiomes,
    SummonableEntities,
}

impl SerializeField for CommandNode {
    fn serialize<W: std::io::Write>(&self, mut writer: W) {
        let mut start_index = 0;
        self.assign_index(&mut start_index);

        VarInt(start_index).serialize(&mut writer);

        self.write(&mut writer);

        VarInt(0).serialize(&mut writer);
    }
}

impl CommandNode {
    pub fn new(
        node_type: NodeType,
        redirect: Option<Box<CommandNode>>,
        is_executable: bool,
        children: Vec<CommandNode>,
    ) -> Self {
        CommandNode {
            node_type,
            redirect,
            is_executable,
            children,
            node_index: Cell::new(None),
        }
    }
    fn write<W: std::io::Write>(&self, mut writer: &mut W) {
        let flags = {
            let mut flags = 0x00u8;

            match self.node_type {
                NodeType::Root => flags |= 0x00,
                NodeType::Literal { .. } => flags |= 0x01,
                NodeType::Argument {
                    ref suggestions_type,
                    ..
                } => {
                    flags |= 0x02;
                    if let Some(_) = suggestions_type {
                        flags |= 0x10;
                    }
                }
            }

            if self.is_executable {
                flags |= 0x04;
            }

            if let Some(_) = self.redirect {
                flags |= 0x08;
            }

            flags
        };

        flags.serialize(&mut writer);

        VarInt(self.children.len() as i32).serialize(&mut writer);
        for child in &self.children {
            VarInt(child.node_index.get().expect("Node indexes not calculated"))
                .serialize(&mut writer);
        }

        if let Some(redirect) = &self.redirect {
            VarInt(
                redirect
                    .node_index
                    .get()
                    .expect("Node indexes not calculated"),
            )
            .serialize(&mut writer);
        }

        match &self.node_type {
            NodeType::Root => (),
            NodeType::Literal { name } => {
                name.serialize(&mut writer);
            }
            NodeType::Argument {
                name,
                parser,
                suggestions_type,
            } => {
                name.serialize(&mut writer);
                parser.serialize(&mut writer);
                if let Some(suggestions_type) = suggestions_type {
                    suggestions_type.serialize(&mut writer);
                }
            }
        }

        for child in &self.children {
            child.write(writer);
        }
    }
    fn assign_index(&self, current_index: &mut i32) {
        self.node_index.set(Some(*current_index));
        *current_index += 1;

        for child in &self.children {
            child.assign_index(current_index);
        }
    }
}

impl SerializeField for SuggestionsType {
    fn serialize<W: std::io::Write>(&self, mut writer: W) {
        match self {
            SuggestionsType::AskServer => {
                "minecraft:ask_server".to_string().serialize(&mut writer);
            }
            SuggestionsType::AllRecipes => {
                "minecraft:all_recipes".to_string().serialize(&mut writer);
            }
            SuggestionsType::AvailableSounds => {
                "minecraft:available_sounds"
                    .to_string()
                    .serialize(&mut writer);
            }
            SuggestionsType::AvailableBiomes => {
                "minecraft:available_biomes"
                    .to_string()
                    .serialize(&mut writer);
            }
            SuggestionsType::SummonableEntities => {
                "minecraft:summonable_entities"
                    .to_string()
                    .serialize(&mut writer);
            }
        }
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        let node = CommandNode::new(
            NodeType::Root,
            None,
            false,
            vec![CommandNode::new(
                NodeType::Literal {
                    name: "test".to_string(),
                },
                None,
                false,
                vec![],
            )],
        );
        let mut buffer = Vec::new();

        node.serialize(&mut buffer);

        println!("{:?}", buffer);
        panic!();
    }
}
