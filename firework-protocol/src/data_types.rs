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

#[derive(Debug, PartialEq, SerializeField)]
pub struct SuggestionMatch {
    pub r#match: String,
    pub tooltip: Option<String>,
}
