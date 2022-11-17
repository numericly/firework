use std::time::Duration;
use tokio::time::sleep;

mod client;
mod commands;
mod server;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let server = server::ServerManager::new();

    tokio::spawn(async move {
        web_interface::web_interface::run_web_interface()
            .await
            .unwrap();
    });

    sleep(Duration::from_secs(10000000000)).await;
}
