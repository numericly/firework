use async_trait::async_trait;
use firework::gui::WindowType;
use firework::protocol::data_types::{ObjectiveAction, ObjectiveType};
use firework::{
    authentication::Profile,
    gui::GUIEvent,
    protocol::{data_types::Enchantment, server_bound::ClickContainer},
};
use firework::{
    client::{Client, GameMode, InventorySlot, Player},
    commands::{Argument, Command, CommandTree},
    entities::{EntityMetadata, Pose},
    AxisAlignedBB, AxisAlignedPlane, ConnectionError, PlayerHandler, Rotation, Server,
    ServerHandler, Vec3, TICKS_PER_SECOND,
};
use firework::{data::items::Item, protocol::data_types::ItemStack};
use firework::{
    gui::GUIInit,
    protocol::{
        client_bound::{CustomSound, IdMapHolder, SoundSource},
        data_types::{
            BossBarAction, BossBarColor, BossBarDivision, ItemNbt, Particle, Particles,
            StackContents,
        },
    },
};
use firework::{gui::GuiScreen, world::World};
use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde_json::json;
use std::collections::HashMap;
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
}

pub enum Maps {
    Canyon,
    Cavern,
    Temple,
}

impl Distribution<Maps> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Maps {
        match rng.gen_range(0..3) {
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
    pub fn get_author_times(&self) -> &'static [f32] {
        match self {
            Maps::Canyon => &canyon::AUTHOR_TIMES,
            Maps::Cavern => &cavern::AUTHOR_TIMES,
            Maps::Temple => &temple::AUTHOR_TIMES,
        }
    }
}

#[derive(Debug, Clone)]
enum GameState {
    Starting { ticks_until_start: u16 },
    Running { start_time: Instant },
    Finished { finish_time: Instant },
}

#[derive(PartialEq, Clone, Debug)]
enum PlayerFinishedState {
    DNF,
    InProgress { percentage: f32 },
    Finished { finish_time: Duration },
}

impl PartialOrd for PlayerFinishedState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (PlayerFinishedState::DNF, PlayerFinishedState::DNF) => Some(std::cmp::Ordering::Equal),
            (PlayerFinishedState::DNF, _) => Some(std::cmp::Ordering::Greater),
            (_, PlayerFinishedState::DNF) => Some(std::cmp::Ordering::Less),
            (
                PlayerFinishedState::InProgress { percentage: p1 },
                PlayerFinishedState::InProgress { percentage: p2 },
            ) => p2.partial_cmp(p1), // there is a better way to reverse comparisons but shhhhhh
            (PlayerFinishedState::InProgress { .. }, PlayerFinishedState::Finished { .. }) => {
                Some(std::cmp::Ordering::Greater)
            }
            (PlayerFinishedState::Finished { .. }, PlayerFinishedState::InProgress { .. }) => {
                Some(std::cmp::Ordering::Less)
            }
            (
                PlayerFinishedState::Finished { finish_time: t1 },
                PlayerFinishedState::Finished { finish_time: t2 },
            ) => t1.partial_cmp(t2),
        }
    }
}

const BOOST_TICKS: usize = 12;

pub struct GlideServerHandler {
    pub map: Maps,
    pub created_at: Instant,
    game_state: Mutex<GameState>,
    player_finished_states: Mutex<HashMap<u128, (String, PlayerFinishedState)>>,
    commands: CommandTree<Self, MiniGameProxy>,
}

pub struct GlidePlayerHandler {
    last_boost: Mutex<Option<(Instant, usize)>>,
    last_loft: Mutex<Option<(Instant, usize)>>,
    boost_status: RwLock<Option<BoostStatus>>,
    animation_frame: Mutex<i32>,
    server: Arc<Server<GlideServerHandler, MiniGameProxy>>,
    _proxy: Arc<MiniGameProxy>,
    last_checkpoint: Mutex<Option<usize>>,
    start_time: Mutex<Option<Instant>>,
    last_damage: Mutex<Instant>,
    recent_packets: Mutex<Vec<Instant>>,
}

fn format_duration(&dur: &Duration) -> String {
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
        println!("{} transferred", client.client_data.profile.name);
        client.send_boss_bar_action(0, BossBarAction::Remove);
        client.clear_scoreboard();
        let mut finished_states_lock = self.server.handler.player_finished_states.lock().await;
        finished_states_lock.insert(
            client.client_data.uuid,
            (
                client.client_data.profile.name.clone(),
                PlayerFinishedState::DNF,
            ),
        );
        let flying_players = finished_states_lock.iter().fold(0, |acc, e| {
            acc + match e.1 .1 {
                PlayerFinishedState::InProgress { .. } => 1,
                _ => 0,
            }
        });
        if flying_players == 0 {
            let mut game_state = self.server.handler.game_state.lock().await;
            *game_state = GameState::Finished {
                finish_time: Instant::now(),
            };
            drop(game_state);
            let mut leaderboard = finished_states_lock
                .iter()
                .map(|e| (e.1 .0.clone(), e.1 .1.clone()))
                .collect::<Vec<_>>();
            leaderboard.sort_unstable_by(|a, b| {
                b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal)
            });
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
                            "text": "Leaderboard:",
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
                                "{}",
                                {
                                    let mut leaderboard_str = String::new();
                                    for (i, (name, state)) in leaderboard.iter().enumerate()
                                    {
                                        leaderboard_str.push_str(&format!(
                                            "{}. {} - {}\n",
                                            i + 1,
                                            name,
                                            match state {
                                                PlayerFinishedState::Finished {
                                                    finish_time,
                                                } => format_duration(finish_time),
                                                PlayerFinishedState::DNF => "DNF".to_string(),
                                                PlayerFinishedState::InProgress { .. } => {
                                                    "DNF".to_string()
                                                }
                                            }
                                        ));
                                    }
                                    leaderboard_str
                                }
                            ),
                            "color": "yellow",
                        }
                    ])
                    .to_string(),
                    false,
                );
                client.clear_scoreboard();
            }
        }
        Ok(())
    }
    async fn on_leave(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        println!("{} left", client.client_data.profile.name);
        let mut finished_states_lock = self.server.handler.player_finished_states.lock().await;
        finished_states_lock.insert(
            client.client_data.uuid,
            (
                client.client_data.profile.name.clone(),
                PlayerFinishedState::DNF,
            ),
        );
        let flying_players = finished_states_lock.iter().fold(0, |acc, e| {
            acc + match e.1 .1 {
                PlayerFinishedState::InProgress { .. } => 1,
                _ => 0,
            }
        });
        if flying_players == 0 {
            let mut game_state = self.server.handler.game_state.lock().await;
            *game_state = GameState::Finished {
                finish_time: Instant::now(),
            };
            drop(game_state);
            let mut leaderboard = finished_states_lock
                .iter()
                .map(|e| (e.1 .0.clone(), e.1 .1.clone()))
                .collect::<Vec<_>>();
            leaderboard.sort_unstable_by(|a, b| {
                b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal)
            });
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
                            "text": "Leaderboard:",
                            "color": "gold",
                        },
                        {
                            "text": "\n\n",
                        },
                        {
                            "text": format!(
                                "{}",
                                {
                                    let mut leaderboard_str = String::new();
                                    for (i, (name, state)) in leaderboard.iter().enumerate()
                                    {
                                        leaderboard_str.push_str(&format!(
                                            "{}. {} - {}\n",
                                            i + 1,
                                            name,
                                            match state {
                                                PlayerFinishedState::Finished {
                                                    finish_time,
                                                } => format_duration(finish_time),
                                                PlayerFinishedState::DNF => "DNF".to_string(),
                                                PlayerFinishedState::InProgress { .. } => {
                                                    "DNF".to_string()
                                                }
                                            }
                                        ));
                                    }
                                    leaderboard_str
                                }
                            ),
                            "color": "yellow",
                        }
                    ])
                    .to_string(),
                    false,
                );
                client.clear_scoreboard();
            }
        }
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
        let mut start_time = self.start_time.lock().await;
        start_time.replace(Instant::now());
        drop(start_time);

        {
            let mut player = client.player.write().await;
            player.elytra_flying = true;
            client.server.broadcast_entity_metadata_update(
                client,
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
            client.sync_position(start.clone(), None).await;
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
            let mut next = self.last_checkpoint.lock().await;
            if let Some(next_checkpoint_id) = *next {
                let next_checkpoint = map.get_checkpoints().get(next_checkpoint_id);

                if let Some(next_checkpoint) = next_checkpoint {
                    // The client reached the next checkpoint
                    if next_checkpoint.plane.intersects(start, end) {
                        drop(start);

                        if next_checkpoint_id + 1 >= map.get_checkpoints().len() {
                            next.take();

                            self.on_win(client, &start_time).await;
                        } else {
                            next.replace(next_checkpoint_id + 1);

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
                const HORIZONTAL_SPEED_FACTOR: f64 = 1.008;

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

        let mut particles: Vec<Particle> = Vec::new();
        let mut animation_frame_lock = self.animation_frame.lock().await;
        let animation_frame = *animation_frame_lock;
        *animation_frame_lock = animation_frame.wrapping_add(1);
        drop(animation_frame_lock);
        {
            let position = client.player.read().await.position.clone();

            for checkpoint in map.get_checkpoints().iter() {
                let (min, max) = checkpoint.plane.to_cartesian_pair();

                let center = checkpoint.plane.center();

                let distance = ((center.x - position.x).abs().powi(2)
                    + (center.y - position.y).abs().powi(2)
                    + (center.z - position.z).abs().powi(2))
                .sqrt();

                if distance > 100. {
                    continue;
                }

                // sadly the particle system is based around gaussian distributions, so here's a hack to
                // make the particles look like a box

                // add one because we want to include a minimum of 1 segment in each dimension
                let count_x = (max.x - min.x).abs() as i32 / 12 + 1;
                let count_y = (max.y - min.y).abs() as i32 / 12 + 1;
                let count_z = (max.z - min.z).abs() as i32 / 12 + 1;

                let segment_width = (max.x - min.x).abs() / count_x as f64;
                let segment_height = (max.y - min.y).abs() / count_y as f64;
                let segment_depth = (max.z - min.z).abs() / count_z as f64;

                for x in 0..count_x {
                    for y in 0..count_y {
                        for z in 0..count_z {
                            if (x + y + z + animation_frame) % 10 != 0 {
                                continue;
                            }
                            let x = min.x + (x as f64 + 0.5) * segment_width;
                            let y = min.y + (y as f64 + 0.5) * segment_height;
                            let z = min.z + (z as f64 + 0.5) * segment_depth;
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
                                20,
                            ));
                        }
                    }
                }
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

                let distance = ((loft.area.min.x as f64 - position.x).abs().powi(2)
                    + (loft.area.min.y as f64 - position.y).abs().powi(2)
                    + (loft.area.min.z as f64 - position.z).abs().powi(2))
                .sqrt();

                if distance > 100. {
                    continue;
                }

                let x = loft.area.min.x as f64
                    + (loft.area.max.x as f64 - loft.area.min.x as f64) * rand::random::<f64>();
                let y = loft.area.min.y as f64;
                let height = loft.area.max.y as f32 - loft.area.min.y as f32;
                let z = loft.area.min.z as f64
                    + (loft.area.max.z as f64 - loft.area.min.z as f64) * rand::random::<f64>();
                const LIFETIME: f32 = 100.0; // lifetime in ticks
                let speed = height / LIFETIME;
                particles.push(Particle::new(
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
                ));
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

                // we define the depth as the direction the boost is pointing
                // so we can use the same code for both directions
                // the width is perpendicular to the depth

                let depth = match boost.particle_type {
                    BoostParticleType::BoostNorth | BoostParticleType::BoostSouth => {
                        boost.area.max.z as f64 - boost.area.min.z as f64
                    }
                    BoostParticleType::BoostEast | BoostParticleType::BoostWest => {
                        boost.area.max.x as f64 - boost.area.min.x as f64
                    }
                };

                let chevron_count = depth as usize / 3;

                // the x and z coords in this vector get swapped depending on the direction of the boost
                // or become negative if the boost is pointing in the negative direction
                let mut particle_positions: Vec<Vec3> = Vec::new();

                for chevron in 0..chevron_count {
                    // x is along "width", z is along "depth".
                    // all are in 0-1 range throughout this loop

                    // top
                    let random_x = rand::random::<f64>();
                    let min_depth = chevron as f64 / chevron_count as f64;
                    let max_depth = (chevron + 1) as f64 / chevron_count as f64;
                    let z = (random_x - 0.5).abs() * 2. * (max_depth - min_depth) + min_depth;

                    particle_positions.push(Vec3::new(random_x, 1., z));

                    // sides
                    let random_y = rand::random::<f64>();
                    let z = (random_y - 0.5).abs() * 2. * (max_depth - min_depth) + min_depth;

                    particle_positions.push(Vec3::new(0., random_y, z));
                    particle_positions.push(Vec3::new(1., random_y, z));
                }

                if boost.particle_type == BoostParticleType::BoostSouth
                    || boost.particle_type == BoostParticleType::BoostEast
                {
                    for position in &mut particle_positions {
                        position.z = 1. - position.z;
                    }
                }

                if boost.particle_type == BoostParticleType::BoostEast
                    || boost.particle_type == BoostParticleType::BoostWest
                {
                    for position in &mut particle_positions {
                        std::mem::swap(&mut position.x, &mut position.z);
                    }
                }

                for particle_position in particle_positions {
                    let x = boost.area.min.x as f64
                        + (boost.area.max.x as f64 - boost.area.min.x as f64) * particle_position.x;
                    let y = boost.area.min.y as f64
                        + (boost.area.max.y as f64 - boost.area.min.y as f64) * particle_position.y;
                    let z = boost.area.min.z as f64
                        + (boost.area.max.z as f64 - boost.area.min.z as f64) * particle_position.z;

                    particles.push(Particle::new(
                        Particles::Dust {
                            red: 1.,
                            green: 0.7,
                            blue: 0.,
                            scale: 3.,
                        },
                        true,
                        x,
                        y,
                        z,
                        0.,
                        0.,
                        0.,
                        0.,
                        4,
                    ));
                }
            }
        }

        client.send_particles(particles);
        let percentage = client.handler.get_race_percentage(client, map).await;
        client.send_boss_bar_action(
            0,
            BossBarAction::UpdateTitle {
                title: json!([
                    {
                        "text": "You are ",
                        "color": "white"
                    },
                    {
                        "text": format!("{:.2}%",
                            percentage * 100.
                    ),
                        "color": "gold"
                    },
                    {
                        "text": " to the finish!",
                        "color": "white"
                    }
                ])
                .to_string(),
            },
        );
        client.send_boss_bar_action(0, BossBarAction::UpdateHealth { health: percentage });
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
                                client
                                    .sync_position(
                                        checkpoint.spawn_position.clone(),
                                        Some(checkpoint.spawn_rotation.clone()),
                                    )
                                    .await;

                                self.boost_status.write().await.take();
                                client.set_velocity(Vec3::scalar(0.));
                                return Ok(on_ground);
                            }
                        }

                        client
                            .sync_position(
                                map.get_spawn_position().clone(),
                                Some(Rotation::new(0., 0.)),
                            )
                            .await;

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

                        self.boost_status.write().await.take();
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
                        client.set_health(health - 2.);
                    }
                }
            }
        }

        Ok(on_ground)
    }
}

impl GlidePlayerHandler {
    async fn get_race_percentage(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
        map: &Maps,
    ) -> f32 {
        let checkpoint_index = *client.handler.last_checkpoint.lock().await;

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

            let mut percentage = 0.;
            let last_checkpoint = client.handler.last_checkpoint.lock().await.unwrap();
            let total_author_time = map.get_author_times().iter().sum::<f32>();
            for (i, author_time) in map
                .get_author_times()
                .iter()
                .map(|f| f / total_author_time)
                .enumerate()
            {
                if last_checkpoint > i {
                    percentage += author_time;
                } else if last_checkpoint == i {
                    percentage += percent_to_later_checkpoint * author_time;
                }
            }

            return percentage;
        };
        0.
    }
    async fn on_checkpoint(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
        start_time: &Instant,
        checkpoint_id: usize,
    ) {
        let elapsed = start_time.elapsed();
        println!(
            "Checkpoint {} in {}",
            checkpoint_id + 1,
            format_duration(&elapsed)
        );
        client.send_title(
            "{\"text\":\"Checkpoint\"}".to_string(),
            json!({
                "text":
                    format!(
                        "Reached Checkpoint {} in {}",
                        checkpoint_id + 1,
                        format_duration(&elapsed)
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
        if let GameState::Running { start_time } = *game_state {
            let elapsed = start_time.elapsed();
            client.send_boss_bar_action(0, BossBarAction::UpdateHealth { health: 1. });
            let mut finished_states_lock = self.server.handler.player_finished_states.lock().await;
            finished_states_lock.insert(
                client.client_data.uuid,
                (
                    client.client_data.profile.name.clone(),
                    PlayerFinishedState::Finished {
                        finish_time: elapsed,
                    },
                ),
            );
            let flying_players = finished_states_lock.iter().fold(0, |acc, e| {
                acc + match e.1 .1 {
                    PlayerFinishedState::InProgress { .. } => 1,
                    _ => 0,
                }
            });

            if flying_players == 0 {
                *game_state = GameState::Finished {
                    finish_time: Instant::now(),
                };
                drop(game_state);
                let mut leaderboard = finished_states_lock
                    .iter()
                    .map(|e| (e.1 .0.clone(), e.1 .1.clone()))
                    .collect::<Vec<_>>();
                leaderboard.sort_unstable_by(|a, b| {
                    b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal)
                });
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
                                "text": "Leaderboard:",
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
                                    "{}",
                                    {
                                        let mut leaderboard_str = String::new();
                                        for (i, (name, state)) in leaderboard.iter().enumerate()
                                        {
                                            leaderboard_str.push_str(&format!(
                                                "{}. {} - {}\n",
                                                i + 1,
                                                name,
                                                match state {
                                                    PlayerFinishedState::Finished {
                                                        finish_time,
                                                    } => format_duration(finish_time),
                                                    PlayerFinishedState::DNF => "DNF".to_string(),
                                                    PlayerFinishedState::InProgress { .. } => {
                                                        "DNF".to_string()
                                                    }
                                                }
                                            ));
                                        }
                                        leaderboard_str
                                    }
                                ),
                                "color": "yellow",
                            }
                        ])
                        .to_string(),
                        false,
                    );
                    client.clear_scoreboard();
                }
            }
        }
        client.show_chat_message(
            json!(
                {
                    "text": format!(
                        "Completed race in {}",
                        format_duration(&start_time.elapsed())
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
                        format_duration(&start_time.elapsed())
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
                            format_duration(&start_time.elapsed())
                        )
                })
                .to_string(),
            },
        );

        client.send_boss_bar_action(0, BossBarAction::UpdateHealth { health: 1. });
        client.send_boss_bar_action(
            0,
            BossBarAction::UpdateStyle {
                color: BossBarColor::Green,
                division: BossBarDivision::NoDivisions,
            },
        );
    }
}

pub struct Menu {
    pub items: Vec<ItemStack>,
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
        _slot: ClickContainer,
        _client: &Client<GlideServerHandler, MiniGameProxy>,
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
                ticks_until_start: 65,
            }),
            player_finished_states: Mutex::new(HashMap::with_capacity(8)),
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
                };

                drop(game_state);

                self.start_game(server, &proxy).await;
            }
            GameState::Running { start_time } => {
                let time_elapsed = format_duration(&start_time.elapsed());

                let map = &server.handler.map;

                let mut finished_states_lock = server.handler.player_finished_states.lock().await;
                for client in server.player_list.iter() {
                    if let Some(finished_state) = finished_states_lock.get(&client.client_data.uuid)
                    {
                        match finished_state.1 {
                            PlayerFinishedState::InProgress { .. } => {
                                finished_states_lock.insert(
                                    client.client_data.uuid,
                                    (
                                        client.client_data.profile.name.clone(),
                                        PlayerFinishedState::InProgress {
                                            percentage: client
                                                .handler
                                                .get_race_percentage(&client, map)
                                                .await,
                                        },
                                    ),
                                );
                            }
                            _ => (),
                        }
                    }
                }

                let mut leaderboard_lines: Vec<(PlayerFinishedState, String)> = Vec::new();

                for (_uuid, finished_state) in finished_states_lock.iter() {
                    let string_message = match finished_state {
                        (name, PlayerFinishedState::Finished { finish_time }) => {
                            format!("§7{}: §b{}", name, format_duration(finish_time))
                        }
                        (name, PlayerFinishedState::InProgress { percentage }) => {
                            format!("§7{}: §b{:.2}%", name, percentage * 100.0)
                        }
                        (name, PlayerFinishedState::DNF) => {
                            format!("§7{}: §cDNF", name)
                        }
                    };
                    leaderboard_lines.push((finished_state.1.clone(), string_message));
                }

                drop(finished_states_lock);

                leaderboard_lines.sort_unstable_by(|a, b| {
                    b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal)
                }); // reverse it in a really scuffed way, just swap a and b

                for client in server.player_list.iter() {
                    client.send_system_chat_message(
                        json!({
                            "text": format!("Time elapsed: {}", time_elapsed),
                        })
                        .to_string(),
                        true,
                    );
                    for (i, leaderboard_line) in leaderboard_lines.iter().enumerate() {
                        client.set_scoreboard_line(i, leaderboard_line.1.clone());
                    }
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
            flying_allowed: false,
            gamemode: GameMode::Adventure,
            profile,
            uuid,
            ..Player::default()
        };

        player.inventory.set_slot(
            InventorySlot::Chestplate,
            Some(StackContents {
                id: Item::Elytra,
                count: 1,
                nbt: ItemNbt {
                    enchantments: Some(vec![Enchantment {
                        id: "minecraft:binding_curse".to_string(),
                        level: 1,
                    }]),
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
            client.update_objectives(
                "leaderboard_id".to_string(),
                ObjectiveAction::Create {
                    objective_value: json!([
                        {
                            "text": " >> ",
                            "italic": false,
                            "color": "dark_aqua"
                        },
                        {
                            "text": "Firework",
                            "color": "aqua",
                            "bold": true
                        },
                        {
                            "text": " << ",
                            "italic": false,
                            "color": "dark_aqua"
                        },
                    ])
                    .to_string(),
                    objective_type: ObjectiveType::Integer,
                },
            );

            client.show_scoreboard(1, "leaderboard_id".to_string());

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
            client
                .sync_position(
                    self.map.get_spawn_position().clone(),
                    Some(Rotation::new(0., 0.)),
                )
                .await;

            client.send_boss_bar_action(
                0,
                BossBarAction::Add {
                    title: json!({
                        "text": "",
                    })
                    .to_string(),
                    health: 0.,
                    color: BossBarColor::Blue,
                    division: BossBarDivision::NoDivisions,
                    flags: 0,
                },
            );

            let mut finished_states_lock = server.handler.player_finished_states.lock().await;
            finished_states_lock.insert(
                client.client_data.uuid,
                (
                    client.client_data.profile.name.clone(),
                    PlayerFinishedState::InProgress { percentage: 0. },
                ),
            );
            drop(finished_states_lock);
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
