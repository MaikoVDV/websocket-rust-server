// Importing local modules
// mod networking::connection;
// mod broadcasting;
mod events;
mod gameplay;
mod networking;
mod proto;

// Importing from local modules
use events::{BroadcastEvents, GameEvents};
use gameplay::{game::Game, game_manager::run};
use proto::*;
use networking::serialization::proto_serialize;

use networking::{broadcasting::interval_broadcast, connection::Connection, listening::listen};

// Logging
extern crate env_logger;
use log::*;

// Networking & Multithreading (tokio)
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, watch};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

use quick_protobuf::{BytesReader, MessageRead, Writer};

// Futures
use futures_util::stream::StreamExt;
use futures_util::{FutureExt, SinkExt};

// Standard Library imports
use std::{collections::HashMap, thread, time};

const PORT: &str = "8080";
const TICKS_PER_SECOND: u64 = 20; // 20tps gameloop & broadcasting

#[tokio::main]
async fn main() {
    _ = env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_secs()
        .format_module_path(false)
        .format_target(false)
        .format_indent(Some(4))
        .try_init(); // Setting up a logger with timestaps

    let addr = format!("127.0.0.1:{}", PORT);

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Listening to TCP failed.");

    /*
        Broadcast data to all clients in a seperate async tokio green thread.
        The game loop will use 'broadcast_sender' to send the game state,
        and join&quit events into this function.
    */
    let (broadcast_sender, broadcast_receiver) = mpsc::unbounded_channel::<BroadcastEvents>();
    tokio::spawn(interval_broadcast(broadcast_receiver));
    /*
        Since I will only use one game loop, I'm using an actual std::thread for the game loop.
        This function takes ownership of the 'broadcast_sender' to send events into the 'broadcast' green thread.
    */
    let (game_sender, game_receiver) = mpsc::unbounded_channel::<GameEvents>();
    tokio::spawn(run(broadcast_sender, game_receiver));
    info!("Listening on port {}", PORT);

    // A counter to use as client ids.
    let mut id = 0;

    // Accept new clients.
    while let Ok((stream, peer)) = listener.accept().await {
        match tokio_tungstenite::accept_async(stream).await {
            Err(e) => info!("Websocket connection error : {}", e),
            Ok(ws_stream) => {
                id += 1;
                info!("New Connection: {} | Set id to: {}", peer, id);
                tokio::spawn(listen(game_sender.clone(), ws_stream, id));
            }
        }
    }
}
