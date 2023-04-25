use firework_protocol::{
    core::SerializeField, data_types::DisplaySkinParts, protocol_derive::SerializeField,
};
use modular_bitfield::bitfield;

macro_rules! define_entity_metadata_types {
    (
        $(
            $type:ident => $id:literal
        ),*
    ) => {
        macro_rules! __map_type {
            $({ $type } => {$id as i32});*
        }
    };
}
macro_rules! define_entity_metadata {
    (
        $(
            $name:ident, $id:literal => $type_idx:ident
        ),*
    ) => {
        #[derive(Debug, Clone)]
        pub enum EntityMetadata {
            $(
                $name($type_idx)
            ),*
        }

        impl EntityMetadata {
            pub fn serialize<T: std::io::Write>(&self, mut writer: T) {
                match self {
                    $(
                        EntityMetadata::$name(value) => {
                            <u8 as firework_protocol::core::SerializeField>::serialize(&$id, &mut writer);
                            <firework_protocol::core::VarInt as firework_protocol::core::SerializeField>::serialize(&firework_protocol::core::VarInt(__map_type!($type_idx)), &mut writer);
                            <$type_idx as firework_protocol::core::SerializeField>::serialize(value, &mut writer);
                        }
                    ),*
                }
            }
        }
    };
}

define_entity_metadata_types! {
    EntityDataFlags => 0,
    f32 => 3,
    Pose => 20,
    DisplaySkinParts => 0
}

define_entity_metadata! {
    EntityFlags, 0 => EntityDataFlags,
    EntityPose, 6 => Pose,
    EntityHealth, 9 => f32,
    PlayerDisplayedSkinParts, 17 => DisplaySkinParts
}

pub const END_INDEX: u8 = 0xFF;

#[bitfield]
#[derive(Debug, Clone)]
pub struct EntityDataFlags {
    pub is_on_fire: bool,
    pub is_crouching: bool,
    #[allow(dead_code)]
    deprecated_riding: bool,
    pub is_sprinting: bool,
    pub is_swimming: bool,
    pub is_invisible: bool,
    pub has_glowing_effect: bool,
    pub is_elytra_flying: bool,
}

impl SerializeField for EntityDataFlags {
    fn serialize<T: std::io::Write>(&self, mut writer: T) {
        self.bytes.serialize(&mut writer);
    }
}

#[derive(Debug, PartialEq, SerializeField, Clone)]
#[protocol(typ = "firework_protocol::core::VarInt")]
pub enum Pose {
    Standing,
    FallFlying,
    Sleeping,
    Swimming,
    SpinAttack,
    Sneaking,
    LongJumping,
    Dying,
    Croaking,
    UsingTongue,
    Sitting,
    Roaring,
    Sniffing,
    Emerging,
    Digging,
}
