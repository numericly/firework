use async_trait::async_trait;
use firework::client::{Client, ClientCommand, GameMode, InventorySlot, Player};
use firework::{ConnectionError, Rotation, Server, ServerHandler, Vec3};
use firework_authentication::Profile;
use firework_data::items::{Compass, Item};
use firework_protocol::data_types::{ItemNbt, Slot};
use firework_protocol_core::VarInt;

use crate::{MiniGameProxy, TransferData};

pub struct LobbyServerHandler {}

#[async_trait]
impl ServerHandler<MiniGameProxy> for LobbyServerHandler {
    fn new() -> Self {
        Self {}
    }
    async fn load_player(&self, profile: Profile, uuid: u128) -> Result<Player, ConnectionError> {
        let mut player = Player {
            gamemode: GameMode::Adventure,
            max_health: 20.0,
            health: 19.99999999,
            position: Vec3::new(0.5, 46.0, 0.5),
            rotation: Rotation::new(-90., 0.),
            profile,
            uuid,
            ..Player::default()
        };

        player.inventory.set_slot(
            InventorySlot::Hotbar { slot: 0 },
            Some(Slot {
                item_id: VarInt(Compass::ID as i32),
                item_count: 1,
                nbt: ItemNbt {
                    ..Default::default()
                },
            }),
        );
        Ok(player)
    }
    async fn on_chat(
        &self,
        server: &Server<Self, MiniGameProxy>,
        proxy: &MiniGameProxy,
        client: &Client<MiniGameProxy>,
        chat: String,
    ) -> Result<Option<String>, ConnectionError> {
        let name = &client.player.read().await.profile.name;
        client.to_client.send(ClientCommand::Transfer {
            data: TransferData::Glide,
        });
        Ok(Some(format!(r#"{{ "text": "<{}> {}"}}"#, name, chat)))
    }
    async fn on_chat_command(
        &self,
        server: &Server<Self, MiniGameProxy>,
        proxy: &MiniGameProxy,
        client: &Client<MiniGameProxy>,
        command: String,
    ) -> Result<Option<String>, ConnectionError> {
        println!("command: {}", command);
        Ok(None)
    }
}
