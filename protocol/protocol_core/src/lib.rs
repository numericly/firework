use std::io::{self, Read, Write};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeserializeError {
    #[error("invalid packet ID 0x{id:02x?} for state {state}")]
    InvalidPacketID { id: i32, state: u8 },

    #[error("could not deserialize VarInt because it is greater than 5 bytes")]
    VarIntTooLong,

    #[error("had some error while deserializing an NBT blob: {0}")]
    NbtBlobError(#[from] nbt::Error),

    #[error("could not parse string {0}")]
    StringParseError(#[from] std::string::FromUtf8Error),

    #[error("unknown variant {0} for {1}")]
    InvalidVariantIndex(i32, &'static str),

    #[error("an IO error occurred {0}")]
    IoError(#[from] io::Error),
}

#[derive(Debug, PartialEq, Clone)]
#[repr(transparent)]
pub struct VarInt(pub i32);

impl From<i32> for VarInt {
    fn from(val: i32) -> Self {
        Self(val)
    }
}

impl From<VarInt> for i32 {
    fn from(val: VarInt) -> Self {
        val.0
    }
}

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct UnsizedVec<T: DeserializeField + SerializeField>(pub Vec<T>);

#[derive(PartialEq)]

pub struct BitSet(
    pub Vec<u64>, // data
    usize,        // number of bits
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
        return (self.0[index / 64] >> (index % 64)) & 1 == 1;
    }
    ///Push a bit to the end of the BitSet
    pub fn push(&mut self, value: bool) {
        self.set(self.1, value);
        self.1 += 1;
    }
    pub fn resize(&mut self, new_size: usize, value: bool) {
        if new_size > self.1 {
            for _ in self.1..new_size {
                self.push(value);
            }
        } else {
            self.1 = new_size;
        }
    }
    pub fn ones(&self) -> usize {
        let mut count = 0;
        for i in 0..self.1 {
            if self.get(i) {
                count += 1;
            }
        }
        count
    }
    pub fn zeros(&self) -> usize {
        self.1 - self.ones()
    }
}

impl std::fmt::Debug for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.1 {
            write!(f, "{}", self.get(i) as u8)?;
            if i % 64 == 63 {
                write!(f, " ")?;
            } else if i % 8 == 7 {
                write!(f, "_")?;
            }
        }
        Ok(())
    }
}

#[test]
fn test_bitset_basic() {
    let mut bitset = BitSet::new();
    bitset.push(false);
    bitset.push(false);
    bitset.push(true);

    println!("length: {}", bitset.1);
    print!("data: ");
    for i in 0..bitset.0.len() {
        print!("{:b} ", bitset.0[i]);
    }
    assert_eq!(bitset.get(2), true);
}

#[test]
fn test_bitset_floating_set() {
    let mut bitset = BitSet::new();
    bitset.set(50, true);
    assert_eq!(bitset.get(50), true);
}

#[test]
fn test_bitset_nonexistent_get() {
    let mut bitset = BitSet::new();
    assert_eq!(bitset.get(50), false);
}

#[test]
fn test_bitset_length() {
    let mut bitset = BitSet::new();
    for _ in 0..64 {
        bitset.push(true);
    }
    println!("length: {}", bitset.1);
    print!("data: ");
    for i in 0..bitset.0.len() {
        print!("{:b} ", bitset.0[i]);
    }
    assert_eq!(bitset.1, 64);
}

pub trait DeserializeField {
    fn deserialize<R: Read>(reader: R) -> Result<Self, DeserializeError>
    where
        Self: Sized;
}

pub trait SerializeField {
    fn serialize<W: Write>(&self, writer: W);
}

mod deserializer {

    use crate::{DeserializeError, DeserializeField, Position, SerializeField, UnsizedVec, VarInt};
    use byteorder::ReadBytesExt;
    use nbt::Blob;
    use std::io::Read;

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

    impl<T: DeserializeField + SerializeField> DeserializeField for UnsizedVec<T> {
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

    impl DeserializeField for String {
        fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
            let length = VarInt::deserialize(&mut reader)?.0 as usize;
            let mut data = vec![0; length];
            reader.read_exact(&mut data)?;
            Ok(String::from_utf8(data)?)
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

    impl DeserializeField for Blob {
        fn deserialize<R: Read>(mut reader: R) -> Result<Self, DeserializeError> {
            Ok(Blob::from_reader(&mut reader)?)
        }
    }
}

mod serializer {
    use std::{collections::HashMap, io::Write};

    use byteorder::{BigEndian, WriteBytesExt};
    use minecraft_data::tags::VarIntList;
    use nbt::Blob;

    use crate::{BitSet, Position, SerializeField, UnsizedVec, VarInt};

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

    impl SerializeField for UnsizedVec<u8> {
        fn serialize<W: Write>(&self, mut writer: W) {
            writer.write_all(self.0.as_ref()).unwrap();
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
}
