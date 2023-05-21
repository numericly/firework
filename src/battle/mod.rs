use crate::{
    MiniGameProxy, TransferData, CAVERN_BATTLE_WORLD, COVE_BATTLE_WORLD, CRUCIBLE_BATTLE_WORLD,
};
use async_trait::async_trait;
use firework::client::DamageType;
use firework::gui::WindowType;
use firework::protocol::data_types::{Enchantment, Equipment, EquipmentEntry, EquipmentSlot};
use firework::{
    authentication::Profile, data::items::Item, gui::GUIInit,
    protocol::data_types::InventoryOperationMode,
};
use firework::{
    client::{Client, GameMode, InventorySlot, Player},
    commands::{Argument, Command, CommandTree},
    entities::{EntityMetadata, Pose},
    ConnectionError, PlayerHandler, Rotation, Server, ServerHandler, Vec3, TICKS_PER_SECOND,
};
use firework::{
    gui::GUIEvent,
    protocol::{
        client_bound::{CustomSound, IdMapHolder, SoundSource},
        data_types::{
            BossBarAction, BossBarColor, BossBarDivision, InteractAction, ItemNbt, StackContents,
        },
        server_bound::{ClickContainer, Interact},
    },
};
use firework::{gui::GuiScreen, protocol::core::Position};
use firework::{protocol::data_types::ItemStack, world::World};
use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde_json::json;
use std::collections::HashMap;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{broadcast, Mutex, RwLock};

mod cavern;

pub struct SpawnPoint {
    position: Vec3,
    rotation: Rotation,
}
pub enum Maps {
    Cove,
    Cavern,
    Crucible,
}

impl Distribution<Maps> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Maps {
        match rng.gen_range(0..=2) {
            _ => Maps::Cavern,
            0 => Maps::Cove,
            1 => Maps::Cavern,
            2 => Maps::Crucible,
            _ => panic!("Invalid random number huh? (https://www.youtube.com/watch?v=4kEO7VjKRB8)"),
        }
    }
}

impl Maps {
    pub fn get_world(&self) -> &'static World {
        match self {
            Maps::Cove => &COVE_BATTLE_WORLD,
            Maps::Cavern => &CAVERN_BATTLE_WORLD,
            Maps::Crucible => &CRUCIBLE_BATTLE_WORLD,
        }
    }
    pub fn get_spawn_point(&self) -> &SpawnPoint {
        match self {
            Maps::Cove => &cavern::SPAWN_POINTS[0],
            Maps::Cavern => &cavern::SPAWN_POINTS[0],
            Maps::Crucible => &cavern::SPAWN_POINTS[0],
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
    chests: HashMap<Position, Arc<Chest>>,
}

pub struct BattlePlayerHandler {
    server: Arc<Server<BattleServerHandler, MiniGameProxy>>,
    _proxy: Arc<MiniGameProxy>,
    start_time: Mutex<Option<Instant>>,
    ticks_since_start: Mutex<u32>,
    recent_packets: Mutex<Vec<Instant>>,
    equipment: Mutex<EquipmentStorage>,
}

struct EquipmentStorage {
    main_hand: ItemStack,
    off_hand: ItemStack,
    head: ItemStack,
    chest: ItemStack,
    legs: ItemStack,
    feet: ItemStack,
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
            server: server,
            _proxy: proxy,
            recent_packets: Mutex::new(Vec::new()),
            start_time: Mutex::new(None),
            equipment: Mutex::new(EquipmentStorage {
                main_hand: None,
                off_hand: None,
                head: None,
                chest: None,
                legs: None,
                feet: None,
            }),
        }
    }
    async fn on_use_item(
        &self,
        client: &Client<BattleServerHandler, MiniGameProxy>,
        item: ItemStack,
        slot_id: InventorySlot,
        location: Option<Position>,
    ) -> Result<(), ConnectionError> {
        if let Some(location) = location {
            let chest = client.server.handler.chests.get(&location);

            if let Some(chest) = chest {
                client.display_gui(chest.clone()).await;
            }
        }
        Ok(())
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
            client.sync_position(start.clone(), None).await;
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
                match *self.server.handler.game_state.lock().await {
                    GameState::Running { start_time, .. } => {
                        if start_time.elapsed().as_secs() < 15 {
                            return Ok(());
                        }
                    }
                    _ => {}
                }
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

                let knockback_resistance = 0.;

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

                let mut attack_damage = 1.;
                let mut attack_speed = 4.;
                let mut damage_bonus = 0.; // damage bonus from enchantments
                let ticker;
                {
                    let player = client.player.read().await;
                    if let Some(held_item) = player.inventory.get_slot(&InventorySlot::Hotbar {
                        slot: player.selected_slot as usize,
                    }) {
                        let id = held_item.id;
                        if let Some(damage) = id.get_attack_damage() {
                            attack_damage = damage;
                        }
                        if let Some(speed) = id.get_attack_speed() {
                            attack_speed = speed;
                        }
                    }
                    ticker = player.attack_strength_ticker;
                }
                let mut attack_strength_scale = (ticker as f32 + 0.5) * attack_speed / 20.;
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
                        if other_client.player.read().await.health - attack_damage as f32 <= 0. {
                            // "kill" the player
                            other_client.set_health(other_client.player.read().await.max_health);
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
                            let mut defense = 0;
                            let mut toughness = 0;

                            {
                                let mut player_lock = other_client.player.write().await;

                                if let Some(helmet) =
                                    player_lock.inventory.get_slot_mut(&InventorySlot::Helmet)
                                {
                                    defense += helmet.id.get_armor_defense().unwrap_or(0);
                                    toughness += helmet.id.get_armor_toughness().unwrap_or(0);
                                };
                                if let Some(chestplate) = player_lock
                                    .inventory
                                    .get_slot_mut(&InventorySlot::Chestplate)
                                {
                                    defense += chestplate.id.get_armor_defense().unwrap_or(0);
                                    toughness += chestplate.id.get_armor_toughness().unwrap_or(0);
                                };
                                if let Some(leggings) =
                                    player_lock.inventory.get_slot_mut(&InventorySlot::Leggings)
                                {
                                    defense += leggings.id.get_armor_defense().unwrap_or(0);
                                    toughness += leggings.id.get_armor_toughness().unwrap_or(0);
                                };
                                if let Some(boots) =
                                    player_lock.inventory.get_slot_mut(&InventorySlot::Boots)
                                {
                                    defense += boots.id.get_armor_defense().unwrap_or(0);
                                    toughness += boots.id.get_armor_toughness().unwrap_or(0);
                                };
                            }

                            dbg!(defense, toughness);

                            let f = 2. + toughness as f32 / 4.;
                            let f1 = (defense as f32 - attack_damage / f)
                                .max(defense as f32 * 0.2)
                                .min(20.);
                            attack_damage *= 1. - f1 / 25.;

                            let protection = 0.;
                            attack_damage *= 1. - protection / 25.;

                            let get_absorption_amount = 0.;
                            let damage_after_absorption =
                                (attack_damage - get_absorption_amount).max(0.);
                            let _new_absorption_amount =
                                get_absorption_amount - attack_damage + damage_after_absorption;
                            other_client.set_health(
                                other_client.player.read().await.health - attack_damage as f32,
                            );

                            knockback(
                                &other_client,
                                0.4,
                                distance_to_hitter.x,
                                distance_to_hitter.z,
                                knockback_resistance,
                            )
                            .await;

                            if is_knockback_hit {
                                knockback(
                                    &other_client,
                                    0.5,
                                    distance_to_hitter.x,
                                    distance_to_hitter.z,
                                    knockback_resistance,
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
    async fn on_click_container(
        &self,
        client: &Client<BattleServerHandler, MiniGameProxy>,
        click: ClickContainer,
    ) -> Result<(), ConnectionError> {
        // println!("click: {:?}", click);

        let action = InventoryAction::from_slot_click(click.mode, click.slot, click.button as u8);

        let Some(action) = action else {
            return Ok(());
        };

        let mut player = client.player.write().await;

        match action {
            InventoryAction::Click { slot, button } => {
                let slot = InventorySlot::from_value(slot).unwrap();
                if let Some(held_item) = player.inventory.held_item.take() {
                    let item = player.inventory.get_slot_mut(&slot);
                    match item {
                        Some(item_content) => {
                            let remainder = item_content.stack_item(&held_item);
                            // self.set_index(&items, slot, client.client_data.uuid).await;

                            player.inventory.held_item = remainder;
                        }
                        None => {
                            *item = Some(held_item);
                            // self.set_index(&items, slot, client.client_data.uuid).await;
                        }
                    }
                } else {
                    let item = player.inventory.get_slot_mut(&slot);
                    if let Some(item_content) = item.take() {
                        player.inventory.held_item = Some(item_content);
                        // self.set_index(&items, slot, client.client_data.uuid).await;
                    }
                }
            }
            InventoryAction::Drag { slot, button } => {
                let slot = InventorySlot::from_value(slot).unwrap();
                if let Some(held_item) = player.inventory.held_item.take() {
                    let item = player.inventory.get_slot_mut(&slot);

                    if let Some(item) = item {
                        let remainder = item.stack_item(&held_item);

                        // self.set_index(&items, slot, client.client_data.uuid).await;

                        player.inventory.held_item = remainder;
                    } else {
                        *item = Some(held_item);
                        // self.set_index(&items, slot, client.client_data.uuid).await;
                    }
                }
            }
            InventoryAction::ShiftClick { slot } => {
                let slot = InventorySlot::from_value(slot).unwrap();

                let stacking_item = player.inventory.get_slot_mut(&slot).take();

                let Some(mut stacking_item) = stacking_item else {
                    return Ok(());
                };

                if let InventorySlot::MainInventory { .. } = slot {
                    for already_stacked in 0..2 {
                        for i in 0..9 {
                            if let Some(hot_bar_item) = player
                                .inventory
                                .get_slot_mut(&InventorySlot::Hotbar { slot: i })
                                .as_mut()
                            {
                                let remainder = hot_bar_item.stack_item(&stacking_item);
                                if let Some(item) = remainder {
                                    stacking_item = item;
                                } else {
                                    // self.set_index(&items, slot, client.client_data.uuid).await;
                                    return Ok(());
                                }
                            } else if already_stacked == 1 {
                                // self.set_index(&items, slot, client.client_data.uuid).await;
                                player
                                    .inventory
                                    .get_slot_mut(&InventorySlot::Hotbar { slot: i })
                                    .replace(stacking_item);
                                return Ok(());
                            }
                        }
                    }

                    player.inventory.get_slot_mut(&slot).replace(stacking_item);

                    // self.set_index(&items, slot, client.client_data.uuid).await;
                } else {
                    for already_stacked in 0..2 {
                        for i in 0..27 {
                            if let Some(inventory_item) = player
                                .inventory
                                .get_slot_mut(&InventorySlot::MainInventory { slot: i })
                                .as_mut()
                            {
                                let remainder = inventory_item.stack_item(&stacking_item);
                                if let Some(item) = remainder {
                                    stacking_item = item;
                                } else {
                                    // self.set_index(&items, slot, client.client_data.uuid).await;
                                    return Ok(());
                                }
                            } else if already_stacked == 1 {
                                // self.set_index(&items, slot, client.client_data.uuid).await;
                                player
                                    .inventory
                                    .get_slot_mut(&InventorySlot::MainInventory { slot: i })
                                    .replace(stacking_item);
                                return Ok(());
                            }
                        }
                    }

                    player.inventory.get_slot_mut(&slot).replace(stacking_item);
                }
            }
            InventoryAction::SwapItem { from, to } => {
                let from_slot = InventorySlot::from_value(from).unwrap();

                let from_item = player.inventory.get_slot_mut(&from_slot).take();
                let to_item = player.inventory.get_slot_mut(&to).take();

                *player.inventory.get_slot_mut(&to) = from_item;
                *player.inventory.get_slot_mut(&from_slot) = to_item;
            }
            _ => {
                println!("action: {:?}", action);
            }
        }

        // update the held item, offhand, and armor slots

        let mut equipment = self.equipment.lock().await;
        let mut equipment_diff = Vec::new();

        if player.inventory.get_slot(&InventorySlot::Hotbar {
            slot: player.selected_slot as usize,
        }) != &equipment.main_hand
        {
            equipment_diff.push(Equipment {
                equipment: vec![EquipmentEntry {
                    slot: EquipmentSlot::MainHand,
                    item: player
                        .inventory
                        .get_slot(&InventorySlot::Hotbar {
                            slot: player.selected_slot as usize,
                        })
                        .clone(),
                }],
            });
        }
        if player.inventory.get_slot(&InventorySlot::Offhand) != &equipment.off_hand {
            equipment_diff.push(Equipment {
                equipment: vec![EquipmentEntry {
                    slot: EquipmentSlot::OffHand,
                    item: player.inventory.get_slot(&InventorySlot::Offhand).clone(),
                }],
            });
        }
        if player.inventory.get_slot(&InventorySlot::Helmet) != &equipment.head {
            equipment_diff.push(Equipment {
                equipment: vec![EquipmentEntry {
                    slot: EquipmentSlot::Helmet,
                    item: player.inventory.get_slot(&InventorySlot::Helmet).clone(),
                }],
            });
        }
        if player.inventory.get_slot(&InventorySlot::Chestplate) != &equipment.chest {
            equipment_diff.push(Equipment {
                equipment: vec![EquipmentEntry {
                    slot: EquipmentSlot::Chestplate,
                    item: player
                        .inventory
                        .get_slot(&InventorySlot::Chestplate)
                        .clone(),
                }],
            });
        }
        if player.inventory.get_slot(&InventorySlot::Leggings) != &equipment.legs {
            equipment_diff.push(Equipment {
                equipment: vec![EquipmentEntry {
                    slot: EquipmentSlot::Leggings,
                    item: player.inventory.get_slot(&InventorySlot::Leggings).clone(),
                }],
            });
        }
        if player.inventory.get_slot(&InventorySlot::Boots) != &equipment.feet {
            equipment_diff.push(Equipment {
                equipment: vec![EquipmentEntry {
                    slot: EquipmentSlot::Boots,
                    item: player.inventory.get_slot(&InventorySlot::Boots).clone(),
                }],
            });
        }

        dbg!(&equipment_diff);

        Ok(())
    }
}

impl BattlePlayerHandler {}

pub struct Chest {
    pub items: RwLock<Vec<ItemStack>>,
    pub channel: broadcast::Sender<GUIEvent>,
    state_id: RwLock<i32>,
}

#[derive(Debug, Clone)]
enum Button {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone)]
enum InventoryAction {
    Click { slot: usize, button: Button },
    ClickDrop { stack: bool },
    ShiftClick { slot: usize },
    SwapItem { from: usize, to: InventorySlot },
    DropItem { slot: usize, stack: bool },
    DragStart { button: Button },
    Drag { slot: usize, button: Button },
    DragEnd { button: Button },
    DoubleClick { slot: usize },
}

impl InventoryAction {
    pub fn from_slot_click_gui(
        container_size: usize,
        mode: InventoryOperationMode,
        slot: i16,
        button: u8,
    ) -> Option<Self> {
        match (mode, slot, button) {
            (InventoryOperationMode::Click, slot, 0 | 1)
                if slot >= 0 && slot < container_size as i16 + 36 =>
            {
                Some(InventoryAction::Click {
                    slot: slot as usize,
                    button: if button == 0 {
                        Button::Left
                    } else {
                        Button::Right
                    },
                })
            }
            (InventoryOperationMode::Click, -999 | -1, 0 | 1) => {
                Some(InventoryAction::ClickDrop { stack: button == 0 })
            }
            (InventoryOperationMode::ShiftClick, slot, 0 | 1)
                if slot >= 0 && slot < container_size as i16 + 36 =>
            {
                Some(InventoryAction::ShiftClick {
                    slot: slot as usize,
                })
            }
            (InventoryOperationMode::MiddleClick, slot, 2)
                if slot >= 0 && slot < container_size as i16 + 36 =>
            {
                Some(InventoryAction::Click {
                    slot: slot as usize,
                    button: Button::Middle,
                })
            }
            (InventoryOperationMode::NumberKey, slot, button)
                if button <= 8 && slot >= 0 && slot < container_size as i16 + 36 =>
            {
                Some(InventoryAction::SwapItem {
                    from: slot as usize,
                    to: InventorySlot::Hotbar {
                        slot: button as usize,
                    },
                })
            }
            (InventoryOperationMode::NumberKey, slot, 40)
                if slot >= 0 && slot < container_size as i16 + 36 =>
            {
                Some(InventoryAction::SwapItem {
                    from: slot as usize,
                    to: InventorySlot::Offhand,
                })
            }
            (InventoryOperationMode::Drop, slot, button)
                if (button == 0 || button == 1)
                    && slot >= 0
                    && slot < container_size as i16 + 36 =>
            {
                Some(match button {
                    0 => InventoryAction::DropItem {
                        slot: slot as usize,
                        stack: false,
                    },
                    1 => InventoryAction::DropItem {
                        slot: slot as usize,
                        stack: true,
                    },
                    _ => unreachable!(),
                })
            }
            (InventoryOperationMode::Drop, -999, 0) => {
                Some(InventoryAction::ClickDrop { stack: false })
            }
            (InventoryOperationMode::Drop, -999, 1) => {
                Some(InventoryAction::ClickDrop { stack: true })
            }
            (InventoryOperationMode::Dragging, -999, 0) => Some(InventoryAction::DragStart {
                button: Button::Left,
            }),
            (InventoryOperationMode::Dragging, -999, 4) => Some(InventoryAction::DragStart {
                button: Button::Right,
            }),
            (InventoryOperationMode::Dragging, -999, 8) => Some(InventoryAction::DragStart {
                button: Button::Middle,
            }),
            (InventoryOperationMode::Dragging, slot, button)
                if (button == 1 || button == 5 || button == 9)
                    && slot >= 0
                    && slot < container_size as i16 + 36 =>
            {
                Some(InventoryAction::Drag {
                    button: match button {
                        1 => Button::Left,
                        5 => Button::Right,
                        9 => Button::Middle,
                        _ => unreachable!(),
                    },
                    slot: slot as usize,
                })
            }
            (InventoryOperationMode::Dragging, -999, 2) => Some(InventoryAction::DragEnd {
                button: Button::Left,
            }),
            (InventoryOperationMode::Dragging, -999, 6) => Some(InventoryAction::DragEnd {
                button: Button::Right,
            }),
            (InventoryOperationMode::Dragging, -999, 10) => Some(InventoryAction::DragEnd {
                button: Button::Middle,
            }),
            (InventoryOperationMode::DoubleClick, slot, 0)
                if slot >= 0 && slot < container_size as i16 + 36 =>
            {
                Some(InventoryAction::DoubleClick {
                    slot: slot as usize,
                })
            }
            (mode, slot, button) => {
                println!("unknown {:?}, {}, {}", mode, slot, button);
                None
            }
        }
    }
    pub fn from_slot_click(mode: InventoryOperationMode, slot: i16, button: u8) -> Option<Self> {
        let slot_parsed = InventorySlot::from_value(slot as usize);
        match (mode, slot, button, slot_parsed) {
            (InventoryOperationMode::Click, slot, 0 | 1, Some(_)) => Some(InventoryAction::Click {
                slot: slot as usize,
                button: if button == 0 {
                    Button::Left
                } else {
                    Button::Right
                },
            }),
            (InventoryOperationMode::Click, -999 | -1, 0 | 1, _) => {
                Some(InventoryAction::ClickDrop { stack: button == 0 })
            }
            (InventoryOperationMode::ShiftClick, slot, 0 | 1, Some(_)) => {
                Some(InventoryAction::ShiftClick {
                    slot: slot as usize,
                })
            }
            (InventoryOperationMode::MiddleClick, slot, 2, Some(_)) => {
                Some(InventoryAction::Click {
                    slot: slot as usize,
                    button: Button::Middle,
                })
            }
            (InventoryOperationMode::NumberKey, slot, button, Some(_))
                if button <= 8 && slot >= 0 =>
            {
                Some(InventoryAction::SwapItem {
                    from: slot as usize,
                    to: InventorySlot::Hotbar {
                        slot: button as usize,
                    },
                })
            }
            (InventoryOperationMode::NumberKey, slot, 40, Some(_)) => {
                Some(InventoryAction::SwapItem {
                    from: slot as usize,
                    to: InventorySlot::Offhand,
                })
            }
            (InventoryOperationMode::Drop, slot, 0 | 1, Some(_)) => Some(match button {
                0 => InventoryAction::DropItem {
                    slot: slot as usize,
                    stack: false,
                },
                1 => InventoryAction::DropItem {
                    slot: slot as usize,
                    stack: true,
                },
                _ => unreachable!(),
            }),
            (InventoryOperationMode::Drop, -999, 0, _) => {
                Some(InventoryAction::ClickDrop { stack: false })
            }
            (InventoryOperationMode::Drop, -999, 1, _) => {
                Some(InventoryAction::ClickDrop { stack: true })
            }
            (InventoryOperationMode::Dragging, -999, 0, _) => Some(InventoryAction::DragStart {
                button: Button::Left,
            }),
            (InventoryOperationMode::Dragging, -999, 4, _) => Some(InventoryAction::DragStart {
                button: Button::Right,
            }),
            (InventoryOperationMode::Dragging, -999, 8, _) => Some(InventoryAction::DragStart {
                button: Button::Middle,
            }),
            (InventoryOperationMode::Dragging, slot, 1 | 5 | 9, Some(_)) => {
                Some(InventoryAction::Drag {
                    button: match button {
                        1 => Button::Left,
                        5 => Button::Right,
                        9 => Button::Middle,
                        _ => unreachable!(),
                    },
                    slot: slot as usize,
                })
            }
            (InventoryOperationMode::Dragging, -999, 2, _) => Some(InventoryAction::DragEnd {
                button: Button::Left,
            }),
            (InventoryOperationMode::Dragging, -999, 6, _) => Some(InventoryAction::DragEnd {
                button: Button::Right,
            }),
            (InventoryOperationMode::Dragging, -999, 10, _) => Some(InventoryAction::DragEnd {
                button: Button::Middle,
            }),
            (InventoryOperationMode::DoubleClick, slot, 0, Some(_)) => {
                Some(InventoryAction::DoubleClick {
                    slot: slot as usize,
                })
            }
            (mode, slot, button, slot_parsed) => {
                println!(
                    "unknown {:?}, {}, {}, {:?}",
                    mode, slot, button, slot_parsed
                );
                None
            }
        }
    }
}

#[async_trait]
impl GuiScreen<BattleServerHandler, MiniGameProxy> for Chest {
    async fn init(&self, _client: &Client<BattleServerHandler, MiniGameProxy>) -> GUIInit {
        GUIInit {
            title: r#"{"text":"Chest"}"#.to_string(),
            window_type: WindowType::Generic9x3,
            items: self.items.read().await.clone(),
            receiver: self.channel.subscribe(),
        }
    }
    async fn handle_click(
        &self,
        slot: ClickContainer,
        client: &Client<BattleServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        let action =
            InventoryAction::from_slot_click_gui(9 * 3, slot.mode, slot.slot, slot.button as u8);

        let Some(action) = action else {
            // None = unknown action
            return Ok(());
        };

        println!("action: {:?}, state {}", action, slot.state_id.0);

        let mut player = client.player.write().await;
        let mut items = self.items.write().await;

        match action {
            InventoryAction::SwapItem { from, to } => {
                let from_item = self.index_mut(&mut items, from, &mut player).await.take();

                let Some(from_item) = from_item else {
                    *self.index_mut(&mut items, from, &mut player).await = player.inventory.get_slot_mut(&to).take();

                    if from < 27 {
                        self.set_index(&items, from, client.client_data.uuid).await;
                    }
                    return Ok(());
                };

                let to_item = player.inventory.get_slot_mut(&to);

                let old_to_item = to_item.replace(from_item);

                *self.index_mut(&mut items, from, &mut player).await = old_to_item;

                if from < 27 {
                    self.set_index(&items, from, client.client_data.uuid).await;
                }
            }
            InventoryAction::Click { slot, button } => {
                if let Some(held_item) = player.inventory.held_item.take() {
                    let item = self.index_mut(&mut items, slot, &mut player).await;
                    match item {
                        Some(item_content) => {
                            let remainder = item_content.stack_item(&held_item);
                            self.set_index(&items, slot, client.client_data.uuid).await;

                            player.inventory.held_item = remainder;
                        }
                        None => {
                            *item = Some(held_item);
                            self.set_index(&items, slot, client.client_data.uuid).await;
                        }
                    }
                } else {
                    let item = self.index_mut(&mut items, slot, &mut player).await;
                    if let Some(item_content) = item.take() {
                        player.inventory.held_item = Some(item_content);
                        self.set_index(&items, slot, client.client_data.uuid).await;
                    }
                }
            }
            InventoryAction::Drag { slot, button } => {
                if let Some(held_item) = player.inventory.held_item.take() {
                    let item = self.index_mut(&mut items, slot, &mut player).await;

                    if let Some(item) = item {
                        let remainder = item.stack_item(&held_item);

                        self.set_index(&items, slot, client.client_data.uuid).await;

                        player.inventory.held_item = remainder;
                    } else {
                        *item = Some(held_item);
                        self.set_index(&items, slot, client.client_data.uuid).await;
                    }
                }
            }
            InventoryAction::ShiftClick { slot } => {
                if slot < 27 {
                    let stacking_item = items[slot].take();

                    let Some(mut stacking_item) = stacking_item else {
                        return Ok(());
                    };

                    for i in (0..9).rev() {
                        if let Some(hot_bar_item) = player
                            .inventory
                            .get_slot_mut(&InventorySlot::Hotbar { slot: i })
                            .as_mut()
                        {
                            let remainder = hot_bar_item.stack_item(&stacking_item);
                            if let Some(item) = remainder {
                                stacking_item = item;
                            } else {
                                self.set_index(&items, slot, client.client_data.uuid).await;
                                return Ok(());
                            }
                        } else {
                            self.set_index(&items, slot, client.client_data.uuid).await;
                            player
                                .inventory
                                .get_slot_mut(&InventorySlot::Hotbar { slot: i })
                                .replace(stacking_item);
                            return Ok(());
                        }
                    }

                    for i in 0..27 {
                        if let Some(hot_bar_item) = player
                            .inventory
                            .get_slot_mut(&InventorySlot::MainInventory { slot: i })
                            .as_mut()
                        {
                            let remainder = hot_bar_item.stack_item(&stacking_item);
                            if let Some(item) = remainder {
                                stacking_item = item;
                            } else {
                                self.set_index(&items, slot, client.client_data.uuid).await;
                                return Ok(());
                            }
                        } else {
                            self.set_index(&items, slot, client.client_data.uuid).await;
                            player
                                .inventory
                                .get_slot_mut(&InventorySlot::MainInventory { slot: i })
                                .replace(stacking_item);
                            return Ok(());
                        }
                    }

                    items[slot] = Some(stacking_item);

                    self.set_index(&items, slot, client.client_data.uuid).await;
                } else {
                    let item = if slot < 27 + 27 {
                        player
                            .inventory
                            .get_slot_mut(&InventorySlot::MainInventory { slot: slot - 27 })
                    } else {
                        player.inventory.get_slot_mut(&InventorySlot::Hotbar {
                            slot: slot - 27 - 27,
                        })
                    };
                    let stacking_item = item.take();

                    let Some(mut stacking_item) = stacking_item else {
                            return Ok(());
                        };

                    for i in 0..27 {
                        if let Some(hot_bar_item) = &mut items[i] {
                            let remainder = hot_bar_item.stack_item(&stacking_item);

                            self.set_index(&items, i, client.client_data.uuid).await;

                            if let Some(item) = remainder {
                                stacking_item = item;
                            } else {
                                return Ok(());
                            }
                        } else {
                            items[i] = Some(stacking_item);

                            self.set_index(&items, i, client.client_data.uuid).await;

                            return Ok(());
                        }
                    }
                }
            }
            InventoryAction::ClickDrop { stack } => {
                client.update_slot(-1, player.inventory.held_item.clone(), -1);
            }
            action => {
                // println!("unhandled action: {:?}", action);
            }
        }

        let mut valid_state = true;
        for item_stack in slot.slots {
            let slot_idx = item_stack.slot_number as usize;
            let item = self.index_mut(&mut items, slot_idx, &mut player).await;

            if *item != item_stack.slot_value {
                valid_state = false;
                client.update_slot(slot_idx as i16, item.clone(), 1);
            }
        }

        if !valid_state {
            client.update_slot(-1, player.inventory.held_item.clone(), -1);
            return Ok(());
        }

        // println!("player {:?}", player.inventory);
        // println!("chest {:?}", items);

        Ok(())
    }
}

impl Chest {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1024);
        Self {
            state_id: RwLock::new(0),
            channel: sender,
            items: RwLock::new(vec![None; 27]),
        }
    }
    async fn index_mut<'a>(
        &self,
        items: &'a mut Vec<ItemStack>,
        index: usize,
        player: &'a mut Player,
    ) -> &'a mut ItemStack {
        if index < WindowType::Generic9x3.len() {
            &mut items[index]
        } else {
            assert!((index - WindowType::Generic9x3.len()) < 27 + 9);
            let inventory_index = index - WindowType::Generic9x3.len();

            if inventory_index < 27 {
                player
                    .inventory
                    .get_slot_mut(&InventorySlot::MainInventory {
                        slot: inventory_index,
                    })
            } else {
                player.inventory.get_slot_mut(&InventorySlot::Hotbar {
                    slot: inventory_index - 27,
                })
            }
        }
    }
    async fn set_index(&self, items: &Vec<ItemStack>, index: usize, uuid: u128) {
        if index < WindowType::Generic9x3.len() {
            let mut state_id = self.state_id.write().await;
            *state_id += 1;
            self.channel
                .send(GUIEvent::SetSlot {
                    slot: index,
                    item: items[index].clone(),
                    state_id: *state_id,
                    setter: uuid,
                })
                .unwrap();
        }
    }
}

#[async_trait]
impl ServerHandler<MiniGameProxy> for BattleServerHandler {
    type ServerGUI = Chest;
    type PlayerHandler = BattlePlayerHandler;
    fn new() -> Self {
        Self {
            chests: {
                let mut map = HashMap::new();
                map.insert(Position { x: 1, y: 99, z: 1 }, Arc::new(Chest::new()));
                map.insert(Position { x: 1, y: 99, z: -1 }, Arc::new(Chest::new()));
                map.insert(
                    Position {
                        x: -1,
                        y: 99,
                        z: -1,
                    },
                    Arc::new(Chest::new()),
                );
                map.insert(Position { x: -1, y: 99, z: 1 }, Arc::new(Chest::new()));
                map.insert(
                    Position {
                        x: -28,
                        y: 110,
                        z: 17,
                    },
                    Arc::new(Chest::new()),
                );
                map.insert(
                    Position {
                        x: 0,
                        y: 108,
                        z: 36,
                    },
                    Arc::new(Chest::new()),
                );
                map.insert(
                    Position {
                        x: 8,
                        y: 117,
                        z: 28,
                    },
                    Arc::new(Chest::new()),
                );
                map.insert(
                    Position {
                        x: 29,
                        y: 115,
                        z: 1,
                    },
                    Arc::new(Chest::new()),
                );
                map.insert(
                    Position {
                        x: 24,
                        y: 109,
                        z: -20,
                    },
                    Arc::new(Chest::new()),
                );
                map.insert(
                    Position {
                        x: 0,
                        y: 108,
                        z: -30,
                    },
                    Arc::new(Chest::new()),
                );
                map.insert(
                    Position {
                        x: -20,
                        y: 111,
                        z: -15,
                    },
                    Arc::new(Chest::new()),
                );
                map.insert(
                    Position {
                        x: 12,
                        y: 116,
                        z: 0,
                    },
                    Arc::new(Chest::new()),
                );
                map
            },
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
                if start_time.elapsed().as_secs() >= 15 {
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
                                health: player_count as f32 / *initial_player_count as f32,
                            },
                        );
                    }
                } else {
                    // TODO don't do this on every tick, instead update it when a player leaves or dies
                    let player_count = server.player_list.len();

                    for player in server.player_list.iter() {
                        player.send_boss_bar_action(
                        0,
                        BossBarAction::UpdateTitle {
                            title: json!({
                                "text": format!("{} seconds left in grace period", 15 - start_time.elapsed().as_secs()),
                            })
                            .to_string(),
                        },
                    );
                        player.send_boss_bar_action(
                            0,
                            BossBarAction::UpdateHealth {
                                health: 1.0 - start_time.elapsed().as_millis() as f32 / 15000.,
                            },
                        );
                    }
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
            Some(StackContents {
                id: Item::EndRod,
                count: 1,
                nbt: ItemNbt {
                    ..Default::default()
                },
            }),
        );

        player.inventory.set_slot(
            InventorySlot::Hotbar { slot: 0 },
            Some(StackContents {
                id: Item::DiamondSword,
                count: 1,
                nbt: ItemNbt {
                    ..Default::default()
                },
            }),
        );
        player.inventory.set_slot(
            InventorySlot::Hotbar { slot: 1 },
            Some(StackContents {
                id: Item::GoldenSword,
                count: 1,
                nbt: ItemNbt {
                    ..Default::default()
                },
            }),
        );
        player.inventory.set_slot(
            InventorySlot::Hotbar { slot: 2 },
            Some(StackContents {
                id: Item::StoneAxe,
                count: 1,
                nbt: ItemNbt {
                    ..Default::default()
                },
            }),
        );
        player.inventory.set_slot(
            InventorySlot::Hotbar { slot: 3 },
            Some(StackContents {
                id: Item::DiamondHelmet,
                count: 1,
                nbt: ItemNbt {
                    ..Default::default()
                },
            }),
        );
        player.inventory.set_slot(
            InventorySlot::Hotbar { slot: 4 },
            Some(StackContents {
                id: Item::DiamondChestplate,
                count: 1,
                nbt: ItemNbt {
                    ..Default::default()
                },
            }),
        );
        player.inventory.set_slot(
            InventorySlot::Hotbar { slot: 5 },
            Some(StackContents {
                id: Item::DiamondLeggings,
                count: 1,
                nbt: ItemNbt {
                    enchantments: Some(Vec::from([Enchantment {
                        id: "minecraft:protection".to_string(),
                        level: 4,
                    }])),
                    display: None,
                },
            }),
        );
        player.inventory.set_slot(
            InventorySlot::Hotbar { slot: 6 },
            Some(StackContents {
                id: Item::DiamondBoots,
                count: 1,
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
                let player = client.player.write().await;
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
            client
                .sync_position(
                    self.map.get_spawn_point().position.clone(),
                    Some(Rotation::new(0., 0.)),
                )
                .await;

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
