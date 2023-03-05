mod connection;
mod events;
mod proto;
mod game;

extern crate env_logger;
use log::*;

use futures_util::{FutureExt, SinkExt};
use quick_protobuf::{BytesReader, Writer};

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use connection::Connection;
use events::{BroadcastEvents, GameEvents};

use futures_util::stream::StreamExt;
use tokio::task::unconstrained;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

use std::collections::HashMap;
use std::{thread, time};

use crate::game::Game;
use crate::proto::proto_all;
use quick_protobuf::MessageRead;

const PORT: &str = "8080";
const TIMESTEP: f32 = 1.0 / 60.0; // 60tps server

#[tokio::main]
async fn main() {
    _ = env_logger::builder()
    .filter_level(LevelFilter::Info)
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
    tokio::spawn(broadcast(broadcast_receiver));
    /*
        Since I will only use one game loop, I'm using an actual std::thread for the game loop.
        This function takes ownership of the 'broadcast_sender' to send events into the 'broadcast' green thread.
    */
    let (game_sender, game_receiver) = mpsc::unbounded_channel::<GameEvents>();
    thread::spawn(move || run(broadcast_sender, game_receiver));
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


// Serialize the game state into bytes Vec<u8> to send through websocket.
fn serialize_state(state: &proto_all::State) -> Vec<u8> {
    let mut out = Vec::new();
    let mut writer = Writer::new(&mut out);

    writer.write_u8(0).unwrap(); // Header, in case we wanna use different headers in the future

    writer
        .write_message_no_len(state) // https://github.com/Descrout/quick-protobuf 'no_len' version of write of message.
        .expect("Cannot serialize state");

    out
}

/*
    60 FPS fixed game loop.
    Update the internal game state and send it to broadcast green thread without any blocking.
*/
fn run(tx: UnboundedSender<BroadcastEvents>, mut receiver: UnboundedReceiver<GameEvents>) {
    // Initialize the game state
    let mut game = Game::new();
    game.init();

    // Time variables
    let sixteen_ms = time::Duration::from_millis(16);
    let mut accum = 0.0;
    let mut dt = 0.0;

    // Start the loop
    loop {
        let start = time::Instant::now();
        /*
            If we have any game event we process those events and continue to update the game.
            If we don't have any game event, we do nothing.
            We do not block here.
         */
        while let Some(is_event) = unconstrained(receiver.recv()).now_or_never() {
            if let Some(event) = is_event {
                match event {
                    GameEvents::Join(conn) => {
                        //game.add_player(conn.id);
                        let _ = tx.send(BroadcastEvents::Join(conn));
                    }
                    GameEvents::Quit(user_id) => {
                        //game.remove_player(user_id);
                        let _ = tx.send(BroadcastEvents::Quit(user_id));
                    }
                    GameEvents::Input(id, input) => {
                        //game.set_input(id, input);
                        let _ = tx.send(BroadcastEvents::StateOut(proto_all::State{entities: Vec::new(), bodies: Vec::new()}));
                    }
                }
            }
        }

        // Fixed game loop
        accum += dt;
        while accum >= TIMESTEP {
            accum -= TIMESTEP;

            // Update the game state (in our case rapier.rs physics simulation and intersection queries)
            game.update();

            // Send the game state to broadcast green thread.
            //let _ = tx.send(BroadcastEvents::StateOut(game.get_state()));
        }

        thread::sleep(sixteen_ms);
        dt = start.elapsed().as_secs_f32();
    }
}

// Broadcast all the incoming game state to the clients.
async fn broadcast(mut rx: UnboundedReceiver<BroadcastEvents>) {
    let mut connections: HashMap<u32, Connection> = HashMap::new();

    // Runs for the entire duration of the server.
    // Code gets executed when an event is received via the rx channel receiver.
    while let Some(event) = rx.recv().await { 
        match event {
            BroadcastEvents::Join(conn) => {
                connections.insert(conn.id, conn);
            }
            BroadcastEvents::Quit(id) => {
                connections.remove(&id);
                info!("Connection lost with client id: {}", id);
            }
            BroadcastEvents::StateOut(state) => {
                info!("Broadcasting state");
                for (_, conn) in connections.iter_mut() {
                    let data = serialize_state(&state);
                    let _ = conn.sender.send(Message::Binary(data)).await.expect("Failed to send.");
                }
            }
        }
    }
}

// Listen for incoming data from clients.
async fn listen(
    game_sender: UnboundedSender<GameEvents>,
    ws_stream: WebSocketStream<TcpStream>,
    id: u32,
) {
    let (sender, mut receiver) = ws_stream.split();
    let conn = Connection::new(id, sender);
    let _ = game_sender.send(GameEvents::Join(conn));
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            if msg.is_binary() {
                info!("Received message: {}", msg.to_string());
                let mut msg = msg.into_data();
                if msg.len() <= 0 {
                    info!("Received a message with a length of 0 or less. Not processing.");
                    break
                }
                let header = msg.remove(0);
                let mut reader = BytesReader::from_bytes(&msg);
                if header == 0 {
                    if let Ok(input) = proto_all::GameInput::from_reader(&mut reader, &msg) {
                        println!("Received the following GameInput:\nx: {}, y: {}, pressed: {}", input.x, input.y, input.pressed);
                        let _ = game_sender.send(GameEvents::Input(id, input));
                    }
                }
            } else if msg.is_close() {
                break; // When we break, we disconnect.
            }
        } else {
            break; // When we break, we disconnect.
        }
    }
    // If we reach here, it means the client got disconnected.
    // Send quit event to game loop, and the game loop will send quit event to the broadcast thread.
    // So all cleanups will be done.
    game_sender.send(GameEvents::Quit(id)).unwrap();
}
