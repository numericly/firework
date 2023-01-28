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
use std::collections::HashMap;

use nbt::Blob;

use reqwest;
use sha1::{Digest, Sha1};

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
    Commands, 0x0F, Play => {
        root: CommandNode
    },
    SetContainerContent, 0x11, Play => {
        window_id: u8,
        state_id: VarInt,
        items: Vec<Option<Slot>>,
        held_item: Option<Slot>
    },
    PlayDisconnect, 0x19, Play => {
        reason: String
    },
    UnloadChunk, 0x1C, Play => {
        x: i32,
        z: i32
    },
    InitializeWorldBorder, 0x1F, Play => {
        x: f64,
        z: f64,
        old_diameter: f64,
        new_diameter: f64,
        speed: VarInt,
        portal_teleport_boundary: VarInt,
        warning_time: VarInt,
        warning_blocks: VarInt
    },
    ClientBoundKeepAlive, 0x20, Play => {
        id: i64
    },
    ChunkUpdateAndLightUpdate, 0x21, Play => {
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
    LoginWorld, 0x25, Play => {
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
    UpdateEntityPosition, 0x28, Play => {
        entity_id: VarInt,
        delta_x: i16,
        delta_y: i16,
        delta_z: i16,
        on_ground: bool
    },
    UpdateEntityPositionAndRotation, 0x29, Play => {
        entity_id: VarInt,
        delta_x: i16,
        delta_y: i16,
        delta_z: i16,
        yaw: i8,
        pitch: i8,
        on_ground: bool
    },
    UpdateEntityRotation, 0x2A, Play => {
        entity_id: VarInt,
        yaw: u8,
        pitch: u8,
        on_ground: bool
    },
    PlayerAbilities, 0x31, Play => {
        flags: PlayerAbilityFlags,
        flying_speed: f32,
        walking_speed: f32
    },
    PlayerInfo, 0x37, Play => {
        action: PlayerInfoAction
    },
    SynchronizePlayerPosition, 0x39, Play => {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        flags: PlayerPositionFlags,
        teleport_id: VarInt,
        dismount_vehicle: bool
    },
    ServerResourcePack, 0x3D, Play => {
        url: String,
        hash: String,
        required: bool,
        prompt: Option<String> // should be a chat not a string
    },
    SetHeldItem, 0x4A, Play => {
        slot: u8
    },
    SetCenterChunk, 0x4B, Play => {
        x: VarInt,
        z: VarInt
    },
    SetDefaultSpawn, 0x4D, Play => {
        position: Position
    },
    SystemChatMessage, 0x62, Play => {
        message: String,
        action_bar: bool
    },
    TeleportEntity, 0x66, Play => {
        entity_id: VarInt,
        x: f64,
        y: f64,
        z: f64,
        yaw: i8,
        pitch: i8,
        on_ground: bool
    },
    SetRecipes, 0x6A, Play => {
        recipes: Vec<Recipe>
    },
    SetTags, 0x6B, Play => {
        tags: &'static HashMap<String, HashMap<String, VarIntList>>
    }
}

impl ServerResourcePack {
    pub async fn new(url: String, prompt: Option<String>) -> Result<Self, reqwest::Error> {
        // get the resource pack bytes
        let resource_pack_bytes = reqwest::get(&url).await?.bytes().await?;
        let required = false;

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
            required,
            prompt,
        })
    }
}
