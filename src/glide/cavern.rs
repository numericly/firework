use super::{Boost, Checkpoint, Loft};
use firework::{AxisAlignedBB, BlockPos, Rotation, Vec3};

pub const SPAWN_POSITION: Vec3 = Vec3::new(0.5, 144.0, 0.5);

pub const SPAWN_AREA: AxisAlignedBB = AxisAlignedBB {
    max: BlockPos { x: 7, y: 146, z: 5 },
    min: BlockPos {
        x: -7,
        y: 142,
        z: -4,
    },
};

pub const BOOSTS: [Boost; 17] = [
    Boost {
        // 102 93 174, 108 97 179
        area: AxisAlignedBB::new(BlockPos::new(102, 93, 174), BlockPos::new(108, 97, 179)),
        speed: 4.,
        particle_type: super::BoostParticleType::BoostEast,
    },
    Boost {
        // 202 76 183, 210 81 188
        area: AxisAlignedBB::new(BlockPos::new(202, 76, 183), BlockPos::new(210, 81, 188)),
        speed: 4.5,
        particle_type: super::BoostParticleType::BoostEast,
    },
    Boost {
        // 292 70 131, 297 75 139
        area: AxisAlignedBB::new(BlockPos::new(292, 70, 131), BlockPos::new(297, 75, 139)),
        speed: 5.,
        particle_type: super::BoostParticleType::BoostNorth,
    },
    Boost {
        // 249 59 89, 254 64 97
        area: AxisAlignedBB::new(BlockPos::new(249, 59, 89), BlockPos::new(254, 64, 97)),
        speed: 5.,
        particle_type: super::BoostParticleType::BoostSouth,
    },
    Boost {
        // 156 61 139, 161 66 147
        area: AxisAlignedBB::new(BlockPos::new(156, 61, 139), BlockPos::new(161, 66, 147)),
        speed: 4.,
        particle_type: super::BoostParticleType::BoostSouth,
    },
    Boost {
        // 187 39 119, 192 44 127
        area: AxisAlignedBB::new(BlockPos::new(187, 39, 119), BlockPos::new(192, 44, 127)),
        speed: 5.,
        particle_type: super::BoostParticleType::BoostNorth,
    },
    Boost {
        // 101 -5 30, 106 0 42
        area: AxisAlignedBB::new(BlockPos::new(101, -5, 30), BlockPos::new(106, 0, 42)),
        speed: 4.5,
        particle_type: super::BoostParticleType::BoostSouth,
    },
    Boost {
        // 102 8 36, 104 11 43
        area: AxisAlignedBB::new(BlockPos::new(102, 8, 36), BlockPos::new(105, 11, 43)),
        speed: 4.5,
        particle_type: super::BoostParticleType::BoostSouth,
    },
    Boost {
        // 2 4 174, 9 9 179
        area: AxisAlignedBB::new(BlockPos::new(2, 4, 174), BlockPos::new(9, 9, 179)),
        speed: 3.,
        particle_type: super::BoostParticleType::BoostWest,
    },
    Boost {
        // -25 0 178, -30 5 182
        area: AxisAlignedBB::new(BlockPos::new(-25, 0, 178), BlockPos::new(-30, 5, 182)),
        speed: 4.,
        particle_type: super::BoostParticleType::BoostWest,
    },
    Boost {
        // -72 -5 144, -67 0 152
        area: AxisAlignedBB::new(BlockPos::new(-72, -5, 144), BlockPos::new(-67, 0, 152)),
        speed: 4.,
        particle_type: super::BoostParticleType::BoostNorth,
    },
    Boost {
        // -30 0 178, -26 3 182
        area: AxisAlignedBB::new(BlockPos::new(-30, 0, 177), BlockPos::new(-26, 3, 182)),
        speed: 5.,
        particle_type: super::BoostParticleType::BoostWest,
    },
    Boost {
        // -19 8 59, -14 13 66
        area: AxisAlignedBB::new(BlockPos::new(-19, 8, 59), BlockPos::new(-14, 13, 66)),
        speed: 4.,
        particle_type: super::BoostParticleType::BoostNorth,
    },
    Boost {
        // 0 29 67, 5 34 75
        area: AxisAlignedBB::new(BlockPos::new(0, 29, 67), BlockPos::new(5, 34, 75)),
        speed: 5.,
        particle_type: super::BoostParticleType::BoostNorth,
    },
    Boost {
        // 0 29 -9, 5 34 -1
        area: AxisAlignedBB::new(BlockPos::new(0, 29, -9), BlockPos::new(5, 34, -1)),
        speed: 2.,
        particle_type: super::BoostParticleType::BoostSouth,
    },
    Boost {
        // 34 13 -38, 39 18 -30
        area: AxisAlignedBB::new(BlockPos::new(34, 13, -38), BlockPos::new(39, 18, -30)),
        speed: 4.,
        particle_type: super::BoostParticleType::BoostSouth,
    },
    Boost {
        // 51 7 -36, 56 12 -28
        area: AxisAlignedBB::new(BlockPos::new(51, 7, -36), BlockPos::new(56, 12, -28)),
        speed: 4.,
        particle_type: super::BoostParticleType::BoostSouth,
    },
];

pub const LOFTS: [Loft; 1] = [Loft {
    area: AxisAlignedBB {
        // 27 95 149, 30 105 152
        min: BlockPos::new(27, 95, 149),
        max: BlockPos::new(30, 105, 152),
    },
    speed: 0.35,
}];

pub const CHECKPOINTS: [Checkpoint; 9] = [
    Checkpoint {
        // 54.5 96 155.5, 54.5 123 178.5
        plane: firework::AxisAlignedPlane::X {
            min: Vec3::new(54.5, 96., 155.5),
            max: Vec3::new(54.5, 123., 178.5),
        },
        spawn_position: Vec3::new(43., 118., 163.),
        spawn_rotation: Rotation::new(-76.4, 29.4),
    },
    Checkpoint {
        // 301 71 127.5, 283 91 127.5
        plane: firework::AxisAlignedPlane::Z {
            min: Vec3::new(283., 71., 127.5),
            max: Vec3::new(301., 91., 127.5),
        },
        spawn_position: Vec3::new(292., 81., 147.5),
        spawn_rotation: Rotation::new(-180., 33.),
    },
    Checkpoint {
        // 152 63 157.5, 164 75 157.5
        plane: firework::AxisAlignedPlane::Z {
            min: Vec3::new(152., 63., 157.5),
            max: Vec3::new(164., 75., 157.5),
        },
        spawn_position: Vec3::new(162., 69., 133.),
        spawn_rotation: Rotation::new(16., 33.),
    },
    Checkpoint {
        // 179.5 26 62.5, 199.5 62 62.5
        plane: firework::AxisAlignedPlane::Z {
            min: Vec3::new(179.5, 26., 62.5),
            max: Vec3::new(199.5, 62., 62.5),
        },
        spawn_position: Vec3::new(189., 50., 74.),
        spawn_rotation: Rotation::new(-180., 52.),
    },
    Checkpoint {
        // 111 -4 26.5, 96 14 26.5
        plane: firework::AxisAlignedPlane::Z {
            min: Vec3::new(96., -4., 26.5),
            max: Vec3::new(111., 14., 26.5),
        },
        spawn_position: Vec3::new(103., 2., 24.),
        spawn_rotation: Rotation::new(0., 30.),
    },
    Checkpoint {
        // 81 18 132.5, 67 36 132.5
        plane: firework::AxisAlignedPlane::Z {
            min: Vec3::new(67., 18., 132.5),
            max: Vec3::new(81., 36., 132.5),
        },
        spawn_position: Vec3::new(75., 27., 130.),
        spawn_rotation: Rotation::new(6.6, 47.),
    },
    // TODO those crazy checkpoints
    Checkpoint {
        // this is technically two checkpoints at slightly different positions
        // but i just bundled them into one because it's easier
        // when there's a solution for the previous checkpoint, this one is solved
        // 3 18 -28.5, 12 26 -28.5
        plane: firework::AxisAlignedPlane::Z {
            min: Vec3::new(3., 18., -29.),
            max: Vec3::new(12., 44., -29.),
        },
        // thus, the spawn position can only be for one of them
        // i chose a spot on the top one arbitrarily
        spawn_position: Vec3::new(4., 37., -30.),
        spawn_rotation: Rotation::new(-169., 55.),
    },
    Checkpoint {
        // 47.5 16 75, 47.5 16 47
        plane: firework::AxisAlignedPlane::X {
            min: Vec3::new(47.5, 16., 47.),
            max: Vec3::new(47.5, 50., 75.),
        },
        spawn_position: Vec3::new(43., 27., 53.),
        spawn_rotation: Rotation::new(-72., 50.),
    },
    Checkpoint {
        // finish line
        // 224.5 -52 68, 224.5 -32 55
        plane: firework::AxisAlignedPlane::X {
            min: Vec3::new(224.5, -52., 55.),
            max: Vec3::new(224.5, -32., 68.),
        },
        spawn_position: Vec3::new(220., -40., 61.),
        spawn_rotation: Rotation::new(-90., -30.),
    },
];
