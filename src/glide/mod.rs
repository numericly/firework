use async_trait::async_trait;
use firework::{
    client::{Client, GameMode, InventorySlot, Player},
    commands::{Argument, CommandNode},
    entities::{EntityMetadata, Pose},
    AxisAlignedBB, AxisAlignedPlane, ConnectionError, PlayerHandler, Rotation, Server,
    ServerHandler, Vec3,
};
use firework_authentication::Profile;
use firework_data::items::{Elytra, Item};
use firework_protocol::{
    client_bound::{CustomSound, IdMapHolder, SoundSource},
    data_types::{
        BossBarAction, BossBarColor, BossBarDivision, ItemNbt, Particle, Particles, Slot,
    },
};
use firework_protocol_core::VarInt;
use firework_world::World;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde_json::json;
use std::{
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tokio::sync::{Mutex, RwLock};

use crate::{MiniGameProxy, TransferData, CANYON_GLIDE_WORLD, CAVERN_GLIDE_WORLD};

mod canyon;
mod cavern;

pub struct Boost {
    area: AxisAlignedBB,
    speed: f64,
    particle_type: BoostParticleType,
}

pub struct Checkpoint {
    plane: AxisAlignedPlane,
    spawn_position: Vec3,
    spawn_rotation: Rotation,
}

#[derive(Debug, Clone)]
pub struct BoostStatus {
    velocity: Vec3,
    times_remaining: usize,
    speed: f64,
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
}

impl Distribution<Maps> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Maps {
        match rng.gen_range(0..=1) {
            0 => Maps::Canyon,
            1 => Maps::Cavern,
            _ => panic!("Invalid random number huh?"),
        }
    }
}

impl Maps {
    pub fn get_world(&self) -> &'static World {
        match self {
            Maps::Canyon => &CANYON_GLIDE_WORLD,
            Maps::Cavern => &CAVERN_GLIDE_WORLD,
        }
    }
    pub fn get_checkpoints(&self) -> &'static [Checkpoint] {
        match self {
            Maps::Canyon => &canyon::CHECKPOINTS,
            Maps::Cavern => &cavern::CHECKPOINTS,
        }
    }
    pub fn get_boosts(&self) -> &'static [Boost] {
        match self {
            Maps::Canyon => &canyon::BOOSTS,
            Maps::Cavern => &cavern::BOOSTS,
        }
    }
    pub fn get_spawn_area(&self) -> &AxisAlignedBB {
        match self {
            Maps::Canyon => &canyon::SPAWN_AREA,
            Maps::Cavern => &cavern::SPAWN_AREA,
        }
    }
    pub fn get_spawn_position(&self) -> &Vec3 {
        match self {
            Maps::Canyon => &canyon::SPAWN_POSITION,
            Maps::Cavern => &cavern::SPAWN_POSITION,
        }
    }
}

enum GameState {
    Starting { ticks_until_start: u16 },
    Running { start_time: Instant },
}

const BOOST_TICKS: usize = 12;

pub struct GlideServerHandler {
    pub map: Maps,
    pub created_at: Instant,
    game_state: Mutex<GameState>,
    commands: CommandNode<Self, MiniGameProxy>,
}

pub struct GlidePlayerHandler {
    boost_status: RwLock<Option<BoostStatus>>,
    animation_frame: Mutex<u8>,
    server: Arc<Server<GlideServerHandler, MiniGameProxy>>,
    proxy: Arc<MiniGameProxy>,
    last_checkpoint: Mutex<Option<usize>>,
    start_time: Mutex<Option<Instant>>,
    last_damage: Mutex<Instant>,
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
            boost_status: RwLock::new(None),
            animation_frame: Mutex::new(0),
            server,
            proxy,
            last_checkpoint: Mutex::new(Some(0)),
            start_time: Mutex::new(None),
            last_damage: Mutex::new(Instant::now()),
        }
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
        let max_velocity = 20.0;
        if (end.clone() - start.clone()).length() > max_velocity {
            client.sync_position(start.clone(), None);
            client.set_velocity(
                client
                    .player
                    .read()
                    .await
                    .velocity
                    .clamp(0., max_velocity * 0.9),
            );
            return Ok(None);
        }

        // check for passing through checkpoints
        if let GameState::Running { start_time } = &*client.server.handler.game_state.lock().await {
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

                            self.on_win(client, start_time).await;
                        } else {
                            self.last_checkpoint
                                .lock()
                                .await
                                .replace(next_checkpoint_id + 1);

                            self.on_checkpoint(client, start_time, next_checkpoint_id)
                                .await;
                        }
                    }
                }
            }
        }

        Ok(Some(pos))
    }
    async fn on_tick(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        let boost_status = self.boost_status.read().await.clone();
        if let Some(BoostStatus {
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
                let new_speed = velocity_speed * 0.80 + speed / BOOST_TICKS as f64;

                let new_vec = new_direction * Vec3::scalar(new_speed);

                client.set_velocity(new_vec.clone());

                self.boost_status.write().await.replace(BoostStatus {
                    times_remaining: times_remaining - 1,
                    speed,
                    velocity: new_vec,
                });
            }
        }

        let map = &self.server.handler.map;

        {
            const PARTICLE_DENSITY: i32 = 2;
            let time = client.server.handler.created_at.elapsed().as_secs_f32();
            let position = client.player.read().await.position.clone();

            for boost in map.get_boosts().iter() {
                if boost.area.within(position.clone()) {
                    let velocity = client.player.read().await.velocity.clone();
                    let mut boost_status = self.boost_status.write().await;
                    if boost_status.is_none() {
                        boost_status.replace(BoostStatus {
                            times_remaining: BOOST_TICKS,
                            speed: boost.speed.clone(),
                            velocity,
                        });
                    }
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

        //
        //
        // 90% sure this code causes deadlocks
        //
        //
        // // find which two checkpoints the player is between
        // let mut checkpoints = self.checkpoints.lock().await;
        // let mut checkpoint_index = 0;
        // for i in 0..checkpoints.len() {
        //     if checkpoints[i] {
        //         checkpoint_index = i;
        //     }
        // }
        // let first_checkpoint = &CANYON_CHECKPOINTS[checkpoint_index];
        // let second_checkpoint = &CANYON_CHECKPOINTS[(checkpoint_index + 1) % checkpoints.len()];
        // // find the center of each checkpoint
        // let first_checkpoint_center = first_checkpoint.plane.center();
        // let second_checkpoint_center = second_checkpoint.plane.center();
        // // get the current position of the player
        // let player_position = client.player.read().await.position.clone();
        // // find the distance between the player and the center of each checkpoint
        // let first_checkpoint_distance = first_checkpoint_center.distance(&player_position);
        // let second_checkpoint_distance = second_checkpoint_center.distance(&player_position);
        // // get the percentage of the way between the two checkpoints
        // let checkpoint_percent =
        //     first_checkpoint_distance / (first_checkpoint_distance + second_checkpoint_distance);
        // // println!(
        // //     "{}% of the way to checkpoint {}",
        // //     (checkpoint_percent * 100.).round(),
        // //     checkpoint_index + 1
        // // );
        // let checkpoint_count = CANYON_CHECKPOINTS.len();
        // // decrease dashes when there are more checkpoints to achieve a constant width
        // let dashes_per_checkpoint = 45 / (checkpoint_count - 1) - 1;
        // let mut checkpoint_representation = String::new();
        // for i in 0..(checkpoint_count - 1) {
        //     if i == checkpoint_index {
        //         checkpoint_representation.push_str(&format!(
        //             "||{}{}{}",
        //             "-".repeat((checkpoint_percent * dashes_per_checkpoint as f64) as usize),
        //             "o",
        //             "-".repeat(((1. - checkpoint_percent) * dashes_per_checkpoint as f64) as usize)
        //         ));
        //     } else {
        //         checkpoint_representation
        //             .push_str(&format!("||{}", "-".repeat(dashes_per_checkpoint),));
        //     }
        // }
        // checkpoint_representation.push_str(&format!("||"));
        // let chat_message = format!(
        //     "{{\"text\": \"{}\",\"bold\": true}}",
        //     checkpoint_representation
        // );
        // client.send_system_chat_message(chat_message.to_string(), true);

        Ok(())
    }
    async fn on_on_ground(
        &self,
        client: &Client<GlideServerHandler, MiniGameProxy>,
        on_ground: bool,
    ) -> Result<bool, ConnectionError> {
        if let GameState::Running { .. } = &*client.server.handler.game_state.lock().await {
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

                    let health = client.player.read().await.health.clone();
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
                        client.set_health(health - 2.);
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
                &client,
                vec![
                    EntityMetadata::EntityFlags(player.entity_flags()),
                    EntityMetadata::EntityPose(Pose::Standing),
                ],
                true,
            );
        }

        client.set_flying(true);

        client.set_max_health(20.).await;

        client.set_health(20.)
    }
}

#[async_trait]
impl ServerHandler<MiniGameProxy> for GlideServerHandler {
    type PlayerHandler = GlidePlayerHandler;
    fn new() -> Self {
        Self {
            map: rand::random(),
            created_at: Instant::now(),
            game_state: Mutex::new(GameState::Starting {
                ticks_until_start: 120,
            }),
            commands: CommandNode::root().sub_command(CommandNode::literal("lobby").executable(
                Box::new(move |args, client, server, proxy| {
                    Box::pin(return_to_lobby(args, client, server, proxy))
                }),
            )),
        }
    }
    fn get_world(&self) -> &'static World {
        self.map.get_world()
    }
    async fn on_tick(&self, server: &Server<Self, MiniGameProxy>, proxy: Arc<MiniGameProxy>) {
        if self.created_at.elapsed().as_millis() / 50 >= 200 && server.player_list.len() == 0 {
            proxy.glide_queue.lock().await.remove_server(server.id);
            println!("Closing server {:x?} due to inactivity", server.id);
        }
        let mut game_state = self.game_state.lock().await;
        match &*game_state {
            GameState::Starting { ticks_until_start } => {
                if *ticks_until_start != 0 {
                    if *ticks_until_start % 20 == 0 && *ticks_until_start <= 100 {
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
                *game_state = GameState::Running {
                    start_time: Instant::now(),
                };

                drop(game_state);

                self.start_game(server, &proxy).await;
            }
            GameState::Running { start_time } => {
                let time_elapsed = format_duration(start_time.elapsed());

                for player in server.player_list.iter() {
                    player.send_boss_bar_action(
                        0,
                        BossBarAction::UpdateTitle {
                            title: json!({
                                "text": format!("Time elapsed: {}", time_elapsed),
                            })
                            .to_string(),
                        },
                    )
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
            client.set_health(6.);
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
            client.sync_position(
                self.map.get_spawn_position().clone(),
                Some(Rotation::new(0., 0.)),
            );

            client.send_boss_bar_action(
                0,
                BossBarAction::Add {
                    title: json!({
                        "text": "Time elapsed: 0:00.000",
                    })
                    .to_string(),
                    health: 1.0,
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
