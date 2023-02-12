use firework_protocol::data_types::DisplaySkinParts;
use firework_protocol_core::SerializeField;
use firework_protocol_derive::SerializeField;
use modular_bitfield::bitfield;

macro_rules! define_entity_metadata_types {
    (
        $(
            $type:ident => $id:literal
        ),*
    ) => {
        macro_rules! __map_type {
            $({ $type } => {$id});*
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
                            <u8 as firework_protocol_core::SerializeField>::serialize(&$id, &mut writer);
                            <firework_protocol_core::VarInt as firework_protocol_core::SerializeField>::serialize(&firework_protocol_core::VarInt(__map_type!($type_idx)), &mut writer);
                            <$type_idx as firework_protocol_core::SerializeField>::serialize(value, &mut writer);
                        }
                    ),*
                }
            }
        }
    };
}

define_entity_metadata_types! {
    EntityDataFlags => 0i32,
    Pose => 19i32,
    DisplaySkinParts => 0i32
}

define_entity_metadata! {
    EntityFlags, 0 => EntityDataFlags,
    EntityPose, 6 => Pose,
    PlayerDisplayedSkinParts, 17 => DisplaySkinParts
}

pub const END_INDEX: u8 = 0xFF;

#[bitfield]
#[derive(Debug, Clone)]
pub struct EntityDataFlags {
    pub is_on_fire: bool,
    pub is_crouching: bool,
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

#[allow(dead_code)]
#[derive(Debug, PartialEq, SerializeField, Clone)]
#[protocol(typ = "firework_protocol_core::VarInt")]
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

#[test]
fn test() {
    let flags = EntityMetadata::EntityFlags(EntityDataFlags::new());
    let mut writer = Vec::new();
    flags.serialize(&mut writer);
    println!("{:?}", writer);
    panic!("test")
}
