use crate::networking::connection::Connection;
use crate::proto::proto_all::*;

#[derive(Debug)]
pub enum GameEvents {
    Join(Connection),
    Quit(u32),
    Input(u32, ClientInput),
}

#[derive(Debug)]
pub enum BroadcastEvents {
    Join(Connection),
    Quit(u32),
    StateOut(GameState),
}
