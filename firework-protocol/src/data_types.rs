use firework_authentication::ProfileProperty;
use firework_protocol_core::{
    DeserializeError, DeserializeField, Position, SerializeField, VarInt,
};
use firework_protocol_derive::{DeserializeField, SerializeField};
use modular_bitfield::bitfield;
use nbt::{de, ser};
use std::fmt::Debug;
use std::io::{Read, Write};

#[derive(PartialEq)]
pub struct BitSet {
    pub data: Vec<u64>,
    /// Number of bits in the BitSet
    size: usize,
}

impl BitSet {
    ///Create a new BitSet
    pub fn new() -> BitSet {
        BitSet {
            data: Vec::new(),
            size: 0,
        }
    }
    ///Set the bit at the given index
    pub fn set(&mut self, index: usize, value: bool) {
        let byte_index = index / 64;
        let bit_index = index % 64;
        if self.data.len() <= byte_index {
            self.data.resize(byte_index + 1, 0);
        }
        self.data[byte_index] |= (value as u64) << bit_index;
    }
    ///Get the bit at the given index
    pub fn get(&self, index: usize) -> bool {
        self.data
            .get(index / 64)
            .map_or(false, |&val| (val >> (index % 64)) & 1 == 1)
    }
    ///Push a bit to the end of the BitSet
    pub fn push(&mut self, value: bool) {
        self.set(self.size, value);
        self.size += 1;
    }
    pub fn resize(&mut self, new_size: usize, value: bool) {
        if new_size > self.size {
            for _ in self.size..new_size {
                self.push(value);
            }
        } else {
            self.size = new_size;
        }
    }
}

impl Debug for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size {
            write!(f, "{}", self.get(i) as u8)?;
            if i % 64 == 63 {
                write!(f, " ")?;
            } else if i % 8 == 7 {
                write!(f, "_")?;
            }
        }
        Ok(())
    }
}

impl SerializeField for BitSet {
    fn serialize<W: Write>(&self, mut writer: W) {
        self.data.serialize(&mut writer);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Particles {
    Dust {
        red: f32,
        green: f32,
        blue: f32,
        /// The scale, will be clamped between 0.01 and 4.
        scale: f32,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Particle {
    particle: Particles,
    long_distance: bool,
    x: f64,
    y: f64,
    z: f64,
    offset_x: f32,
    offset_y: f32,
    offset_z: f32,
    max_speed: f32,
    count: i32,
}

impl SerializeField for Particle {
    fn serialize<W: Write>(&self, mut writer: W) {
        #[allow(unused_variables)]
        VarInt::from(match self.particle {
            Particles::Dust {
                red,
                green,
                blue,
                scale,
            } => 14,
        })
        .serialize(&mut writer);

        self.long_distance.serialize(&mut writer);
        self.x.serialize(&mut writer);
        self.y.serialize(&mut writer);
        self.z.serialize(&mut writer);
        self.offset_x.serialize(&mut writer);
        self.offset_y.serialize(&mut writer);
        self.offset_z.serialize(&mut writer);
        self.max_speed.serialize(&mut writer);
        self.count.serialize(&mut writer);

        #[allow(unreachable_patterns)]
        match self.particle {
            Particles::Dust {
                red,
                green,
                blue,
                scale,
            } => {
                red.serialize(&mut writer);
                green.serialize(&mut writer);
                blue.serialize(&mut writer);
                scale.serialize(&mut writer);
            }
            _ => {}
        }
    }
}

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

#[derive(Debug, Default, serde::Serialize, PartialEq, serde::Deserialize, Clone)]
pub struct ItemNbt {
    pub display: Option<ItemNbtDisplay>,
}

#[derive(Debug, Default, serde::Serialize, PartialEq, serde::Deserialize, Clone)]
pub struct ItemNbtDisplay {
    #[serde(rename = "Name")]
    pub name: Option<String>, // TODO chat
    #[serde(rename = "Lore")]
    pub lore: Option<Vec<String>>, // TODO chat
}

impl SerializeField for ItemNbt {
    fn serialize<W: Write>(&self, mut writer: W) {
        ser::to_writer(&mut writer, &self, None).unwrap();
    }
}

impl DeserializeField for ItemNbt {
    fn deserialize<R: Read>(reader: R) -> Result<Self, DeserializeError> {
        Ok(de::from_reader(reader)?)
    }
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
#[protocol(typ = "firework_protocol_core::VarInt")]
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
#[protocol(typ = "firework_protocol_core::VarInt")]
pub enum BlockFace {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
}

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "firework_protocol_core::VarInt")]
pub enum ResourcePackLoadStatus {
    Success,
    Declined,
    FailedDownload,
    Accepted,
}

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "firework_protocol_core::VarInt")]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "firework_protocol_core::VarInt")]
pub enum MainHand {
    Left,
    Right,
}

#[derive(Debug, PartialEq, DeserializeField, Clone)]
#[protocol(typ = "firework_protocol_core::VarInt")]
pub enum Arm {
    Main,
    Off,
}

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "firework_protocol_core::VarInt")]
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
#[protocol(typ = "firework_protocol_core::VarInt")]
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

#[derive(Debug, PartialEq, SerializeField, Clone, DeserializeField)]
pub struct Slot {
    pub item_id: VarInt,
    pub item_count: u8,
    pub nbt: ItemNbt,
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

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "firework_protocol_core::VarInt")]

pub enum InventoryOperationMode {
    Click,
    ShiftClick,
    NumberKey,
    MiddleClick,
    Drop,
    Dragging,
    DoubleClick,
}

#[derive(Debug, PartialEq, DeserializeField)]
pub struct SlotUpdate {
    pub slot_number: i16,
    pub slot_value: Option<Slot>,
}

impl SerializeField for DisplaySkinParts {
    fn serialize<W: std::io::Write>(&self, mut writer: W) {
        self.bytes.serialize(&mut writer);
    }
}

#[derive(Debug, PartialEq)]
pub enum Attribute {
    MaxHealth { value: f64 },
}

impl SerializeField for Attribute {
    fn serialize<W: std::io::Write>(&self, mut writer: W) {
        match self {
            Attribute::MaxHealth { value } => {
                "generic.max_health".to_string().serialize(&mut writer);
                value.serialize(&mut writer);
                VarInt(0).serialize(&mut writer);
            }
        }
    }
}

pub mod commands {
    use async_recursion::async_recursion;
    use firework_protocol_core::{SerializeField, VarInt};
    use firework_protocol_derive::SerializeField;
    use std::{fmt::Debug, io::Write};
    use std::{future::Future, pin::Pin};
    use tokio::sync::Mutex;

    #[derive(Debug, PartialEq, SerializeField)]
    pub struct SuggestionMatch {
        pub r#match: String,
        pub tooltip: Option<String>,
    }

    pub struct CommandNode {
        pub node_type: NodeType,
        pub redirect: Option<Box<CommandNode>>,
        pub execution:
            Option<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>>,
        pub children: Vec<CommandNode>,

        node_index: Mutex<Option<i32>>,
    }

    #[derive(Debug, PartialEq)]
    pub enum NodeType {
        Root,
        Literal { name: String },
        Argument { name: String, parser: ArgumentType },
    }

    #[derive(Debug, PartialEq)]
    pub enum SuggestionsType {
        AskServer,
        AllRecipes,
        AvailableSounds,
        AvailableBiomes,
        SummonableEntities,
    }

    #[derive(SerializeField, Debug, PartialEq)]
    #[protocol(typ = "firework_protocol_core::VarInt")]
    pub enum StringTypes {
        SingleWord,
        QuotablePhrase,
        GreedyPhrase,
    }

    #[derive(Debug, PartialEq)]
    pub enum ArgumentType {
        Bool,
        Float {
            min: Option<f32>,
            max: Option<f32>,
        },
        String {
            string_type: StringTypes,
            suggestions: Option<Vec<String>>,
        },
    }

    impl CommandNode {
        pub fn root() -> Self {
            Self {
                node_type: NodeType::Root,
                redirect: None,
                execution: None,
                children: Vec::new(),
                node_index: Mutex::new(None),
            }
        }
        pub fn literal(name: &'static str) -> Self {
            Self {
                node_type: NodeType::Literal {
                    name: name.to_string(),
                },
                redirect: None,
                execution: None,
                children: Vec::new(),
                node_index: Mutex::new(None),
            }
        }
        pub fn argument(name: &'static str, argument: ArgumentType) -> Self {
            Self {
                node_type: NodeType::Argument {
                    name: name.to_string(),
                    parser: argument,
                },
                redirect: None,
                execution: None,
                children: Vec::new(),
                node_index: Mutex::new(None),
            }
        }
        pub fn sub_command(mut self, node: CommandNode) -> Self {
            self.children.push(node);
            self
        }
        pub fn executable<
            T: Fn() -> F + 'static + Send + Sync,
            F: Future<Output = ()> + Send + 'static,
        >(
            mut self,
            exec: &'static T,
        ) -> Self {
            self.execution = Some(Box::new(|| Box::pin(exec())));
            self
        }
        pub async fn serialize<W: Write + Send + Sync>(&self, mut writer: W) {
            let mut start_index = 0;
            self.assign_index(&mut start_index).await;

            VarInt(start_index).serialize(&mut writer);

            self.write(&mut writer).await;

            VarInt(0).serialize(&mut writer);
        }
        #[async_recursion]
        pub async fn suggestions(&self, input: &str, index: usize) {
            match &self.node_type {
                NodeType::Root => {
                    if input.len() == 0 {
                        return;
                    }
                    for child in &self.children {
                        child.suggestions(&input[1..], 1).await;
                    }
                }
                NodeType::Literal { name } => {
                    if input.len() == 0 {
                        return;
                    }
                    let (data, next_data) = if let Some((data, next_data)) = input.split_once(" ") {
                        (data, Some(next_data))
                    } else {
                        (input, None)
                    };
                    if data == name {
                        if let Some(next_data) = next_data {
                            for child in &self.children {
                                child.suggestions(next_data, index + data.len() + 1).await;
                            }
                        } else if let Some(exec) = &self.execution {
                            exec().await;
                        }
                    }
                }
                NodeType::Argument { name, parser } => match parser {
                    ArgumentType::String {
                        string_type,
                        suggestions,
                    } => {
                        match string_type {
                            StringTypes::SingleWord => {
                                let (data, next_data) =
                                    if let Some((data, next_data)) = input.split_once(" ") {
                                        (data, Some(next_data))
                                    } else {
                                        (input, None)
                                    };
                            }
                        }
                        println!("Index: {}", index);
                        println!("Checking string: {}", input);
                        println!("Suggestions: {:?}", suggestions);
                        // return Ok((index, index + input.len(), Vec::new()));
                    }
                    _ => {}
                },
            }
        }
        #[async_recursion]
        async fn write<W: Write + Send + Sync>(&self, mut writer: &mut W) {
            let flags = {
                let mut flags = 0x00u8;

                match &self.node_type {
                    NodeType::Root => flags |= 0x00,
                    NodeType::Literal { .. } => flags |= 0x01,
                    NodeType::Argument { parser, .. } => {
                        flags |= 0x02;
                        match parser {
                            ArgumentType::String { suggestions, .. } => {
                                if suggestions.is_some() {
                                    flags |= 0x10
                                }
                            }
                            _ => {}
                        }
                    }
                }

                if self.execution.is_some() {
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
                VarInt(
                    child
                        .node_index
                        .lock()
                        .await
                        .expect("Node indexes not calculated"),
                )
                .serialize(&mut writer);
            }

            if let Some(redirect) = &self.redirect {
                VarInt(
                    redirect
                        .node_index
                        .lock()
                        .await
                        .expect("Node indexes not calculated"),
                )
                .serialize(&mut writer);
            }

            match &self.node_type {
                NodeType::Root => (),
                NodeType::Literal { name } => {
                    name.serialize(&mut writer);
                }
                NodeType::Argument { name, parser } => {
                    name.serialize(&mut writer);
                    parser.serialize(&mut writer);
                }
            }

            for child in &self.children {
                child.write(writer).await;
            }
        }
        #[async_recursion]
        async fn assign_index(&self, current_index: &mut i32) {
            self.node_index.lock().await.replace(*current_index);
            *current_index += 1;

            for child in &self.children {
                child.assign_index(current_index).await;
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

    impl SerializeField for ArgumentType {
        fn serialize<W: std::io::Write>(&self, mut writer: W) {
            match self {
                ArgumentType::Bool => {
                    VarInt(0).serialize(&mut writer);
                }
                ArgumentType::Float { min, max } => {
                    VarInt(1).serialize(&mut writer);
                    let flags = {
                        let mut flags = 0x00u8;
                        if min.is_some() {
                            flags |= 0x01;
                        }
                        if max.is_some() {
                            flags |= 0x02;
                        }
                        flags
                    };
                    flags.serialize(&mut writer);
                    if let Some(min) = min {
                        min.serialize(&mut writer);
                    }
                    if let Some(max) = max {
                        max.serialize(&mut writer);
                    }
                }
                ArgumentType::String {
                    string_type,
                    suggestions,
                } => {
                    VarInt(5).serialize(&mut writer);
                    string_type.serialize(&mut writer);
                    if let Some(_) = suggestions {
                        SuggestionsType::AskServer.serialize(&mut writer);
                    }
                }
            }
        }
    }

    impl Debug for CommandNode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut debug = f.debug_struct("CommandNode");
            debug.field("node_type", &self.node_type);
            debug.field("redirect", &self.redirect);
            debug.field("children", &self.children);
            debug.field("node_index", &self.node_index);
            debug.finish()
        }
    }
}
