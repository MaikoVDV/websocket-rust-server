use crate::*;

pub async fn interval_broadcast(
    mut event_rx: mpsc::UnboundedReceiver<BroadcastEvents>,
    state_rx: watch::Receiver<proto_all::GameState>,
) {
    let mut connections: HashMap<u32, Connection> = HashMap::new();
    let mut interval = tokio::time::interval(time::Duration::from_millis(1000 / 20));
    loop {
        tokio::select! {
            event = event_rx.recv() => {
                match event {
                    Some(BroadcastEvents::Join(conn)) => {
                        connections.insert(conn.id, conn);
                    }
                    Some(BroadcastEvents::Quit(id)) => {
                        connections.remove(&id);
                        info!("Connection lost with client id: {}", id);
                    }
                    Some(BroadcastEvents::StateOut(_)) => {
                        // Received an input from some client,
                        // But not broadcasting state, because that's not very nice for performance.
                        // State will be transmitted on tick.
                    }
                    None => {

                    }
                }
            }
            _ = interval.tick() => {
                for (_, conn) in connections.iter_mut() {
                    let data = serialize_state(&state_rx.borrow());
                    let _ = conn
                        .sender
                        .send(Message::Binary(data))
                        .await
                        .unwrap_or_default();
                }
            }
        }
    }
}
