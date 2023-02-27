use std::{sync::Arc};

use async_trait::async_trait;

use firework::{
    client::{Client, GameMode, InventorySlot, Player},
    commands::{Argument, ArgumentType, CommandNode, StringTypes},
    PlayerHandler,
};
use firework::{ConnectionError, Rotation, Server, ServerHandler, Vec3};
use firework_authentication::Profile;
use firework_data::items::{Compass, Item};
use firework_protocol::data_types::{ItemNbt, Slot};
use firework_protocol_core::VarInt;
use serde_json::json;


use crate::{MiniGameProxy, TransferData};

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
        _client: &Client<LobbyServerHandler, MiniGameProxy>,
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
                        .executable(Box::new(
                            move |args, client, server, proxy| {
                                Box::pin(play(args, client, server, proxy))
                            },
                        )),
                    ),
                )
                .sub_command(
                    CommandNode::literal("echo").sub_command(
                        CommandNode::argument(
                            "text",
                            ArgumentType::String {
                                string_type: StringTypes::SingleWord,
                                suggestions: None,
                            },
                        )
                        .executable(Box::new(
                            move |args, client, server, proxy| {
                                Box::pin(echo(args, client, server, proxy))
                            },
                        )),
                    ),
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
        _server: &Server<LobbyServerHandler, MiniGameProxy>,
        _proxy: &MiniGameProxy,
    ) -> Result<&CommandNode<LobbyServerHandler, MiniGameProxy>, ConnectionError> {
        Ok(&self.commands)
    }
}

async fn play(
    args: Vec<Argument>,
    client: &Client<LobbyServerHandler, MiniGameProxy>,
    _server: &Server<LobbyServerHandler, MiniGameProxy>,
    _proxy: &MiniGameProxy,
) {
    let Argument::String { value } = args.get(1).expect("Arg not found") else {
        return
    };
    match value.as_str() {
        "glide" => client.transfer(TransferData::Glide),
        value => client.show_chat_message(
            json!(
                {
                    "text": format!("error: game \"{}\" does not exist", value),
                }
            )
            .to_string(),
        ),
    }
}

async fn echo(
    args: Vec<Argument>,
    client: &Client<LobbyServerHandler, MiniGameProxy>,
    _server: &Server<LobbyServerHandler, MiniGameProxy>,
    _proxy: &MiniGameProxy,
) {
    let Argument::String { value } = args.get(1).expect("Arg not found") else {
        return
    };
    client.show_chat_message(
        json!(
            {
                "text": value,
            }
        )
        .to_string(),
    )
}
