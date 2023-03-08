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