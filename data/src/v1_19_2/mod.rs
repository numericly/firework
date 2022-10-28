use std::hash::Hash;

pub trait Palette {
    fn get_palette(state: &Self) -> i32
    where
        Self: Sized + Hash + Eq;
}

pub mod biomes;
pub mod blocks;
pub mod chunk;
pub mod data_structure;
pub mod tags;
