use async_trait::async_trait;
use authentication::Profile;
use client::{Client, ClientCommand, GameMode, InventorySlot, Player};
use minecraft_data::items::{Compass, Elytra, Item, Items};
use protocol::{
    data_types::{ItemNbt, Slot},
    Protocol,
};
use protocol_core::VarInt;
use server::{
    ClientData, ConnectionError, Server, ServerHandler, ServerManager, ServerProxy, Vec3,
};
use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::sleep};
use world::World;

mod client;
// mod commands;
mod entities;
mod gui;
mod server;

struct LobbyServerHandler {}

#[async_trait]
impl ServerHandler<MiniGameProxy> for LobbyServerHandler {
    fn new() -> Self {
        Self {}
    }
    async fn load_player(&self, profile: Profile, uuid: u128) -> Result<Player, ConnectionError> {
        let mut player = Player {
            gamemode: GameMode::Adventure,
            position: Vec3::new(0.0, 48.0, 0.0),
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
}

struct GlideServerHandler {}

#[async_trait]
impl ServerHandler<MiniGameProxy> for GlideServerHandler {
    fn new() -> Self {
        Self {}
    }
    async fn load_player(&self, profile: Profile, uuid: u128) -> Result<Player, ConnectionError> {
        let mut player = Player {
            position: Vec3::new(0.0, 168.0, 0.0),
            gamemode: GameMode::Adventure,
            flying_allowed: true,
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

struct MiniGameProxy {
    connected_players: RwLock<u32>,
    lobby_server: Arc<Server<LobbyServerHandler, MiniGameProxy>>,
    glide_server: Arc<Server<GlideServerHandler, MiniGameProxy>>,
}

#[derive(Debug, Clone)]
enum TransferData {
    Glide,
}

#[async_trait]
impl ServerProxy for MiniGameProxy {
    type TransferData = TransferData;
    async fn new() -> Self {
        let lobby_server =
            Server::new(World::new("./world/region", false), "rust minecraft server").await;
        let glide_server = Server::new(
            World::new("./world/glide/canyon", true),
            "§arust minecraft glide§r",
        )
        .await;
        Self {
            lobby_server,
            glide_server,
            connected_players: RwLock::new(0),
        }
    }
    async fn handle_connection(self: Arc<Self>, connection: Protocol, client_data: ClientData) {
        let client_data = Arc::new(client_data);
        let connection = Arc::new(connection);

        *self.connected_players.write().await += 1;

        let result = self
            .lobby_server
            .clone()
            .handle_connection(self.clone(), connection.clone(), client_data.clone())
            .await;

        if let Ok(TransferData::Glide) = result {
            self.glide_server
                .clone()
                .handle_connection(self.clone(), connection.clone(), client_data.clone())
                .await;
        };

        *self.connected_players.write().await -= 1;
    }
    async fn motd(&self) -> Result<String, ConnectionError> {
        let motd = format!(
            "{{\"favicon\": \"{}\", \"previewsChat\":{},\"enforcesSecureChat\":{},\"description\":{{\"text\":\"{}\"}},\"players\":{{\"max\":{},\"online\":{}}},\"version\":{{\"name\":\"{}\",\"protocol\":{}}}}}",
            "data:image/png;base64,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",
            false,
            false,
            r#"                  \u00A7dRust Network \u00A75\u00A7l[1.19.3]\u00A7r\n            \u00A76\u00A7lNOW IN PRE\u00A7f\u00A7l-\u00A76\u00A7lPRE\u00A7f\u00A7l-\u00A76\u00A7lALPHA!"#,
            100,
            self.connected_players.read().await,
            "1.19.3",
            761,
        );
        Ok(motd)
    }
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    const PORT: u16 = 25566;

    let _server = ServerManager::<MiniGameProxy>::run(PORT).await;

    sleep(Duration::from_secs(10000000000)).await; // Lmao
}
