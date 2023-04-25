use async_trait::async_trait;
use dashmap::DashSet;
use firework::{
    client::{Client, GameMode, InventorySlot, Player},
    commands::{Argument, Command, CommandNode, CommandTree},
    entities::{EntityMetadata, Pose},
    ConnectionError, PlayerHandler, Rotation, Server, ServerHandler, Vec3, TICKS_PER_SECOND,
};
use firework_authentication::Profile;
use firework_data::items::{DiamondSword, Elytra, EndRod, Item};
use firework_protocol::{
    client_bound::{CustomSound, IdMapHolder, SoundSource},
    data_types::{
        BossBarAction, BossBarColor, BossBarDivision, EntityEventStatus, InteractAction, ItemNbt,
        SlotInner,
    },
    server_bound::Interact,
};
use firework_protocol_core::VarInt;
use firework_world::World;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde_json::json;
use std::{
    cmp::min,
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
    commands: CommandTree<Self, MiniGameProxy>,
}

pub struct BattlePlayerHandler {
    server: Arc<Server<BattleServerHandler, MiniGameProxy>>,
    _proxy: Arc<MiniGameProxy>,
    start_time: Mutex<Option<Instant>>,
    ticks_since_start: Mutex<u32>,
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
            ticks_since_start: Mutex::new(0),
            server,
            _proxy: proxy,
            recent_packets: Mutex::new(Vec::new()),
            start_time: Mutex::new(None),
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
    async fn on_tick(&self, client: &Client<BattleServerHandler, MiniGameProxy>) {
        let mut ticks_since_start = client.handler.ticks_since_start.lock().await;
        *ticks_since_start += 1;

        if *ticks_since_start % 5 == 0 {
            let player = client.player.read().await;
            if player.health + 1. <= player.max_health {
                client.set_health(player.health + 1.);
            }
        }
    }
    async fn on_on_ground(
        &self,
        client: &Client<BattleServerHandler, MiniGameProxy>,
        on_ground: bool,
    ) -> Result<bool, ConnectionError> {
        client.player.write().await.on_ground = on_ground;
        Ok(on_ground)
    }
    async fn on_interact(
        &self,
        client: &Client<BattleServerHandler, MiniGameProxy>,
        interact: Interact,
    ) -> Result<(), ConnectionError> {
        match interact.action {
            InteractAction::Attack => {
                let player_list = &client.server.player_list;

                // find the player with the index of the entity in the player list

                let Some(other_client) = player_list
                    .iter()
                    .find(|list_entry| list_entry.client_data.entity_id == interact.entity_id.0)
                    else {
                        return Ok(());
                    };

                let distance_to_hitter = other_client.player.read().await.position.clone()
                    - client.player.read().await.position.clone();

                if distance_to_hitter.magnitude() > 4. {
                    println!("Too far away to hit");
                    return Ok(());
                }

                let KNOCKBACK_RESISTANCE = 0.;

                async fn knockback(
                    client: &Client<BattleServerHandler, MiniGameProxy>,
                    kb_amount: f64, // 0.5 for first hit sprint, 0.4 otherwise
                    x_to_hitter: f64,
                    z_to_hitter: f64,
                    kb_resistance: f64,
                ) {
                    let kb_amount = kb_amount * (1. - kb_resistance);
                    if kb_amount > 0. {
                        let current_velocity = client.player.read().await.velocity.clone();
                        let new_force_vec = Vec3::new(x_to_hitter, 0., z_to_hitter).normalize()
                            * Vec3::scalar(kb_amount);
                        let new_velocity = Vec3::new(
                            current_velocity.x / 2. + new_force_vec.x,
                            if client.player.read().await.on_ground {
                                if 0.4 <= current_velocity.y / 2. + kb_amount {
                                    0.4
                                } else {
                                    current_velocity.y / 2. + kb_amount
                                }
                            } else {
                                current_velocity.y
                            },
                            current_velocity.z / 2. + new_force_vec.z,
                        );
                        client.set_velocity(new_velocity);
                    }
                }

                let hurt_yaw = distance_to_hitter.yaw()
                    - other_client.player.read().await.rotation.yaw as f64
                    + 180.;

                // float attack_damage = (float)this.getAttributeValue(Attributes.ATTACK_DAMAGE);
                // float damage_bonus;
                // if (p_36347_ instanceof LivingEntity) { // this is like for smite and bane of arthropods and stuff i think
                //     damage_bonus = EnchantmentHelper.getDamageBonus(this.getMainHandItem(), ((LivingEntity)p_36347_).getMobType());
                // } else {
                //     damage_bonus = EnchantmentHelper.getDamageBonus(this.getMainHandItem(), MobType.UNDEFINED);
                // }

                // float attack_strength_scale = Mth.clamp(((float)this.attackStrengthTicker + 0.5F) * this.getAttributeValue(Attributes.ATTACK_SPEED) / 20.0F, 0.0F, 1.0F);
                // attack_damage *= 0.2F + attack_strength_scale * attack_strength_scale * 0.8F;
                // damage_bonus *= attack_strength_scale;
                // this.resetAttackStrengthTicker();
                // if (attack_damage > 0.0F || damage_bonus > 0.0F) {
                //     boolean is_strong_hit = attack_strength_scale > 0.9F;
                //     boolean is_knockback_hit = false;
                //     int i = 0;
                //     i += EnchantmentHelper.getKnockbackBonus(this);
                //     if (this.isSprinting() && is_strong_hit) {
                //         this.level.playSound((Player)null, this.getX(), this.getY(), this.getZ(), SoundEvents.PLAYER_ATTACK_KNOCKBACK, this.getSoundSource(), 1.0F, 1.0F);
                //         ++i;
                //         is_knockback_hit = true;
                //     }

                //     boolean is_falling = is_strong_hit && this.fallDistance > 0.0F && !this.onGround && !this.onClimbable() && !this.isInWater() && !this.hasEffect(MobEffects.BLINDNESS) && !this.isPassenger() && p_36347_ instanceof LivingEntity;
                //     is_critical_hit = is_falling && !this.isSprinting();
                //     if (is_critical_hit) {
                //         attack_damage *= 1.5F;
                //     }

                //     attack_damage += damage_bonus;

                let mut attack_damage = 7.; // damage of a diamond sword
                let mut damage_bonus = 0.; // damage bonus from enchantments

                let attack_speed = 1.6; // attack speed of a diamond sword
                let ticker = client.player.read().await.attack_strength_ticker.clone();
                let mut attack_strength_scale = (ticker as f64 + 0.5) * attack_speed / 20.;
                if attack_strength_scale < 0. {
                    attack_strength_scale = 0.;
                } else if attack_strength_scale > 1. {
                    attack_strength_scale = 1.;
                }
                attack_damage *= 0.2 + attack_strength_scale * attack_strength_scale * 0.8;
                damage_bonus *= attack_strength_scale;

                if attack_damage > 0. || damage_bonus > 0. {
                    let is_strong_hit = attack_strength_scale > 0.9;
                    let mut is_knockback_hit = false;
                    if client.player.read().await.first_sprinting_hit && is_strong_hit {
                        is_knockback_hit = true;
                        client.player.write().await.first_sprinting_hit = false;
                    }

                    let is_critical_hit = is_strong_hit
                        && client.player.read().await.fall_distance > 0.
                        && !client.player.read().await.on_ground
                        && !client.player.read().await.sprinting;
                    // && !client.player.read().await.on_climbable
                    // && !client.player.read().await.in_water
                    // && !client.player.read().await.has_effect(Effect::Blindness)
                    // && !client.player.read().await.is_passenger
                    // && other_client.player.read().await.is_living();
                    if is_critical_hit {
                        attack_damage *= 1.5;
                    }

                    attack_damage += damage_bonus;

                    let blocked_by_shield = false;

                    if blocked_by_shield {
                        attack_damage = 0.;
                    }

                    // FIXME hitregs can happen if the ticks are off sync just right
                    if other_client.player.read().await.invulnerable_time == 0 {
                        if other_client.player.read().await.health.clone() - attack_damage as f32
                            <= 0.
                        {
                            // "kill" the player
                            other_client
                                .set_health(other_client.player.read().await.max_health.clone());
                            for iter_client in player_list.iter() {
                                if iter_client.client_data.uuid == other_client.client_data.uuid {
                                    continue;
                                }
                                iter_client.kill_player(
                                    other_client.client_data.entity_id,
                                    other_client.client_data.uuid,
                                );
                            }
                        } else {
                            other_client.set_health(
                                other_client.player.read().await.health.clone()
                                    - attack_damage as f32,
                            );

                            knockback(
                                &other_client,
                                0.4,
                                distance_to_hitter.x,
                                distance_to_hitter.z,
                                KNOCKBACK_RESISTANCE,
                            )
                            .await;

                            if is_knockback_hit {
                                knockback(
                                    &other_client,
                                    0.5,
                                    distance_to_hitter.x,
                                    distance_to_hitter.z,
                                    KNOCKBACK_RESISTANCE,
                                )
                                .await;
                            }

                            {
                                let mut other_player = other_client.player.write().await;
                                other_player.last_damage_amount = attack_damage as f32;
                                other_player.invulnerable_time = 10;
                            }

                            for iter_client in player_list.iter() {
                                iter_client.send_hurt_animation(
                                    other_client.client_data.entity_id,
                                    hurt_yaw as f32,
                                );
                                iter_client.send_sound(
                                    IdMapHolder::Direct(CustomSound {
                                        resource_location: if is_critical_hit {
                                            "minecraft:entity.player.attack.crit"
                                        } else if is_knockback_hit {
                                            "minecraft:entity.player.attack.knockback"
                                        } else if is_strong_hit {
                                            "minecraft:entity.player.attack.strong"
                                        } else {
                                            "minecraft:entity.player.attack.weak"
                                        }
                                        .to_string(),
                                        range: Some(16.),
                                    }),
                                    SoundSource::Player,
                                    other_client.player.read().await.position.clone(),
                                    1.,
                                    1.,
                                );
                                iter_client.send_sound(
                                    IdMapHolder::Direct(CustomSound {
                                        resource_location: "minecraft:entity.player.hurt"
                                            .to_string(),
                                        range: Some(16.),
                                    }),
                                    SoundSource::Player,
                                    other_client.player.read().await.position.clone(),
                                    1.,
                                    1.,
                                );
                                iter_client.send_system_chat_message(
                                    json!([
                                        {
                                          "text": "The hit type is ",
                                          "color": "yellow"
                                        },
                                        {
                                          "text": format!("{}", if is_critical_hit {
                                                "minecraft:entity.player.attack.crit"
                                            } else if is_knockback_hit {
                                                "minecraft:entity.player.attack.knockback"
                                            } else if is_strong_hit {
                                                "minecraft:entity.player.attack.strong"
                                            } else {
                                                "minecraft:entity.player.attack.weak"
                                            }),
                                          "color": "green"
                                        },
                                    ])
                                    .to_string(),
                                    false,
                                );
                            }
                        }
                    } else {
                        client.send_sound(
                            IdMapHolder::Direct(CustomSound {
                                resource_location: "minecraft:entity.player.attack.nodamage"
                                    .to_string(),
                                range: Some(16.),
                            }),
                            SoundSource::Player,
                            other_client.player.read().await.position.clone(),
                            1.,
                            1.,
                        );
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }
}

impl BattlePlayerHandler {}

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
                // TODO don't do this on every tick, instead update it when a player leaves or dies
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
            max_health: 20.,
            health: 20.,
            flying_allowed: false,
            gamemode: GameMode::Adventure,
            profile,
            uuid,
            ..Player::default()
        };

        player.inventory.set_slot(
            InventorySlot::Helmet,
            Some(SlotInner {
                item_id: VarInt(EndRod::ID as i32),
                item_count: 1,
                nbt: ItemNbt {
                    ..Default::default()
                },
            }),
        );

        player.inventory.set_slot(
            InventorySlot::Hotbar { slot: 0 },
            Some(SlotInner {
                item_id: VarInt(DiamondSword::ID as i32),
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
    ) -> Result<&CommandTree<Self, MiniGameProxy>, ConnectionError> {
        Ok(&self.commands)
    }
}

impl BattleServerHandler {
    async fn start_game(&self, server: &Server<Self, MiniGameProxy>, _proxy: &MiniGameProxy) {
        for client in server.player_list.iter() {
            {
                let mut player = client.player.write().await;
                server.broadcast_entity_metadata_update(
                    &client,
                    vec![
                        EntityMetadata::EntityFlags(player.entity_flags()),
                        EntityMetadata::EntityPose(Pose::Standing),
                    ],
                    true,
                );
            }
            client.set_health(20.);
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
