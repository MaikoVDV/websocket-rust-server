use crate::*;

// Listen for incoming data from clients.
pub async fn listen(
    game_sender: mpsc::UnboundedSender<GameEvents>,
    ws_stream: WebSocketStream<TcpStream>,
    id: u32,
) {
    let (sender, mut receiver) = ws_stream.split();
    let conn = Connection::new(id, sender);

    let _ = game_sender.send(GameEvents::Join(conn));
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            if msg.is_binary() {
                //info!("Received message: {}", msg.to_string());
                let mut msg = msg.into_data();
                if msg.len() <= 0 {
                    error!("Received a message with a length of 0 or less. Not processing.");
                    continue;
                }
                let header = msg.remove(0);
                let mut reader = BytesReader::from_bytes(&msg);
                if header == 0 {
                    if let Ok(input) = proto_all::GameInput::from_reader(&mut reader, &msg) {
                        info!(
                            "Received the following GameInput from client {}:\nx: {}, y: {}, pressed: {}",
                            id, input.x, input.y, input.pressed
                        );
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
