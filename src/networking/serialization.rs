use crate::*;

// Serialize a message from a struct inside the proto module to a Message::Binary that will be sent over the websocket.
pub fn proto_serialize<T: quick_protobuf::MessageWrite>(data: T, message_header: u8) -> Vec<u8> {
    let mut out = Vec::new();
    let mut writer = Writer::new(&mut out);

    writer.write_u8(message_header).unwrap();

    writer
        .write_message_no_len(&data)
        .expect("Cannot serialize message.");

    return out;
}

