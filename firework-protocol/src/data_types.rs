use crate as firework_protocol;

use byteorder::WriteBytesExt;
use firework_authentication::ProfileProperty;
use firework_data::items::Item;
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
    Block {
        block_state: i32,
    },
    BlockMarker {
        block_state: i32,
    },
    Dust {
        red: f32,
        green: f32,
        blue: f32,
        /// The scale, will be clamped between 0.01 and 4.
        scale: f32,
    },
    CampfireCozySmoke,
    CampfireSignalSmoke,
    EndRod,
    Firework,
    Flame,
    CrimsonSpore,
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

impl Particle {
    pub fn new(
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
    ) -> Self {
        Self {
            particle,
            long_distance,
            x,
            y,
            z,
            offset_x,
            offset_y,
            offset_z,
            max_speed,
            count,
        }
    }
}

impl SerializeField for Particle {
    fn serialize<W: Write>(&self, mut writer: W) {
        #[allow(unused_variables)]
        VarInt::from(match self.particle {
            Particles::Block { .. } => 2,
            Particles::BlockMarker { .. } => 3,
            Particles::Dust { .. } => 14,
            Particles::CampfireCozySmoke => 67,
            Particles::CampfireSignalSmoke => 68,
            Particles::EndRod => 20,
            Particles::Firework => 26,
            Particles::Flame => 28,
            Particles::CrimsonSpore => 75,
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
            Particles::Block { block_state } => {
                VarInt(block_state).serialize(&mut writer);
            }
            Particles::BlockMarker { block_state } => {
                VarInt(block_state).serialize(&mut writer);
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
    #[serde(rename = "Enchantments")]
    pub enchantments: Option<Vec<Enchantment>>,
}

#[derive(Debug, serde::Serialize, PartialEq, serde::Deserialize, Clone)]
pub struct Enchantment {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "lvl")]
    pub level: i16,
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
#[protocol(typ = "firework_protocol::core::VarInt")]
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
#[protocol(typ = "firework_protocol::core::VarInt")]
pub enum BlockFace {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
}

#[derive(Debug, PartialEq, SerializeField, Clone)]
#[protocol(typ = "firework_protocol::core::VarInt")]
pub enum EntityAnimationType {
    SwingMainArm,
    TakeDamage,
    LeaveBed,
    SwingOffhand,
    CriticalEffect,
    MagicCriticalEffect,
}

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "firework_protocol::core::VarInt")]
pub enum ClientCommandAction {
    PerformRespawn,
    RequestStats,
}

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "firework_protocol::core::VarInt")]
pub enum ResourcePackLoadStatus {
    Success,
    Declined,
    FailedDownload,
    Accepted,
}

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "firework_protocol::core::VarInt")]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "firework_protocol::core::VarInt")]
pub enum MainHand {
    Left,
    Right,
}

#[derive(Debug, PartialEq, DeserializeField)]
#[protocol(typ = "firework_protocol::core::VarInt")]
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
#[protocol(typ = "firework_protocol::core::VarInt")]
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
pub struct StackContents {
    pub id: Item,
    pub count: u8,
    pub nbt: ItemNbt,
}

impl StackContents {
    /// attempts combine two stacks and returns the remainder
    pub fn stack_item(&mut self, other_stack: &StackContents) -> Option<StackContents> {
        if self.id == other_stack.id && self.nbt == other_stack.nbt {
            let new_count = self.count as usize + other_stack.count as usize;
            if new_count > self.id.get_stack_size() {
                self.count = self.id.get_stack_size() as u8;
                Some(StackContents {
                    id: other_stack.id,
                    count: (new_count - self.id.get_stack_size()) as u8,
                    nbt: other_stack.nbt.clone(),
                })
            } else {
                self.count = new_count as u8;
                None
            }
        } else {
            Some(other_stack.clone())
        }
    }
}

pub type ItemStack = Option<StackContents>;

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
#[protocol(typ = "firework_protocol::core::VarInt")]

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
    pub slot_value: ItemStack,
}

impl SerializeField for DisplaySkinParts {
    fn serialize<W: std::io::Write>(&self, mut writer: W) {
        self.bytes.serialize(&mut writer);
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, SerializeField, Clone)]
#[protocol(typ = "firework_protocol::core::VarInt")]

pub enum BossBarAction {
    Add {
        title: String,
        health: f32,
        color: BossBarColor,
        division: BossBarDivision,
        flags: u8,
    },
    Remove,
    UpdateHealth {
        health: f32,
    },
    UpdateTitle {
        title: String,
    },
    UpdateStyle {
        color: BossBarColor,
        division: BossBarDivision,
    },
    UpdateFlags {
        flags: u8,
    },
}

#[derive(Debug, PartialEq, SerializeField, Clone)]
#[protocol(typ = "firework_protocol::core::VarInt")]
pub enum BossBarColor {
    Pink,
    Blue,
    Red,
    Green,
    Yellow,
    Purple,
    White,
}

#[derive(Debug, PartialEq, SerializeField, Clone)]
#[protocol(typ = "firework_protocol::core::VarInt")]
pub enum BossBarDivision {
    NoDivisions,
    SixDivisions,
    TenDivisions,
    TwelveDivisions,
    TwentyDivisions,
}

#[derive(Debug, PartialEq, SerializeField, DeserializeField)]
#[protocol(typ = "firework_protocol::core::VarInt")]
pub enum Hand {
    MainHand,
    OffHand,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EntityEventStatus {
    SpawnTippedArrowParticleEffect,                 // 0
    ResetMinecartSpawnerDelay,                      // 1
    UseRabbitRotatedJumpingAnimation,               // 1
    DisplayIronCrackParticlesAtEggLocation,         // 3
    PlayLivingEntityDeathSoundAndAnimation,         // 3
    DisplaySnowballPoofParticlesAtSnowballLocation, // 3
    StartEvokerAttackAnimationAndSound,             // 4
    StartHoglinAttackAnimationAndSound,             // 4
    StartIronGolemAttackAnimationAndSound,          // 4
    StartRavageAttackAnimation,                     // 4
    StartWardenAttackAnimationAndSound,             // 4
    StartZoglinAttackAnimationAndSound,             // 4
    AbstractHorseSpawnSmokeParticles,               // 6
    TameableAnimalSpawnSmokeParticles,              // 6
    AbstractHorseSpawnHeartParticles,               // 7
    TameableAnimalSpawnHeartParticles,              // 7
    StartWolfShakingAnimation,                      // 8
    MarkPlayerItemAsUsed,                           // 9
    IgniteMinecartTNTWithoutSound,                  // 10
    StartSheepGrassEatingAnimation,                 // 10
    StartIronGolemOfferFlowerAnimation,             // 11
    SpawnVillagerHeartParticles,                    // 12
    SpawnVillagerAngryParticles,                    // 13
    SpawnVillagerHappyParticles,                    // 14
    SpawnWitchMagicParticles,                       // 15
    PlayZombieVillagerCureSound, // 16 (why is this a thing, why not just play the sound)
    TriggerFireworkExplosion,    // 17
    SpawnAnimalLoveParticles,    // 18
    SpawnAllayHeartParticles,    // 18
    ResetSquidRotation,          // 19
    SpawnExplosionParticles,     // 20 (used with silverfish and mob spawners)
    PlayGuardianAttackSound,     // 21
    // TODO not doing the rest (╯°□°）╯︵ ┻━┻
    SpawnDeathSmokeParticles, // 60
}

impl SerializeField for EntityEventStatus {
    fn serialize<W: Write>(&self, mut writer: W) {
        let u8 = match self {
            EntityEventStatus::SpawnDeathSmokeParticles => 60u8,
            Self::PlayLivingEntityDeathSoundAndAnimation => 3u8,
            _ => todo!(),
        };
        dbg!(u8);
        writer.write_u8(u8).unwrap();
    }
}

#[derive(Debug, PartialEq)]
pub enum InteractAction {
    Interact {
        hand: Hand,
    },
    Attack,
    InteractAt {
        target_x: f32,
        target_y: f32,
        hand: Hand,
        target_z: f32,
    },
}

impl DeserializeField for InteractAction {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let action = VarInt::deserialize(&mut reader)?.0;
        match action {
            0 => Ok(InteractAction::Interact {
                hand: Hand::deserialize(&mut reader)?,
            }),
            1 => Ok(InteractAction::Attack),
            2 => Ok(InteractAction::InteractAt {
                target_x: f32::deserialize(&mut reader)?,
                target_y: f32::deserialize(&mut reader)?,
                target_z: f32::deserialize(&mut reader)?,
                hand: Hand::deserialize(&mut reader)?,
            }),
            _ => Err(DeserializeError::InvalidVariantIndex(
                action,
                "InteractAction",
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Equipment {
    pub equipment: Vec<EquipmentEntry>,
}

#[derive(Debug, Clone)]
pub struct EquipmentEntry {
    pub slot: EquipmentSlot,
    pub item: ItemStack,
}

#[derive(Debug, Clone)]

pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Boots,
    Leggings,
    Chestplate,
    Helmet,
}

impl SerializeField for Equipment {
    fn serialize<W: Write>(&self, mut writer: W) {
        for (i, equipment) in self.equipment.iter().enumerate() {
            let mut slot: u8 = match &equipment.slot {
                EquipmentSlot::MainHand => 0,
                EquipmentSlot::OffHand => 1,
                EquipmentSlot::Boots => 2,
                EquipmentSlot::Leggings => 3,
                EquipmentSlot::Chestplate => 4,
                EquipmentSlot::Helmet => 5,
            };
            if i != self.equipment.len() - 1 {
                slot |= 0b1000_0000;
            }
            slot.serialize(&mut writer);
            equipment.item.serialize(&mut writer);
        }
    }
}

#[derive(Debug, PartialEq, SerializeField, Clone)]
#[protocol(typ = "firework_protocol::core::VarInt")]
pub enum ObjectiveAction {
    Create {
        objective_value: String,
        objective_type: ObjectiveType,
    },
    Remove,
    Update {
        objective_value: String,
        objective_type: ObjectiveType,
    },
}

#[derive(Debug, PartialEq, SerializeField, Clone)]
#[protocol(typ = "firework_protocol::core::VarInt")]
pub enum ObjectiveType {
    Integer,
    Hearts,
}

#[derive(Debug, PartialEq, SerializeField, Clone)]
#[protocol(typ = "firework_protocol::core::VarInt")]
pub enum ScoreAction {
    CreateOrUpdate {
        objective_name: String,
        value: VarInt,
    },
    Remove {
        objective_name: String,
    },
}
