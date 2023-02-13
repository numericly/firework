use async_trait::async_trait;
use firework::{
    client::{Client, ClientCommand, GameMode, InventorySlot, Player},
    AxisAlignedBB, BlockPos, ConnectionError, Rotation, Server, ServerHandler, Vec3,
};
use firework_authentication::Profile;
use firework_data::items::{Elytra, Item};
use firework_protocol::data_types::{ItemNbt, Slot};
use firework_protocol_core::VarInt;
use tokio::sync::RwLock;

use crate::MiniGameProxy;

const SPAWN_AREA: AxisAlignedBB = AxisAlignedBB {
    min: BlockPos {
        x: -4,
        y: 166,
        z: -3,
    },
    max: BlockPos { x: 4, y: 168, z: 7 },
};

enum GameState {
    Waiting,
    Running,
}

pub struct GlideServerHandler {
    game_state: RwLock<GameState>,
}

#[async_trait]
impl ServerHandler<MiniGameProxy> for GlideServerHandler {
    fn new() -> Self {
        Self {
            game_state: RwLock::new(GameState::Waiting),
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

        Vec3::new(4.5, 168.0, 7.5);
        Vec3::new(-3.5, 166.0, -2.5);

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
    async fn on_player_on_ground(
        &self,
        server: &Server<Self, MiniGameProxy>,
        proxy: &MiniGameProxy,
        client: &Client<MiniGameProxy>,
        on_ground: bool,
    ) -> Result<bool, ConnectionError> {
        // return Ok(on_ground);
        let player_pos = client.player.read().await.position.clone();
        if !SPAWN_AREA.within(BlockPos::from(player_pos)) && on_ground {
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
        }

        Ok(on_ground)
    }
    async fn on_player_death(
        &self,
        server: &Server<Self, MiniGameProxy>,
        proxy: &MiniGameProxy,
        client: &Client<MiniGameProxy>,
    ) -> Result<bool, ConnectionError> {
        client.to_client.send(ClientCommand::SyncPosition {
            position: Vec3::new(0.5, 168.0, 0.5),
            rotation: Rotation { yaw: 0., pitch: 0. },
        });
        Ok(true)
    }
}
