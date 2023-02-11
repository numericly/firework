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
    ClientData, ConnectionError, Rotation, Server, ServerHandler, ServerManager, ServerProxy, Vec3,
};
use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::sleep};
use world::World;

mod client;
// mod commands;
mod entities;
mod gui;
mod server;

enum ColorCodes {
    Obfuscated,
    Bold,
    Strikethrough,
    Underline,
    Italic,
    Reset,
    White,
    Black,
    Gold,
    Aqua,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    DarkGray,
    LightGray,
    LightBlue,
    LightGreen,
    LightRed,
    LightPurple,
    LightYellow,
}

impl ColorCodes {
    pub const fn chat_formatting(&self) -> &str {
        match self {
            ColorCodes::Obfuscated => "§k",
            ColorCodes::Bold => "§l",
            ColorCodes::Strikethrough => "§m",
            ColorCodes::Underline => "§n",
            ColorCodes::Italic => "§o",
            ColorCodes::Reset => "§r",
            ColorCodes::White => "§f",
            ColorCodes::Black => "§0",
            ColorCodes::Gold => "§6",
            ColorCodes::Aqua => "§b",
            ColorCodes::DarkBlue => "§1",
            ColorCodes::DarkGreen => "§2",
            ColorCodes::DarkAqua => "§3",
            ColorCodes::DarkRed => "§4",
            ColorCodes::DarkPurple => "§5",
            ColorCodes::DarkGray => "§8",
            ColorCodes::LightGray => "§7",
            ColorCodes::LightBlue => "§9",
            ColorCodes::LightGreen => "§a",
            ColorCodes::LightRed => "§c",
            ColorCodes::LightPurple => "§d",
            ColorCodes::LightYellow => "§e",
        }
    }
    pub const fn motd_formatting(&self) -> &str {
        match self {
            ColorCodes::Obfuscated => "\\u00A7k",
            ColorCodes::Bold => "\\u00A7l",
            ColorCodes::Strikethrough => "\\u00A7m",
            ColorCodes::Underline => "\\u00A7n",
            ColorCodes::Italic => "\\u00A7o",
            ColorCodes::Reset => "\\u00A7r",
            ColorCodes::White => "\\u00A7f",
            ColorCodes::Black => "\\u00A70",
            ColorCodes::Gold => "\\u00A76",
            ColorCodes::Aqua => "\\u00A7b",
            ColorCodes::DarkBlue => "\\u00A71",
            ColorCodes::DarkGreen => "\\u00A72",
            ColorCodes::DarkAqua => "\\u00A73",
            ColorCodes::DarkRed => "\\u00A74",
            ColorCodes::DarkPurple => "\\u00A75",
            ColorCodes::DarkGray => "\\u00A78",
            ColorCodes::LightGray => "\\u00A77",
            ColorCodes::LightBlue => "\\u00A79",
            ColorCodes::LightGreen => "\\u00A7a",
            ColorCodes::LightRed => "\\u00A7c",
            ColorCodes::LightPurple => "\\u00A7d",
            ColorCodes::LightYellow => "\\u00A7e",
        }
    }
}

struct LobbyServerHandler {}

#[async_trait]
impl ServerHandler<MiniGameProxy> for LobbyServerHandler {
    fn new() -> Self {
        Self {}
    }
    async fn load_player(&self, profile: Profile, uuid: u128) -> Result<Player, ConnectionError> {
        let mut player = Player {
            gamemode: GameMode::Adventure,
            position: Vec3::new(0.0, 46.0, 0.0),
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
        let lobby_server = Server::new(
            World::new("./firework-world/lobby", false),
            format!(
                "{0}F{1}i{2}r{3}e{4}w{5}ork {6}{7}Lobby{8}",
                ColorCodes::DarkRed.chat_formatting(),
                ColorCodes::LightRed.chat_formatting(),
                ColorCodes::Gold.chat_formatting(),
                ColorCodes::LightYellow.chat_formatting(),
                ColorCodes::LightGreen.chat_formatting(),
                ColorCodes::Aqua.chat_formatting(),
                ColorCodes::White.chat_formatting(),
                ColorCodes::Underline.chat_formatting(),
                ColorCodes::Reset.chat_formatting(),
            ),
        )
        .await;
        let glide_server = Server::new(
            World::new("./firework-world/glide/canyon", true),
            "§aFirework Glide MiniGame§r".to_string(),
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

        if let Ok(TransferData::Glide) = &result {
            self.glide_server
                .clone()
                .handle_connection(self.clone(), connection.clone(), client_data.clone())
                .await;
        };

        println!("result: {:?}, player: {}", result, client_data.profile.name);

        *self.connected_players.write().await -= 1;
    }
    async fn motd(&self) -> Result<String, ConnectionError> {
        let motd = format!(
            "{{\"favicon\": \"{}\", \"previewsChat\":{},\"enforcesSecureChat\":{},\"description\":{{\"text\":\"{}\n{}\"}},\"players\":{{\"max\":{},\"online\":{}}},\"version\":{{\"name\":\"{}\",\"protocol\":{}}}}}",
            include_str!("../server_image_base64.txt"),
            false,
            false,
            format!(
                "                {}F{}i{}r{}e{}w{}ork {}Network {}[1.19.3]{}",
                ColorCodes::DarkRed.motd_formatting(),
                ColorCodes::LightRed.motd_formatting(),
                ColorCodes::Gold.motd_formatting(),
                ColorCodes::LightYellow.motd_formatting(),
                ColorCodes::LightGreen.motd_formatting(),
                ColorCodes::Aqua.motd_formatting(),
                ColorCodes::White.motd_formatting(),
                ColorCodes::LightGray.motd_formatting(),
                ColorCodes::Reset.motd_formatting(),
            ),
            format!(
                "               {}{}GLIDE {}|{}{} TUMBLE {}|{}{} BATTLE",
                ColorCodes::LightBlue.motd_formatting(),
                ColorCodes::Bold.motd_formatting(),
                ColorCodes::DarkGray.motd_formatting(),
                ColorCodes::Gold.motd_formatting(),
                ColorCodes::Bold.motd_formatting(),
                ColorCodes::DarkGray.motd_formatting(),
                ColorCodes::LightGreen.motd_formatting(),
                ColorCodes::Bold.motd_formatting(),
            ),
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

    const PORT: u16 = 25565;

    let _server = ServerManager::<MiniGameProxy>::run(PORT).await;

    sleep(Duration::from_secs(10000000000)).await; // Lmao
}
