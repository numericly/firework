use firework::{Rotation, Vec3};

use super::{ChestPosition, SpawnPoint};

pub const SPAWN_POINTS: [SpawnPoint; 8] = [
    // 0.5 104 -5.5
    SpawnPoint {
        position: Vec3::new(0.5, 104.0, -5.5),
        rotation: Rotation::new(0., 0.),
    },
    // 4.5 104 -3.5
    SpawnPoint {
        position: Vec3::new(4.5, 104.0, -3.5),
        rotation: Rotation::new(45., 0.),
    },
    // 6.5 104 0.5
    SpawnPoint {
        position: Vec3::new(6.5, 104.0, 0.5),
        rotation: Rotation::new(90., 0.),
    },
    // 4.5 104 4.5
    SpawnPoint {
        position: Vec3::new(4.5, 104.0, 4.5),
        rotation: Rotation::new(135., 0.),
    },
    // 0.5 104 6.5
    SpawnPoint {
        position: Vec3::new(0.5, 104.0, 6.5),
        rotation: Rotation::new(180., 0.),
    },
    // -3.5 104 4.5
    SpawnPoint {
        position: Vec3::new(-3.5, 104.0, 4.5),
        rotation: Rotation::new(-135., 0.),
    },
    // -5.5 104 0.5
    SpawnPoint {
        position: Vec3::new(-5.5, 104.0, 0.5),
        rotation: Rotation::new(-90., 0.),
    },
    // -3.5 104 -3.5
    SpawnPoint {
        position: Vec3::new(-3.5, 104.0, -3.5),
        rotation: Rotation::new(-45., 0.),
    },
];

pub const CHESTS: [ChestPosition; 13] = [
    // -28 113 27
    // -2 101 37
    // 16 113 33
    // 25 115 25
    // 31 102 -23
    // 13 114 -25
    // -14 116 -18
    // -32 107 -6
    // 0 99 0
    // -1 98 1
    // 1 98 1
    // 1 98 -1
    // -1 98 -1
    ChestPosition {
        x: -28,
        y: 113,
        z: 27,
    },
    ChestPosition {
        x: -2,
        y: 101,
        z: 37,
    },
    ChestPosition {
        x: 16,
        y: 113,
        z: 33,
    },
    ChestPosition {
        x: 25,
        y: 115,
        z: 25,
    },
    ChestPosition {
        x: 31,
        y: 102,
        z: -23,
    },
    ChestPosition {
        x: 13,
        y: 114,
        z: -25,
    },
    ChestPosition {
        x: -14,
        y: 116,
        z: -18,
    },
    ChestPosition {
        x: -32,
        y: 107,
        z: -6,
    },
    ChestPosition { x: 0, y: 99, z: 0 },
    ChestPosition { x: -1, y: 98, z: 1 },
    ChestPosition { x: 1, y: 98, z: 1 },
    ChestPosition { x: 1, y: 98, z: -1 },
    ChestPosition {
        x: -1,
        y: 98,
        z: -1,
    },
];
