macro_rules! define_server_bound_protocol {
    (
        $(
            $name:ident, $id:literal, $state:ident => {
                $($field:ident: $type:ty),*
                $(,)?
            }
        ),*
        $(,)?
    ) => {
        #[derive(Debug, PartialEq)]
        pub enum ServerBoundPacket {
            $(
                $name($name),
            )*
        }

        $(
            #[derive(Debug, PartialEq)]
            pub struct $name {
                $(pub $field: $type),*
            }
            #[allow(unused_variables)]
            impl DeserializePacket for $name {
                fn deserialize<R: Read>(mut _reader: R) -> Result<Self, DeserializeError> {
                    Ok(Self {
                        $(
                            $field: <$type>::deserialize(&mut _reader)?,
                        )*
                    })
                }
            }
        )*



        impl ServerBoundPacket {
            pub async fn deserialize<R: Read>(mut reader: R, state: &tokio::sync::RwLock<ConnectionState>) -> Result<Self, DeserializeError> {
                let state = *state.read().await;
                match (state, VarInt::deserialize(&mut reader)?.0) {
                    $(
                        (ConnectionState::$state, $id) => Ok(Self::$name($name::deserialize(reader).map_err(|e| {
                            let string = stringify!($name);
                            println!("Error while deserializing packet: {}", string);
                            e
                        })?)),
                    )*
                    (state, id) => Err(DeserializeError::InvalidPacketID { id, state: state as u8 }),
                }
            }
        }
    };
}

use crate::{
    data_types::{
        Arm, BlockFace, ChatMode, DisplaySkinParts, InventoryOperationMode, MainHand,
        PlayerAbilityFlags, PlayerActionStatus, PlayerCommandAction, RecipeBookType, SignatureData,
        SlotUpdate,
    },
    ConnectionState,
};
use firework_protocol_core::{DeserializeError, DeserializeField, Position, UnsizedVec, VarInt};
use std::io::Read;

pub trait ServerBoundPacketID {
    fn id() -> i32;
}

pub trait DeserializePacket {
    fn deserialize<R>(reader: R) -> Result<Self, DeserializeError>
    where
        Self: Sized,
        R: Read;
}

define_server_bound_protocol! {
    Handshake, 0x00, HandShaking => {
        protocol_version: VarInt,
        server_address: String,
        server_port: u16,
        next_state: ConnectionState,
    },
    StatusRequest, 0x00, Status => {},
    Ping, 0x01, Status => {
        payload: i64
    },
    LoginStart, 0x00, Login => {
        name: String,
        uuid: Option<u128>
    },
    EncryptionResponse, 0x01, Login => {
        shared_secret: Vec<u8>,
        verify_token: Vec<u8>,
    },
    LoginPluginResponse, 0x02, Login => {
        message_id: VarInt,
        successful: bool,
        data: UnsizedVec<u8>
    },
    ConfirmTeleport, 0x00, Play => {
        teleport_id: VarInt
    },
    ChatCommand, 0x04, Play => {
        command: String // cryptography is mean
    },
    ChatMessage, 0x05, Play => {
        message: String // cryptography is mean
    },
    ClientInformation, 0x07, Play => {
        locale: String,
        view_distance: u8,
        chat_mode: ChatMode,
        chat_colors: bool,
        displayed_skin_parts: DisplaySkinParts,
        main_hand: MainHand
    },
    ClickContainer, 0x0A, Play => {
        window_id: u8,
        state_id: VarInt,
        slot: i16,
        button: i8,
        mode: InventoryOperationMode,
        slots: Vec<SlotUpdate>,
        // cursor: Option<Slot>
    },
    CloseContainerServerBound, 0x0B, Play => {
        window_id: u8
    },
    PluginMessageServerBound, 0x0C, Play => {
        channel: String,
        data: UnsizedVec<u8>
    },
    ServerBoundKeepAlive, 0x11, Play => {
        id: i64
    },
    SetPlayerPosition, 0x13, Play => {
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool
    },
    SetPlayerPositionAndRotation, 0x14, Play => {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool
    },
    SetPlayerRotation, 0x15, Play => {
        yaw: f32,
        pitch: f32,
        on_ground: bool
    },
    SetPlayerOnGround, 0x16, Play => {
        on_ground: bool
    },
    PlayerAbilitiesServerBound, 0x1B, Play => {
        flags: PlayerAbilityFlags
    },
    PlayerAction, 0x1C, Play => {
        status: PlayerActionStatus,
        location: Position,
        face: BlockFace,
        sequence: VarInt

    },
    PlayerCommand, 0x1D, Play => {
        entity_id: VarInt,
        action: PlayerCommandAction,
        action_parameter: VarInt,
    },
    PlayerSession, 0x20, Play => {
        session_id: u128,
        expires_at: u64,
        public_key: Vec<u8>,
        key_signature: Vec<u8>
    },
    ChangeRecipeBookSettings, 0x21, Play => {
        book_type: RecipeBookType,
        book_open: bool,
        filter_active: bool
    },
    ResourcePackResponse, 0x24, Play => {
        result: VarInt // this is technically an enum but i'm not gonna make one; it's just 4 values that we'll throw away
    },
    SetHeldItemServerBound, 0x28, Play => {
        slot: u16
    },
    SwingArm, 0x2F, Play => {
        arm: Arm
    },
    UseItemOn, 0x31, Play => {
        arm: Arm,
        location: Position,
        face: BlockFace,
        cursor_x: f32,
        cursor_y: f32,
        cursor_z: f32,
        inside_block: bool,
        sequence: VarInt
    },
    UseItem, 0x32, Play => {
        arm: Arm,
        sequence: VarInt
    }
}

impl DeserializeField for SignatureData {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        Ok(Self {
            timestamp: i64::deserialize(&mut reader)?,
            public_key: <Vec<u8>>::deserialize(&mut reader)?,
            signature: <Vec<u8>>::deserialize(&mut reader)?,
        })
    }
}

impl DeserializeField for DisplaySkinParts {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let mask = u8::deserialize(&mut reader)?;
        Ok(Self::new()
            .with_cape(mask & 0x01 == 0x01)
            .with_jacket(mask & 0x02 == 0x02)
            .with_left_sleeve(mask & 0x04 == 0x04)
            .with_right_sleeve(mask & 0x08 == 0x08)
            .with_left_pants_leg(mask & 0x10 == 0x10)
            .with_right_pants_leg(mask & 0x20 == 0x20)
            .with_hat(mask & 0x40 == 0x40))
    }
}

impl DeserializeField for PlayerAbilityFlags {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let flags = u8::deserialize(&mut reader)?;
        Ok(Self::new()
            .with_invulnerable(flags & 0x01 == 0x01)
            .with_flying(flags & 0x02 == 0x02)
            .with_allow_flying(flags & 0x04 == 0x04)
            .with_creative_mode(flags & 0x08 == 0x08))
    }
}
