use super::Palette;
use core::hash::Hash;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PalettedContainer<T, const CONTAINER_SIZE: usize> {
    pub palette: Vec<T>,
    pub data: Option<Vec<i64>>,
}

const BITS_PER_ENTRY: usize = 64;

impl<T, const CONTAINER_SIZE: usize> PalettedContainer<T, CONTAINER_SIZE>
where
    T: std::fmt::Debug + Eq + Hash + Palette,
{
    pub fn get(&self, index: usize) -> Option<&T> {
        let bits_per_value = self.bits_per_value();
        if bits_per_value == 0 && self.palette.len() == 1 {
            return Some(&self.palette[0]);
        }
        let values_per_long = BITS_PER_ENTRY / bits_per_value;
        let array_index = index / values_per_long;
        let long = self.data.as_ref().unwrap()[array_index];
        let offset = (index % values_per_long) * bits_per_value;
        let mask = (1 << bits_per_value) - 1;
        let value_index = (long >> offset) & mask as i64;
        Some(&self.palette[value_index as usize])
    }
    pub fn bits_per_value(&self) -> usize {
        if let Some(data) = self.data.as_ref() {
            data.len() * BITS_PER_ENTRY / CONTAINER_SIZE
        } else {
            0
        }
    }
}
