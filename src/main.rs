use async_trait::async_trait;
use battle::BattleServerHandler;
use firework::protocol::Protocol;
use firework::world::{world, World};
use firework::{ClientData, ConnectionError, Server, ServerManager, ServerOptions, ServerProxy};
use glide::GlideServerHandler;
use lazy_static::lazy_static;
use lobby_server::LobbyServerHandler;
use queue::Queue;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio_util::sync::CancellationToken;

mod battle;
mod glide;
mod lobby_server;
mod queue;

lazy_static! {
    static ref LOBBY_WORLD: World = world!("./firework-world/lobby", false);
    static ref CANYON_GLIDE_WORLD: World = world!("./firework-world/glide/canyon", true);
    static ref CAVERN_GLIDE_WORLD: World = world!("./firework-world/glide/cavern", true);
    static ref TEMPLE_GLIDE_WORLD: World = world!("./firework-world/glide/temple", true);
    static ref COVE_BATTLE_WORLD: World = world!("./firework-world/battle/cove", true);
    static ref CAVERN_BATTLE_WORLD: World = world!("./firework-world/battle/cavern", true);
    static ref CRUCIBLE_BATTLE_WORLD: World = world!("./firework-world/battle/crucible", true);
}

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
    pub glide_queue: Mutex<Queue<MiniGameProxy, GlideServerHandler, 8>>,
    pub battle_queue: Mutex<Queue<MiniGameProxy, BattleServerHandler, 8>>,
    pub tumble_queue: Mutex<Queue<MiniGameProxy, BattleServerHandler, 8>>,
}

#[derive(Debug, Clone)]
enum TransferData {
    Lobby,
    Glide { game_id: u128 },
    Battle { game_id: u128 },
    Tumble { game_id: u128 },
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
            0,
        );
        Self {
            lobby_server,
            glide_queue: Mutex::new(Queue::new(format!(
                "{}F{}i{}r{}e{}w{}ork Glide{}",
                ColorCodes::DarkRed.chat_formatting(),
                ColorCodes::LightRed.chat_formatting(),
                ColorCodes::Gold.chat_formatting(),
                ColorCodes::LightYellow.chat_formatting(),
                ColorCodes::LightGreen.chat_formatting(),
                ColorCodes::Aqua.chat_formatting(),
                ColorCodes::Reset.chat_formatting(),
            ))),
            battle_queue: Mutex::new(Queue::new(format!(
                "{}F{}i{}r{}e{}w{}ork Battle{}",
                ColorCodes::DarkRed.chat_formatting(),
                ColorCodes::LightRed.chat_formatting(),
                ColorCodes::Gold.chat_formatting(),
                ColorCodes::LightYellow.chat_formatting(),
                ColorCodes::LightGreen.chat_formatting(),
                ColorCodes::Aqua.chat_formatting(),
                ColorCodes::Reset.chat_formatting(),
            ))),
            tumble_queue: Mutex::new(Queue::new(format!(
                "{}F{}i{}r{}e{}w{}ork Tumble{}",
                ColorCodes::DarkRed.chat_formatting(),
                ColorCodes::LightRed.chat_formatting(),
                ColorCodes::Gold.chat_formatting(),
                ColorCodes::LightYellow.chat_formatting(),
                ColorCodes::LightGreen.chat_formatting(),
                ColorCodes::Aqua.chat_formatting(),
                ColorCodes::Reset.chat_formatting(),
            ))),
            connected_players: RwLock::new(0),
        }
    }
    async fn run(self: Arc<Self>) {
        self.lobby_server
            .clone()
            .run(self.clone(), CancellationToken::new())
            .await;
    }
    async fn handle_connection(self: Arc<Self>, connection: Protocol, client_data: ClientData) {
        let client_data = Arc::new(client_data);
        let connection = Arc::new(connection);

        *self.connected_players.write().await += 1;

        loop {
            let result = self
                .lobby_server
                .clone()
                .handle_connection(self.clone(), connection.clone(), client_data.clone())
                .await;

            println!("result: {:?}", result);

            let Ok(transfer_data) = result else {
                break;
            };

            match transfer_data {
                TransferData::Glide { game_id } => {
                    let server = self.glide_queue.lock().await.get_server(game_id).clone();
                    let Some(server) = server else {
                        continue;
                    };
                    match server
                        .handle_connection(self.clone(), connection.clone(), client_data.clone())
                        .await
                    {
                        Ok(_) => {}
                        Err(_err) => {
                            break;
                        }
                    }
                }
                TransferData::Battle { game_id } => {
                    let server = self.battle_queue.lock().await.get_server(game_id).clone();
                    let Some(server) = server else {
                        continue;
                    };
                    match server
                        .handle_connection(self.clone(), connection.clone(), client_data.clone())
                        .await
                    {
                        Ok(_) => {}
                        Err(_err) => {
                            break;
                        }
                    }
                }
                TransferData::Tumble { game_id } => {
                    let server = self.tumble_queue.lock().await.get_server(game_id).clone();
                    let Some(server) = server else {
                        continue;
                    };
                    match server
                        .handle_connection(self.clone(), connection.clone(), client_data.clone())
                        .await
                    {
                        Ok(_) => {}
                        Err(_err) => {
                            break;
                        }
                    }
                }
                TransferData::Lobby => {
                    break;
                }
            }
        }

        *self.connected_players.write().await -= 1;
    }
    async fn motd(&self) -> Result<String, ConnectionError> {
        let line1 = format!(
            "                {}F{}i{}r{}e{}w{}o{}r{}k {}N{}e{}t{}w{}o{}r{}k {}[1.19.4]{}",
            ColorCodes::DarkRed.motd_formatting(),
            ColorCodes::LightRed.motd_formatting(),
            ColorCodes::Gold.motd_formatting(),
            ColorCodes::LightYellow.motd_formatting(),
            ColorCodes::LightGreen.motd_formatting(),
            ColorCodes::Aqua.motd_formatting(),
            ColorCodes::LightBlue.motd_formatting(),
            ColorCodes::DarkBlue.motd_formatting(),
            ColorCodes::DarkRed.motd_formatting(),
            ColorCodes::LightRed.motd_formatting(),
            ColorCodes::Gold.motd_formatting(),
            ColorCodes::LightYellow.motd_formatting(),
            ColorCodes::LightGreen.motd_formatting(),
            ColorCodes::Aqua.motd_formatting(),
            ColorCodes::LightBlue.motd_formatting(),
            ColorCodes::LightGray.motd_formatting(),
            ColorCodes::Reset.motd_formatting(),
        );
        let line2 = format!(
            "        {}{}GLIDE MINIGAME {}-{}{} BATTLE MINIGAME{}",
            ColorCodes::Gold.motd_formatting(),
            ColorCodes::Bold.motd_formatting(),
            ColorCodes::LightGray.motd_formatting(),
            ColorCodes::Aqua.motd_formatting(),
            ColorCodes::Bold.motd_formatting(),
            ColorCodes::Reset.motd_formatting(),
        );
        let motd = format!(
            "{{\"favicon\": \"{}\", \"previewsChat\":{},\"enforcesSecureChat\":{},\"description\":{{\"text\":\"{}\n{}\"}},\"players\":{{\"max\":{},\"online\":{}}},\"version\":{{\"name\":\"{}\",\"protocol\":{}}}}}",
            include_str!("../server_image_base64.txt"),
            false,
            false,
            line1,
            line2,
            1000,
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

    ServerManager::<MiniGameProxy>::run(ServerOptions {
        encryption: false,
        host: true,
        ..Default::default()
    })
    .await;
}

// #[tokio::test]
// async fn test_parse() {
//     use firework::commands::*;

//     let command_tree: CommandTree<LobbyServerHandler, MiniGameProxy> = CommandTree::new()
//         .register_command(
//             Command::new("test")
//                 .set_aliases(vec!["t", "t1"])
//                 .add_node(CommandNode::literal("test2")),
//         );

//     let mut buf = Vec::new();

//     command_tree.serialize(&mut buf).await;

//     println!("{:?}", buf);

//     panic!();
// }
