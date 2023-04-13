use std::{collections::VecDeque, sync::Arc};

use firework::{Server, ServerHandler, ServerProxy, TICKS_PER_SECOND};
use std::collections::HashMap;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio_util::sync::CancellationToken;

enum GameState {
    NoPlayers,
    Waiting { elapsed_ticks: usize },
    Starting { elapsed_ticks: usize },
}

#[derive(Debug, Clone)]
pub enum QueueMessage {
    NotEnoughPlayers,
    Starting {
        seconds: usize,
        connected_players: usize,
        max_players: usize,
    },
    Started {
        game_id: u128,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum QueueError {
    #[error("player is already in the queue")]
    AlreadyQueued,
}

pub struct Queue<Proxy, Handler, const MAX_PLAYERS: usize>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    servers: HashMap<u128, (Arc<Server<Handler, Proxy>>, CancellationToken)>,
    queue: VecDeque<QueuedPlayer>,
    state: GameState,
    brand: String,
}

impl<Proxy, Handler, const MAX_PLAYERS: usize> Queue<Proxy, Handler, MAX_PLAYERS>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    pub fn new(brand: String) -> Self {
        Self {
            servers: HashMap::new(),
            queue: VecDeque::new(),
            state: GameState::NoPlayers,
            brand,
        }
    }
    pub async fn queue(&mut self, uuid: u128) -> Result<Receiver<QueueMessage>, QueueError> {
        let (sender, receiver) = broadcast::channel(100);

        if self.queue.iter().any(|p| p.uuid == uuid) {
            return Err(QueueError::AlreadyQueued);
        }
        self.queue.push_back(QueuedPlayer { sender, uuid });

        Ok(receiver)
    }
    pub async fn leave_queue(&mut self, uuid: u128) {
        self.queue.retain(|p| p.uuid != uuid);
    }
    pub async fn update(&mut self, proxy: Arc<Proxy>) {
        if self.queue.len() < 2 {
            if self.queue.len() == 0 {
                self.state = GameState::NoPlayers;
                return;
            }

            let elapsed = if let GameState::Waiting {
                ref mut elapsed_ticks,
            } = self.state
            {
                *elapsed_ticks += 1;
                *elapsed_ticks
            } else {
                self.state = GameState::Waiting { elapsed_ticks: 0 };
                0
            };

            if elapsed % TICKS_PER_SECOND == 0 {
                for player in self.queue.iter() {
                    player.sender.send(QueueMessage::NotEnoughPlayers).unwrap();
                }
            }
            return;
        }

        let elapsed = if let GameState::Starting {
            ref mut elapsed_ticks,
        } = self.state
        {
            *elapsed_ticks += 1;
            *elapsed_ticks
        } else {
            self.state = GameState::Starting { elapsed_ticks: 0 };
            0
        };

        let percentage = self.queue.len() as f32 / MAX_PLAYERS as f32;

        let time_to_start;

        if percentage < 0.5 {
            time_to_start = 12 * TICKS_PER_SECOND;
        } else if percentage < 0.75 {
            time_to_start = 6 * TICKS_PER_SECOND;
        } else {
            time_to_start = 0 * TICKS_PER_SECOND;
        }

        if elapsed >= time_to_start {
            let game_id: u128 = self.create_server(proxy);

            for _ in 0..MAX_PLAYERS {
                let player = self.queue.pop_front();
                if let Some(player) = player {
                    player
                        .sender
                        .send(QueueMessage::Started { game_id })
                        .unwrap();
                }
            }
            return;
        }

        let time_remaining = time_to_start - elapsed;

        if time_remaining % TICKS_PER_SECOND == 0 {
            let seconds = time_remaining / TICKS_PER_SECOND;
            for player in self.queue.iter() {
                player
                    .sender
                    .send(QueueMessage::Starting {
                        seconds,
                        connected_players: self.queue.len(),
                        max_players: MAX_PLAYERS,
                    })
                    .unwrap();
            }
        }
    }
    pub fn get_server(&self, game_id: u128) -> Option<Arc<Server<Handler, Proxy>>> {
        self.servers.get(&game_id).map(|(server, _)| server.clone())
    }
    pub fn create_server(&mut self, proxy: Arc<Proxy>) -> u128 {
        let game_id: u128 = rand::random();
        let server = Server::new(self.brand.clone(), game_id);

        let cancel = CancellationToken::new();
        let run_cancel = cancel.clone();
        let run_server = server.clone();
        tokio::spawn(async move {
            run_server.run(proxy, run_cancel).await;
        });

        self.servers.insert(game_id, (server.clone(), cancel));

        game_id
    }
    pub fn remove_server(&mut self, game_id: u128) {
        if let Some((_, token)) = self.servers.remove(&game_id) {
            token.cancel();
        }
    }
}

struct QueuedPlayer {
    sender: Sender<QueueMessage>,
    uuid: u128,
}
