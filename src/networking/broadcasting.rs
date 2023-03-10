use crate::*;

pub async fn interval_broadcast(mut event_rx: mpsc::UnboundedReceiver<BroadcastEvents>) {
    let mut connections: HashMap<u32, Connection> = HashMap::new();
    loop {
        tokio::select! {
            event = event_rx.recv() => {
                match event {
                    Some(BroadcastEvents::Join(mut new_client, full_game_state)) => {
                        let new_client_id = new_client.id;
                        new_client.sender.send( // Sending the full state to the client
                            Message::binary(proto_serialize(full_game_state, 0))).await.expect("Failed to send full state to a client on Join.");

                        connections.insert(new_client_id, new_client);
                        // Send the id of the new client to all other clients
                        for (_, conn) in connections.iter_mut() {
                            let data = proto_serialize(proto_all::ClientJoined { client_id:  new_client_id}.to_owned(), 2);

                            let _ = conn
                                .sender
                                .send(Message::Binary(data))
                                .await
                                .unwrap_or_default();
                        }
                    }
                    Some(BroadcastEvents::Quit(id)) => {
                        connections.remove(&id);
                        info!("Connection lost with client id: {}", id);
                    }
                    Some(BroadcastEvents::StateOut(_state)) => {

                    }
                    Some(BroadcastEvents::StateUpdateOut(state)) => {
                        // Received an input from some client,
                        // But not broadcasting state, because that's not very nice for performance.
                        // State will be transmitted on tick.
                        let data = proto_serialize(state, 3);
                        for (_, conn) in connections.iter_mut() {
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
            // _ = interval.tick() => {
            //     let data = proto_serialize(state_update_rx.borrow().to_owned(), 3);
            //     for (_, conn) in connections.iter_mut() {
            //         let _ = conn
            //             .sender
            //             .send(Message::Binary(data.clone()))
            //             .await
            //             .unwrap_or_default();
            //     }
            //     // When state updates have been sent to all clients, reset the state update struct.
            //     // want to do game.reset().
            // }
        }
    }
}
