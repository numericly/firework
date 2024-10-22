use crate::{queue::QueueMessage, MiniGameProxy, TransferData, LOBBY_WORLD};
use async_trait::async_trait;
use firework::protocol::{
    client_bound::SetContainerSlot,
    core::Position,
    data_types::{InventoryOperationMode, ItemNbt, ItemNbtDisplay, ItemStack, StackContents},
    server_bound::ClickContainer,
};
use firework::world::World;
use firework::{authentication::Profile, gui::GUIEvent};
use firework::{
    client::{Client, GameMode, InventorySlot, Player},
    commands::{Argument, ArgumentType, Command, CommandNode, CommandTree, StringType},
    gui::{GUIInit, GuiScreen, WindowType},
    PlayerHandler, TICKS_PER_SECOND,
};
use firework::{data::items::Item, protocol::core::VarInt};
use firework::{ConnectionError, Rotation, Server, ServerHandler, Vec3};
use serde_json::json;
use std::{sync::Arc, time::Instant};
use tokio::sync::{
    broadcast::{self, Receiver},
    Mutex,
};

#[allow(dead_code)]
#[derive(Debug)]
enum MiniGame {
    Battle,
    Glide,
}

pub struct QueuedPlayer {
    receiver: Receiver<QueueMessage>,
    mini_game: MiniGame,
}

pub struct LobbyPlayerHandler {
    proxy: Arc<MiniGameProxy>,
    pub queued: Mutex<Option<QueuedPlayer>>,
    pub recent_packets: Mutex<Vec<Instant>>,
}

#[async_trait]
impl PlayerHandler<LobbyServerHandler, MiniGameProxy> for LobbyPlayerHandler {
    fn new(
        _server: Arc<Server<LobbyServerHandler, MiniGameProxy>>,
        proxy: Arc<MiniGameProxy>,
    ) -> Self {
        Self {
            recent_packets: Mutex::new(Vec::new()),
            queued: Mutex::new(None),
            proxy,
        }
    }
    async fn on_server_bound_packet(
        &self,
        client: &Client<LobbyServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        let mut recent_packets = self.recent_packets.lock().await;
        recent_packets.push(Instant::now());

        const PACKETS_PER_TICK: usize = 4;
        const SAMPLE_TIME: usize = 3;

        recent_packets.retain(|i| i.elapsed().as_secs() < SAMPLE_TIME as u64);

        if recent_packets.len() > SAMPLE_TIME * PACKETS_PER_TICK * TICKS_PER_SECOND {
            client.disconnect(
                json!({
                    "text": "Kicked for sending too many packets",
                    "color": "red"
                })
                .to_string(),
            );
        }

        Ok(())
    }
    async fn on_post_load(
        &self,
        client: &Client<LobbyServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        client.server.broadcast_chat(format!(
            r#"{{"text": "{} joined the lobby","color":"yellow"}}"#,
            client.player.read().await.profile.name
        ));
        client.show_chat_message(
            json!([
                {
                    "text": "\n"
                },
                {
                    "text": "Welcome to the Firework Network, ",
                    "color": "aqua"
                },
                {
                    "text": format!("{}",client.player.read().await.profile.name.clone()),
                    "color": "gold"
                },
                {
                    "text": "!\n\n",
                    "color": "aqua"
                },
                {
                    "text": "Minigames",
                    "color": "dark_green"
                },
                {
                    "text": "\n"
                },
                {
                    "text": "- Glide: Race other players through a course using an elytra.\n- Battle: Fight your friends in an arena, getting items to help you in the fight.\n",
                    "color": "green"
                },
                {
                    "text": "To get started, right click while holding compass or stick in your hotbar.",
                    "color": "gray"
                }
              ])
            .to_string(),
        );

        Ok(())
    }
    async fn on_tick(&self, client: &Client<LobbyServerHandler, MiniGameProxy>) {
        let mut queued = self.queued.lock().await;
        if let Some(queued) = queued.as_mut() {
            for _ in 0..queued.receiver.len() {
                let msg = queued.receiver.recv().await.unwrap();
                let game = match queued.mini_game {
                    MiniGame::Battle => "Battle",
                    MiniGame::Glide => "Glide",
                };
                match msg {
                    QueueMessage::NotEnoughPlayers => client.send_system_chat_message(
                        json!([
                            {
                              "text": format!("Not enough players to start {}, need at least 2", game),
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
                        client.transfer(match queued.mini_game {
                            MiniGame::Battle => TransferData::Battle { game_id },
                            MiniGame::Glide => TransferData::Glide { game_id },
                        });
                    }
                }
            }
        }
    }
    async fn on_transfer(
        &self,
        client: &Client<LobbyServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        self.leave_queue(client.client_data.uuid).await;
        Ok(())
    }
    async fn on_leave(
        &self,
        client: &Client<LobbyServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        self.leave_queue(client.client_data.uuid).await;
        Ok(())
    }
    async fn on_drop_item(
        &self,
        client: &Client<LobbyServerHandler, MiniGameProxy>,
        _is_stack: bool,
    ) -> Result<(), ConnectionError> {
        let item_slot = client.player.read().await.selected_slot as usize;
        let inv_slot = InventorySlot::Hotbar { slot: item_slot };

        let item = client
            .player
            .read()
            .await
            .inventory
            .get_slot(&inv_slot)
            .clone();

        client
            .send_packet(SetContainerSlot {
                window_id: 0,
                state_id: VarInt(1),
                slot: inv_slot.value() as i16,
                item,
            })
            .await?;
        Ok(())
    }
    async fn on_use_item(
        &self,
        client: &Client<LobbyServerHandler, MiniGameProxy>,
        item: ItemStack,
        _slot_id: InventorySlot,
        _location: Option<Position>,
    ) -> Result<(), ConnectionError> {
        let Some(item) = item else {
            return Ok(())
        };

        match item.id {
            Item::Compass => {
                client
                    .display_gui(client.server.handler.game_menu.clone())
                    .await;
            }
            Item::Stick => {
                client
                    .display_gui(client.server.handler.practice_menu.clone())
                    .await;
            }
            Item::RedstoneBlock => {
                leave_queue(client).await;
            }
            _ => return Ok(()),
        }

        Ok(())
    }
    async fn on_click_container(
        &self,
        client: &Client<LobbyServerHandler, MiniGameProxy>,
        click: ClickContainer,
    ) -> Result<(), ConnectionError> {
        let ClickContainer {
            window_id,
            state_id,
            slot,
            button: _button,
            mode,
            slots,
        } = click;

        match mode {
            InventoryOperationMode::Click => {
                if slot >= 0 {
                    client
                        .send_packet(SetContainerSlot {
                            window_id: -1,
                            state_id: VarInt(state_id.0 + 1),
                            slot: -1,
                            item: None,
                        })
                        .await?;

                    let Some(inv_slot) = InventorySlot::from_value(click.slot as usize) else {
                        return Ok(())
                    };

                    let item = client
                        .player
                        .read()
                        .await
                        .inventory
                        .get_slot(&inv_slot)
                        .clone();

                    client
                        .send_packet(SetContainerSlot {
                            window_id: window_id as i8,
                            state_id: VarInt(state_id.0 + 2),
                            slot,
                            item,
                        })
                        .await?;
                }
            }
            _ => {}
        }

        for (i, updated_slot) in slots.iter().enumerate() {
            if updated_slot.slot_number >= 0 {
                let inv_slot = InventorySlot::from_value(updated_slot.slot_number as usize);

                let Some(inv_slot) = inv_slot else {
                    continue;
                };

                let item = client
                    .player
                    .read()
                    .await
                    .inventory
                    .get_slot(&inv_slot)
                    .clone();

                client
                    .send_packet(SetContainerSlot {
                        window_id: window_id as i8,
                        state_id: VarInt(state_id.0 + i as i32 + 1),
                        slot: updated_slot.slot_number,
                        item,
                    })
                    .await?;
            }
        }
        Ok(())
    }
}

impl LobbyPlayerHandler {
    async fn leave_queue(&self, uuid: u128) {
        let mut queued = self.queued.lock().await.take();
        if let Some(queued) = queued.as_mut() {
            match queued.mini_game {
                MiniGame::Battle => self.proxy.battle_queue.lock().await.leave_queue(uuid).await,
                MiniGame::Glide => self.proxy.glide_queue.lock().await.leave_queue(uuid).await,
            }
        }
    }
}

pub struct GameMenu {
    pub items: Vec<ItemStack>,
    pub channel: broadcast::Sender<GUIEvent>,
    pub r#type: MenuType,
}

#[async_trait]
impl GuiScreen<LobbyServerHandler, MiniGameProxy> for GameMenu {
    async fn init(&self, _client: &Client<LobbyServerHandler, MiniGameProxy>) -> GUIInit {
        GUIInit {
            title: r#"{"text":"      Minigame Selector","bold":true}"#.to_string(),
            window_type: WindowType::Generic9x1,
            items: self.items.clone(),
            receiver: self.channel.subscribe(),
        }
    }
    async fn handle_click(
        &self,
        slot: ClickContainer,
        client: &Client<LobbyServerHandler, MiniGameProxy>,
    ) -> Result<(), ConnectionError> {
        let ClickContainer {
            window_id,
            state_id,
            slot,
            button: _button,
            mode,
            slots,
        } = slot;

        match mode {
            InventoryOperationMode::Click => {
                if slot >= 0 {
                    client
                        .send_packet(SetContainerSlot {
                            window_id: -1,
                            state_id: VarInt(state_id.0 + 1),
                            slot: -1,
                            item: None,
                        })
                        .await?;
                    let item = self.correct_item(client, slot as usize).await;

                    client
                        .send_packet(SetContainerSlot {
                            window_id: window_id as i8,
                            state_id: VarInt(state_id.0 + 2),
                            slot,
                            item,
                        })
                        .await?;
                }
            }
            _ => {}
        }

        for (i, updated_slot) in slots.iter().enumerate() {
            if updated_slot.slot_number >= 0 {
                let item = self
                    .correct_item(client, updated_slot.slot_number as usize)
                    .await;

                client
                    .send_packet(SetContainerSlot {
                        window_id: window_id as i8,
                        state_id: VarInt(state_id.0 + i as i32 + 1),
                        slot: updated_slot.slot_number,
                        item,
                    })
                    .await?;
            }
        }

        match self.r#type {
            MenuType::Play => match slot {
                3 => {
                    queue(client, &client.proxy, MiniGame::Glide).await;
                    client.close_gui();
                }
                5 => {
                    queue(client, &client.proxy, MiniGame::Battle).await;
                    client.close_gui();
                }
                _ => {}
            },
            MenuType::Practice => match slot {
                3 => {
                    let game_id = client
                        .proxy
                        .glide_queue
                        .lock()
                        .await
                        .create_server(client.proxy.clone());
                    client.transfer(TransferData::Glide { game_id });
                }
                5 => {
                    let game_id = client
                        .proxy
                        .battle_queue
                        .lock()
                        .await
                        .create_server(client.proxy.clone());
                    client.transfer(TransferData::Battle { game_id })
                }
                _ => {}
            },
        }

        Ok(())
    }
}

impl GameMenu {
    pub fn play() -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self {
            r#type: MenuType::Play,
            channel: sender,
            items: vec![
                None,
                None,
                None,
                Some(StackContents {
                    id: Item::Elytra, // elytra
                    count: 1,
                    nbt: ItemNbt {
                        display: Some(ItemNbtDisplay {
                            name: Some(r#"{"text":"Glide Minigame","italic":"false","color":"green"}"#.to_string()),
                            lore: Some(vec![
                                r#"{"text":"Race other players through a course","italic":"false","color":"gray"}"#.to_string(),
                                r#"{"text":"with an elytra.","italic":"false","color":"gray"}"#.to_string(),
                                r#"{"text":""}"#.to_string(),
                                r#"{"text":"Click to Connect","color":"green","italic":false}"#.to_string(),
                                r#"{"italic":false,"color":"gray","extra":[
                                    {"text":"12","obfuscated":true},
                                    {"text":" Currently Playing"}
                                ],"text":""}"#.to_string()
                            ]),
                        }),
                        ..Default::default()
                    },
                }),
                None,
                Some(StackContents {
                    id: Item::IronSword, // iron sword
                    count: 1,
                    nbt: ItemNbt {
                        display: Some(ItemNbtDisplay {
                            name: Some(r#"{"text":"Battle Minigame","italic":"false","color":"green"}"#.to_string()),
                            lore: Some(vec![
                                r#"{"text":"Fight your friends in an arena, getting","italic":"false","color":"gray"}"#.to_string(),
                                r#"{"text":"items to help you in the fight.","italic":"false","color":"gray"}"#.to_string(),
                                r#"{"text":""}"#.to_string(),
                                r#"{"text":"Click to Connect","color":"green","italic":false}"#.to_string(),
                                r#"{"italic":false,"color":"gray","extra":[
                                    {"text":"12","obfuscated":true},
                                    {"text":" Currently Playing"}
                                    ],"text":""}"#.to_string()
                            ]),
                        }),
                        ..Default::default()
                    },
                }),
                None,
                None,
                None,
            ]
        }
    }
    pub fn practice() -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self {
            r#type: MenuType::Practice,
            channel: sender,
            items: vec![
                None,
                None,
                None,
                Some(StackContents {
                    id: Item::Elytra, // elytra
                    count: 1,
                    nbt: ItemNbt {
                        display: Some(ItemNbtDisplay {
                            name: Some(r#"{"text":"Glide Minigame","italic":"false","color":"green"}"#.to_string()),
                            lore: Some(vec![
                                r#"{"text":"Race other players through a course","italic":"false","color":"gray"}"#.to_string(),
                                r#"{"text":"with an elytra.","italic":"false","color":"gray"}"#.to_string(),
                                r#"{"text":""}"#.to_string(),
                                r#"{"text":"Click to Connect","color":"green","italic":false}"#.to_string(),
                                r#"{"italic":false,"color":"gray","extra":[
                                    {"text":"12","obfuscated":true},
                                    {"text":" Currently Playing"}
                                ],"text":""}"#.to_string()
                            ]),
                        }),
                        ..Default::default()
                    },
                }),
                None,
                Some(StackContents {
                    id: Item::IronSword, // iron sword
                    count: 1,
                    nbt: ItemNbt {
                        display: Some(ItemNbtDisplay {
                            name: Some(r#"{"text":"Battle Minigame","italic":"false","color":"green"}"#.to_string()),
                            lore: Some(vec![
                                r#"{"text":"Fight your friends in an arena, getting","italic":"false","color":"gray"}"#.to_string(),
                                r#"{"text":"items to help you in the fight.","italic":"false","color":"gray"}"#.to_string(),
                                r#"{"text":""}"#.to_string(),
                                r#"{"text":"Click to Connect","color":"green","italic":false}"#.to_string(),
                                r#"{"italic":false,"color":"gray","extra":[
                                    {"text":"12","obfuscated":true},
                                    {"text":" Currently Playing"}
                                    ],"text":""}"#.to_string()
                            ]),
                        }),
                        ..Default::default()
                    },
                }),
                None,
                None,
                None,
            ]
        }
    }
    async fn correct_item(
        &self,
        client: &Client<LobbyServerHandler, MiniGameProxy>,
        slot: usize,
    ) -> ItemStack {
        if slot < WindowType::Generic9x1.len() {
            self.items[slot].clone()
        } else if slot >= WindowType::Generic9x1.len() && slot < WindowType::Generic9x1.len() + 9 {
            client
                .player
                .read()
                .await
                .inventory
                .get_main_slot_from_container(slot - (WindowType::Generic9x1.len()))
                .clone()
        } else if slot >= WindowType::Generic9x1.len() + 27
            && slot < WindowType::Generic9x1.len() + 27 + 9
        {
            client
                .player
                .read()
                .await
                .inventory
                .get_hotbar_slot_from_container(slot - (WindowType::Generic9x1.len() + 27))
                .clone()
        } else {
            None
        }
    }
}

pub struct LobbyServerHandler {
    commands: CommandTree<Self, MiniGameProxy>,
    game_menu: Arc<GameMenu>,
    practice_menu: Arc<GameMenu>,
}

#[derive(Debug, Clone)]
pub enum MenuType {
    Play,
    Practice,
}

#[async_trait]
impl ServerHandler<MiniGameProxy> for LobbyServerHandler {
    type ServerGUI = GameMenu; // FIXME
    type PlayerHandler = LobbyPlayerHandler;
    fn new() -> Self {
        Self {
            game_menu: Arc::new(GameMenu::play()),
            practice_menu: Arc::new(GameMenu::practice()),
            commands: CommandTree::new()
                .register_command(
                    Command::new("practice", "practice a minigame in a private lobby").add_node(
                        CommandNode::server_argument(
                            "game",
                            ArgumentType::String {
                                string_type: StringType::SingleWord,
                            },
                            vec!["glide".to_string(), "battle".to_string()],
                        )
                        .with_execution(Box::new(
                            move |args, client, server, proxy| {
                                Box::pin(play(args, client, server, proxy))
                            },
                        )),
                    ),
                )
                .register_command(
                    Command::new("play", "play a minigame online")
                        .set_aliases(vec!["join"])
                        .add_node(
                            CommandNode::server_argument(
                                "game",
                                ArgumentType::String {
                                    string_type: StringType::SingleWord,
                                },
                                vec!["glide".to_string(), "battle".to_string()],
                            )
                            .with_execution(Box::new(
                                move |args, client, server, proxy| {
                                    Box::pin(queue_command(args, client, server, proxy))
                                },
                            )),
                        ),
                )
                .build_help_command(),
        }
    }
    fn get_world(&self) -> &'static World {
        &LOBBY_WORLD
    }
    async fn on_tick(&self, _server: &Server<Self, MiniGameProxy>, proxy: Arc<MiniGameProxy>) {
        proxy.glide_queue.lock().await.update(proxy.clone()).await;
        proxy.battle_queue.lock().await.update(proxy.clone()).await;
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
            Some(StackContents {
                id: Item::Compass,
                count: 1,
                nbt: ItemNbt {
                    display: Some(ItemNbtDisplay {
                        name: Some(
                            r#"{"text":"Play Online","italic":"false","color":"green"}"#
                                .to_string(),
                        ),
                        lore: None,
                    }),
                    ..Default::default()
                },
            }),
        );

        player.inventory.set_slot(
            InventorySlot::Hotbar { slot: 1 },
            Some(StackContents {
                id: Item::Stick,
                count: 1,
                nbt: ItemNbt {
                    display: Some(ItemNbtDisplay {
                        name: Some(
                            r#"{"text":"Practice","italic":"false","color":"green"}"#.to_string(),
                        ),
                        lore: None,
                    }),
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
    ) -> Result<&CommandTree<LobbyServerHandler, MiniGameProxy>, ConnectionError> {
        Ok(&self.commands)
    }
}

async fn play(
    args: Vec<Argument>,
    client: &Client<LobbyServerHandler, MiniGameProxy>,
    _server: &Server<LobbyServerHandler, MiniGameProxy>,
    _proxy: &MiniGameProxy,
) {
    let Argument::String { value } = args.get(0).expect("Arg not found") else {
        return
    };
    match value.as_str() {
        "battle" => {
            let game_id = client
                .proxy
                .battle_queue
                .lock()
                .await
                .create_server(client.proxy.clone());
            client.transfer(TransferData::Battle { game_id })
        }
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
                    "text": format!("error: game \"{value}\" does not exist"),
                }
            )
            .to_string(),
        ),
    }
}

async fn queue(
    client: &Client<LobbyServerHandler, MiniGameProxy>,
    proxy: &MiniGameProxy,
    game: MiniGame,
) {
    client
        .update_inventory_slot(
            InventorySlot::Hotbar { slot: 8 },
            Some(StackContents {
                id: Item::RedstoneBlock,
                count: 1,
                nbt: ItemNbt {
                    display: Some(ItemNbtDisplay {
                        name: Some(
                            r#"{"text":"Leave queue","italic":"false","color":"red"}"#.to_string(),
                        ),
                        lore: None,
                    }),
                    ..Default::default()
                },
            }),
        )
        .await;

    // client.player.write().await.inventory.set_slot(
    //     InventorySlot::Hotbar { slot: 8 },
    //     Some(Slot {
    //         item_id: VarInt(RedstoneBlock::ID as i32),
    //         item_count: 1,
    //         nbt: ItemNbt {
    //             ..Default::default()
    //         },
    //     }),
    // );
    // client
    //     .send_packet(SetContainerSlot {
    //         window_id: -2,
    //         state_id: VarInt(0),
    //         slot: 8,
    //         item: Some(Slot {
    //             item_id: VarInt(RedstoneBlock::ID as i32),
    //             item_count: 1,
    //             nbt: ItemNbt {
    //                 ..Default::default()
    //             },
    //         }),
    //     })
    //     .await;

    let uuid = client.client_data.uuid;

    let receiver = match game {
        MiniGame::Battle => proxy.battle_queue.lock().await.queue(uuid).await,
        MiniGame::Glide => proxy.glide_queue.lock().await.queue(uuid).await,
    };

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
        mini_game: game,
    });
}

async fn leave_queue(client: &Client<LobbyServerHandler, MiniGameProxy>) {
    client
        .update_inventory_slot(InventorySlot::Hotbar { slot: 8 }, None)
        .await;
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

async fn queue_command(
    args: Vec<Argument>,
    client: &Client<LobbyServerHandler, MiniGameProxy>,
    _server: &Server<LobbyServerHandler, MiniGameProxy>,
    proxy: &MiniGameProxy,
) {
    let Argument::String { value } = args.get(0).expect("Arg not found") else {
        return
    };
    match value.as_str() {
        "battle" => queue(client, proxy, MiniGame::Battle).await,
        "glide" => queue(client, proxy, MiniGame::Glide).await,
        value => client.show_chat_message(
            json!(
                {
                    "text": format!("error: game \"{value}\" does not exist", ),
                }
            )
            .to_string(),
        ),
    }
}

async fn leave_queue_command(
    _args: Vec<Argument>,
    client: &Client<LobbyServerHandler, MiniGameProxy>,
    _server: &Server<LobbyServerHandler, MiniGameProxy>,
    _proxy: &MiniGameProxy,
) {
    leave_queue(client).await;
}
