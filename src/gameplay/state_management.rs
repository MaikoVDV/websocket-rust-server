use crate::*;

impl Game {
    pub fn get_state(&self) -> proto_all::GameState {
        let mut state = proto_all::GameState {
            entities: Vec::new(),
            bodies: Vec::new(),
        };

        for (_, entity) in self.players.iter() {
            state.entities.push(entity.clone());
        }

        for body in self.bodies.iter() {
            state.bodies.push(proto_all::Body {
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
    pub fn get_state_updates(&self) -> proto_all::GameStateUpdate {
        let mut state = proto_all::GameStateUpdate {
            entities: Vec::new(),
            bodies: Vec::new(),
        };

        for (_, entity) in self.game_state_updates.players.iter() {
            state.entities.push(entity.clone());
        }

        for body in self.game_state_updates.bodies.iter() {
            state.bodies.push(proto_all::Body {
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
