use async_trait::async_trait;
use firework::{
    client::{Client, GameMode, InventorySlot, Player},
    commands::{Argument, CommandNode},
    entities::{EntityMetadata, Pose},
    ConnectionError, PlayerHandler, Rotation, Server, ServerHandler, Vec3, TICKS_PER_SECOND,
};
use firework_authentication::Profile;
use firework_data::items::{Elytra, Item};
use firework_protocol::{
    client_bound::{CustomSound, IdMapHolder, SoundSource},
    data_types::{BossBarAction, BossBarColor, BossBarDivision, ItemNbt, SlotInner},
};
use firework_protocol_core::VarInt;
use firework_world::World;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde_json::json;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

use crate::{MiniGameProxy, TransferData, CAVERN_BATTLE_WORLD};

mod cavern;

pub struct SpawnPoint {
    position: Vec3,
    rotation: Rotation,
}
pub enum Maps {
    Cavern,
}

impl Distribution<Maps> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Maps {
        match rng.gen_range(0..1) {
            0 => Maps::Cavern,
            _ => panic!("Invalid random number huh? (https://www.youtube.com/watch?v=4kEO7VjKRB8)"),
        }
    }
}

impl Maps {
    pub fn get_world(&self) -> &'static World {
        match self {
            Maps::Cavern => &CAVERN_BATTLE_WORLD,
        }
    }
    pub fn get_spawn_point(&self) -> &SpawnPoint {
        match self {
            Maps::Cavern => &cavern::SPAWN_POINTS[0],
        }
    }
}

enum GameState {
    Starting {
        ticks_until_start: u16,
    },
    Running {
        start_time: Instant,
        initial_player_count: usize,
    },
}

pub struct BattleServerHandler {
    pub map: Maps,
    pub created_at: Instant,
    game_state: Mutex<GameState>,
    commands: CommandNode<Self, MiniGameProxy>,
}

pub struct BattlePlayerHandler {
    animation_frame: Mutex<u8>,
    server: Arc<Server<BattleServerHandler, MiniGameProxy>>,
    _proxy: Arc<MiniGameProxy>,
    start_time: Mutex<Option<Instant>>,
    last_damage: Mutex<Instant>,
    grounded: Mutex<bool>,
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
impl PlayerHandler<BattleServerHandler, MiniGameProxy> for BattlePlayerHandler {
    fn new(
        server: Arc<Server<BattleServerHandler, MiniGameProxy>>,
        proxy: Arc<MiniGameProxy>,
    ) -> Self {
        Self {
            animation_frame: Mutex::new(0),
            server,
            _proxy: proxy,
            recent_packets: Mutex::new(Vec::new()),
            start_time: Mutex::new(None),
            last_damage: Mutex::new(Instant::now()),
            grounded: Mutex::new(false),
        }
    }
    async fn on_server_bound_packet(
        &self,
        client: &Client<BattleServerHandler, MiniGameProxy>,
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
        client: &Client<BattleServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        client.send_boss_bar_action(0, BossBarAction::Remove);
        Ok(())
    }
    async fn on_post_load(
        &self,
        client: &Client<BattleServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        // Show the welcome message
        client.show_chat_message(
            json!([
              {
                  "text": "\n"
              },
              {
                  "text": "Welcome to the Battle Minigame!",
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
                  "text": "- battle",
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
        client: &Client<BattleServerHandler, MiniGameProxy>,
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

        Ok(Some(pos))
    }
    async fn on_tick(&self, client: &Client<BattleServerHandler, MiniGameProxy>) {}
    async fn on_on_ground(
        &self,
        client: &Client<BattleServerHandler, MiniGameProxy>,
        on_ground: bool,
    ) -> Result<bool, ConnectionError> {
        Ok(on_ground)
    }
}

impl BattlePlayerHandler {
    async fn on_checkpoint(
        &self,
        client: &Client<BattleServerHandler, MiniGameProxy>,
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
        client: &Client<BattleServerHandler, MiniGameProxy>,
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
impl ServerHandler<MiniGameProxy> for BattleServerHandler {
    type PlayerHandler = BattlePlayerHandler;
    fn new() -> Self {
        Self {
            map: rand::random(),
            created_at: Instant::now(),
            game_state: Mutex::new(GameState::Starting {
                ticks_until_start: 120,
            }),
            commands: CommandNode::root(),
        }
    }
    fn get_world(&self) -> &'static World {
        self.map.get_world()
    }
    async fn on_tick(&self, server: &Server<Self, MiniGameProxy>, proxy: Arc<MiniGameProxy>) {
        if self.created_at.elapsed().as_millis() / 50 >= 200 && server.player_list.len() == 0 {
            proxy.battle_queue.lock().await.remove_server(server.id);
        }
        let mut game_state = self.game_state.lock().await;
        match &*game_state {
            GameState::Starting { ticks_until_start } => {
                if *ticks_until_start != 0 {
                    if *ticks_until_start % TICKS_PER_SECOND as u16 == 0
                        && *ticks_until_start <= 100
                    {
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
                    initial_player_count: server.player_list.len(),
                };

                drop(game_state);

                self.start_game(server, &proxy).await;
            }
            GameState::Running {
                start_time,
                initial_player_count,
            } => {
                let player_count = server.player_list.len();

                for player in server.player_list.iter() {
                    player.send_boss_bar_action(
                        0,
                        BossBarAction::UpdateTitle {
                            title: json!({
                                "text": format!("{} Player{} Remaining", player_count, if player_count == 1 { "" } else { "s" }),
                            })
                            .to_string(),
                        },
                    );
                    player.send_boss_bar_action(
                        0,
                        BossBarAction::UpdateHealth {
                            health: player_count as f32 / initial_player_count.clone() as f32,
                        },
                    );
                }
            }
        }
    }
    async fn load_player(&self, profile: Profile, uuid: u128) -> Result<Player, ConnectionError> {
        let mut player = Player {
            position: self.map.get_spawn_point().position.clone(),
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
        _server: &Server<BattleServerHandler, MiniGameProxy>,
        _proxy: &MiniGameProxy,
    ) -> Result<&CommandNode<Self, MiniGameProxy>, ConnectionError> {
        Ok(&self.commands)
    }
}

impl BattleServerHandler {
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
                self.map.get_spawn_point().position.clone(),
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
    client: &Client<BattleServerHandler, MiniGameProxy>,
    _server: &Server<BattleServerHandler, MiniGameProxy>,
    _proxy: &MiniGameProxy,
) {
    client.transfer(TransferData::Lobby);
}
