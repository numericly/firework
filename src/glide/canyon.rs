use firework::{AxisAlignedBB, AxisAlignedPlane, BlockPos, Rotation, Vec3};

use super::{Boost, BoostParticleType, Checkpoint, Loft};

pub const SPAWN_POSITION: Vec3 = Vec3::new(0.5, 171.0, 10.5);

pub const SPAWN_AREA: AxisAlignedBB = AxisAlignedBB {
    max: BlockPos { x: 4, y: 169, z: 8 },
    min: BlockPos {
        x: -4,
        y: 166,
        z: -3,
    },
};

pub const BOOSTS: [Boost; 8] = [
    Boost {
        area: AxisAlignedBB {
            max: BlockPos {
                x: -38,
                y: 106,
                z: 315,
            },
            min: BlockPos {
                x: -43,
                y: 101,
                z: 307,
            },
        },
        speed: 2.,
        particle_type: BoostParticleType::BoostSouth,
    },
    Boost {
        area: AxisAlignedBB {
            max: BlockPos {
                x: -38,
                y: 105,
                z: 360,
            },
            min: BlockPos {
                x: -43,
                y: 100,
                z: 352,
            },
        },
        speed: 2.,
        particle_type: BoostParticleType::BoostSouth,
    },
    Boost {
        area: AxisAlignedBB {
            max: BlockPos {
                x: 127,
                y: 55,
                z: 615,
            },
            min: BlockPos {
                x: 122,
                y: 50,
                z: 607,
            },
        },
        speed: 3.,
        particle_type: BoostParticleType::BoostNorth,
    },
    Boost {
        area: AxisAlignedBB {
            max: BlockPos {
                x: 140,
                y: 56,
                z: 506,
            },
            min: BlockPos {
                x: 135,
                y: 51,
                z: 498,
            },
        },
        speed: 1.5,
        particle_type: BoostParticleType::BoostNorth,
    },
    Boost {
        area: AxisAlignedBB {
            max: BlockPos {
                x: 41,
                y: -16,
                z: 363,
            },
            min: BlockPos {
                x: 36,
                y: -21,
                z: 355,
            },
        },
        speed: 2.,
        particle_type: BoostParticleType::BoostNorth,
    },
    Boost {
        area: AxisAlignedBB {
            max: BlockPos {
                x: -10,
                y: -20,
                z: 277,
            },
            min: BlockPos {
                x: -15,
                y: -25,
                z: 269,
            },
        },
        speed: 2.2,
        particle_type: BoostParticleType::BoostNorth,
    },
    Boost {
        area: AxisAlignedBB {
            max: BlockPos {
                x: -36,
                y: -16,
                z: 451,
            },
            min: BlockPos {
                x: -41,
                y: -21,
                z: 443,
            },
        },
        speed: 2.5,
        particle_type: BoostParticleType::BoostSouth,
    },
    Boost {
        area: AxisAlignedBB {
            max: BlockPos {
                x: -75,
                y: -21,
                z: 497,
            },
            min: BlockPos {
                x: -83,
                y: -26,
                z: 492,
            },
        },
        speed: 2.,
        particle_type: BoostParticleType::BoostWest,
    },
];

pub const LOFTS: [Loft; 8] = [
    Loft {
        area: AxisAlignedBB {
            min: BlockPos::new(107, 43, 590),
            max: BlockPos::new(109, 48, 592),
        },
        speed: 0.1,
    },
    Loft {
        // 115 41 554, 120 49 559
        area: AxisAlignedBB {
            min: BlockPos::new(115, 41, 554),
            max: BlockPos::new(120, 49, 559),
        },
        speed: 0.2,
    },
    Loft {
        // 6 -33 397, 10 -21 401
        area: AxisAlignedBB {
            min: BlockPos::new(6, -33, 397),
            max: BlockPos::new(10, -21, 401),
        },
        speed: 0.2,
    },
    Loft {
        // -24 12 204, -21 18 207
        area: AxisAlignedBB {
            min: BlockPos::new(-24, 12, 204),
            max: BlockPos::new(-21, 22, 207),
        },
        speed: 0.2,
    },
    Loft {
        // -46 12 265, -42 22 269
        area: AxisAlignedBB {
            min: BlockPos::new(-46, 12, 265),
            max: BlockPos::new(-42, 22, 269),
        },
        speed: 0.2,
    },
    Loft {
        // -123 12 259, -120 22 262
        area: AxisAlignedBB {
            min: BlockPos::new(-123, 12, 259),
            max: BlockPos::new(-120, 22, 262),
        },
        speed: 0.1,
    },
    Loft {
        // -76 29 281, -74 42 283
        area: AxisAlignedBB {
            min: BlockPos::new(-76, 29, 281),
            max: BlockPos::new(-74, 42, 283),
        },
        speed: 0.25,
    },
    Loft {
        // 12 99 302, 15 109 305
        area: AxisAlignedBB {
            min: BlockPos::new(12, 99, 302),
            max: BlockPos::new(15, 109, 305),
        },
        speed: 0.2,
    },
];

pub const CHECKPOINTS: [Checkpoint; 9] = [
    Checkpoint {
        plane: AxisAlignedPlane::Z {
            min: Vec3::new(-4.5, 66., 193.5),
            max: Vec3::new(26.5, 187., 193.5),
        },
        spawn_position: Vec3::new(0.5, 145.5, 168.5),
        spawn_rotation: Rotation::new(-14.7, 36.7),
    },
    Checkpoint {
        plane: AxisAlignedPlane::Z {
            min: Vec3::new(-16.5, 22., 460.5),
            max: Vec3::new(13.5, 150., 460.5),
        },
        spawn_position: Vec3::new(-2., 122.5, 439.),
        spawn_rotation: Rotation::new(4.8, 21.2),
    },
    Checkpoint {
        plane: AxisAlignedPlane::Z {
            min: Vec3::new(106., 35., 575.5),
            max: Vec3::new(142., 71., 575.5),
        },
        spawn_position: Vec3::new(121., 59., 584.),
        spawn_rotation: Rotation::new(-177.7, 9.3),
    },
    Checkpoint {
        plane: AxisAlignedPlane::Y {
            min: Vec3::new(88., 37., 416.),
            max: Vec3::new(155., 37., 458.),
        },
        spawn_position: Vec3::new(145., 51., 434.),
        spawn_rotation: Rotation::new(90., 59.),
    },
    Checkpoint {
        plane: AxisAlignedPlane::Z {
            min: Vec3::new(23., -23., 369.5),
            max: Vec3::new(46., 0., 369.5),
        },
        spawn_position: Vec3::new(33., -10.5, 376.),
        spawn_rotation: Rotation::new(-165., 29.),
    },
    Checkpoint {
        plane: AxisAlignedPlane::X {
            min: Vec3::new(-67.5, 25., 266.),
            max: Vec3::new(-67.5, 51., 283.),
        },
        spawn_position: Vec3::new(-64., 40., 275.),
        spawn_rotation: Rotation::new(109., 37.),
    },
    Checkpoint {
        plane: AxisAlignedPlane::Z {
            min: Vec3::new(-47., -20., 428.5),
            max: Vec3::new(-27., 2., 428.5),
        },
        spawn_position: Vec3::new(-36., -3., 421.),
        spawn_rotation: Rotation::new(9., 26.3),
    },
    Checkpoint {
        plane: AxisAlignedPlane::X {
            min: Vec3::new(-87.5, -28., 485.),
            max: Vec3::new(-87.5, -3., 504.),
        },
        spawn_position: Vec3::new(-69.7, -14., 493.4),
        spawn_rotation: Rotation::new(84.4, 39.9),
    },
    Checkpoint {
        // finish line
        plane: AxisAlignedPlane::Z {
            min: Vec3::new(-112., -55., 669.5),
            max: Vec3::new(-84., 40., 669.5),
        },
        spawn_position: Vec3::new(-98., -35., 660.),
        spawn_rotation: Rotation::new(0., 27.),
    },
];

pub const AUTHOR_TIMES: [f32; 9] = [
    8.511, 10.298, 11.847, 3.898, 6.656, 14.799, 10.302, 4.949, 8.350,
];
