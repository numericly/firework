pub mod serializer;

pub struct VarInt(pub i32);

pub struct VarLong(pub i64);

pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct SizedArray<T> {
    pub len: VarInt,
    pub data: Vec<T>,
}