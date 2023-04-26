macro_rules! define_client_bound_protocol {
    (
        $(
            $name:ident, $id:literal, $state:ident => {
                $($field:ident: $type:ty),*
                $(,)?
            }
        ),*
        $(,)?
    ) => {
        $(
            #[derive(Debug)]
            pub struct $name {
                $(pub $field: $type),*
            }

            impl ClientBoundPacket for $name {
                fn id() -> i32 {
                    $id
                }
                fn name() -> &'static str {
                    stringify!($name)
                }
            }

            impl SerializePacket for $name {
                fn serialize(&self) -> Vec<u8> {
                    let mut buffer = Vec::new();
                    <i32 as Into<VarInt>>::into(Self::id()).serialize(&mut buffer);
                    $(
                        self.$field.serialize(&mut buffer);
                    )*
                    buffer
                }
            }

        )*
    };
}

use crate as firework_protocol;

use firework_authentication::ProfileProperty;
use firework_data::tags::VarIntList;
use firework_data::Palette;
use firework_protocol_core::{Position, SerializeField};
use firework_protocol_core::{UnsizedVec, VarInt};
use firework_protocol_derive::SerializeField;
use sha1::{Digest, Sha1};
use std::collections::HashMap;

use nbt::Blob;

use crate::data_types::{
    Attribute, BitSet, BossBarAction, DeathLocation, EntityAnimationType, EntityEventStatus, Hand,
    Particle, PlayerAbilityFlags, PlayerInfoAction, PlayerPositionFlags, Recipe, Slot,
    SuggestionMatch,
};

pub trait ClientBoundPacket {
    fn id() -> i32;
    fn name() -> &'static str;
}

pub trait SerializePacket {
    fn serialize(&self) -> Vec<u8>;
}

define_client_bound_protocol! {
    ServerStatus, 0x00, Status => {
        motd: String
    },
    Pong, 0x01, Status => {
        payload: i64
    },
    LoginDisconnect, 0x00, Login => {
        reason: String
    },
    EncryptionRequest, 0x01, Login => {
        server_id: String,
        public_key: Vec<u8>,
        verify_token: Vec<u8>
    },
    LoginSuccess, 0x02, Login => {
        uuid: u128,
        username: String,
        properties: Vec<ProfileProperty>
    },
    SetCompression, 0x03, Login => {
        threshold: VarInt
    },
    LoginPluginRequest, 0x04, Login => {
        message_id: VarInt,
        channel: String,
        data: UnsizedVec<u8>
    },
    SpawnPlayer, 0x03, Play => {
        entity_id: VarInt,
        uuid: u128,
        x: f64,
        y: f64,
        z: f64,
        yaw: i8,
        pitch: i8,
    },
    EntityAnimation, 0x04, Play => {
        entity_id: VarInt,
        animation: EntityAnimationType
    },
    BossBar, 0x0B, Play => {
        uuid: u128,
        action: BossBarAction
    },
    ChangeDifficulty, 0x0C, Play => {
        difficulty: u8,
        locked: bool
    },
    CommandSuggestionsResponse, 0x0F, Play => {
        transaction_id: VarInt,
        start: VarInt,
        length: VarInt,
        suggestions: Vec<SuggestionMatch>
    },
    Commands, 0x10, Play => {
        data: UnsizedVec<u8>
    },
    CloseContainer, 0x11, Play => {
        window_id: u8
    },
    SetContainerContent, 0x12, Play => {
        window_id: u8,
        state_id: VarInt,
        items: Vec<Slot>,
        held_item: Slot
    },
    SetContainerSlot, 0x14, Play => {
        window_id: i8,
        state_id: VarInt,
        slot: i16,
        item: Slot
    },
    PluginMessage, 0x17, Play => {
        channel: String,
        data: UnsizedVec<u8>
    },
    PlayDisconnect, 0x1A, Play => {
        reason: String
    },
    EntityEvent, 0x1C, Play => {
        entity_id: i32,
        event_id: EntityEventStatus
    },
    UnloadChunk, 0x1E, Play => {
        x: i32,
        z: i32
    },
    HurtAnimation, 0x21, Play => {
        entity_id: VarInt,
        yaw: f32
    },
    InitializeWorldBorder, 0x22, Play => {
        x: f64,
        z: f64,
        old_diameter: f64,
        new_diameter: f64,
        speed: VarInt,
        portal_teleport_boundary: VarInt,
        warning_time: VarInt,
        warning_blocks: VarInt
    },
    ClientBoundKeepAlive, 0x23, Play => {
        id: u64
    },
    ChunkUpdateAndLightUpdate, 0x24, Play => {
        x: i32,
        z: i32,
        heightmaps: Blob,
        data: Vec<u8>,
        block_entities: UnsizedVec<u8>,
        trust_edges: bool,
        sky_light_mask: BitSet,
        block_light_mask: BitSet,
        empty_sky_light_mask: BitSet,
        empty_block_light_mask: BitSet,
        sky_light: Vec<Vec<i8>>,
        block_light: Vec<Vec<i8>>
    },
    ParticlePacket, 0x26, Play => {
        particle: Particle
    },
    LoginPlay, 0x28, Play => {
        entity_id: i32,
        is_hardcore: bool,
        game_mode: u8,
        previous_game_mode: i8,
        dimensions: Vec<String>,
        registry_codec: UnsizedVec<u8>,
        dimension_type: String,
        dimension_name: String,
        hashed_seed: i64,
        max_players: VarInt,
        view_distance: VarInt,
        simulation_distance: VarInt,
        reduced_debug_info: bool,
        enable_respawn_screen: bool,
        is_debug: bool,
        is_flat: bool,
        death_location: Option<DeathLocation>
    },
    MapData, 0x29, Play => {
        map_id: VarInt,
        scale: u8,
        locked: bool,
        icons: Option<Vec<u8>>,
        columns: u8,
        rows: u8,
        offset_x: u8,
        offset_z: u8,
        data: Vec<u8>
    },
    UpdateEntityPosition, 0x2B, Play => {
        entity_id: VarInt,
        delta_x: i16,
        delta_y: i16,
        delta_z: i16,
        on_ground: bool
    },
    UpdateEntityPositionAndRotation, 0x2C, Play => {
        entity_id: VarInt,
        delta_x: i16,
        delta_y: i16,
        delta_z: i16,
        yaw: i8,
        pitch: i8,
        on_ground: bool
    },
    UpdateEntityRotation, 0x2D, Play => {
        entity_id: VarInt,
        yaw: i8,
        pitch: i8,
        on_ground: bool
    },
    SwingArm, 0x2F, Play => {
        hand: Hand
    },
    OpenScreen, 0x30, Play => {
        window_id: VarInt,
        window_type: VarInt,
        title: String // TODO: Chat
    },
    PlayerAbilities, 0x34, Play => {
        flags: PlayerAbilityFlags,
        flying_speed: f32,
        walking_speed: f32
    },
    RemoveInfoPlayer, 0x39, Play => {
        players: Vec<u128>
    },
    PlayerInfo, 0x3A, Play => {
        action: PlayerInfoAction
    },
    SynchronizePlayerPosition, 0x3C, Play => {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        flags: PlayerPositionFlags,
        teleport_id: VarInt,
    },
    RemoveEntities, 0x3E, Play => {
        entity_ids: Vec<VarInt>
    },
    ResourcePack, 0x40, Play => {
        url: String,
        hash: String,
        forced: bool,
        prompt: Option<String> // TODO: Chat
    },
    Respawn, 0x41, Play => {
        dimension_type: String,
        dimension_name: String,
        hashed_seed: i64,
        gamemode: u8,
        previous_gamemode: i8,
        is_debug: bool,
        is_flat: bool,
        copy_metadata: bool,
        death_location: Option<DeathLocation>
    },
    UpdateEntityHeadRotation, 0x42, Play => {
        entity_id: VarInt,
        yaw: i8
    },
    SetHeldItem, 0x4D, Play => {
        slot: u8
    },
    SetCenterChunk, 0x4E, Play => {
        x: VarInt,
        z: VarInt
    },
    SetDefaultSpawn, 0x50, Play => {
        position: Position,
        yaw: f32
    },
    SetEntityMetadata, 0x52, Play => {
        entity_id: VarInt,
        metadata: UnsizedVec<u8>
    },
    SetEntityVelocity, 0x54, Play => {
        entity_id: VarInt,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16
    },
    // This packet is unreasonably weird so I am not going to implement it properly
    // Source: https://wiki.vg/Protocol#Set_Equipment
    SetEquipment, 0x55, Play => {
        entity_id: VarInt,
        slot: u8,
        item: Slot
    },
    SetHealth, 0x57, Play => {
        health: f32,
        food: VarInt,
        food_saturation: f32
    },
    SetSubtitleText, 0x5D, Play => {
        subtitle: String // TODO: Chat
    },
    SetTitleText, 0x5F, Play => {
        title: String // TODO: Chat
    },
    SetTitleAnimationTimes, 0x60, Play => {
        fade_in: i32,
        stay: i32,
        fade_out: i32
    },
    SoundEffect, 0x62, Play => {
        sound: IdMapHolder<CustomSound, VanillaSound>,
        sound_source: SoundSource,
        x: i32,
        y: i32,
        z: i32,
        volume: f32,
        pitch: f32,
        seed: i64
    },
    SystemChatMessage, 0x64, Play => {
        message: String,
        action_bar: bool
    },
    TeleportEntity, 0x68, Play => {
        entity_id: VarInt,
        x: f64,
        y: f64,
        z: f64,
        yaw: i8,
        pitch: i8,
        on_ground: bool
    },
    UpdateAttributes, 0x6A, Play => {
        entity_id: VarInt,
        attributes: Vec<Attribute>
    },
    SetRecipes, 0x6D, Play => {
        recipes: Vec<Recipe>
    },
    SetTags, 0x6E, Play => {
        tags: &'static HashMap<String, HashMap<String, VarIntList>>
    }
}

impl ResourcePack {
    pub async fn new(url: String, prompt: Option<String>) -> Result<Self, reqwest::Error> {
        // get the resource pack bytes
        let resource_pack_bytes = reqwest::get(&url).await?.bytes().await?;
        let forced = false;

        // compute the sha1 hash of the resource pack
        let mut hasher = Sha1::new();
        hasher.update(&resource_pack_bytes);
        let hash = format!("{:x}", hasher.finalize());

        if hash.len() != 40 {
            panic!("invalid hash length"); // should never happen
        }

        println!("hash of pack {}: {}", url, hash);

        Ok(Self {
            url,
            hash,
            forced,
            prompt,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IdMapHolder<T, U> {
    Direct(T),
    Reference(U),
}

impl<T: SerializeField, U: Palette> SerializeField for IdMapHolder<T, U> {
    fn serialize<W: std::io::Write>(&self, mut writer: W) {
        match self {
            IdMapHolder::Reference(data) => {
                VarInt::from(data.get() + 1).serialize(writer);
            }
            IdMapHolder::Direct(data) => {
                VarInt::from(0).serialize(&mut writer);
                data.serialize(writer);
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum VanillaSound {}

impl Palette for VanillaSound {
    fn get(&self) -> i32 {
        unimplemented!()
    }
}

#[derive(SerializeField, Debug, PartialEq, Clone)]
pub struct CustomSound {
    pub resource_location: String,
    pub range: Option<f32>,
}

#[derive(Debug, PartialEq, SerializeField, Clone)]
#[protocol(typ = "u8")]
pub enum SoundSource {
    Master,
    Music,
    Record,
    Weather,
    Block,
    Hostile,
    Neutral,
    Player,
    Ambient,
    Voice,
}
