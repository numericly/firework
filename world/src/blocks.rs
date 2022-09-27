use crate::{
    materials::{MaterialColor, Materials},
    sound::SoundType,
};

pub struct BlockProperties {
    material: Materials,
    material_color: MaterialColor,
    has_collision: bool,
    sound_type: SoundType,
}
