use crate::*;

pub async fn interval_broadcast(
    mut event_rx: mpsc::UnboundedReceiver<BroadcastEvents>,
    state_update_rx: watch::Receiver<proto_all::GameStateUpdate>,
    tick_receiver: watch::Receiver<u8>,
) {
    let mut connections: HashMap<u32, Connection> = HashMap::new();
    let mut interval = tokio::time::interval(time::Duration::from_millis(1000 / 20));
    loop {
        tokio::select! {
            event = event_rx.recv() => {
                match event {
                    Some(BroadcastEvents::Join(new_client)) => {
                        let new_client_id = new_client.id;
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
                    Some(BroadcastEvents::StateOut(state)) => {
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
