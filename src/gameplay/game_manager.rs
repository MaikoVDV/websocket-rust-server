use crate::*;

/*
Update the internal game state and send it to broadcast green thread without any blocking.
*/
pub async fn run(
    broadcast_event_sender: mpsc::UnboundedSender<BroadcastEvents>,
    mut game_event_receiver: mpsc::UnboundedReceiver<GameEvents>,
) {
    let mut interval = tokio::time::interval(time::Duration::from_millis(1000 / TICKS_PER_SECOND));

    // Initialize the game state
    let mut game = Game::new();
    game.init();
    // Start the loop
    loop {
        //let start = time::Instant::now();
        tokio::select! {
            game_event = game_event_receiver.recv() => {
                if let Some(event) = game_event {
                    match event {
                        GameEvents::Join(conn) => {
                            game.add_player(conn.id);
                            let initial_state_message = proto_all::InitialState {
                                client_id: conn.id,
                                full_state: Some(proto_all::GameStateUpdate {
                                    // Converting entities & bodies from HashMap to Vec<>
                                    players: game.players.values().cloned().collect(),
                                    bodies: game.bodies.values().cloned().collect(),
                                })
                            };
                            let _ = broadcast_event_sender.send(
                                BroadcastEvents::Join(conn, initial_state_message));
                        }
                        GameEvents::Quit(user_id) => {
                            game.remove_player(user_id);
                            let _ = broadcast_event_sender.send(BroadcastEvents::Quit(user_id));
                        }
                        GameEvents::Input(id, input) => {
                            game.set_input(id, input);
                            //let new_state = proto_all::State::default(); // SOME ACTUAL STATE STILL NEEDS TO BE SAVED AT SOME POINT IN THE FUTURE!!!
                            //let _ = event_sender.send(BroadcastEvents::StateOut(new_state));
                        }
                    }
                }
            }
            _ = interval.tick() => {
                // Update the game state (in our case rapier.rs physics simulation and intersection queries)
                game.update();

                // Send the game state to broadcast green thread.
                let _ = broadcast_event_sender.send(BroadcastEvents::StateUpdateOut(game.get_state_updates()));
                game.game_state_updates.reset();
            }
        }
    }
}
