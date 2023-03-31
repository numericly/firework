use std::{char::MAX, collections::VecDeque, sync::Arc, time::Instant};

use firework::{Server, ServerHandler, ServerProxy};
use tokio::sync::broadcast::{self, Receiver, Sender};

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
    Started,
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
    servers: Vec<Arc<Server<Handler, Proxy>>>,
    queue: VecDeque<QueuedPlayer>,
    state: GameState,
}

const TICKS_PER_SECOND: usize = 20;

impl<Proxy, Handler, const MAX_PLAYERS: usize> Queue<Proxy, Handler, MAX_PLAYERS>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
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
    pub async fn update(&mut self) {
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
            time_to_start = 25 * TICKS_PER_SECOND;
        } else if percentage < 0.75 {
            time_to_start = 10 * TICKS_PER_SECOND;
        } else {
            time_to_start = 0 * TICKS_PER_SECOND;
        }

        if elapsed >= time_to_start {
            for _ in 0..MAX_PLAYERS {
                let player = self.queue.pop_front();
                if let Some(player) = player {
                    player.sender.send(QueueMessage::Started).unwrap();
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
}

impl<Proxy, Handler, const MAX_PLAYERS: usize> Default for Queue<Proxy, Handler, MAX_PLAYERS>
where
    Proxy: ServerProxy + Send + Sync + 'static,
    Handler: ServerHandler<Proxy> + Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            servers: Vec::new(),
            queue: VecDeque::new(),
            state: GameState::NoPlayers,
        }
    }
}

struct QueuedPlayer {
    sender: Sender<QueueMessage>,
    uuid: u128,
}
