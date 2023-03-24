use async_trait::async_trait;
use firework::{ClientData, ConnectionError, Server, ServerManager, ServerProxy};
use firework_protocol::Protocol;
use firework_world::World;
use glide_server::GlideServerHandler;
use lobby_server::LobbyServerHandler;
use std::sync::Arc;
use tokio::sync::RwLock;

mod glide_server;
mod lobby_server;

#[allow(dead_code)]
pub enum ColorCodes {
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

struct MiniGameProxy {
    connected_players: RwLock<u32>,
    lobby_server: Arc<Server<LobbyServerHandler, MiniGameProxy>>,
    glide_server: Arc<Server<GlideServerHandler, MiniGameProxy>>,
}

#[derive(Debug, Clone)]
enum TransferData {
    Glide,
}

#[derive(Debug, Clone, Default)]
enum Roles {
    #[default]
    Default,
}

#[derive(Debug, Clone)]
enum Permissions {}

#[async_trait]
impl ServerProxy for MiniGameProxy {
    type TransferData = TransferData;
    type Roles = Roles;
    type Permissions = Permissions;
    async fn new() -> Self {
        let lobby_server = Server::new(
            World::new("./firework-world/lobby", false),
            format!(
                "{}F{}i{}r{}e{}w{}ork Lobby{}",
                ColorCodes::DarkRed.chat_formatting(),
                ColorCodes::LightRed.chat_formatting(),
                ColorCodes::Gold.chat_formatting(),
                ColorCodes::LightYellow.chat_formatting(),
                ColorCodes::LightGreen.chat_formatting(),
                ColorCodes::Aqua.chat_formatting(),
                ColorCodes::Reset.chat_formatting(),
            ),
        )
        .await;
        let glide_server = Server::new(
            World::new("./firework-world/glide/canyon", true),
            format!(
                "{}F{}i{}r{}e{}w{}ork Glide{}",
                ColorCodes::DarkRed.chat_formatting(),
                ColorCodes::LightRed.chat_formatting(),
                ColorCodes::Gold.chat_formatting(),
                ColorCodes::LightYellow.chat_formatting(),
                ColorCodes::LightGreen.chat_formatting(),
                ColorCodes::Aqua.chat_formatting(),
                ColorCodes::Reset.chat_formatting(),
            ),
        )
        .await;
        Self {
            lobby_server,
            glide_server,
            connected_players: RwLock::new(0),
        }
    }
    async fn run(self: Arc<Self>) {
        let glide_server_proxy = self.clone();
        let glide_server = self.glide_server.clone();
        tokio::spawn(async move {
            glide_server.run(glide_server_proxy).await;
        });

        self.lobby_server.clone().run(self.clone()).await;
    }
    async fn handle_connection(self: Arc<Self>, connection: Protocol, client_data: ClientData) {
        let client_data = Arc::new(client_data);
        let connection = Arc::new(connection);

        *self.connected_players.write().await += 1;

        let result = match self
            .lobby_server
            .clone()
            .handle_connection(self.clone(), connection.clone(), client_data.clone())
            .await
        {
            Ok(transfer_data) => match transfer_data {
                TransferData::Glide => {
                    self.glide_server
                        .clone()
                        .handle_connection(self.clone(), connection.clone(), client_data.clone())
                        .await
                }
            },
            Err(e) => Err(e),
        };

        dbg!(result, &client_data.profile.name);

        *self.connected_players.write().await -= 1;
    }
    async fn motd(&self) -> Result<String, ConnectionError> {
        let motd = format!(
            "{{\"favicon\": \"{}\", \"previewsChat\":{},\"enforcesSecureChat\":{},\"description\":{{\"text\":\"{}\n{}\"}},\"players\":{{\"max\":{},\"online\":{}}},\"version\":{{\"name\":\"{}\",\"protocol\":{}}}}}",
            include_str!("../server_image_base64.txt"),
            false,
            false,
            format!(
                "                {}F{}i{}r{}e{}w{}ork {}Network {}[1.19.4]{}",
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
                "              {}{}GLIDE {}|{}{} TUMBLE {}|{}{} BATTLE",
                ColorCodes::LightBlue.motd_formatting(),
                ColorCodes::Bold.motd_formatting(),
                ColorCodes::DarkGray.motd_formatting(),
                ColorCodes::LightGray.motd_formatting(),
                ColorCodes::Bold.motd_formatting(),
                ColorCodes::DarkGray.motd_formatting(),
                ColorCodes::LightGray.motd_formatting(),
                ColorCodes::Bold.motd_formatting(),
            ),
            100,
            self.connected_players.read().await,
            "1.19.4",
            762,
        );
        Ok(motd)
    }
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    ServerManager::<MiniGameProxy>::run(25565).await;
}
