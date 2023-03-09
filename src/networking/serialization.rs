use crate::*;

// Serialize a message from a struct inside the proto module to a Message::Binary that will be sent over the websocket.
pub fn proto_serialize<T: quick_protobuf::MessageWrite>(client_join_data: T, message_header: u8) -> Vec<u8> {
    let mut out = Vec::new();
    let mut writer = Writer::new(&mut out);

    writer.write_u8(message_header).unwrap();

    writer
        .write_message_no_len(&client_join_data)
        .expect("Cannot serialize ClientJoin message.");

    return out;
}