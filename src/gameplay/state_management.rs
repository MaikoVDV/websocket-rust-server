use crate::*;

impl Game {
    pub fn get_state_updates(&self) -> state_messages::GameStateUpdate {
        let mut state = state_messages::GameStateUpdate {
            players: Vec::new(),
            bodies: Vec::new(),
        };

        for (_, entity) in self.game_state_updates.players.iter() {
            state.players.push(entity.clone());
        }

        for body in self.game_state_updates.bodies.iter() {
            state.bodies.push(generic_protobufs::Body {
                id: *body.0,
                color: body.1.color.to_owned(),
                x: body.1.x,
                y: body.1.y,
                w: body.1.w,
                h: body.1.h,
                rotation: body.1.rotation,
            });
        }

        return state;
    }
}
