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

pub const BOOSTS: [Boost; 10] = [
    Boost {
        // 9 95 265, 14 100 273
        area: AxisAlignedBB {
            min: BlockPos::new(9, 95, 265),
            max: BlockPos::new(14, 100, 273),
        },
        speed: 2.85,
        particle_type: BoostParticleType::BoostNorth,
    },
    Boost {
        // -17 70 262, -12 75 270
        area: AxisAlignedBB {
            min: BlockPos::new(-17, 70, 262),
            max: BlockPos::new(-12, 75, 270),
        },
        speed: 2.85,
        particle_type: BoostParticleType::BoostSouth,
    },
    Boost {
        // 30 70 258, 35 75 266
        area: AxisAlignedBB {
            min: BlockPos::new(30, 70, 258),
            max: BlockPos::new(35, 75, 266),
        },
        speed: 2.85,
        particle_type: BoostParticleType::BoostSouth,
    },
    Boost {
        // 19 74 424, 27 79 429
        area: AxisAlignedBB {
            min: BlockPos::new(19, 74, 424),
            max: BlockPos::new(27, 79, 429),
        },
        speed: 2.85,
        particle_type: BoostParticleType::BoostWest,
    },
    Boost {
        // 24 71 465, 32 76 470
        area: AxisAlignedBB {
            min: BlockPos::new(24, 71, 465),
            max: BlockPos::new(32, 76, 470),
        },
        speed: 2.85,
        particle_type: BoostParticleType::BoostEast,
    },
    Boost {
        // 107 79 408, 115 84 413
        area: AxisAlignedBB {
            min: BlockPos::new(107, 79, 408),
            max: BlockPos::new(115, 84, 413),
        },
        speed: 2.85,
        particle_type: BoostParticleType::BoostEast,
    },
    Boost {
        // 214 55 417, 222 60 422
        area: AxisAlignedBB {
            min: BlockPos::new(214, 55, 417),
            max: BlockPos::new(222, 60, 422),
        },
        speed: 2.85,
        particle_type: BoostParticleType::BoostWest,
    },
    Boost {
        // 178 50 389, 183 55 397
        area: AxisAlignedBB {
            min: BlockPos::new(178, 50, 389),
            max: BlockPos::new(183, 55, 397),
        },
        speed: 2.85,
        particle_type: BoostParticleType::BoostNorth,
    },
    Boost {
        // 188 -26 176, 193 -21 184
        area: AxisAlignedBB {
            min: BlockPos::new(188, -26, 176),
            max: BlockPos::new(193, -21, 184),
        },
        speed: 2.85,
        particle_type: BoostParticleType::BoostSouth,
    },
    Boost {
        // 188 -39 301, 193 -34 309
        area: AxisAlignedBB {
            min: BlockPos::new(188, -39, 301),
            max: BlockPos::new(193, -34, 309),
        },
        speed: 2.85,
        particle_type: BoostParticleType::BoostSouth,
    },
];

pub const LOFTS: [Loft; 24] = [
    Loft {
        // 25 125 158, 28 135 161
        area: AxisAlignedBB {
            min: BlockPos::new(25, 125, 158),
            max: BlockPos::new(28, 135, 161),
        },
        speed: 0.2,
    },
    Loft {
        // positively massive
        // -10 66 323, 29 79 335
        area: AxisAlignedBB {
            min: BlockPos::new(-10, 66, 323),
            max: BlockPos::new(29, 79, 335),
        },
        speed: 0.3,
    },
    Loft {
        // -6 72 438, -3 82 441
        area: AxisAlignedBB {
            min: BlockPos::new(-6, 72, 438),
            max: BlockPos::new(-3, 82, 441),
        },
        speed: 0.2,
    },
    Loft {
        // 87 72 466, 90 82 469
        area: AxisAlignedBB {
            min: BlockPos::new(87, 72, 466),
            max: BlockPos::new(90, 82, 469),
        },
        speed: 0.2,
    },
    Loft {
        // 87 72 471, 90 82 474
        area: AxisAlignedBB {
            min: BlockPos::new(87, 72, 471),
            max: BlockPos::new(90, 82, 474),
        },
        speed: 0.2,
    },
    Loft {
        // 97 74 464, 100 84 467
        area: AxisAlignedBB {
            min: BlockPos::new(97, 74, 464),
            max: BlockPos::new(100, 84, 467),
        },
        speed: 0.2,
    },
    Loft {
        // 101 74 468, 104 84 471
        area: AxisAlignedBB {
            min: BlockPos::new(101, 74, 468),
            max: BlockPos::new(104, 84, 471),
        },
        speed: 0.2,
    },
    Loft {
        // 98 79 453, 101 89 456
        area: AxisAlignedBB {
            min: BlockPos::new(98, 79, 453),
            max: BlockPos::new(101, 89, 456),
        },
        speed: 0.2,
    },
    Loft {
        // 103 80 453, 106 90 456
        area: AxisAlignedBB {
            min: BlockPos::new(103, 80, 453),
            max: BlockPos::new(106, 90, 456),
        },
        speed: 0.2,
    },
    Loft {
        // 140 77 402, 143 83 405
        area: AxisAlignedBB {
            min: BlockPos::new(140, 77, 402),
            max: BlockPos::new(143, 83, 405),
        },
        speed: 0.2,
    },
    Loft {
        // 140 77 416, 143 83 419
        area: AxisAlignedBB {
            min: BlockPos::new(140, 77, 416),
            max: BlockPos::new(143, 83, 419),
        },
        speed: 0.2,
    },
    Loft {
        // 156 86 442, 159 92 445
        area: AxisAlignedBB {
            min: BlockPos::new(156, 86, 442),
            max: BlockPos::new(159, 92, 445),
        },
        speed: 0.2,
    },
    Loft {
        // 221 76 431, 224 82 434
        area: AxisAlignedBB {
            min: BlockPos::new(221, 76, 431),
            max: BlockPos::new(224, 82, 434),
        },
        speed: 0.2,
    },
    Loft {
        // 221 77 387, 224, 83, 390
        area: AxisAlignedBB {
            min: BlockPos::new(221, 77, 387),
            max: BlockPos::new(224, 83, 390),
        },
        speed: 0.2,
    },
    Loft {
        // 179 81 409, 182 86 412
        area: AxisAlignedBB {
            min: BlockPos::new(179, 81, 409),
            max: BlockPos::new(182, 86, 412),
        },
        speed: 0.2,
    },
    Loft {
        // 169 41 317, 172 50 320
        area: AxisAlignedBB {
            min: BlockPos::new(169, 41, 317),
            max: BlockPos::new(172, 50, 320),
        },
        speed: 0.2,
    },
    Loft {
        // 192 53 271, 195 62 274
        area: AxisAlignedBB {
            min: BlockPos::new(192, 53, 271),
            max: BlockPos::new(195, 62, 274),
        },
        speed: 0.2,
    },
    Loft {
        // 168 40 262, 171 49 265
        area: AxisAlignedBB {
            min: BlockPos::new(168, 40, 262),
            max: BlockPos::new(171, 49, 265),
        },
        speed: 0.2,
    },
    Loft {
        // 163 65 224, 166 74 227
        area: AxisAlignedBB {
            min: BlockPos::new(163, 65, 224),
            max: BlockPos::new(166, 74, 227),
        },
        speed: 0.2,
    },
    Loft {
        // 182 29 222, 185 38 225
        area: AxisAlignedBB {
            min: BlockPos::new(182, 29, 222),
            max: BlockPos::new(185, 38, 225),
        },
        speed: 0.2,
    },
    Loft {
        // 191 39 184, 194 48 187
        area: AxisAlignedBB {
            min: BlockPos::new(191, 39, 184),
            max: BlockPos::new(194, 48, 187),
        },
        speed: 0.2,
    },
    Loft {
        // 182 20 174, 185 29 177
        area: AxisAlignedBB {
            min: BlockPos::new(182, 20, 174),
            max: BlockPos::new(185, 29, 177),
        },
        speed: 0.2,
    },
    Loft {
        // -10 88 364, -7 94 367
        area: AxisAlignedBB {
            min: BlockPos::new(-10, 88, 364),
            max: BlockPos::new(-7, 94, 367),
        },
        speed: 0.2,
    },
    Loft {
        // -15 93 386, -12 99 389
        area: AxisAlignedBB {
            min: BlockPos::new(-15, 93, 386),
            max: BlockPos::new(-12, 99, 389),
        },
        speed: 0.2,
    },
];

pub const CHECKPOINTS: [Checkpoint; 10] = [
    Checkpoint {
        // -40 110 249.5, 52 150 249.5
        plane: AxisAlignedPlane::Z {
            min: Vec3::new(-40.0, 110.0, 249.5),
            max: Vec3::new(52.0, 150.0, 249.5),
        },
        spawn_position: Vec3::new(4.5, 143., 233.5),
        spawn_rotation: Rotation::new(0., 10.),
    },
    Checkpoint {
        // -9 68 303.5, 26 89 303.5
        plane: AxisAlignedPlane::Z {
            min: Vec3::new(-9.0, 68.0, 303.5),
            max: Vec3::new(26.0, 89.0, 303.5),
        },
        spawn_position: Vec3::new(10., 79., 302.),
        spawn_rotation: Rotation::new(0., 10.),
    },
    Checkpoint {
        // 29.5 70 459, 29.5 89 474
        plane: AxisAlignedPlane::X {
            min: Vec3::new(29.5, 70.0, 459.0),
            max: Vec3::new(29.5, 89.0, 474.0),
        },
        spawn_position: Vec3::new(20.5, 78.5, 467.5),
        spawn_rotation: Rotation::new(-90., 10.),
    },
    Checkpoint {
        // 120.5 77 401.5, 120.5 98 419.5
        plane: AxisAlignedPlane::X {
            min: Vec3::new(120.5, 77.0, 401.5),
            max: Vec3::new(120.5, 98.0, 419.5),
        },
        spawn_position: Vec3::new(103., 88., 411.),
        spawn_rotation: Rotation::new(-90., 10.),
    },
    Checkpoint {
        // 229.5 76 382, 229.5 109 439
        plane: AxisAlignedPlane::X {
            min: Vec3::new(229.5, 76.0, 382.0),
            max: Vec3::new(229.5, 109.0, 439.0),
        },
        spawn_position: Vec3::new(195., 111., 411.),
        spawn_rotation: Rotation::new(-90., 10.),
    },
    Checkpoint {
        // 174 49 389.5, 187 70 389.5
        plane: AxisAlignedPlane::Z {
            min: Vec3::new(174.0, 49.0, 389.5),
            max: Vec3::new(187.0, 70.0, 389.5),
        },
        spawn_position: Vec3::new(180.5, 60.5, 403.5),
        spawn_rotation: Rotation::new(180., 35.),
    },
    Checkpoint {
        // 169 38 135.5, 191 59 135.5
        plane: AxisAlignedPlane::Z {
            min: Vec3::new(169.0, 38.0, 135.5),
            max: Vec3::new(191.0, 59.0, 135.5),
        },
        spawn_position: Vec3::new(180., 56., 149.5),
        spawn_rotation: Rotation::new(180., 10.),
    },
    Checkpoint {
        // 169 -26 148.5, 209 12 148.5
        plane: AxisAlignedPlane::Z {
            min: Vec3::new(169.0, -26.0, 148.5),
            max: Vec3::new(209.0, 12.0, 148.5),
        },
        spawn_position: Vec3::new(189., -2., 142.5),
        spawn_rotation: Rotation::new(0., 10.),
    },
    Checkpoint {
        // 170 -36 387.5, 208 7 387.5
        plane: AxisAlignedPlane::Z {
            min: Vec3::new(170.0, -36.0, 387.5),
            max: Vec3::new(208.0, 7.0, 387.5),
        },
        spawn_position: Vec3::new(195., -2., 391.5),
        spawn_rotation: Rotation::new(0., 10.),
    },
    Checkpoint {
        // 191 -56 506, 204 -46 506
        // finish line, so spawn positions are random copilot positions
        plane: AxisAlignedPlane::Z {
            min: Vec3::new(191.0, -56.0, 506.0),
            max: Vec3::new(204.0, -46.0, 506.0),
        },
        spawn_position: Vec3::new(197.5, -51., 499.5),
        spawn_rotation: Rotation::new(-90., 10.),
    },
];

pub const AUTHOR_TIMES: [f32; 10] = [
    9.167, 19.865, 31.115, 35.465, 43.360, 58.715, 74.015, 81.867, 90.316, 95.316,
];
