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
            #[derive(Debug, PartialEq)]
            pub struct $name {
                $(pub $field: $type),*
            }

            impl ClientBoundPacketID for $name {
                fn id() -> i32 {
                    $id
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

use authentication::ProfileProperty;
use minecraft_data::tags::VarIntList;
use protocol_core::{BitSet, UnsizedVec, VarInt};
use protocol_core::{Position, SerializeField};
use sha1::{Digest, Sha1};
use std::collections::HashMap;

use nbt::Blob;

use crate::data_types::{
    CommandNode, DeathLocation, PlayerAbilityFlags, PlayerInfoAction, PlayerPositionFlags, Recipe,
    Slot,
};

pub trait ClientBoundPacketID {
    fn id() -> i32;
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
        data: Vec<u8>
    },
    SpawnPlayer, 0x02, Play => {
        entity_id: VarInt,
        uuid: u128,
        x: f64,
        y: f64,
        z: f64,
        yaw: i8,
        pitch: i8,
    },
    ChangeDifficulty, 0x0B, Play => {
        difficulty: u8,
        locked: bool
    },
    Commands, 0x0E, Play => {
        root: CommandNode
    },
    SetContainerContent, 0x10, Play => {
        window_id: u8,
        state_id: VarInt,
        items: Vec<Option<Slot>>,
        held_item: Option<Slot>
    },
    PlayDisconnect, 0x17, Play => {
        reason: String
    },
    UnloadChunk, 0x1B, Play => {
        x: i32,
        z: i32
    },
    InitializeWorldBorder, 0x1E, Play => {
        x: f64,
        z: f64,
        old_diameter: f64,
        new_diameter: f64,
        speed: VarInt,
        portal_teleport_boundary: VarInt,
        warning_time: VarInt,
        warning_blocks: VarInt
    },
    ClientBoundKeepAlive, 0x1F, Play => {
        id: i64
    },
    ChunkUpdateAndLightUpdate, 0x20, Play => {
        x: i32,
        z: i32,
        heightmaps: Blob,
        data: Vec<u8>,
        block_entities: Vec<Blob>,
        trust_edges: bool,
        sky_light_mask: BitSet,
        block_light_mask: BitSet,
        empty_sky_light_mask: BitSet,
        empty_block_light_mask: BitSet,
        sky_light: Vec<Vec<i8>>,
        block_light: Vec<Vec<i8>>
    },
    LoginPlay, 0x24, Play => {
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
    UpdateEntityPosition, 0x27, Play => {
        entity_id: VarInt,
        delta_x: i16,
        delta_y: i16,
        delta_z: i16,
        on_ground: bool
    },
    UpdateEntityPositionAndRotation, 0x28, Play => {
        entity_id: VarInt,
        delta_x: i16,
        delta_y: i16,
        delta_z: i16,
        yaw: i8,
        pitch: i8,
        on_ground: bool
    },
    UpdateEntityRotation, 0x29, Play => {
        entity_id: VarInt,
        yaw: i8,
        pitch: i8,
        on_ground: bool
    },
    OpenScreen, 0x2C, Play => {
        window_id: VarInt,
        window_type: VarInt,
        title: String // TODO: Chat
    },
    PlayerAbilities, 0x30, Play => {
        flags: PlayerAbilityFlags,
        flying_speed: f32,
        walking_speed: f32
    },
    RemoveInfoPlayer, 0x35, Play => {
        players: Vec<u128>
    },
    PlayerInfo, 0x36, Play => {
        action: PlayerInfoAction
    },
    SynchronizePlayerPosition, 0x38, Play => {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        flags: PlayerPositionFlags,
        teleport_id: VarInt,
        dismount_vehicle: bool
    },
    RemoveEntities, 0x3A, Play => {
        entity_ids: Vec<VarInt>
    },
    ResourcePack, 0x3C, Play => {
        url: String,
        hash: String,
        forced: bool,
        prompt: Option<String> // TODO: Chat
    },
    UpdateEntityHeadRotation, 0x3E, Play => {
        entity_id: VarInt,
        yaw: i8
    },
    SetHeldItem, 0x49, Play => {
        slot: u8
    },
    SetCenterChunk, 0x4A, Play => {
        x: VarInt,
        z: VarInt
    },
    SetDefaultSpawn, 0x4C, Play => {
        position: Position,
        yaw: f32
    },
    SetEntityMetadata, 0x4E, Play => {
        entity_id: VarInt,
        metadata: UnsizedVec<u8>
    },
    SetEntityVelocity, 0x50, Play => {
        entity_id: VarInt,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16
    },
    SystemChatMessage, 0x60, Play => {
        message: String,
        action_bar: bool
    },
    TeleportEntity, 0x64, Play => {
        entity_id: VarInt,
        x: f64,
        y: f64,
        z: f64,
        yaw: i8,
        pitch: i8,
        on_ground: bool
    },
    SetRecipes, 0x69, Play => {
        recipes: Vec<Recipe>
    },
    SetTags, 0x6A, Play => {
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
            panic!("invalid hash length"); // what even is an error handling
        }

        println!("hash of pack {}: {}", url, hash);

        Ok(Self {
            url,
            hash: hash,
            forced,
            prompt,
        })
    }
}
