use modular_bitfield::bitfield;

use super::server_bound::DeserializeField;

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct UnsizedVec<T: DeserializeField>(pub Vec<T>);

#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct VarInt(pub i32);

impl From<i32> for VarInt {
    fn from(num: i32) -> VarInt {
        VarInt(num)
    }
}

#[derive(Debug, PartialEq)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct SignatureData {
    pub timestamp: i64,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum VerifyTokenOrSignature {
    VerifyToken {
        verify_token: Vec<u8>,
    },
    MessageSignature {
        salt: i64,
        message_signature: Vec<u8>,
    },
}

#[derive(Debug, PartialEq)]
pub enum PlayerActionStatus {
    StartDigging,
    CancelDigging,
    FinishDigging,
    DropItemStack,
    DropItem,
    ShootArrowOrFinishEating,
    SwapItemInHand,
}

#[derive(Debug, PartialEq)]
pub enum BlockFace {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
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
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

#[derive(Debug, PartialEq)]
pub enum MainHand {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum Arm {
    Main,
    Off,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum RecipeBookType {
    Crafting,
    Furnace,
    BlastFurnace,
    Smoker,
}

#[derive(Debug, PartialEq)]
pub struct Recipe {}

#[derive(Debug, PartialEq)]
pub struct BlockEntity {}

#[derive(Debug, PartialEq)]
pub struct TestBytes(pub Vec<u8>);

pub struct BitSet(
    pub Vec<u64>, // data
    pub usize, // number of bits
);
impl BitSet {
    ///Create a new BitSet
    pub fn new() -> BitSet {
        BitSet(Vec::new(), 0)
    }
    ///Set the bit at the given index
    pub fn set(&mut self, index: usize, value: bool) {
        let byte_index = index / 64;
        let bit_index = index % 64;
        if self.0.len() <= byte_index {
            self.0.resize(byte_index + 1, 0);
        }
        self.0[byte_index] |= (value as u64) << bit_index;
    }
    ///Get the bit at the given index
    pub fn get(&self, index: usize) -> bool {
        let byte_index = index / 64;
        let bit_index = index % 64;
        if self.0.len() <= byte_index {
            false
        } else {
            self.0[byte_index] & (1 << bit_index) != 0
        }
    }
    ///Push a bit to the end of the BitSet
    pub fn push(&mut self, value: bool) {
        self.set(self.1, value);
        self.1 += 1;
    }
}