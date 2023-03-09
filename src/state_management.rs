use crate::*;

// Serialize the game state into bytes Vec<u8> to send through websocket.
pub fn serialize_state(state: &proto_all::GameState) -> Vec<u8> {
    let mut out = Vec::new();
    let mut writer = Writer::new(&mut out);

    writer.write_u8(0).unwrap(); // Header, in case we wanna use different headers in the future

    writer
        .write_message_no_len(state) // https://github.com/Descrout/quick-protobuf 'no_len' version of write of message.
        .expect("Cannot serialize state");

    return out;
}

// Serialize the ClientJoin message.
// This function should probably be moved to a helper module in ./networking
pub fn serialize_client_join(client_join_data: &proto_all::ClientJoined) -> Vec<u8> {
    let mut out = Vec::new();
    let mut writer = Writer::new(&mut out);

    writer.write_u8(2).unwrap();

    writer
        .write_message_no_len(client_join_data)
        .expect("Cannot serialize ClientJoin message.");

    return out;
}

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
}
