use std::time::Instant;

use std::sync::Arc;

use async_trait::async_trait;
use dashmap::{mapref::entry::Entry, DashMap};
use firework::{
    client::{Client, GameMode, InventorySlot, Player},
    AxisAlignedBB, BlockPos, ConnectionError, PlayerHandler, Rotation, Server, ServerHandler, Vec3,
};

use firework_authentication::Profile;
use firework_data::items::{Elytra, Item};
use firework_protocol::data_types::{ItemNbt, Slot};
use firework_protocol_core::VarInt;
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
}
const CANYON_BOOSTS: [Boost; 3] = [
    Boost {
        area: AxisAlignedBB {
            max: BlockPos {
                x: -39,
                y: 106,
                z: 315,
            },
            min: BlockPos {
                x: -43,
                y: 101,
                z: 306,
            },
        },

        velocity: Vec3::new(0., 0.02, 0.35),
    },
    Boost {
        area: AxisAlignedBB {
            max: BlockPos {
                x: -39,
                y: 105,
                z: 360,
            },
            min: BlockPos {
                x: -43,
                y: 100,
                z: 351,
            },
        },
        velocity: Vec3::new(0., 0.02, 0.35),
    },
    Boost {
        area: AxisAlignedBB {
            max: BlockPos {
                x: 126,
                y: 55,
                z: 615,
            },
            min: BlockPos {
                x: 122,
                y: 50,
                z: 606,
            },
        },
        velocity: Vec3::new(0., 0.05, -0.4),
    },
];

enum BoostStatus {
    Active { percent: f32 },
    Cooldown { start_time: Instant },
}

enum GameState {
    Waiting,
    Running,
}

pub struct GlideServerHandler {
    game_state: RwLock<GameState>,
    damage_grace_period: DashMap<u128, Instant>,
    boost_status: DashMap<u128, BoostStatus>,
}

pub struct GlidePlayerHandler {}

impl PlayerHandler<GlideServerHandler, MiniGameProxy> for GlidePlayerHandler {
    fn new(
        server: Arc<Server<GlideServerHandler, MiniGameProxy>>,
        proxy: Arc<MiniGameProxy>,
    ) -> Self {
        Self {}
    }
}

#[async_trait]
impl ServerHandler<MiniGameProxy> for GlideServerHandler {
    type PlayerHandler = GlidePlayerHandler;
    fn new() -> Self {
        Self {
            game_state: RwLock::new(GameState::Waiting),
            damage_grace_period: DashMap::new(),
            boost_status: DashMap::new(),
        }
    }
    async fn on_tick(&self, server: &Server<Self, MiniGameProxy>, proxy: &MiniGameProxy) {}
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
    async fn on_player_move(
        &self,
        server: &Server<Self, MiniGameProxy>,
        proxy: &MiniGameProxy,
        client: &Client<MiniGameProxy>,
        position: Vec3,
    ) -> Result<Option<Vec3>, ConnectionError> {
        for boost in CANYON_BOOSTS.iter() {
            if boost.area.within(BlockPos::from(position.clone())) {
                if let Some(BoostStatus::Cooldown { start_time }) =
                    self.boost_status.get(&client.client_data.uuid).as_deref()
                {
                    if start_time.elapsed().as_millis() < 1_000 {
                        return Ok(Some(position));
                    }
                }

                client
                    .set_velocity(client.get_velocity().await + boost.velocity.clone())
                    .await?;

                match self.boost_status.entry(client.client_data.uuid) {
                    Entry::Occupied(mut entry) => {
                        let BoostStatus::Active { percent } = entry.get_mut() else {
                            return Ok(Some(position));
                        };
                        if *percent >= 1. {
                            entry.insert(BoostStatus::Cooldown {
                                start_time: Instant::now(),
                            });
                        } else {
                            *percent += 0.167;
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(BoostStatus::Active { percent: 0. });
                    }
                }
                return Ok(Some(position));
            }
        }
        Ok(Some(position))
    }
    async fn on_player_on_ground(
        &self,
        server: &Server<Self, MiniGameProxy>,
        proxy: &MiniGameProxy,
        client: &Client<Self, MiniGameProxy>,
        on_ground: bool,
    ) -> Result<bool, ConnectionError> {
        return Ok(on_ground);
        let player_pos = client.player.read().await.position.clone();

        if SPAWN_AREA.within(BlockPos::from(player_pos)) {
            return Ok(on_ground);
        };

        let grace = self.damage_grace_period.get(&client.client_data.uuid);

        if let Some(grace) = &grace {
            if grace.elapsed().as_millis() < 1_000 {
                return Ok(on_ground);
            }
        }

        drop(grace);

        if on_ground {
            client
                .set_velocity(client.get_velocity().await + Vec3::new(0., 0.5, 0.))
                .await?;
            let health = client.player.read().await.health.clone();
            let new_health = health - 2.;
            if new_health <= 0. {
                server.handle_death(server, proxy, client).await?;
            } else {
                client.set_health(new_health).await?;
            }
            let entry = self.damage_grace_period.entry(client.client_data.uuid);
            match entry {
                dashmap::mapref::entry::Entry::Occupied(mut entry) => {
                    entry.insert(Instant::now());
                }
                dashmap::mapref::entry::Entry::Vacant(entry) => {
                    entry.insert(Instant::now());
                }
            }
        }

        Ok(on_ground)
    }
    async fn on_player_death(
        &self,
        server: &Server<Self, MiniGameProxy>,
        proxy: &MiniGameProxy,
        client: &Client<Self, MiniGameProxy>,
    ) -> Result<bool, ConnectionError> {
        client.sync_position(Vec3::new(0.5, 168.0, 0.5), Rotation { yaw: 0., pitch: 0. });
        Ok(true)
    }
}
