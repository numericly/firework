use async_trait::async_trait;
use firework::{
    client::{Client, GameMode, InventorySlot, Player},
    commands::{Argument, ArgumentType, CommandNode, StringTypes},
    entities::{EntityMetadata, Pose},
    AxisAlignedBB, AxisAlignedPlane, BlockPos, ConnectionError, PlayerHandler, Rotation, Server,
    ServerHandler, Vec3,
};
use firework_authentication::Profile;
use firework_data::items::{Elytra, Item};
use firework_protocol::{
    client_bound::{CustomSound, IdMapHolder, SoundSource, VanillaSound},
    data_types::{ItemNbt, Particle, Particles, Slot},
};
use firework_protocol_core::VarInt;
use serde_json::json;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{Mutex, RwLock};

use crate::MiniGameProxy;

const SPAWN_AREA: AxisAlignedBB = AxisAlignedBB {
    max: BlockPos { x: 4, y: 169, z: 7 },
    min: BlockPos {
        x: -4,
        y: 166,
        z: -3,
    },
};
struct Boost {
    area: AxisAlignedBB,
    velocity: Vec3,
    particle_type: BoostParticleType,
}

struct Checkpoint {
    plane: AxisAlignedPlane,
    spawn_position: Vec3,
    spawn_rotation: Rotation,
}

#[derive(Debug, PartialEq, Eq)]
enum BoostParticleType {
    BoostEast,
    BoostWest,
    BoostNorth,
    BoostSouth,
    Smoke,
}

const CANYON_BOOSTS: [Boost; 8] = [
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
        velocity: Vec3::new(0., 0., 0.06),
        particle_type: BoostParticleType::BoostSouth,
    },
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
        velocity: Vec3::new(0., 0., 0.),
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
        velocity: Vec3::new(0., 0., 0.),
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
        velocity: Vec3::new(0., 0., 0.),
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
        velocity: Vec3::new(0., 0., 0.),
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
        velocity: Vec3::new(0., 0., 0.),
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
        velocity: Vec3::new(0., 0., 0.),
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
        velocity: Vec3::new(0., 0., 0.),
        particle_type: BoostParticleType::BoostWest,
    },
];

const CANYON_CHECKPOINTS: [Checkpoint; 10] = [
    Checkpoint {
        // start
        plane: AxisAlignedPlane::X {
            min: Vec3::new(0.5, 173., 0.5),
            max: Vec3::new(0.5, 173., 0.5),
        },
        spawn_position: Vec3::new(0.5, 173., 0.5),
        spawn_rotation: Rotation::new(0., 22.2),
    },
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
            max: Vec3::new(13.5, 187., 460.5),
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
        spawn_position: Vec3::new(-69.7, -14., -493.4),
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

#[derive(Debug, Clone)]
struct BoostStatus {
    percent: f32,
    direction: Vec3,
}

enum GameState {
    Waiting,
    Starting { ticks_until_start: u16 },
    Running,
}

pub struct GlideServerHandler {
    pub created_at: Instant,
    game_state: Mutex<GameState>,
    commands: CommandNode<Self, MiniGameProxy>,
}

pub struct GlidePlayerHandler {
    boost_status: RwLock<Option<BoostStatus>>,
    animation_frame: Mutex<u8>,
    server: Arc<Server<GlideServerHandler, MiniGameProxy>>,
    proxy: Arc<MiniGameProxy>,
    checkpoints: Mutex<[bool; 10]>,
    start_time: Mutex<Option<Instant>>,
    last_damage: Mutex<Instant>,
}

#[async_trait]
impl PlayerHandler<GlideServerHandler, MiniGameProxy> for GlidePlayerHandler {
    fn new(
        server: Arc<Server<GlideServerHandler, MiniGameProxy>>,
        proxy: Arc<MiniGameProxy>,
    ) -> Self {
        Self {
            boost_status: RwLock::new(None),
            animation_frame: Mutex::new(0),
            server,
            proxy,
            checkpoints: Mutex::new([
                true, false, false, false, false, false, false, false, false, false,
            ]),
            start_time: Mutex::new(None),
            last_damage: Mutex::new(Instant::now()),
        }
    }
    async fn on_post_load(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        // Show the welcome message
        client.show_chat_message(
            json!([
                {
                    "text": "\n"
                },
                {
                    "text": "Welcome to the Glide Minigame!",
                    "color": "aqua"
                },
                {
                    "text": "\n\n"
                },
                {
                    "text": "How to play",
                    "color": "dark_green"
                },
                {
                    "text": "\n"
                },
                {
                    "text": "- Fly through boost pads to increase your speed\n- Use checkpoints to save your position\n- Reach the finish line first to win the game",
                "color": "green"
                },
                {
                    "text": "\n "
                }
              ])
            .to_string(),
        );
        self.start_time.lock().await.replace(Instant::now());
        Ok(())
    }
    async fn on_move(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
        pos: Vec3,
    ) -> Result<Option<Vec3>, ConnectionError> {
        let start = &client.player.read().await.position;
        let end = &pos;
        // check for passing through checkpoints
        for (i, checkpoint) in CANYON_CHECKPOINTS.iter().enumerate() {
            if checkpoint.plane.intersects(start, end) {
                {
                    self.checkpoints.lock().await[i] = true;

                    let is_finish_line = i == CANYON_CHECKPOINTS.len() - 1;

                    if is_finish_line {
                        // send title to the client
                        client.send_title(
                            json!({
                                "text":
                                    format!(
                                        "Finished race in {}",
                                        format_duration(
                                            self.start_time.lock().await.unwrap().elapsed()
                                        )
                                    )
                            })
                            .to_string(),
                            "{\"text\":\"\"}".to_string(),
                            0,
                            100,
                            0,
                        );
                        // send an exp level up sound for the finish line
                        client.send_sound(
                            IdMapHolder::Direct(CustomSound {
                                resource_location: "minecraft:entity.player.levelup".to_string(),
                                range: Some(999999.),
                            }),
                            SoundSource::Master,
                            client.player.read().await.position.clone(),
                            1.,
                            1.,
                        );
                    } else {
                        // send title to the client
                        client.send_title(
                            "{\"text\":\"Checkpoint\"}".to_string(),
                            json!({
                                "text":
                                    format!(
                                        "Reached Checkpoint {} in {}",
                                        i,
                                        format_duration(
                                            self.start_time.lock().await.unwrap().elapsed()
                                        )
                                    )
                            })
                            .to_string(),
                            0,
                            20,
                            0,
                        );
                        // send an exp orb pickup sound for all checkpoints
                        client.send_sound(
                            IdMapHolder::Direct(CustomSound {
                                resource_location: "minecraft:entity.experience_orb.pickup"
                                    .to_string(),
                                range: Some(999999.),
                            }),
                            SoundSource::Master,
                            client.player.read().await.position.clone(),
                            1.,
                            1.,
                        );
                    }
                }
                println!("Checkpoints: {:?}", self.checkpoints.lock().await);
            }
        }

        fn format_duration(dur: Duration) -> String {
            let secs = dur.as_millis();
            let mins = secs / 1000 / 60;
            let millis = secs % 1000;
            let secs = secs / 1000 % 60;
            format!("{}:{:02}.{:03}", mins, secs, millis)
        }

        Ok(Some(pos))
    }
    async fn on_tick(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        const PARTICLE_DENSITY: i32 = 2;

        let time = client.server.handler.created_at.elapsed().as_secs_f32();
        let position = client.player.read().await.position.clone();
        for boost in CANYON_BOOSTS.iter() {
            if boost.area.within(BlockPos::from(position.clone())) {
                let mut boost_status = self.boost_status.write().await;
                if boost_status.is_none() {
                    boost_status.replace(BoostStatus {
                        percent: 0.,
                        direction: boost.velocity.clone(),
                    });
                }
            }
            // check if any of the dimensions are too far away from the min
            // if so, don't bother
            if (boost.area.min.x as f64 - position.x).abs() > 30.
                || (boost.area.min.y as f64 - position.y).abs() > 30.
                || (boost.area.min.z as f64 - position.z).abs() > 30.
            {
                continue;
            }
            let boost_direction_multiplier;
            let mut particles = Vec::new();
            let middle_x = (boost.area.max.x + boost.area.min.x) as f32 / 2.;
            let middle_y = (boost.area.max.y + boost.area.min.y) as f32 / 2.;
            let middle_z = (boost.area.max.z + boost.area.min.z) as f32 / 2.;
            if [
                BoostParticleType::BoostSouth,
                BoostParticleType::BoostEast,
                BoostParticleType::Smoke,
            ]
            .contains(&boost.particle_type)
            {
                boost_direction_multiplier = 1.;
            } else {
                boost_direction_multiplier = -1.;
            }

            fn particle(x: f64, y: f64, z: f64) -> Particle {
                return Particle::new(
                    Particles::Dust {
                        red: 1.,
                        green: 0.7,
                        blue: 0.,
                        scale: 3.,
                    },
                    false,
                    x,
                    y,
                    z,
                    0.,
                    0.,
                    0.,
                    0.,
                    1,
                );
            }

            let animation_phase = (time * 0.4 * boost_direction_multiplier).rem_euclid(6.);

            if boost.particle_type == BoostParticleType::Smoke {
            } else if [BoostParticleType::BoostSouth, BoostParticleType::BoostNorth]
                .contains(&boost.particle_type)
            {
                // each chevron is 6 blocks long
                // loop through the amount of chevrons
                for chevron_number in 0..=((boost.area.max.z - boost.area.min.z) / 6) {
                    // loop through the amount of particles in each chevron
                    // first, draw the top chevrons
                    for particle_number in
                        0..=((boost.area.max.x - boost.area.min.x) * PARTICLE_DENSITY)
                    {
                        let particle_x = boost.area.min.x as f32
                            + particle_number as f32 / PARTICLE_DENSITY as f32;
                        let particle_y = boost.area.max.y as f32;
                        let particle_z =
                            boost.area.min.z as f32 + chevron_number as f32 * 6. + animation_phase;
                        // offset the z based on the distance to the center of x (45deg angle)
                        let particle_z =
                            particle_z - (particle_x - middle_x).abs() * boost_direction_multiplier;
                        // bounds check
                        if particle_z > boost.area.max.z as f32
                            || particle_z < boost.area.min.z as f32
                        {
                            continue;
                        }
                        particles.push(particle(
                            particle_x as f64,
                            particle_y as f64,
                            particle_z as f64,
                        ));
                    }
                    //then, draw the left and right chevrons
                    for particle_number in
                        0..=((boost.area.max.y - boost.area.min.y) * PARTICLE_DENSITY)
                    {
                        let particle_x = boost.area.min.x as f32;
                        let particle_y = boost.area.min.y as f32
                            + particle_number as f32 / PARTICLE_DENSITY as f32;
                        let particle_z =
                            boost.area.min.z as f32 + chevron_number as f32 * 6. + animation_phase;
                        // offset the z based on the distance to the center of x (45deg angle)
                        let particle_z =
                            particle_z - (particle_y - middle_y).abs() * boost_direction_multiplier;
                        // bounds check
                        if particle_z > boost.area.max.z as f32
                            || particle_z < boost.area.min.z as f32
                        {
                            continue;
                        }
                        particles.push(particle(
                            particle_x as f64,
                            particle_y as f64,
                            particle_z as f64,
                        ));
                        let particle_x = boost.area.max.x as f32;
                        particles.push(particle(
                            particle_x as f64,
                            particle_y as f64,
                            particle_z as f64,
                        ));
                    }
                }
            } else {
                for chevron_number in 0..=((boost.area.max.x - boost.area.min.x) / 6) {
                    // loop through the amount of particles in each chevron
                    // first, draw the top chevrons
                    for particle_number in
                        0..=((boost.area.max.z - boost.area.min.z) * PARTICLE_DENSITY)
                    {
                        let particle_x =
                            boost.area.min.x as f32 + chevron_number as f32 * 6. + animation_phase;
                        let particle_y = boost.area.max.y as f32;
                        let particle_z = boost.area.min.z as f32
                            + particle_number as f32 / PARTICLE_DENSITY as f32;
                        // offset the z based on the distance to the center of x (45deg angle)
                        let particle_x =
                            particle_x - (particle_z - middle_z).abs() * boost_direction_multiplier;
                        // bounds check
                        if particle_x > boost.area.max.x as f32
                            || particle_x < boost.area.min.x as f32
                        {
                            continue;
                        }
                        particles.push(particle(
                            particle_x as f64,
                            particle_y as f64,
                            particle_z as f64,
                        ));
                    }
                    //then, draw the left and right chevrons
                    for particle_number in
                        0..=((boost.area.max.y - boost.area.min.y) * PARTICLE_DENSITY)
                    {
                        let particle_x =
                            boost.area.min.x as f32 + chevron_number as f32 * 6. + animation_phase;
                        let particle_y = boost.area.min.y as f32
                            + particle_number as f32 / PARTICLE_DENSITY as f32;
                        let particle_z = boost.area.min.z as f32;
                        // offset the z based on the distance to the center of x (45deg angle)
                        let particle_x =
                            particle_x - (particle_y - middle_y).abs() * boost_direction_multiplier;
                        // bounds check
                        if particle_x > boost.area.max.x as f32
                            || particle_x < boost.area.min.x as f32
                        {
                            continue;
                        }
                        particles.push(particle(
                            particle_x as f64,
                            particle_y as f64,
                            particle_z as f64,
                        ));
                        let particle_z = boost.area.max.z as f32;
                        particles.push(particle(
                            particle_x as f64,
                            particle_y as f64,
                            particle_z as f64,
                        ));
                    }
                }
            }
            // send the particles to the client
            let mut animation_frame = self.animation_frame.lock().await;
            if *animation_frame == 5 {
                client.send_particles(particles);
                *animation_frame = 0;
            } else {
                *animation_frame += 1;
            }
        }

        let boost_status = self.boost_status.read().await.clone();
        if let Some(BoostStatus { percent, direction }) = boost_status {
            if percent >= 1. {
                self.boost_status.write().await.take();
            } else {
                let velocity = client.player.read().await.velocity.clone();
                client.set_velocity(velocity + direction.clone());
                self.boost_status.write().await.replace(BoostStatus {
                    percent: percent + 0.10,
                    direction,
                });
            }
        }

        // find which two checkpoints the player is between
        let mut checkpoints = self.checkpoints.lock().await;
        let mut checkpoint_index = 0;
        for i in 0..checkpoints.len() {
            if checkpoints[i] {
                checkpoint_index = i;
            }
        }
        let first_checkpoint = &CANYON_CHECKPOINTS[checkpoint_index];
        let second_checkpoint = &CANYON_CHECKPOINTS[(checkpoint_index + 1) % checkpoints.len()];

        // find the center of each checkpoint
        let first_checkpoint_center = first_checkpoint.plane.center();
        let second_checkpoint_center = second_checkpoint.plane.center();

        // get the current position of the player
        let player_position = client.player.read().await.position.clone();

        // find the distance between the player and the center of each checkpoint
        let first_checkpoint_distance = first_checkpoint_center.distance(&player_position);
        let second_checkpoint_distance = second_checkpoint_center.distance(&player_position);

        // get the percentage of the way between the two checkpoints
        let checkpoint_percent =
            first_checkpoint_distance / (first_checkpoint_distance + second_checkpoint_distance);

        // println!(
        //     "{}% of the way to checkpoint {}",
        //     (checkpoint_percent * 100.).round(),
        //     checkpoint_index + 1
        // );

        let checkpoint_count = CANYON_CHECKPOINTS.len();
        // decrease dashes when there are more checkpoints to achieve a constant width
        let dashes_per_checkpoint = 45 / (checkpoint_count - 1) - 1;
        let mut checkpoint_representation = String::new();
        for i in 0..(checkpoint_count - 1) {
            if i == checkpoint_index {
                checkpoint_representation.push_str(&format!(
                    "||{}{}{}",
                    "-".repeat((checkpoint_percent * dashes_per_checkpoint as f64) as usize),
                    "o",
                    "-".repeat(((1. - checkpoint_percent) * dashes_per_checkpoint as f64) as usize)
                ));
            } else {
                checkpoint_representation
                    .push_str(&format!("||{}", "-".repeat(dashes_per_checkpoint),));
            }
        }
        checkpoint_representation.push_str(&format!("||"));

        let chat_message = format!(
            "{{\"text\": \"{}\",\"bold\": true}}",
            checkpoint_representation
        );

        client.send_system_chat_message(chat_message.to_string(), true);
        Ok(())
    }

    async fn on_on_ground(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
        on_ground: bool,
    ) -> Result<bool, ConnectionError> {
        if on_ground {
            let mut last_damage = client.handler.last_damage.lock().await;
            if last_damage.elapsed().as_millis() < 1000 {
                println!("last damage too recent");
            } else {
                println!("damaging player");
                *last_damage = Instant::now();
                let health = client.player.read().await.health;
                if health <= 1. {
                    client.set_health(6.);
                    let mut greatest_checkpoint = &Checkpoint {
                        plane: AxisAlignedPlane::X {
                            min: Vec3::scalar(0.),
                            max: Vec3::scalar(0.),
                        },
                        spawn_position: Vec3::new(0.5, 173., 0.5),
                        spawn_rotation: Rotation::new(0., 22.2),
                    };
                    for (i, has_checkpoint) in
                        client.handler.checkpoints.lock().await.iter().enumerate()
                    {
                        if *has_checkpoint {
                            greatest_checkpoint = &CANYON_CHECKPOINTS[i];
                        }
                    }
                    client.sync_position(
                        greatest_checkpoint.spawn_position.clone(),
                        greatest_checkpoint.spawn_rotation.clone(),
                    );
                    client.set_velocity(Vec3::scalar(0.));
                } else {
                    client.set_health(health - 1.);
                }
            }
        }
        Ok(on_ground)
    }
}

#[async_trait]
impl ServerHandler<MiniGameProxy> for GlideServerHandler {
    type PlayerHandler = GlidePlayerHandler;
    fn new() -> Self {
        Self {
            created_at: Instant::now(),
            game_state: Mutex::new(GameState::Waiting),
            commands: CommandNode::root().sub_command(CommandNode::literal("start").executable(
                Box::new(move |args, client, server, proxy| {
                    Box::pin(start(args, client, server, proxy))
                }),
            )),
        }
    }
    async fn on_tick(&self, server: &Server<Self, MiniGameProxy>, proxy: &MiniGameProxy) {
        let mut game_state = self.game_state.lock().await;
        match &*game_state {
            GameState::Waiting => {}
            GameState::Starting { ticks_until_start } => {
                if *ticks_until_start != 0 {
                    if *ticks_until_start % 20 == 0 {
                        server.broadcast_chat(
                            json!([
                                {
                                  "text": "The game will start in ",
                                  "color": "yellow"
                                },
                                {
                                  "text": format!("{}", *ticks_until_start / 20),
                                  "color": "green"
                                },
                                {
                                  "text": " seconds...",
                                  "color": "yellow"
                                }
                            ])
                            .to_string(),
                        );
                    }
                    *game_state = GameState::Starting {
                        ticks_until_start: ticks_until_start - 1,
                    };
                    return;
                }
                *game_state = GameState::Running;

                drop(game_state);

                self.start_game(server, proxy).await;
            }
            GameState::Running => {}
        }
    }
    async fn load_player(&self, profile: Profile, uuid: u128) -> Result<Player, ConnectionError> {
        let mut player = Player {
            position: Vec3::new(0.5, 168.0, 0.5),
            max_health: 6.0,
            health: 6.0,
            flying_allowed: true,
            gamemode: GameMode::Adventure,
            profile,
            uuid,
            ..Player::default()
        };

        player.inventory.set_slot(
            InventorySlot::Chestplate,
            Some(Slot {
                item_id: VarInt(Elytra::ID as i32),
                item_count: 1,
                nbt: ItemNbt {
                    ..Default::default()
                },
            }),
        );

        Ok(player)
    }
    async fn get_commands(
        &self,
        server: &Server<GlideServerHandler, MiniGameProxy>,
        proxy: &MiniGameProxy,
    ) -> Result<&CommandNode<Self, MiniGameProxy>, ConnectionError> {
        Ok(&self.commands)
    }
}

impl GlideServerHandler {
    async fn start_game(&self, server: &Server<Self, MiniGameProxy>, _proxy: &MiniGameProxy) {
        for client in server.player_list.iter() {
            {
                let mut player = client.player.write().await;
                player.elytra_flying = false;
                server.broadcast_entity_metadata_update(
                    &client,
                    vec![
                        EntityMetadata::EntityFlags(player.entity_flags()),
                        EntityMetadata::EntityPose(Pose::Standing),
                    ],
                    true,
                );
            }
            client.show_chat_message(
                json!(
                    {
                        "text": "The game has started!",
                        "color": "green",
                        "bold": true
                    }
                )
                .to_string(),
            );
            client.sync_position(Vec3::new(0.5, 168.0, 0.5), Rotation { yaw: 0., pitch: 0. })
        }
    }
}

async fn start(
    _args: Vec<Argument>,
    client: &Client<GlideServerHandler, MiniGameProxy>,
    _server: &Server<GlideServerHandler, MiniGameProxy>,
    _proxy: &MiniGameProxy,
) {
    *client.server.handler.game_state.lock().await = GameState::Starting {
        ticks_until_start: 100,
    };
}
