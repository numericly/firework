use super::{Boost, Checkpoint};
use firework::{AxisAlignedBB, BlockPos, Vec3};

pub const SPAWN_POSITION: Vec3 = Vec3::new(0.5, 144.0, 0.5);

pub const SPAWN_AREA: AxisAlignedBB = AxisAlignedBB {
    max: BlockPos { x: 7, y: 146, z: 5 },
    min: BlockPos {
        x: -7,
        y: 142,
        z: -4,
    },
};

pub const BOOSTS: [Boost; 0] = [];

pub const CHECKPOINTS: [Checkpoint; 0] = [];
