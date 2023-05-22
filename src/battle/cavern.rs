use firework::{Rotation, Vec3};

use super::{ChestPosition, SpawnPoint};

pub const SPAWN_POINTS: [SpawnPoint; 8] = [
    // -9.5 97 -9.5
    SpawnPoint {
        position: Vec3::new(-9.5, 97.0, -9.5),
        rotation: Rotation::new(-45., 0.),
    },
    // -12.5 97 0.5
    SpawnPoint {
        position: Vec3::new(-12.5, 97.0, 0.5),
        rotation: Rotation::new(-90., 0.),
    },
    // -9.5 97 10.5
    SpawnPoint {
        position: Vec3::new(-9.5, 97.0, 10.5),
        rotation: Rotation::new(-135., 0.),
    },
    // 0.5 97 13.5
    SpawnPoint {
        position: Vec3::new(0.5, 97.0, 13.5),
        rotation: Rotation::new(180., 0.),
    },
    // 10.5 97 10.5
    SpawnPoint {
        position: Vec3::new(10.5, 97.0, 10.5),
        rotation: Rotation::new(135., 0.),
    },
    // 13.5 97 0.5
    SpawnPoint {
        position: Vec3::new(13.5, 97.0, 0.5),
        rotation: Rotation::new(90., 0.),
    },
    // 10.5 97 -9.5
    SpawnPoint {
        position: Vec3::new(10.5, 97.0, -9.5),
        rotation: Rotation::new(45., 0.),
    },
    // 0.5 97 -12.5
    SpawnPoint {
        position: Vec3::new(0.5, 97.0, -12.5),
        rotation: Rotation::new(0., 0.),
    },
];

pub const CHESTS: [ChestPosition; 12] = [
    ChestPosition { x: 1, y: 99, z: 1 },
    ChestPosition { x: 1, y: 99, z: -1 },
    ChestPosition {
        x: -1,
        y: 99,
        z: -1,
    },
    ChestPosition { x: -1, y: 99, z: 1 },
    ChestPosition {
        x: -28,
        y: 110,
        z: 17,
    },
    ChestPosition {
        x: 0,
        y: 108,
        z: 36,
    },
    ChestPosition {
        x: 8,
        y: 117,
        z: 28,
    },
    ChestPosition {
        x: 29,
        y: 115,
        z: 1,
    },
    ChestPosition {
        x: 24,
        y: 109,
        z: -20,
    },
    ChestPosition {
        x: 0,
        y: 108,
        z: -30,
    },
    ChestPosition {
        x: -20,
        y: 111,
        z: -15,
    },
    ChestPosition {
        x: 12,
        y: 116,
        z: 0,
    },
];
