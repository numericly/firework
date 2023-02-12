use async_trait::async_trait;
use firework::{
    client::{GameMode, InventorySlot, Player},
    ConnectionError, Server, ServerHandler, Vec3,
};
use firework_authentication::Profile;
use firework_data::items::{Elytra, Item};
use firework_protocol::data_types::{ItemNbt, Slot};
use firework_protocol_core::VarInt;

use crate::MiniGameProxy;

pub struct GlideServerHandler {}

#[async_trait]
impl ServerHandler<MiniGameProxy> for GlideServerHandler {
    fn new() -> Self {
        Self {}
    }
    async fn on_tick(&self, server: &Server<Self, MiniGameProxy>, proxy: &MiniGameProxy) {
        // println!("tick");
    }
    async fn load_player(&self, profile: Profile, uuid: u128) -> Result<Player, ConnectionError> {
        let mut player = Player {
            position: Vec3::new(0.5, 168.0, 0.5),
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
}
