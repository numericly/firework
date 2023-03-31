use std::sync::Arc;

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
use tokio::sync::{broadcast::Receiver, Mutex};

use crate::{queue::QueueMessage, MiniGameProxy, TransferData};

enum MiniGame {
    Glide,
    Tumble,
    Battle,
}

pub struct QueuedPlayer {
    receiver: Receiver<QueueMessage>,
    mini_game: MiniGame,
}

pub struct LobbyPlayerHandler {
    pub queued: Mutex<Option<QueuedPlayer>>,
}

#[async_trait]
impl PlayerHandler<LobbyServerHandler, MiniGameProxy> for LobbyPlayerHandler {
    fn new(
        _server: Arc<Server<LobbyServerHandler, MiniGameProxy>>,
        _proxy: Arc<MiniGameProxy>,
    ) -> Self {
        Self {
            queued: Mutex::new(None),
        }
    }
    async fn on_tick(
        &self,
        client: &Client<LobbyServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        let mut queued = self.queued.lock().await;
        if let Some(queued) = queued.as_mut() {
            for _ in 0..queued.receiver.len() {
                let msg = queued.receiver.recv().await.unwrap();
                let game = match queued.mini_game {
                    MiniGame::Glide => "Glide",
                    MiniGame::Tumble => "Tumble",
                    MiniGame::Battle => "Battle",
                };
                match msg {
                    QueueMessage::NotEnoughPlayers => client.send_system_chat_message(
                        json!([
                            {
                              "text": format!("Not enough players to start {}", game),
                              "color": "red"
                            }
                        ])
                        .to_string(),
                        true,
                    ),
                    QueueMessage::Starting {
                        seconds,
                        connected_players,
                        max_players,
                    } => {
                        client.send_system_chat_message(
                            json!([
                                {
                                    "text": "Joining ",
                                    "color": "green"
                                },
                                {
                                    "text": game,
                                    "color": "gold"
                                },
                                {
                                    "text": " in ",
                                    "color": "green"
                                },
                                {
                                    "text": seconds.to_string(),
                                    "color": "yellow"
                                },
                                {
                                    "text": " seconds ",
                                    "color": "green"
                                },
                                {
                                    "text": connected_players.to_string(),
                                    "color": "yellow"
                                },
                                {
                                    "text": "/",
                                    "color": "green"
                                },
                                {
                                    "text": max_players.to_string(),
                                    "color": "yellow"
                                }
                            ])
                            .to_string(),
                            true,
                        );
                    }
                    QueueMessage::Started => {
                        client.transfer(TransferData::Glide);
                    }
                }
                dbg!(msg);
            }
        }
        Ok(())
    }
    async fn on_leave(
        &self,
        client: &Client<LobbyServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        client
            .proxy
            .glide_queue
            .lock()
            .await
            .leave_queue(client.client_data.uuid)
            .await;
        Ok(())
    }
    async fn on_chat_command(
        &self,
        _client: &Client<LobbyServerHandler, MiniGameProxy>,
        command: String,
    ) -> Result<Option<String>, ConnectionError> {
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
                    CommandNode::literal("queue").sub_command(
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
                                Box::pin(queue(args, client, server, proxy))
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
    async fn on_tick(&self, _server: &Server<Self, MiniGameProxy>, proxy: &MiniGameProxy) {
        proxy.glide_queue.lock().await.update().await;
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

async fn queue(
    args: Vec<Argument>,
    client: &Client<LobbyServerHandler, MiniGameProxy>,
    _server: &Server<LobbyServerHandler, MiniGameProxy>,
    proxy: &MiniGameProxy,
) {
    let Argument::String { value } = args.get(1).expect("Arg not found") else {
        return
    };
    match value.as_str() {
        "glide" => {
            let receiver = proxy
                .glide_queue
                .lock()
                .await
                .queue(client.client_data.uuid.clone())
                .await
                .unwrap();

            let mut queued = client.handler.queued.lock().await;

            if queued.is_some() {
                // TODO: self.leave_queue().await;
                queued.take();
            }

            queued.replace(QueuedPlayer {
                receiver,
                mini_game: MiniGame::Glide,
            });
        }
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
