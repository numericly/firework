use server::ServerManager;
use std::time::Duration;
use tokio::time::sleep;

mod client;
// mod commands;
mod server;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let _server = ServerManager::run();

    sleep(Duration::from_secs(10000000000)).await;

    // let listener = TcpListener::bind("127.0.0.1:25566").await.unwrap();
    // let server = Arc::new(Server::new());

    // loop {
    //     let stream = listener.accept().await.unwrap().0;
    //     let server = Arc::clone(&server);

    //     tokio::task::spawn(async move {
    //         Client::handle(stream).await.unwrap();
    //         // if let Err(e) = handle_client(stream, server).await {
    //         //     println!("Error: {}", e);
    //         // }
    //     });
    // }
}
