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
                    VarInt::from(Self::id()).serialize(&mut buffer);
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
use std::{collections::HashMap, io::Write};

use byteorder::{BigEndian, WriteBytesExt};
use nbt::{Blob, Value};

use super::data_types::*;

pub trait ClientBoundPacketID {
    fn id() -> i32;
}

pub trait SerializeField {
    fn serialize<W: Write>(&self, writer: W);
}

pub trait SerializePacket {
    fn serialize(&self) -> Vec<u8>;
}

define_client_bound_protocol! {
    ServerStatus, 0x00, Status => {
        response: String
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
    ChangeDifficulty, 0x0B, Play => {
        difficulty: u8,
        locked: bool
    },
    PlayDisconnect, 0x19, Play => {
        reason: String
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
        registry_codec: TestBytes,
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
    PlayerAbilities, 0x31, Play => {
        flags: PlayerAbilityFlags,
        flying_speed: f32,
        walking_speed: f32
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
    SetHeldItem, 0x4A, Play => {
        slot: u8
    },
    SetCenterChunk, 0x4B, Play => {
        x: VarInt,
        z: VarInt
    },
    SetRecipes, 0x6A, Play => {
        recipes: Vec<Recipe>
    },
    SetTags, 0x6B, Play => {
        tags: &'static HashMap<String, HashMap<String, VarIntList>>
    }
}

impl SerializeField for Position {
    fn serialize<W: Write>(&self, writer: W) {
        let x = if self.x >= 0 {
            self.x as u64 & 0x1FFFFFF
        } else {
            self.x as u64 & 0x1FFFFFF | 0x2000000
        };
        let y = if self.y >= 0 {
            self.y as u64 & 0xFFF
        } else {
            self.y as u64 & 0xFFF | 0x1000
        };
        let z = if self.z >= 0 {
            self.z as u64 & 0x1FFFFFF
        } else {
            self.z as u64 & 0x1FFFFFF | 0x2000000
        };
        let pos_u64 = (x << 38) | y | (z << 12);
        pos_u64.serialize(writer);
    }
}

impl SerializeField for DeathLocation {
    fn serialize<W: Write>(&self, mut writer: W) {
        self.dimension_name.serialize(&mut writer);
        self.position.serialize(writer);
    }
}

impl SerializeField for ProfileProperty {
    fn serialize<W: Write>(&self, mut writer: W) {
        self.name.serialize(&mut writer);
        self.value.serialize(&mut writer);
        self.signature.serialize(&mut writer);
    }
}

impl SerializeField for Recipe {
    fn serialize<W: Write>(&self, _writer: W) {
        unimplemented!()
    }
}

impl SerializeField for BlockEntity {
    fn serialize<W: Write>(&self, _writer: W) {
        unimplemented!()
    }
}

impl SerializeField for TestBytes {
    fn serialize<W: Write>(&self, mut writer: W) {
        writer.write_all(&self.0).unwrap();
    }
}

impl<T: SerializeField> SerializeField for Option<T> {
    fn serialize<W: Write>(&self, mut writer: W) {
        match self {
            Some(value) => {
                true.serialize(&mut writer);
                value.serialize(&mut writer);
            }
            None => false.serialize(&mut writer),
        }
    }
}

impl<T: SerializeField> SerializeField for Vec<T> {
    fn serialize<W: Write>(&self, mut writer: W) {
        VarInt(self.len() as i32).serialize(&mut writer);
        for item in self {
            item.serialize(&mut writer);
        }
    }
}

impl<K: SerializeField, V: SerializeField> SerializeField for HashMap<K, V> {
    fn serialize<W: Write>(&self, mut writer: W) {
        VarInt(self.len() as i32).serialize(&mut writer);
        for (key, value) in self {
            key.serialize(&mut writer);
            value.serialize(&mut writer);
        }
    }
}

impl<T: SerializeField, const N: usize> SerializeField for [T; N] {
    fn serialize<W: Write>(&self, mut writer: W) {
        for item in self {
            item.serialize(&mut writer);
        }
    }
}

impl SerializeField for VarIntList {
    fn serialize<W: Write>(&self, mut writer: W) {
        VarInt(self.0.len() as i32).serialize(&mut writer);
        for item in &self.0 {
            VarInt(*item as i32).serialize(&mut writer);
        }
    }
}

impl SerializeField for PlayerAbilityFlags {
    fn serialize<W: Write>(&self, writer: W) {
        self.into_bytes().serialize(writer);
    }
}

impl SerializeField for VarInt {
    fn serialize<W: Write>(&self, mut writer: W) {
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;

        let mut val = self.0 as u32;

        loop {
            let mut current_byte = (val & SEGMENT_BITS as u32) as u8;
            val >>= 7;
            if val != 0 {
                current_byte |= CONTINUE_BIT;
            }
            writer.write(&[current_byte]).unwrap();
            if val == 0 {
                break;
            }
        }
    }
}

impl SerializeField for PlayerPositionFlags {
    fn serialize<W: Write>(&self, writer: W) {
        self.into_bytes().serialize(writer);
    }
}

impl SerializeField for Value {
    fn serialize<W: Write>(&self, mut writer: W) {
        self.to_writer(&mut writer).unwrap();
    }
}

impl SerializeField for Blob {
    fn serialize<W: Write>(&self, mut writer: W) {
        self.to_writer(&mut writer).unwrap();
    }
}

impl SerializeField for String {
    fn serialize<W: Write>(&self, mut writer: W) {
        let bytes = self.as_bytes();
        VarInt(bytes.len() as i32).serialize(&mut writer);
        writer.write_all(bytes).unwrap();
    }
}

impl SerializeField for u8 {
    fn serialize<W: Write>(&self, mut writer: W) {
        writer.write_u8(*self).unwrap();
    }
}

impl SerializeField for u16 {
    fn serialize<W: Write>(&self, mut writer: W) {
        writer.write_u16::<BigEndian>(*self).unwrap();
    }
}

impl SerializeField for u32 {
    fn serialize<W: Write>(&self, mut writer: W) {
        writer.write_u32::<BigEndian>(*self).unwrap();
    }
}

impl SerializeField for u64 {
    fn serialize<W: Write>(&self, mut writer: W) {
        writer.write_u64::<BigEndian>(*self).unwrap();
    }
}

impl SerializeField for u128 {
    fn serialize<W: Write>(&self, mut writer: W) {
        writer.write_u128::<BigEndian>(*self).unwrap();
    }
}

impl SerializeField for i8 {
    fn serialize<W: Write>(&self, mut writer: W) {
        writer.write_i8(*self).unwrap();
    }
}

impl SerializeField for i16 {
    fn serialize<W: Write>(&self, mut writer: W) {
        writer.write_i16::<BigEndian>(*self).unwrap();
    }
}

impl SerializeField for i32 {
    fn serialize<W: Write>(&self, mut writer: W) {
        writer.write_i32::<BigEndian>(*self).unwrap();
    }
}

impl SerializeField for i64 {
    fn serialize<W: Write>(&self, mut writer: W) {
        writer.write_i64::<BigEndian>(*self).unwrap();
    }
}

impl SerializeField for i128 {
    fn serialize<W: Write>(&self, mut writer: W) {
        writer.write_i128::<BigEndian>(*self).unwrap();
    }
}

impl SerializeField for f32 {
    fn serialize<W: Write>(&self, mut writer: W) {
        writer.write_f32::<BigEndian>(*self).unwrap();
    }
}

impl SerializeField for f64 {
    fn serialize<W: Write>(&self, mut writer: W) {
        writer.write_f64::<BigEndian>(*self).unwrap();
    }
}

impl SerializeField for bool {
    fn serialize<W: Write>(&self, mut writer: W) {
        writer.write(&[*self as u8]).unwrap();
    }
}

impl SerializeField for BitSet {
    fn serialize<W: Write>(&self, mut writer: W) {
        self.0.serialize(&mut writer);
    }
}

pub mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_i8() {
        let mut buffer = Vec::new();
        0i8.serialize(&mut buffer);
        assert_eq!(buffer, vec![0]);
        buffer.clear();
        1i8.serialize(&mut buffer);
        assert_eq!(buffer, vec![1]);
        buffer.clear();
        (-1i8).serialize(&mut buffer);
        assert_eq!(buffer, vec![255]);
        buffer.clear();
        i8::MAX.serialize(&mut buffer);
        assert_eq!(buffer, vec![127]);
    }

    #[test]
    fn test_i16() {
        let mut buffer = Vec::new();
        0i16.serialize(&mut buffer);
        assert_eq!(buffer, vec![0, 0]);
        buffer.clear();
        1i16.serialize(&mut buffer);
        assert_eq!(buffer, vec![0, 1]);
        buffer.clear();
        (-1i16).serialize(&mut buffer);
        assert_eq!(buffer, vec![255, 255]);
        buffer.clear();
        i16::MAX.serialize(&mut buffer);
        assert_eq!(buffer, vec![127, 255]);
    }

    #[test]
    fn test_i32() {
        let mut buffer = Vec::new();
        1_i32.serialize(&mut buffer);
        assert_eq!(buffer, vec![0, 0, 0, 1]);
        buffer.clear();
        0_i32.serialize(&mut buffer);
        assert_eq!(buffer, vec![0, 0, 0, 0]);
        buffer.clear();
        (-1_i32).serialize(&mut buffer);
        assert_eq!(buffer, vec![255, 255, 255, 255]);
        buffer.clear();
        i32::MAX.serialize(&mut buffer);
        assert_eq!(buffer, vec![127, 255, 255, 255]);
    }

    #[test]
    fn test_i64() {
        let mut buffer = Vec::new();
        1_i64.serialize(&mut buffer);
        assert_eq!(buffer, vec![0, 0, 0, 0, 0, 0, 0, 1]);
        buffer.clear();
        0_i64.serialize(&mut buffer);
        assert_eq!(buffer, vec![0, 0, 0, 0, 0, 0, 0, 0]);
        buffer.clear();
        (-1_i64).serialize(&mut buffer);
        assert_eq!(buffer, vec![255, 255, 255, 255, 255, 255, 255, 255]);
        buffer.clear();
        i64::MAX.serialize(&mut buffer);
        assert_eq!(buffer, vec![127, 255, 255, 255, 255, 255, 255, 255]);
    }

    #[test]
    fn test_u8() {
        let mut buffer = Vec::new();
        1_u8.serialize(&mut buffer);
        assert_eq!(buffer, vec![1]);
        buffer.clear();
        0_u8.serialize(&mut buffer);
        assert_eq!(buffer, vec![0]);
        buffer.clear();
        u8::MAX.serialize(&mut buffer);
        assert_eq!(buffer, vec![255]);
    }

    #[test]
    fn test_u16() {
        let mut buffer = Vec::new();
        1_u16.serialize(&mut buffer);
        assert_eq!(buffer, vec![0, 1]);
        buffer.clear();
        0_u16.serialize(&mut buffer);
        assert_eq!(buffer, vec![0, 0]);
        buffer.clear();
        u16::MAX.serialize(&mut buffer);
        assert_eq!(buffer, vec![255, 255]);
    }

    #[test]
    fn test_u32() {
        let mut buffer = Vec::new();
        1_u32.serialize(&mut buffer);
        assert_eq!(buffer, vec![0, 0, 0, 1]);
        buffer.clear();
        0_u32.serialize(&mut buffer);
        assert_eq!(buffer, vec![0, 0, 0, 0]);
        buffer.clear();
        u32::MAX.serialize(&mut buffer);
        assert_eq!(buffer, vec![255, 255, 255, 255]);
    }

    #[test]
    fn test_u64() {
        let mut buffer = Vec::new();
        1_u64.serialize(&mut buffer);
        assert_eq!(buffer, vec![0, 0, 0, 0, 0, 0, 0, 1]);
        buffer.clear();
        0_u64.serialize(&mut buffer);
        assert_eq!(buffer, vec![0, 0, 0, 0, 0, 0, 0, 0]);
        buffer.clear();
        u64::MAX.serialize(&mut buffer);
        assert_eq!(buffer, vec![255, 255, 255, 255, 255, 255, 255, 255]);
    }

    #[test]
    fn test_u128() {
        let mut buffer = Vec::new();
        1_u128.serialize(&mut buffer);
        assert_eq!(buffer, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        buffer.clear();
        0_u128.serialize(&mut buffer);
        assert_eq!(buffer, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        buffer.clear();
        u128::MAX.serialize(&mut buffer);
        assert_eq!(
            buffer,
            vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]
        );
    }

    #[test]
    fn test_bool() {
        let mut buffer = Vec::new();
        true.serialize(&mut buffer);
        assert_eq!(buffer, vec![1]);
        buffer.clear();
        false.serialize(&mut buffer);
        assert_eq!(buffer, vec![0]);
    }

    #[test]
    fn test_option() {
        let mut buffer = Vec::new();
        let value: Option<i32> = None;
        value.serialize(&mut buffer);
        assert_eq!(buffer, vec![0]);
        buffer.clear();
        let value: Option<i32> = Some(1);
        value.serialize(&mut buffer);
        assert_eq!(buffer, vec![1, 0, 0, 0, 1]);
    }

    #[test]
    fn test_vec() {
        let mut buffer = Vec::new();
        let vec = vec![1, 2, 3, 4, 5];
        vec.serialize(&mut buffer);
        assert_eq!(
            buffer,
            vec![5, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5]
        );
        buffer.clear();
        let vec = vec![true, false, true, false, true, false];
        vec.serialize(&mut buffer);
        assert_eq!(buffer, vec![6, 1, 0, 1, 0, 1, 0]);
    }

    #[test]
    fn test_var_int() {
        let mut buffer = Vec::new();
        VarInt(0).serialize(&mut buffer);
        assert_eq!(buffer, vec![0]);
        buffer.clear();
        VarInt(1).serialize(&mut buffer);
        assert_eq!(buffer, vec![1]);
        buffer.clear();
        VarInt(127).serialize(&mut buffer);
        assert_eq!(buffer, vec![127]);
        buffer.clear();
        VarInt(128).serialize(&mut buffer);
        assert_eq!(buffer, vec![128, 1]);
        buffer.clear();
        VarInt(255).serialize(&mut buffer);
        assert_eq!(buffer, vec![255, 1]);
        buffer.clear();
        VarInt(256).serialize(&mut buffer);
        assert_eq!(buffer, vec![128, 2]);
        buffer.clear();
        VarInt(2147483647).serialize(&mut buffer);
        assert_eq!(buffer, vec![255, 255, 255, 255, 7]);
        buffer.clear();
        VarInt(-1).serialize(&mut buffer);
        assert_eq!(buffer, vec![255, 255, 255, 255, 15]);
        buffer.clear();
        VarInt(-2147483648).serialize(&mut buffer);
        assert_eq!(buffer, vec![128, 128, 128, 128, 8]);
    }

    #[test]
    fn test_string() {
        let mut buffer = Vec::new();
        "Hello, world!".to_string().serialize(&mut buffer);
        assert_eq!(
            buffer,
            vec![
                0x0D, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21
            ]
        );
    }

    #[test]
    fn test_profile_property() {
        let mut buffer = Vec::new();
        ProfileProperty {
            name: "textures".to_string(),
            value: "value".to_string(),
            signature: Some("signature".to_string()),
        }
        .serialize(&mut buffer);
        assert_eq!(
            buffer,
            vec![
                0x08, 0x74, 0x65, 0x78, 0x74, 0x75, 0x72, 0x65, 0x73, 0x05, 0x76, 0x61, 0x6C, 0x75,
                0x65, 0x01, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65
            ]
        );
        buffer.clear();
        ProfileProperty {
            name: "textures".to_string(),
            value: "value".to_string(),
            signature: None,
        }
        .serialize(&mut buffer);
        assert_eq!(
            buffer,
            vec![
                0x08, 0x74, 0x65, 0x78, 0x74, 0x75, 0x72, 0x65, 0x73, 0x05, 0x76, 0x61, 0x6C, 0x75,
                0x65, 0x00
            ]
        );
    }

    #[test]
    fn test_position() {
        let mut buffer = Vec::new();
        Position { x: 0, y: 0, z: 0 }.serialize(&mut buffer);
        let pos_u64 = u64::from_be_bytes(buffer.try_into().unwrap());
        assert_eq!(
            pos_u64,
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
        );
        let mut buffer = Vec::new();
        Position { x: 1, y: 1, z: 1 }.serialize(&mut buffer);
        let pos_u64 = u64::from_be_bytes(buffer.try_into().unwrap());
        assert_eq!(
            pos_u64,
            0b00000000_00000000_00000000_01000000_00000000_00000000_00010000_00000001
        );
        let mut buffer = Vec::new();
        Position {
            x: -1,
            y: -1,
            z: -1,
        }
        .serialize(&mut buffer);
        let pos_u64 = u64::from_be_bytes(buffer.try_into().unwrap());
        assert_eq!(
            pos_u64,
            0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111
        );
        let mut buffer = Vec::new();
        Position {
            x: 2147483647,
            y: 2047,
            z: 2147483647,
        }
        .serialize(&mut buffer);
        let pos_u64 = u64::from_be_bytes(buffer.try_into().unwrap());
        assert_eq!(
            pos_u64,
            0b01111111_11111111_11111111_11011111_11111111_11111111_11110111_11111111
        );
    }
}
