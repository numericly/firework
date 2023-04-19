use firework::{AxisAlignedBB, AxisAlignedPlane, BlockPos, Rotation, Vec3};

use super::{Boost, BoostParticleType, Checkpoint, Loft};

pub const SPAWN_POSITION: Vec3 = Vec3::new(0.0, 165.0, 0.5);

pub const SPAWN_AREA: AxisAlignedBB = AxisAlignedBB {
    max: BlockPos { x: 4, y: 169, z: 8 },
    min: BlockPos {
        x: -4,
        y: 166,
        z: -3,
    },
};

pub const BOOSTS: [Boost; 0] = [];

pub const LOFTS: [Loft; 0] = [];

pub const CHECKPOINTS: [Checkpoint; 0] = [];
