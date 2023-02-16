use std::sync::Arc;

use async_trait::async_trait;
use cipher::typenum::Min;
use firework::{
    client::{Client, GameMode, InventorySlot, Player},
    commands::{ArgumentType, CommandNode, StringTypes},
    PlayerHandler,
};
use firework::{ConnectionError, Rotation, Server, ServerHandler, Vec3};
use firework_authentication::Profile;
use firework_data::items::{Compass, Item};
use firework_protocol::data_types::{ItemNbt, Slot};
use firework_protocol_core::VarInt;
use tokio::pin;

use crate::MiniGameProxy;

pub struct LobbyPlayerHandler {}

#[async_trait]
impl PlayerHandler<LobbyServerHandler, MiniGameProxy> for LobbyPlayerHandler {
    fn new(
        _server: Arc<Server<LobbyServerHandler, MiniGameProxy>>,
        _proxy: Arc<MiniGameProxy>,
    ) -> Self {
        Self {}
    }
    async fn on_chat_command(
        &self,
        client: &Client<LobbyServerHandler, MiniGameProxy>,
        command: String,
    ) -> Result<Option<String>, ConnectionError> {
        let command_data = command.split(' ').collect::<Vec<&str>>();

        println!("Command: {:?}", command_data);

        Ok(Some(command))
    }
}

pub struct LobbyServerHandler {
    commands: CommandNode<Self, MiniGameProxy>,
}

#[async_trait]
impl ServerHandler<MiniGameProxy> for LobbyServerHandler {
    type PlayerHandler = LobbyPlayerHandler;
    fn new() -> Self {
        Self {
            commands: CommandNode::root()
                .sub_command(
                    CommandNode::literal("play").sub_command(
                        CommandNode::argument(
                            "game",
                            ArgumentType::String {
                                string_type: StringTypes::SingleWord,
                                suggestions: Some(vec![
                                    "glide".to_string(),
                                    "tumble".to_string(),
                                    "battle".to_string(),
                                ]),
                            },
                        )
                        .executable(Box::new(move |string: &str| Box::pin(play(string)))),
                    ),
                )
                .sub_command(
                    CommandNode::literal("echo").sub_command(CommandNode::argument(
                        "text",
                        ArgumentType::String {
                            string_type: StringTypes::SingleWord,
                            suggestions: None,
                        },
                    )),
                ),
        }
    }
    async fn load_player(&self, profile: Profile, uuid: u128) -> Result<Player, ConnectionError> {
        let mut player = Player {
            gamemode: GameMode::Adventure,
            max_health: 20.0,
            health: 20.0,
            position: Vec3::new(0.5, 46.0, 0.5),
            rotation: Rotation::new(-90.0, 0.0),
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
    async fn get_commands(
        &self,
        server: &Server<LobbyServerHandler, MiniGameProxy>,
        proxy: &MiniGameProxy,
    ) -> Result<&CommandNode<LobbyServerHandler, MiniGameProxy>, ConnectionError> {
        Ok(&self.commands)
    }
}

async fn play(server: &str) {
    println!("playing server {server:?}")
}

async fn echo(server: Arc<Server<LobbyServerHandler, MiniGameProxy>>) {
    println!("echoing")
}
