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
            pub fn deserialize<R: Read>(mut reader: R, state: &ConnectionState) -> Result<Self, DeserializeError> {
                match (state, VarInt::deserialize(&mut reader)?.0) {
                    $(
                        (ConnectionState::$state, $id) => Ok(Self::$name($name::deserialize(reader)?)),
                    )*
                    (state, id) => Err(DeserializeError::InvalidPacketID { id, state: state.clone() }),
                }
            }
        }
    };
}

use std::io::{self, Read};

use byteorder::ReadBytesExt;
use thiserror::Error;

use crate::protocol::ConnectionState;

use super::data_types::*;

#[derive(Debug, Error)]
pub enum DeserializeError {
    #[error("Could not find packet with id: {id:#04X} for state: {state:?}")]
    InvalidPacketID { id: i32, state: ConnectionState },

    #[error("Could not deserialize VarInt because it is greater than 5 bytes")]
    VarIntTooLong,

    #[error("Could not parse string: {0}")]
    StringParseError(#[from] std::string::FromUtf8Error),

    #[error("Invalid next connection state: {0}")]
    InvalidNextConnectionState(i32),

    #[error("Invalid chat mode: {0}")]
    InvalidChatMode(i32),

    #[error("Invalid main hand: {0}")]
    InvalidMainHand(i32),

    #[error("Invalid arm: {0}")]
    InvalidArm(i32),

    #[error("Invalid player command action: {0}")]
    InvalidPlayerCommandAction(i32),

    #[error("Invalid action action status: {0}")]
    InvalidPlayerActionStatus(i32),

    #[error("Invalid block face: {0}")]
    InvalidBlockFace(u8),

    #[error("Invalid recipe book type: {0}")]
    InvalidRecipeBookType(i32),

    #[error("An IO error occurred: {0}")]
    IoError(#[from] io::Error),
}

pub trait ServerBoundPacketID {
    fn id() -> i32;
}

pub trait DeserializeField {
    fn deserialize<R>(reader: R) -> Result<Self, DeserializeError>
    where
        Self: Sized,
        R: Read;
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
        signature_data: Option<SignatureData>,
        uuid: Option<u128>
    },
    EncryptionResponse, 0x01, Login => {
        shared_secret: Vec<u8>,
        token_or_signature: VerifyTokenOrSignature
    },
    ConfirmTeleport, 0x00, Play => {
        teleport_id: VarInt
    },
    CloseContainer, 0x0C, Play => {
        window_id: u8
    },
    ClientInformation, 0x08, Play => {
        locale: String,
        view_distance: u8,
        chat_mode: ChatMode,
        chat_colors: bool,
        displayed_skin_parts: DisplaySkinParts,
        main_hand: MainHand
    },
    PluginMessageServerBound, 0x0D, Play => {
        channel: String,
        data: UnsizedVec<u8>
    },
    ServerBoundKeepAlive, 0x12, Play => {
        id: i64
    },
    SetPlayerPosition, 0x14, Play => {
        x: f64,
        y: f64,
        z: f64,
        on_ground: bool
    },
    SetPlayerPositionAndRotation, 0x15, Play => {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool
    },
    SetPlayerRotation, 0x16, Play => {
        yaw: f32,
        pitch: f32,
        on_ground: bool
    },
    SetPlayerOnGround, 0x17, Play => {
        on_ground: bool
    },
    PlayerAbilitiesServerBound, 0x1C, Play => {
        flags: PlayerAbilityFlags
    },
    PlayerAction, 0x1D, Play => {
        status: PlayerActionStatus,
        location: Position,
        face: BlockFace,
        sequence: VarInt

    },
    PlayerCommand, 0x1E, Play => {
        entity_id: VarInt,
        action: PlayerCommandAction,
        action_parameter: VarInt,
    },
    ChangeRecipeBookSettings, 0x21, Play => {
        book_type: RecipeBookType,
        book_open: bool,
        filter_active: bool
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
    }
}

impl DeserializeField for PlayerActionStatus {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let value = VarInt::deserialize(&mut reader)?.0;
        match value {
            0 => Ok(PlayerActionStatus::StartDigging),
            1 => Ok(PlayerActionStatus::CancelDigging),
            2 => Ok(PlayerActionStatus::FinishDigging),
            3 => Ok(PlayerActionStatus::DropItemStack),
            4 => Ok(PlayerActionStatus::DropItem),
            5 => Ok(PlayerActionStatus::ShootArrowOrFinishEating),
            6 => Ok(PlayerActionStatus::SwapItemInHand),
            _ => Err(DeserializeError::InvalidPlayerActionStatus(value)),
        }
    }
}

impl DeserializeField for BlockFace {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let value = u8::deserialize(&mut reader)?;
        match value {
            0 => Ok(BlockFace::Bottom),
            1 => Ok(BlockFace::Top),
            2 => Ok(BlockFace::North),
            3 => Ok(BlockFace::South),
            4 => Ok(BlockFace::West),
            5 => Ok(BlockFace::East),
            _ => Err(DeserializeError::InvalidBlockFace(value)),
        }
    }
}

impl DeserializeField for Position {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let pos_u64 = i64::deserialize(&mut reader)?;
        let mut x = pos_u64 >> 38;
        let mut y = pos_u64 << 52 >> 52;
        let mut z = pos_u64 << 26 >> 38;
        if x >= 1 << 25 {
            x -= 1 << 26
        }
        if y >= 1 << 11 {
            y -= 1 << 12
        }
        if z >= 1 << 25 {
            z -= 1 << 26
        }
        Ok(Position {
            x: x as i32,
            y: y as i16,
            z: z as i32,
        })
    }
}

impl DeserializeField for VarInt {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;

        let mut val = 0;

        for i in 0..4 {
            let position = i * 7;
            let current_byte = (&mut reader).read_u8()?;

            val |= ((current_byte & SEGMENT_BITS) as i32) << position;

            if (current_byte & CONTINUE_BIT) == 0 {
                break;
            } else if i == 4 {
                return Err(DeserializeError::VarIntTooLong);
            }
        }
        Ok(VarInt(val))
    }
}

impl DeserializeField for String {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let length = VarInt::deserialize(&mut reader)?.0;
        let mut string = String::new();
        reader.take(length as u64).read_to_string(&mut string)?;
        Ok(string)
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

impl DeserializeField for VerifyTokenOrSignature {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let has_verify_token = bool::deserialize(&mut reader)?;
        Ok(match has_verify_token {
            true => Self::VerifyToken {
                verify_token: <Vec<u8>>::deserialize(&mut reader)?,
            },
            false => Self::MessageSignature {
                salt: i64::deserialize(&mut reader)?,
                message_signature: <Vec<u8>>::deserialize(&mut reader)?,
            },
        })
    }
}

impl DeserializeField for ChatMode {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let mode = VarInt::deserialize(&mut reader)?.0;
        Ok(match mode {
            0 => Self::Enabled,
            1 => Self::CommandsOnly,
            2 => Self::Hidden,
            _ => return Err(DeserializeError::InvalidChatMode(mode)),
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

impl DeserializeField for MainHand {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let hand = VarInt::deserialize(&mut reader)?.0;
        Ok(match hand {
            0 => Self::Left,
            1 => Self::Right,
            _ => return Err(DeserializeError::InvalidMainHand(hand)),
        })
    }
}

impl DeserializeField for Arm {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let arm = VarInt::deserialize(&mut reader)?.0;
        Ok(match arm {
            0 => Self::Main,
            1 => Self::Off,
            _ => return Err(DeserializeError::InvalidArm(arm)),
        })
    }
}

impl DeserializeField for PlayerCommandAction {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let action = VarInt::deserialize(&mut reader)?.0;
        Ok(match action {
            0 => Self::StartSneaking,
            1 => Self::StopSneaking,
            2 => Self::LeaveBed,
            3 => Self::StartSprinting,
            4 => Self::StopSprinting,
            5 => Self::StartJumpWithHorse,
            6 => Self::StopJumpWithHorse,
            7 => Self::OpenHorseInventory,
            8 => Self::StartFlyingWithElytra,
            _ => return Err(DeserializeError::InvalidPlayerCommandAction(action)),
        })
    }
}

impl DeserializeField for ConnectionState {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let state = VarInt::deserialize(&mut reader)?.0;
        Ok(match state {
            1 => Self::Status,
            2 => Self::Login,
            3 => Self::Play,
            _ => return Err(DeserializeError::InvalidNextConnectionState(state)),
        })
    }
}

impl DeserializeField for RecipeBookType {
    fn deserialize<R: Read>(reader: R) -> Result<Self, DeserializeError> {
        let value = VarInt::deserialize(reader)?.0;
        Ok(match value {
            0 => Self::Crafting,
            1 => Self::Furnace,
            2 => Self::BlastFurnace,
            3 => Self::Smoker,
            _ => return Err(DeserializeError::InvalidRecipeBookType(value)),
        })
    }
}

impl<T: DeserializeField> DeserializeField for Vec<T> {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let length = VarInt::deserialize(&mut reader)?.0 as usize;
        let mut data = Vec::with_capacity(length);
        for _ in 0..length {
            data.push(T::deserialize(&mut reader)?);
        }
        Ok(data)
    }
}

impl<T: DeserializeField> DeserializeField for UnsizedVec<T> {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let mut data = Vec::new();
        while let Ok(value) = T::deserialize(&mut reader) {
            data.push(value);
        }
        Ok(Self(data))
    }
}

impl<T: DeserializeField> DeserializeField for Option<T> {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let has_value = bool::deserialize(&mut reader)?;
        Ok(match has_value {
            true => {
                let value = T::deserialize(&mut reader)?;
                Some(value)
            }
            false => None,
        })
    }
}

impl DeserializeField for i8 {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        Ok(reader.read_i8()?)
    }
}

impl DeserializeField for i16 {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        Ok(reader.read_i16::<byteorder::BigEndian>()?)
    }
}

impl DeserializeField for i32 {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        Ok(reader.read_i32::<byteorder::BigEndian>()?)
    }
}

impl DeserializeField for i64 {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        Ok(reader.read_i64::<byteorder::BigEndian>()?)
    }
}

impl DeserializeField for i128 {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        Ok(reader.read_i128::<byteorder::BigEndian>()?)
    }
}

impl DeserializeField for u8 {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        Ok(reader.read_u8()?)
    }
}

impl DeserializeField for u16 {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        Ok(reader.read_u16::<byteorder::BigEndian>()?)
    }
}

impl DeserializeField for u32 {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        Ok(reader.read_u32::<byteorder::BigEndian>()?)
    }
}

impl DeserializeField for u64 {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        Ok(reader.read_u64::<byteorder::BigEndian>()?)
    }
}

impl DeserializeField for u128 {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        Ok(reader.read_u128::<byteorder::BigEndian>()?)
    }
}

impl DeserializeField for f32 {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        Ok(reader.read_f32::<byteorder::BigEndian>()?)
    }
}

impl DeserializeField for f64 {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        Ok(reader.read_f64::<byteorder::BigEndian>()?)
    }
}

impl DeserializeField for bool {
    fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
        let value = reader.read_u8()?;
        Ok(if value == 1 { true } else { false })
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::client_bound::SerializeField;

    #[test]
    fn test_position() {
        let position = Position { x: -1, y: 2, z: 3 };
        let mut buffer = Vec::new();
        position.serialize(&mut buffer);
        let deserialized = Position::deserialize(&mut buffer.as_slice()).unwrap();
        assert_eq!(position, deserialized);
    }
}
