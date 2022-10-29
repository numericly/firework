pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Play,
}

pub mod data_types {
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
}

pub mod client_bound {
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

    use std::{collections::HashMap, io::Write};

    use byteorder::{BigEndian, WriteBytesExt};
    use data::v1_19_2::tags::VarIntList;
    use quartz_nbt::{
        io::{write_nbt, Flavor},
        NbtCompound,
    };

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
            heightmaps: NbtCompound,
            data: Vec<u8>,
            block_entities: Vec<BlockEntity>,
            manual_data: TestBytes
        },
        LoginWorld, 0x25, Play => {
            entity_id: i32,
            is_hardcore: bool,
            game_mode: u8,
            previous_game_mode: i8,
            dimensions: Vec<String>,
            registry_codec: NbtCompound,
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

    impl SerializeField for NbtCompound {
        fn serialize<W: Write>(&self, mut writer: W) {
            write_nbt(&mut writer, None, self, Flavor::Uncompressed)
                .expect("Failed to serialize NBT");
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
                vec![
                    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255
                ]
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
                    0x0D, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64,
                    0x21
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
                    0x08, 0x74, 0x65, 0x78, 0x74, 0x75, 0x72, 0x65, 0x73, 0x05, 0x76, 0x61, 0x6C,
                    0x75, 0x65, 0x01, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65
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
                    0x08, 0x74, 0x65, 0x78, 0x74, 0x75, 0x72, 0x65, 0x73, 0x05, 0x76, 0x61, 0x6C,
                    0x75, 0x65, 0x00
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
}

pub mod server_bound {
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
        SetPlayerAndRotationPosition, 0x15, Play => {
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
        use crate::tesr::client_bound::SerializeField;

        #[test]
        fn test_position() {
            let position = Position { x: -1, y: 2, z: 3 };
            let mut buffer = Vec::new();
            position.serialize(&mut buffer);
            let deserialized = Position::deserialize(&mut buffer.as_slice()).unwrap();
            assert_eq!(position, deserialized);
        }
    }
}
