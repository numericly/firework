use std::{sync::Mutex, time::Duration};
use tokio::time::sleep;
use web_interface::web_interface::{ApiData, ApiServer, ApiWorld};

mod client;
mod commands;
mod server;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let server = server::ServerManager::new();

    tokio::spawn(async move {
        web_interface::web_interface::run_web_interface(ApiData {
            players: Vec::new(),
            world: ApiWorld {
                name: "world".to_string(),
                time: 0,
                day: 0,
                weather: "clear".to_string(),
                difficulty: "easy".to_string(),
                gamemode: "survival".to_string(),
                chunks: Vec::new(),
            },
            server: ApiServer {
                version: "1.19.2".to_string(),
                max_players: 0,
                render_distance: 0,
                ram: 0,
                tps: 0.0,
                cpu: 0.0,
                console: Vec::new(),
            },
        })
        .await
        .unwrap();
    });

    sleep(Duration::from_secs(10000000000)).await;
}
