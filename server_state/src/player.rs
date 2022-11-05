pub struct Player {
    pub position: Vec3,
    pub rotation: Rotation,
    pub selected_slot: u8,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub reduced_debug_info: bool,
    pub is_invulnerable: bool,
    pub is_flying: bool,
    pub can_fly: bool,
    pub is_instantly_breaking: bool,
    pub flying_speed: f32,
    pub fov_modifier: f32,
    pub brand: Option<String>,
    pub chunk_x: i32,
    pub chunk_z: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Rotation::new(0.0, 0.0),
            selected_slot: 0,
            game_mode: 0,
            previous_game_mode: 1,
            reduced_debug_info: false,
            is_invulnerable: false,
            is_flying: false,
            can_fly: false,
            is_instantly_breaking: false,
            flying_speed: 0.05,
            fov_modifier: 0.1,
            brand: None,
            chunk_x: 0,
            chunk_z: 0,
        }
    }
}

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
}

pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Rotation {
    pub fn new(yaw: f32, pitch: f32) -> Rotation {
        Rotation { yaw, pitch }
    }
}
