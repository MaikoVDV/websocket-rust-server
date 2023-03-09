use crate::*;

/*
Update the internal game state and send it to broadcast green thread without any blocking.
*/
pub async fn run(
    broadcast_event_sender: mpsc::UnboundedSender<BroadcastEvents>,
    state_sender: watch::Sender<proto_all::GameStateUpdate>,
    mut game_event_receiver: mpsc::UnboundedReceiver<GameEvents>,
    tick_sender: watch::Sender<u8>,
) {
    info!("run() called!");
    // Time variables
    // let sixteen_ms = time::Duration::from_millis(16);
    // let mut accum = 0.0;
    // let mut dt = 0.0;

    let mut interval =
        tokio::time::interval(time::Duration::from_millis(1000 / TICKS_PER_SECOND));
        info!("Interval duration: {}", interval.period().as_millis());

    // Initialize the game state
    let mut game = Game::new();
    game.init();
    // Start the loop
    loop {
        // /*
        // If we have any game event we process those events and continue to update the game.
        // If we don't have any game event, we do nothing.
        // We do not block here.
        // */
        // while let Some(is_event) = unconstrained(game_event_receiver.recv()).now_or_never() {
        //     if let Some(event) = is_event {
        //         match event {
        //             GameEvents::Join(conn) => {
        //                 game.add_player(conn.id);
        //                 let _ = broadcast_event_sender.send(BroadcastEvents::Join(conn));
        //             }
        //             GameEvents::Quit(user_id) => {
        //                 game.remove_player(user_id);
        //                 let _ = broadcast_event_sender.send(BroadcastEvents::Quit(user_id));
        //             }
        //             GameEvents::Input(id, input) => {
        //                 game.set_input(id, input);
        //                 //let new_state = proto_all::State::default(); // SOME ACTUAL STATE STILL NEEDS TO BE SAVED AT SOME POINT IN THE FUTURE!!!
        //                 //let _ = event_sender.send(BroadcastEvents::StateOut(new_state));
        //             }
        //         }
        //     }
        // }

        // // Fixed game loop
        // accum += dt;
        // while accum >= TIMESTEP {
        //     accum -= TIMESTEP;

        //     // Update the game state (in our case rapier.rs physics simulation and intersection queries)
        //     game.update();

        //     // Send the game state to broadcast green thread.
        //     let _ = broadcast_event_sender.send(BroadcastEvents::StateOut(game.get_state())); // Should prob be removed in favor of the following method.
        //     let _ = state_sender.send(game.get_state_updates());
        // }
        // 
        // thread::sleep(sixteen_ms);
        // dt = start.elapsed().as_secs_f32();

        //let start = time::Instant::now();
        tokio::select! {
            game_event = game_event_receiver.recv() => {
                if let Some(event) = game_event {
                    match event {
                        GameEvents::Join(conn) => {
                            game.add_player(conn.id);
                            let _ = broadcast_event_sender.send(BroadcastEvents::Join(conn));
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
                let _ = broadcast_event_sender.send(BroadcastEvents::StateOut(game.get_state())); // Should prob be removed in favor of the following method.
                //let _ = state_sender.send(game.get_state_updates());
            }
        }
    }
}

