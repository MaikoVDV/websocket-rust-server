use crate::*;

// Serialize the game state into bytes Vec<u8> to send through websocket.
pub fn serialize_state(state: &proto_all::State) -> Vec<u8> {
    let mut out = Vec::new();
    let mut writer = Writer::new(&mut out);

    writer.write_u8(0).unwrap(); // Header, in case we wanna use different headers in the future

    writer
        .write_message_no_len(state) // https://github.com/Descrout/quick-protobuf 'no_len' version of write of message.
        .expect("Cannot serialize state");

    out
}

impl Game {
    pub fn get_state(&self) -> proto_all::State {
        let mut state = proto_all::State {
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

        return state
    }
}
