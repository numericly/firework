use async_trait::async_trait;
use cipher::typenum::Min;
use firework::{
    client::{Client, GameMode, InventorySlot, Player},
    commands::{ArgumentType, CommandNode, StringTypes},
    AxisAlignedBB, AxisAlignedPlane, BlockPos, ConnectionError, PlayerHandler, Rotation, Server,
    ServerHandler, Vec3,
};
use firework_authentication::Profile;
use firework_data::items::{Elytra, Item};
use firework_protocol::data_types::{ItemNbt, Particle, Particles, Slot};
use firework_protocol_core::VarInt;
use std::{sync::Arc, time::Instant};
use tokio::sync::RwLock;

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
        velocity: Vec3::new(0., 0.02, 0.11),
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

const CANYON_CHECKPOINTS: [AxisAlignedPlane; 1] =
    [AxisAlignedPlane::new(Vec3::new(-4.5, 66., 193.5), Vec3::new(26.5, 187., 193.5)).unwrap()];

#[derive(Debug, Clone)]
struct BoostStatus {
    percent: f32,
    direction: Vec3,
}

enum GameState {
    Waiting,
    Running,
}

pub struct GlideServerHandler {
    pub created_at: Instant,
    game_state: RwLock<GameState>,
    commands: CommandNode<Self, MiniGameProxy>,
}

pub struct GlidePlayerHandler {
    boost_status: RwLock<Option<BoostStatus>>,
    server: Arc<Server<GlideServerHandler, MiniGameProxy>>,
    proxy: Arc<MiniGameProxy>,
}

#[async_trait]
impl PlayerHandler<GlideServerHandler, MiniGameProxy> for GlidePlayerHandler {
    fn new(
        server: Arc<Server<GlideServerHandler, MiniGameProxy>>,
        proxy: Arc<MiniGameProxy>,
    ) -> Self {
        Self {
            boost_status: RwLock::new(None),
            server,
            proxy,
        }
    }

    async fn on_tick(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        let time = client.server.handler.created_at.elapsed().as_secs_f32();
        let particle_density = 2f32;
        // check if particle_density is a whole number, if not, panic
        if particle_density != particle_density.round() {
            panic!("particle_density must be a whole number")
        }
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

            let animation_phase = (time * 0.5 * boost_direction_multiplier).rem_euclid(6.);

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
                        0..=((boost.area.max.x - boost.area.min.x) * particle_density as i32)
                    {
                        let particle_x =
                            boost.area.min.x as f32 + particle_number as f32 / particle_density;
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
                        0..=((boost.area.max.y - boost.area.min.y) * particle_density as i32)
                    {
                        let particle_x = boost.area.min.x as f32;
                        let particle_y =
                            boost.area.min.y as f32 + particle_number as f32 / particle_density;
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
                        0..=((boost.area.max.z - boost.area.min.z) * particle_density as i32)
                    {
                        let particle_x =
                            boost.area.min.x as f32 + chevron_number as f32 * 6. + animation_phase;
                        let particle_y = boost.area.max.y as f32;
                        let particle_z =
                            boost.area.min.z as f32 + particle_number as f32 / particle_density;
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
                        0..=((boost.area.max.y - boost.area.min.y) * particle_density as i32)
                    {
                        let particle_x =
                            boost.area.min.x as f32 + chevron_number as f32 * 6. + animation_phase;
                        let particle_y =
                            boost.area.min.y as f32 + particle_number as f32 / particle_density;
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
            client.send_particles(particles)
        }
        let boost_status = self.boost_status.read().await.clone();
        if let Some(BoostStatus { percent, direction }) = boost_status {
            if percent >= 1. {
                self.boost_status.write().await.take();
            } else {
                let velocity = client.player.read().await.velocity.clone();
                client.set_velocity(velocity + direction.clone());
                self.boost_status.write().await.replace(BoostStatus {
                    percent: percent + 0.08,
                    direction,
                });
            }
        }

        Ok(())
    }
}

#[async_trait]
impl ServerHandler<MiniGameProxy> for GlideServerHandler {
    type PlayerHandler = GlidePlayerHandler;
    fn new() -> Self {
        Self {
            created_at: Instant::now(),
            game_state: RwLock::new(GameState::Waiting),
            commands: CommandNode::root()
                .sub_command(
                    CommandNode::literal("play").sub_command(CommandNode::argument(
                        "game",
                        ArgumentType::String {
                            string_type: StringTypes::SingleWord,
                            suggestions: None,
                        },
                    )),
                )
                .sub_command(CommandNode::literal("echo")),
        }
    }
    async fn on_tick(&self, _server: &Server<Self, MiniGameProxy>, _proxy: &MiniGameProxy) {}
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
