use authentication::authenticate;
use data::v1_19_2::chunk::Chunk;
use data::v1_19_2::tags::TAGS;
use protocol::client_bound::{
    ChangeDifficulty, ChunkUpdateAndLightUpdate, ClientBoundKeepAlive, EncryptionRequest,
    InitializeWorldBorder, LoginDisconnect, LoginSuccess, LoginWorld, PlayDisconnect,
    PlayerAbilities, Pong, ServerStatus, SetCenterChunk, SetCompression, SetHeldItem, SetRecipes,
    SetTags, SynchronizePlayerPosition,
};
use protocol::protocol::{ConnectionState, Protocol, ProtocolError};
use protocol::server_bound::ServerBoundPacket;

use protocol::data_types::{PlayerAbilityFlags, PlayerPositionFlags, TestBytes, VarInt};
use quartz_nbt::{snbt, NbtCompound};
use rand::rngs::OsRng;
use rand::{Rng, RngCore};
use server::VanillaServerHandler;
use server_state::player::{Player, Rotation, Vec3};
use server_state::server::Server;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use world::world::{World, Write};

mod client;
mod commands;
mod server;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let server = server::Server::new();

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
