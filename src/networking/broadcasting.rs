use crate::*;

pub async fn interval_broadcast(
    mut event_rx: mpsc::UnboundedReceiver<BroadcastEvents>) {
    let mut connections: HashMap<u32, Connection> = HashMap::new();
    loop {
        tokio::select! {
            event = event_rx.recv() => {
                match event {
                    Some(BroadcastEvents::Join(mut new_client, initial_state_message)) => {
                        let new_client_id = new_client.id;
                        info!("A new client has joined the game. Sending their client_id ({}) with the full game state", new_client_id);
                        new_client.sender.send( // Sending the full state to the client
                            Message::binary(proto_serialize(initial_state_message, 11)))
                                .await.expect("Failed to send full state to a client on Join.");

                        connections.insert(new_client_id, new_client);
                        // Send the id of the new client to all other clients
                        for (_, conn) in connections.iter_mut() {
                            let data = proto_serialize(conn_event_messages::ClientConnect { client_id:  new_client_id}.to_owned(), 0);

                            let _ = conn
                                .sender
                                .send(Message::Binary(data))
                                .await
                                .unwrap_or_default();
                        }
                    }
                    Some(BroadcastEvents::Quit(id)) => {
                        connections.remove(&id);
                        info!("Connection lost with client {}", id);

                        let data = proto_serialize(conn_event_messages::ClientDisconnect{client_id: id}, 1);
                        for (_, conn) in connections.iter_mut() {
                            //info!("Sending state update to client {}", conn.id);
                            let _ = conn
                                .sender
                                .send(Message::Binary(data.clone()))
                                .await
                                .unwrap_or_default();
                        }
                    }
                    Some(BroadcastEvents::StateUpdateOut(state)) => {
                        let data = proto_serialize(state, 10);
                        for (_, conn) in connections.iter_mut() {
                            //info!("Sending state update to client {}", conn.id);
                            let _ = conn
                                .sender
                                .send(Message::Binary(data.clone()))
                                .await
                                .unwrap_or_default();
                        }
                    }
                    None => {

                    }
                }
            }
        }
    }
}
