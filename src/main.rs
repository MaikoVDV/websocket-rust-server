// Importing local modules
// mod networking::connection;
// mod broadcasting;
mod networking;
mod events;
mod game;
mod game_manager;
mod proto;
mod state_management;

// Importing from local modules
use game::Game;
use game_manager::run;
use proto::proto_all;
use state_management::serialize_state;
use events::{BroadcastEvents, GameEvents};

use networking::{
    connection::Connection,
    broadcasting::interval_broadcast,
    listening::listen,
};

// Logging
extern crate env_logger;
use log::*;

// Networking & Multithreading (tokio)
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{
    mpsc,
    watch,
};
use tokio::task::unconstrained;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

use quick_protobuf::{BytesReader, Writer, MessageRead};

// Futures
use futures_util::stream::StreamExt;
use futures_util::{FutureExt, SinkExt};


// Standard Library imports
use std::{
    thread,
    time,
    collections::HashMap,
};


const PORT: &str = "8080";
const TIMESTEP: f32 = 1.0 / 60.0; // 60tps server

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
    let (state_sender, state_receiver) = watch::channel::<proto_all::State>(proto_all::State::default());
    tokio::spawn(interval_broadcast(broadcast_receiver, state_receiver));
    /*
        Since I will only use one game loop, I'm using an actual std::thread for the game loop.
        This function takes ownership of the 'broadcast_sender' to send events into the 'broadcast' green thread.
    */
    let (game_sender, game_receiver) = mpsc::unbounded_channel::<GameEvents>();
    thread::spawn(move || run(broadcast_sender, state_sender, game_receiver));
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