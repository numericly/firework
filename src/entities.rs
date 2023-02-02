use modular_bitfield::bitfield;
use protocol_core::SerializeField;
use protocol_derive::SerializeField;

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
                            <u8 as protocol_core::SerializeField>::serialize(&$id, &mut writer);
                            <protocol_core::VarInt as protocol_core::SerializeField>::serialize(&protocol_core::VarInt(__map_type!($type_idx)), &mut writer);
                            <$type_idx as protocol_core::SerializeField>::serialize(value, &mut writer);
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
    DisplayedSkinPartsFlags => 0i32
}

define_entity_metadata! {
    EntityFlags, 0 => EntityDataFlags,
    EntityPose, 6 => Pose,
    PlayerDisplayedSkinParts, 17 => DisplayedSkinPartsFlags
}

pub const END_INDEX: u8 = 0xFF;

#[bitfield]
#[derive(Debug, Clone)]
pub struct EntityDataFlags {
    pub is_on_fire: bool,
    pub is_crouching: bool,
    _riding: bool,
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

#[bitfield(bits = 7)]
#[derive(Debug, Clone)]
pub struct DisplayedSkinPartsFlags {
    pub cape: bool,
    pub jacket: bool,
    pub left_sleeve: bool,
    pub right_sleeve: bool,
    pub left_pants: bool,
    pub right_pants: bool,
    pub hat: bool,
}

#[derive(Debug, PartialEq, SerializeField, Clone)]
#[protocol(typ = "protocol_core::VarInt")]
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

impl SerializeField for DisplayedSkinPartsFlags {
    fn serialize<T: std::io::Write>(&self, mut writer: T) {
        self.bytes.serialize(&mut writer);
    }
}

#[test]
fn test() {
    let flags = EntityMetadata::EntityFlags(EntityDataFlags::new());
    let mut writer = Vec::new();
    flags.serialize(&mut writer);
    println!("{:?}", writer);
    panic!("test")
}
