use firework::{Rotation, Vec3};

use super::{ChestPosition, SpawnPoint};

pub const SPAWN_POINTS: [SpawnPoint; 8] = [
    // 0.5 103 -10.5
    SpawnPoint {
        position: Vec3::new(0.5, 103.0, -10.5),
        rotation: Rotation::new(0., 0.),
    },
    // -6.5 103 -6.5
    SpawnPoint {
        position: Vec3::new(-6.5, 103.0, -6.5),
        rotation: Rotation::new(-45., 0.),
    },
    // -10.5 103 0.5
    SpawnPoint {
        position: Vec3::new(-10.5, 103.0, 0.5),
        rotation: Rotation::new(-90., 0.),
    },
    // -6.5 103 6.5
    SpawnPoint {
        position: Vec3::new(-6.5, 103.0, 6.5),
        rotation: Rotation::new(-135., 0.),
    },
    // 0.5 103 10.5
    SpawnPoint {
        position: Vec3::new(0.5, 103.0, 10.5),
        rotation: Rotation::new(180., 0.),
    },
    // 6.5 103 6.5
    SpawnPoint {
        position: Vec3::new(6.5, 103.0, 6.5),
        rotation: Rotation::new(135., 0.),
    },
    // 10.5 103 0.5
    SpawnPoint {
        position: Vec3::new(10.5, 103.0, 0.5),
        rotation: Rotation::new(90., 0.),
    },
    // 6.5 103 -6.5
    SpawnPoint {
        position: Vec3::new(6.5, 103.0, -6.5),
        rotation: Rotation::new(45., 0.),
    },
];
pub const CHESTS: [ChestPosition; 13] = [
    // 1 94 -38
    // 28 91 -28
    // -26 104 -21
    // -29 91 30
    // 0 88 32
    // 33 102 28
    // -14 88 -14
    // 0 88 0
    // 0 100 0
    // 0 99 -1
    // 0 99 1
    // 1 99 0
    // -1 99 0
    ChestPosition {
        x: 1,
        y: 94,
        z: -38,
    },
    ChestPosition {
        x: 28,
        y: 91,
        z: -28,
    },
    ChestPosition {
        x: -26,
        y: 104,
        z: -21,
    },
    ChestPosition {
        x: -29,
        y: 91,
        z: 30,
    },
    ChestPosition { x: 0, y: 88, z: 32 },
    ChestPosition {
        x: 33,
        y: 102,
        z: 28,
    },
    ChestPosition {
        x: -14,
        y: 88,
        z: -14,
    },
    ChestPosition { x: 0, y: 88, z: 0 },
    ChestPosition { x: 0, y: 100, z: 0 },
    ChestPosition { x: 0, y: 99, z: -1 },
    ChestPosition { x: 0, y: 99, z: 1 },
    ChestPosition { x: 1, y: 99, z: 0 },
    ChestPosition { x: -1, y: 99, z: 0 },
];
