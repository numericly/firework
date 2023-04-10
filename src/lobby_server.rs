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
use firework_protocol::{
    client_bound::MapData,
    data_types::{ItemNbt, Slot},
};
use firework_protocol_core::VarInt;
use firework_world::World;
use serde_json::json;
use tokio::sync::{broadcast::Receiver, Mutex};

use crate::{queue::QueueMessage, MiniGameProxy, TransferData, LOBBY_WORLD};

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
    proxy: Arc<MiniGameProxy>,
    pub queued: Mutex<Option<QueuedPlayer>>,
}

#[async_trait]
impl PlayerHandler<LobbyServerHandler, MiniGameProxy> for LobbyPlayerHandler {
    fn new(
        _server: Arc<Server<LobbyServerHandler, MiniGameProxy>>,
        proxy: Arc<MiniGameProxy>,
    ) -> Self {
        Self {
            queued: Mutex::new(None),
            proxy,
        }
    }
    async fn on_post_load(
        &self,
        client: &Client<LobbyServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        client.server.broadcast_chat(format!(
            r#"{{"text": "{} joined the lobby","color":"yellow"}}"#,
            client.player.read().await.profile.name
        ));

        // warning only crashes if less than 35.2 terabytes of memory allocated
        // let values: Vec<u8> = vec![0; 128 * 128];
        // for i in 0..=i32::MAX {
        //     client
        //         .send_packet(MapData {
        //             map_id: VarInt(i),
        //             scale: 1,
        //             locked: false,
        //             icons: None,
        //             columns: 128,
        //             rows: 128,
        //             offset_x: 0,
        //             offset_z: 0,
        //             data: values.clone(),
        //         })
        //         .await?;
        // }
        Ok(())
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
                    QueueMessage::Started { game_id } => {
                        client.transfer(TransferData::Glide { game_id });
                    }
                }
            }
        }
        Ok(())
    }
    async fn on_transfer(
        &self,
        client: &Client<LobbyServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        self.leave_queue(client.client_data.uuid).await;
        Ok(())
    }
    async fn on_use_item(
        &self,
        client: &Client<LobbyServerHandler, MiniGameProxy>,
        item: Option<Slot>,
        slot_id: InventorySlot,
    ) -> Result<(), ConnectionError> {
        let Some(item) = item else {
            return Ok(())
        };

        match item.item_id.0 as u32 {
            Compass::ID => {
                println!("Compass used");
            }
            _ => return Ok(()),
        }

        Ok(())
    }
}

impl LobbyPlayerHandler {
    async fn leave_queue(&self, uuid: u128) {
        let mut queued = self.queued.lock().await.take();
        if let Some(queued) = queued.as_mut() {
            match queued.mini_game {
                MiniGame::Glide => self.proxy.glide_queue.lock().await.leave_queue(uuid).await,
                MiniGame::Tumble => {}
                MiniGame::Battle => {}
            }
        }
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
                    CommandNode::literal("practice").sub_command(
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
                    CommandNode::literal("play")
                        .sub_command(
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
                        )
                        .set_aliases(vec!["join", "p", "queue"]),
                )
                .sub_command(
                    CommandNode::literal("leave_queue")
                        .executable(Box::new(move |args, client, server, proxy| {
                            Box::pin(leave_queue(args, client, server, proxy))
                        }))
                        .set_aliases(vec!["leave", "cancel"]),
                ),
        }
    }
    fn get_world(&self) -> &'static World {
        &LOBBY_WORLD
    }
    async fn on_tick(&self, _server: &Server<Self, MiniGameProxy>, proxy: Arc<MiniGameProxy>) {
        proxy.glide_queue.lock().await.update(proxy.clone()).await;
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
        "glide" => {
            let game_id = client
                .proxy
                .glide_queue
                .lock()
                .await
                .create_server(client.proxy.clone());
            client.transfer(TransferData::Glide { game_id })
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
                .await;

            let receiver = match receiver {
                Ok(receiver) => receiver,
                Err(_) => {
                    client.send_system_chat_message(
                        json!(
                            {
                                "text": "error: already queued",
                            }
                        )
                        .to_string(),
                        false,
                    );
                    return;
                }
            };

            client.handler.leave_queue(client.client_data.uuid).await;

            client.handler.queued.lock().await.replace(QueuedPlayer {
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

async fn leave_queue(
    _args: Vec<Argument>,
    client: &Client<LobbyServerHandler, MiniGameProxy>,
    _server: &Server<LobbyServerHandler, MiniGameProxy>,
    _proxy: &MiniGameProxy,
) {
    client.handler.leave_queue(client.client_data.uuid).await;
    client.send_system_chat_message(
        json!(
            {
                "text": "",
            }
        )
        .to_string(),
        true,
    )
}
