use async_trait::async_trait;
use firework::{authentication::Profile, gui::GUIEvent, protocol::server_bound::ClickContainer};
use firework::{
    client::{Client, GameMode, InventorySlot, Player},
    commands::{Argument, Command, CommandTree},
    entities::{EntityMetadata, Pose},
    AxisAlignedBB, AxisAlignedPlane, ConnectionError, PlayerHandler, Rotation, Server,
    ServerHandler, Vec3, TICKS_PER_SECOND,
};
use firework::{
    data::items::{Elytra, Item},
    protocol::data_types::Slot,
};
use firework::{
    gui::GUIInit,
    protocol::{
        client_bound::{CustomSound, IdMapHolder, SoundSource},
        data_types::{
            BossBarAction, BossBarColor, BossBarDivision, ItemNbt, Particle, Particles, SlotInner,
        },
    },
};
use firework::{gui::GuiScreen, world::World};
use firework::{gui::WindowType, protocol::core::VarInt};
use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde_json::json;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{broadcast, Mutex, RwLock};

use crate::{
    MiniGameProxy, TransferData, CANYON_GLIDE_WORLD, CAVERN_GLIDE_WORLD, TEMPLE_GLIDE_WORLD,
};

mod canyon;
mod cavern;
mod temple;

pub struct Boost {
    area: AxisAlignedBB,
    speed: f64,
    particle_type: BoostParticleType,
}

pub struct Loft {
    area: AxisAlignedBB,
    speed: f64,
}

pub struct Checkpoint {
    plane: AxisAlignedPlane,
    spawn_position: Vec3,
    spawn_rotation: Rotation,
}

#[derive(Debug, Clone)]
pub enum BoostStatus {
    ArrowBoost {
        velocity: Vec3,
        times_remaining: usize,
        speed: f64,
    },
    Loft {
        velocity: Vec3,
        times_remaining: usize,
        speed: f64,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoostParticleType {
    BoostEast,
    BoostWest,
    BoostNorth,
    BoostSouth,
    Smoke,
}

pub enum Maps {
    Canyon,
    Cavern,
    Temple,
}

impl Distribution<Maps> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Maps {
        match rng.gen_range(0..3) {
            _ => Maps::Cavern,
            0 => Maps::Canyon,
            1 => Maps::Cavern,
            2 => Maps::Temple,
            _ => panic!("Invalid random number huh? (https://www.youtube.com/watch?v=4kEO7VjKRB8)"),
        }
    }
}

impl Maps {
    pub fn get_world(&self) -> &'static World {
        match self {
            Maps::Canyon => &CANYON_GLIDE_WORLD,
            Maps::Cavern => &CAVERN_GLIDE_WORLD,
            Maps::Temple => &TEMPLE_GLIDE_WORLD,
        }
    }
    pub fn get_checkpoints(&self) -> &'static [Checkpoint] {
        match self {
            Maps::Canyon => &canyon::CHECKPOINTS,
            Maps::Cavern => &cavern::CHECKPOINTS,
            Maps::Temple => &temple::CHECKPOINTS,
        }
    }
    pub fn get_boosts(&self) -> &'static [Boost] {
        match self {
            Maps::Canyon => &canyon::BOOSTS,
            Maps::Cavern => &cavern::BOOSTS,
            Maps::Temple => &temple::BOOSTS,
        }
    }
    pub fn get_lofts(&self) -> &'static [Loft] {
        match self {
            Maps::Canyon => &canyon::LOFTS,
            Maps::Cavern => &cavern::LOFTS,
            Maps::Temple => &temple::LOFTS,
        }
    }
    pub fn get_spawn_area(&self) -> &AxisAlignedBB {
        match self {
            Maps::Canyon => &canyon::SPAWN_AREA,
            Maps::Cavern => &cavern::SPAWN_AREA,
            Maps::Temple => &temple::SPAWN_AREA,
        }
    }
    pub fn get_spawn_position(&self) -> &Vec3 {
        match self {
            Maps::Canyon => &canyon::SPAWN_POSITION,
            Maps::Cavern => &cavern::SPAWN_POSITION,
            Maps::Temple => &temple::SPAWN_POSITION,
        }
    }
}

#[derive(Debug, Clone)]
enum GameState {
    Starting {
        ticks_until_start: u16,
    },
    Running {
        start_time: Instant,
        finished_players: usize,
    },
    Finished {
        finish_time: Instant,
    },
}

const BOOST_TICKS: usize = 12;

pub struct GlideServerHandler {
    pub map: Maps,
    pub created_at: Instant,
    game_state: Mutex<GameState>,
    commands: CommandTree<Self, MiniGameProxy>,
}

pub struct GlidePlayerHandler {
    last_boost: Mutex<Option<(Instant, usize)>>,
    last_loft: Mutex<Option<(Instant, usize)>>,
    boost_status: RwLock<Option<BoostStatus>>,
    animation_frame: Mutex<u8>,
    server: Arc<Server<GlideServerHandler, MiniGameProxy>>,
    _proxy: Arc<MiniGameProxy>,
    last_checkpoint: Mutex<Option<usize>>,
    start_time: Mutex<Option<Instant>>,
    last_damage: Mutex<Instant>,
    recent_packets: Mutex<Vec<Instant>>,
}

fn format_duration(dur: Duration) -> String {
    let secs = dur.as_millis();
    let mins = secs / 1000 / 60;
    let millis = secs % 1000;
    let secs = secs / 1000 % 60;
    format!("{}:{:02}.{:03}", mins, secs, millis)
}

#[async_trait]
impl PlayerHandler<GlideServerHandler, MiniGameProxy> for GlidePlayerHandler {
    fn new(
        server: Arc<Server<GlideServerHandler, MiniGameProxy>>,
        proxy: Arc<MiniGameProxy>,
    ) -> Self {
        Self {
            last_loft: Mutex::new(None),
            last_boost: Mutex::new(None),
            boost_status: RwLock::new(None),
            animation_frame: Mutex::new(0),
            server,
            _proxy: proxy,
            recent_packets: Mutex::new(Vec::new()),
            last_checkpoint: Mutex::new(Some(0)),
            start_time: Mutex::new(None),
            last_damage: Mutex::new(Instant::now()),
        }
    }
    async fn on_server_bound_packet(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        let mut recent_packets = self.recent_packets.lock().await;
        recent_packets.push(Instant::now());

        const PACKETS_PER_TICK: usize = 4;
        const SAMPLE_TIME: usize = 3;

        recent_packets.retain(|i| i.elapsed().as_secs() < SAMPLE_TIME as u64);

        if recent_packets.len() > SAMPLE_TIME * PACKETS_PER_TICK * TICKS_PER_SECOND {
            client.disconnect(
                json!({
                    "text": "Kicked for sending too many packets",
                    "color": "red"
                })
                .to_string(),
            );
        }

        Ok(())
    }
    async fn on_transfer(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        client.send_boss_bar_action(0, BossBarAction::Remove);
        Ok(())
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

        {
            let mut player = client.player.write().await;
            player.elytra_flying = true;
            client.server.broadcast_entity_metadata_update(
                &client,
                vec![
                    EntityMetadata::EntityFlags(player.entity_flags()),
                    EntityMetadata::EntityPose(Pose::FallFlying),
                ],
                true,
            );
        }
        Ok(())
    }
    async fn on_move(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
        pos: Vec3,
    ) -> Result<Option<Vec3>, ConnectionError> {
        let map = &self.server.handler.map;
        let start = &client.player.read().await.position.clone();
        let end = &pos;

        // check if the movement is way too fast
        let max_velocity = 10.0;
        if (end.clone() - start.clone()).length() > max_velocity {
            client.sync_position(start.clone(), None);
            client.set_velocity(
                client
                    .player
                    .read()
                    .await
                    .previous_velocity
                    .clamp(0., max_velocity * 0.6),
            );
            return Ok(None);
        }

        let game_state_lock = self.server.handler.game_state.lock().await;
        // check for passing through checkpoints
        if let GameState::Running { start_time, .. } = *game_state_lock {
            drop(game_state_lock);
            let next = self.last_checkpoint.lock().await;
            if let Some(next_checkpoint_id) = *next {
                let next_checkpoint = map.get_checkpoints().get(next_checkpoint_id);

                if let Some(next_checkpoint) = next_checkpoint {
                    // The client reached the next checkpoint
                    if next_checkpoint.plane.intersects(start, end) {
                        drop(next);
                        drop(start);

                        if next_checkpoint_id + 1 >= map.get_checkpoints().len() {
                            self.last_checkpoint.lock().await.take();

                            self.on_win(client, &start_time).await;
                        } else {
                            self.last_checkpoint
                                .lock()
                                .await
                                .replace(next_checkpoint_id + 1);

                            self.on_checkpoint(client, &start_time, next_checkpoint_id)
                                .await;
                        }
                    }
                }
            }
        }

        Ok(Some(pos))
    }
    async fn on_tick(&self, client: &Client<GlideServerHandler, MiniGameProxy>) {
        let boost_status = self.boost_status.read().await.clone();
        if let Some(BoostStatus::ArrowBoost {
            speed,
            times_remaining,
            velocity,
        }) = boost_status
        {
            if times_remaining == 0 {
                self.boost_status.write().await.take();
            } else {
                let direction = client.player.read().await.rotation.direction().normalize();
                let velocity_direction = velocity.normalize();
                let velocity_speed = velocity.magnitude();

                let new_direction = velocity_direction.lerp(&direction, 0.25).normalize();
                let new_speed = velocity_speed * 0.92 + speed / BOOST_TICKS as f64;

                let new_vec = new_direction * Vec3::scalar(new_speed);

                client.set_velocity(new_vec.clone());

                self.boost_status
                    .write()
                    .await
                    .replace(BoostStatus::ArrowBoost {
                        times_remaining: times_remaining - 1,
                        speed,
                        velocity: new_vec,
                    });
            }
        } else if let Some(BoostStatus::Loft {
            velocity,
            times_remaining,
            speed,
        }) = boost_status
        {
            if times_remaining == 0 {
                self.boost_status.write().await.take();
            } else {
                const HORIZONTAL_SPEED_FACTOR: f64 = 1.015;

                let horizontal_boost = Vec3::new(velocity.x, 0., velocity.z);

                let direction = client.player.read().await.rotation.direction().normalize();
                let new_direction = horizontal_boost
                    .normalize()
                    .lerp(&direction, 0.25)
                    .normalize();
                let new_magnitude = horizontal_boost.magnitude() * HORIZONTAL_SPEED_FACTOR;

                let new_vec = new_direction * Vec3::scalar(new_magnitude);

                let new_vec = Vec3::new(new_vec.x, velocity.y * 0.90 + speed, new_vec.z);

                client.set_velocity(new_vec.clone());
                self.boost_status.write().await.replace(BoostStatus::Loft {
                    times_remaining: times_remaining - 1,
                    speed: speed * 0.92,
                    velocity: new_vec,
                });
            }
        }

        let map = &self.server.handler.map;

        {
            const PARTICLE_DENSITY: i32 = 2;
            let time = client.server.handler.created_at.elapsed().as_secs_f32();
            let position = client.player.read().await.position.clone();

            for checkpoint in map.get_checkpoints().iter() {
                fn checkpoint_particle_box(
                    x1: f64,
                    y1: f64,
                    z1: f64,
                    x2: f64,
                    y2: f64,
                    z2: f64,
                ) -> Vec<Particle> {
                    let mut particles = Vec::new();

                    // sadly the particle system is based around gaussian distributions, so here's a hack to
                    // make the particles look like a box

                    // add one because we want to include a minimum of 1 segment in each dimension
                    let count_x = (x2 - x1).abs() as i32 / 12 + 1;
                    let count_y = (y2 - y1).abs() as i32 / 12 + 1;
                    let count_z = (z2 - z1).abs() as i32 / 12 + 1;

                    let segment_width = (x2 - x1).abs() / count_x as f64;
                    let segment_height = (y2 - y1).abs() / count_y as f64;
                    let segment_depth = (z2 - z1).abs() / count_z as f64;

                    for x in 0..count_x {
                        for y in 0..count_y {
                            for z in 0..count_z {
                                if (x + y + z) % 2 == 0 {
                                    continue;
                                }
                                let x = x1 + (x as f64 + 0.5) * segment_width;
                                let y = y1 + (y as f64 + 0.5) * segment_height;
                                let z = z1 + (z as f64 + 0.5) * segment_depth;
                                particles.push(Particle::new(
                                    Particles::EndRod,
                                    true,
                                    x,
                                    y,
                                    z,
                                    (segment_width / 2.) as f32,
                                    (segment_height / 2.) as f32,
                                    (segment_depth / 2.) as f32,
                                    0.0,
                                    3,
                                ));
                            }
                        }
                    }
                    particles
                }

                let (min, max) = checkpoint.plane.to_cartesian_pair();

                let center = checkpoint.plane.center();

                if (center.x - position.x).abs() > 100.
                    || (center.y - position.y).abs() > 100.
                    || (center.z - position.z).abs() > 100.
                {
                    continue;
                }

                let particles = checkpoint_particle_box(min.x, min.y, min.z, max.x, max.y, max.z);
                client.send_particles(particles);
            }

            for (i, loft) in map.get_lofts().iter().enumerate() {
                if loft.area.within(position.clone()) {
                    let mut last_loft = self.last_loft.lock().await;

                    if let Some((time, last_i)) = last_loft.as_ref() {
                        if i == *last_i && time.elapsed().as_secs_f32() < 1.0 {
                            continue;
                        }
                    }

                    let velocity = client.player.read().await.velocity.clone();
                    let mut boost_status = self.boost_status.write().await;

                    last_loft.replace((Instant::now(), i));

                    boost_status.replace(BoostStatus::Loft {
                        times_remaining: 8,
                        speed: loft.speed,
                        velocity,
                    });
                }
                fn smoke_particle(x: f64, y: f64, z: f64, height: f32) -> Particle {
                    const LIFETIME: f32 = 100.0; // lifetime in ticks
                    let speed = height / LIFETIME;
                    Particle::new(
                        Particles::CampfireCozySmoke,
                        true,
                        x,
                        y,
                        z,
                        0.,
                        1.,
                        0.,
                        speed, // blocks per tick
                        0,
                    )
                }

                let mut particles: Vec<Particle> = Vec::new();

                for _ in 0..2 {
                    let x = loft.area.min.x as f64
                        + (loft.area.max.x as f64 - loft.area.min.x as f64) * rand::random::<f64>();
                    let y = loft.area.min.y as f64;
                    let height = loft.area.max.y as f32 - loft.area.min.y as f32;
                    let z = loft.area.min.z as f64
                        + (loft.area.max.z as f64 - loft.area.min.z as f64) * rand::random::<f64>();
                    particles.push(smoke_particle(x, y, z, height));
                }

                client.send_particles(particles);
            }

            for (i, boost) in map.get_boosts().iter().enumerate() {
                if boost.area.within(position.clone()) {
                    let mut boost_status = self.boost_status.write().await;

                    let mut last_boost = self.last_boost.lock().await;

                    if let Some((time, last_i)) = last_boost.as_ref() {
                        if i == *last_i && time.elapsed().as_secs_f32() < 1.0 {
                            continue;
                        }
                    }

                    let velocity = client.player.read().await.velocity.clone();

                    last_boost.replace((Instant::now(), i));

                    boost_status.replace(BoostStatus::ArrowBoost {
                        times_remaining: BOOST_TICKS,
                        speed: boost.speed,
                        velocity,
                    });
                }
                // check if any of the dimensions are too far away from the min
                // if so, don't bother
                if (boost.area.min.x as f64 - position.x).abs() > 50.
                    || (boost.area.min.y as f64 - position.y).abs() > 50.
                    || (boost.area.min.z as f64 - position.z).abs() > 50.
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
                    Particle::new(
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
                    )
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
                            let particle_z = boost.area.min.z as f32
                                + chevron_number as f32 * 6.
                                + animation_phase;
                            // offset the z based on the distance to the center of x (45deg angle)
                            let particle_z = particle_z
                                - (particle_x - middle_x).abs() * boost_direction_multiplier;
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
                            let particle_z = boost.area.min.z as f32
                                + chevron_number as f32 * 6.
                                + animation_phase;
                            // offset the z based on the distance to the center of x (45deg angle)
                            let particle_z = particle_z
                                - (particle_y - middle_y).abs() * boost_direction_multiplier;
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
                            let particle_x = boost.area.min.x as f32
                                + chevron_number as f32 * 6.
                                + animation_phase;
                            let particle_y = boost.area.max.y as f32;
                            let particle_z = boost.area.min.z as f32
                                + particle_number as f32 / PARTICLE_DENSITY as f32;
                            // offset the z based on the distance to the center of x (45deg angle)
                            let particle_x = particle_x
                                - (particle_z - middle_z).abs() * boost_direction_multiplier;
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
                            let particle_x = boost.area.min.x as f32
                                + chevron_number as f32 * 6.
                                + animation_phase;
                            let particle_y = boost.area.min.y as f32
                                + particle_number as f32 / PARTICLE_DENSITY as f32;
                            let particle_z = boost.area.min.z as f32;
                            // offset the z based on the distance to the center of x (45deg angle)
                            let particle_x = particle_x
                                - (particle_y - middle_y).abs() * boost_direction_multiplier;
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
        }

        let checkpoint_index = client.handler.last_checkpoint.lock().await.clone();

        if let Some(checkpoint_index) = checkpoint_index {
            let (earlier_checkpoint, later_checkpoint) = match checkpoint_index {
                0 => (
                    map.get_spawn_position().clone(),
                    map.get_checkpoints()[0].plane.center(),
                ),
                last_checkpoint => (
                    map.get_checkpoints()[last_checkpoint - 1].plane.center(),
                    map.get_checkpoints()[last_checkpoint].plane.center(),
                ),
            };

            let distance_to_earlier_checkpoint = (client.player.read().await.position.clone()
                - earlier_checkpoint.clone())
            .magnitude();
            let distance_to_later_checkpoint = (client.player.read().await.position.clone()
                - later_checkpoint.clone())
            .magnitude();

            let percent_to_later_checkpoint = (distance_to_earlier_checkpoint
                / (distance_to_earlier_checkpoint + distance_to_later_checkpoint))
                as f32;

            client.send_boss_bar_action(
            0,
            BossBarAction::UpdateTitle {
            title:json!([
                {
                    "text": "You are ",
                    "color": "white"
                },
                {
                    "text": format!("{:.2}%", 
                        (client.handler.last_checkpoint.lock().await.unwrap() as f32 + percent_to_later_checkpoint) / map.get_checkpoints().len() as f32 * 100.
                ).to_string(),
                    "color": "gold"
                },
                {
                    "text": " to the finish!",
                    "color": "white"
                },
            ])
            .to_string()}
        );
            client.send_boss_bar_action(
                0,
                BossBarAction::UpdateHealth {
                    health: (client.handler.last_checkpoint.lock().await.unwrap() as f32
                        + percent_to_later_checkpoint)
                        / map.get_checkpoints().len() as f32,
                },
            );
        };
    }
    async fn on_on_ground(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
        on_ground: bool,
    ) -> Result<bool, ConnectionError> {
        if let GameState::Running { .. } = *client.server.handler.game_state.lock().await {
            let map = &self.server.handler.map;
            let mut last_damage = client.handler.last_damage.lock().await;

            let next = client.handler.last_checkpoint.lock().await;
            if let Some(checkpoint) = *next {
                if on_ground
                    && last_damage.elapsed().as_millis() > 500
                    && !map
                        .get_spawn_area()
                        .within(client.player.read().await.position.clone())
                {
                    *last_damage = Instant::now();

                    let health = client.player.read().await.health;
                    if health - 2.0 <= 0. {
                        client.set_health(6.);

                        // If they haven't reached a checkpoint yet, spawn them at the start
                        if checkpoint != 0 {
                            let checkpoint = map.get_checkpoints().get(checkpoint - 1);
                            if let Some(checkpoint) = checkpoint {
                                client.sync_position(
                                    checkpoint.spawn_position.clone(),
                                    Some(checkpoint.spawn_rotation.clone()),
                                );

                                client.set_velocity(Vec3::scalar(0.));
                                return Ok(on_ground);
                            }
                        }

                        client.sync_position(
                            map.get_spawn_position().clone(),
                            Some(Rotation::new(0., 0.)),
                        );

                        client.send_sound(
                            IdMapHolder::Direct(CustomSound {
                                resource_location: "minecraft:entity.player.death".to_string(),
                                range: None,
                            }),
                            SoundSource::Player,
                            client.player.read().await.position.clone(),
                            1.,
                            1.,
                        );

                        client.set_velocity(Vec3::scalar(0.));
                    } else {
                        client.send_sound(
                            IdMapHolder::Direct(CustomSound {
                                resource_location: "minecraft:entity.player.hurt".to_string(),
                                range: None,
                            }),
                            SoundSource::Player,
                            client.player.read().await.position.clone(),
                            1.,
                            1.,
                        );
                        // client.send_hurt(client.client_data.entity_id, 0.);
                        // client.set_health(health - 2.);
                    }
                }
            }
        }

        Ok(on_ground)
    }
}

impl GlidePlayerHandler {
    async fn on_checkpoint(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
        start_time: &Instant,
        check_point_id: usize,
    ) {
        client.send_title(
            "{\"text\":\"Checkpoint\"}".to_string(),
            json!({
                "text":
                    format!(
                        "Reached Checkpoint {} in {}",
                        check_point_id + 1,
                        format_duration(start_time.elapsed())
                    )
            })
            .to_string(),
            0,
            40,
            3,
        );

        client.send_sound(
            IdMapHolder::Direct(CustomSound {
                resource_location: "minecraft:entity.experience_orb.pickup".to_string(),
                range: None,
            }),
            SoundSource::Master,
            client.player.read().await.position.clone(),
            0.5,
            1.,
        );
    }
    async fn on_win(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
        start_time: &Instant,
    ) {
        let mut game_state = self.server.handler.game_state.lock().await;
        if let GameState::Running {
            mut finished_players,
            ..
        } = *game_state
        {
            finished_players += 1;
            if finished_players == self.server.player_list.len() {
                *game_state = GameState::Finished {
                    finish_time: Instant::now(),
                };
                for client in self.server.player_list.iter() {
                    client.send_boss_bar_action(0, BossBarAction::Remove);
                    client.send_system_chat_message(
                        json!([
                            {
                                "text": "Game Finished!",
                                "color": "green",
                            },
                            {
                                "text": "\n",
                                "color": "white",
                            },
                            {
                                "text": "Leaderboard: (TODO actual functionality)",
                                "color": "gold",
                            },
                            {
                                "text": "\n",
                                "color": "white",
                            },
                            {
                                "text": "1. ",
                                "color": "gold",
                            },
                            {
                                "text": format!(
                                    "{} - {}",
                                    client.player.read().await.profile.name,
                                    format_duration(start_time.elapsed())
                                ),
                                "color": "yellow",
                            }
                        ])
                        .to_string(),
                        false,
                    );
                }
            }
        }
        drop(game_state);
        client.show_chat_message(
            json!(
                {
                    "text": format!(
                        "Completed race in {}",
                        format_duration(start_time.elapsed())
                    ),
                    "color": "green",
                }
            )
            .to_string(),
        );
        client.send_title(
            json!({
                "text": "Finished!"
            })
            .to_string(),
            json!({
                "text":
                    format!(
                        "Completed race in {}",
                        format_duration(start_time.elapsed())
                    )
            })
            .to_string(),
            1,
            120,
            40,
        );
        // send an exp level up sound for the finish line
        client.send_sound(
            IdMapHolder::Direct(CustomSound {
                resource_location: "minecraft:entity.player.levelup".to_string(),
                range: None,
            }),
            SoundSource::Master,
            client.player.read().await.position.clone(),
            0.5,
            1.,
        );

        {
            let mut player = client.player.write().await;
            player.elytra_flying = false;
            client.server.broadcast_entity_metadata_update(
                client,
                vec![
                    EntityMetadata::EntityFlags(player.entity_flags()),
                    EntityMetadata::EntityPose(Pose::Standing),
                ],
                true,
            );
        }

        client.set_flying(true);

        client.set_max_health(20.).await;

        client.set_health(20.);

        client.send_boss_bar_action(
            0,
            BossBarAction::UpdateTitle {
                title: json!({
                    "text":
                        format!(
                            "Completed race in {}",
                            format_duration(start_time.elapsed())
                        )
                })
                .to_string(),
            },
        );
    }
}

pub struct Menu {
    pub items: Vec<Slot>,
    pub channel: broadcast::Sender<GUIEvent>,
}

#[async_trait]
impl GuiScreen<GlideServerHandler, MiniGameProxy> for Menu {
    async fn init(&self, _client: &Client<GlideServerHandler, MiniGameProxy>) -> GUIInit {
        GUIInit {
            title: r#"{"text":"      Minigame Selector","bold":true}"#.to_string(),
            window_type: WindowType::Generic9x1,
            items: self.items.clone(),
            receiver: self.channel.subscribe(),
        }
    }
    async fn handle_click(
        &self,
        slot: ClickContainer,
        client: &Client<GlideServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        Ok(())
    }
}

impl Menu {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self {
            channel: sender,
            items: vec![None, None, None, None, None, None, None, None, None],
        }
    }
}

#[async_trait]
impl ServerHandler<MiniGameProxy> for GlideServerHandler {
    type ServerGUI = Menu;
    type PlayerHandler = GlidePlayerHandler;
    fn new() -> Self {
        Self {
            map: rand::random(),
            created_at: Instant::now(),
            game_state: Mutex::new(GameState::Starting {
                ticks_until_start: 120,
            }),
            commands: CommandTree::new()
                .register_command(
                    Command::new("lobby", "return to the main lobby")
                        .set_aliases(vec!["hub", "l"])
                        .with_execution(Box::new(move |args, client, server, proxy| {
                            Box::pin(return_to_lobby(args, client, server, proxy))
                        })),
                )
                .build_help_command(),
        }
    }
    fn get_world(&self) -> &'static World {
        self.map.get_world()
    }
    async fn on_tick(&self, server: &Server<Self, MiniGameProxy>, proxy: Arc<MiniGameProxy>) {
        if self.created_at.elapsed().as_millis() / 1000 * TICKS_PER_SECOND as u128 >= 200
            && server.player_list.len() == 0
        {
            proxy.glide_queue.lock().await.remove_server(server.id);
        }

        let mut game_state = self.game_state.lock().await;

        match *game_state {
            // OK
            GameState::Starting { ticks_until_start } => {
                if ticks_until_start != 0 {
                    if ticks_until_start % TICKS_PER_SECOND as u16 == 0 && ticks_until_start <= 100
                    {
                        server.broadcast_chat(
                            json!([
                                {
                                  "text": "The game will start in ",
                                  "color": "yellow"
                                },
                                {
                                  "text": format!("{}", ticks_until_start / 20),
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
                *game_state = GameState::Running {
                    start_time: Instant::now(),
                    finished_players: 0,
                };

                drop(game_state);

                self.start_game(server, &proxy).await;
            }
            GameState::Running {
                start_time,
                finished_players,
            } => {
                let time_elapsed = format_duration(start_time.elapsed());

                for player in server.player_list.iter() {
                    player.send_system_chat_message(
                        json!({
                            "text": format!("Time elapsed: {}", time_elapsed),
                        })
                        .to_string(),
                        true,
                    )
                }
            }
            GameState::Finished { finish_time } => {
                if finish_time.elapsed().as_millis() > 10000 {
                    for client in server.player_list.iter() {
                        client.transfer(TransferData::Lobby);
                    }
                }
            }
        }
    }
    async fn load_player(&self, profile: Profile, uuid: u128) -> Result<Player, ConnectionError> {
        let mut player = Player {
            position: self.map.get_spawn_position().clone(),
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
            Some(SlotInner {
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
        _server: &Server<GlideServerHandler, MiniGameProxy>,
        _proxy: &MiniGameProxy,
    ) -> Result<&CommandTree<Self, MiniGameProxy>, ConnectionError> {
        Ok(&self.commands)
    }
}

impl GlideServerHandler {
    async fn start_game(&self, server: &Server<Self, MiniGameProxy>, _proxy: &MiniGameProxy) {
        for client in server.player_list.iter() {
            {
                let mut player = client.player.write().await;
                player.elytra_flying = true;
                server.broadcast_entity_metadata_update(
                    &client,
                    vec![
                        EntityMetadata::EntityFlags(player.entity_flags()),
                        EntityMetadata::EntityPose(Pose::FallFlying),
                    ],
                    true,
                );
            }
            client.set_health(6.);
            client.show_chat_message(
                json!(
                    {
                        "text": "The game has started!",
                        "color": "green",
                    }
                )
                .to_string(),
            );
            client.sync_position(
                self.map.get_spawn_position().clone(),
                Some(Rotation::new(0., 0.)),
            );

            client.send_boss_bar_action(
                0,
                BossBarAction::Add {
                    title: json!({
                        "text": "",
                    })
                    .to_string(),
                    health: 0.,
                    color: BossBarColor::Green,
                    division: BossBarDivision::NoDivisions,
                    flags: 0,
                },
            )
        }
    }
}

async fn return_to_lobby(
    _args: Vec<Argument>,
    client: &Client<GlideServerHandler, MiniGameProxy>,
    _server: &Server<GlideServerHandler, MiniGameProxy>,
    _proxy: &MiniGameProxy,
) {
    client.transfer(TransferData::Lobby);
}
