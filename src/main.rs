use std::time::Duration;

use futures::channel::oneshot::Receiver;
use tokio::{sync::mpsc, time::sleep};
use web_interface::web_interface::{ApiPlayer, ApiServer, ApiWorld};

mod client;
mod commands;
mod server;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let (tx, mut rx) = mpsc::channel::<bool>(100);

    let server = server::ServerManager::new();

    tokio::spawn(async move { web_interface::web_interface::web_interface().await });

    sleep(Duration::from_secs(10000000000)).await;
}
