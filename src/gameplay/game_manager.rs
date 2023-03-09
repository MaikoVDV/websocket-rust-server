use crate::*;

/*
60 FPS fixed game loop.
Update the internal game state and send it to broadcast green thread without any blocking.
*/
pub fn run(
    event_sender: mpsc::UnboundedSender<BroadcastEvents>,
    state_sender: watch::Sender<proto_all::GameState>,
    mut receiver: mpsc::UnboundedReceiver<GameEvents>,
) {
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
                        game.add_player(conn.id);
                        let _ = event_sender.send(BroadcastEvents::Join(conn));
                    }
                    GameEvents::Quit(user_id) => {
                        game.remove_player(user_id);
                        let _ = event_sender.send(BroadcastEvents::Quit(user_id));
                    }
                    GameEvents::Input(id, input) => {
                        game.set_input(id, input);
                        //let new_state = proto_all::State::default(); // SOME ACTUAL STATE STILL NEEDS TO BE SAVED AT SOME POINT IN THE FUTURE!!!
                        //let _ = event_sender.send(BroadcastEvents::StateOut(new_state));
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
            let _ = event_sender.send(BroadcastEvents::StateOut(game.get_state())); // Should prob be removed in favor of the following method.
            let _ = state_sender.send(game.get_state());
        }

        thread::sleep(sixteen_ms);
        dt = start.elapsed().as_secs_f32();
    }
}

