use crate::networking::connection::Connection;
use crate::proto::{generic_protobufs::ClientInput, state_messages::*};

#[derive(Debug)]
pub enum GameEvents {
    Join(Connection),
    Quit(u32),
    Input(u32, ClientInput),
}

#[derive(Debug)]
pub enum BroadcastEvents {
    Join(Connection, InitialState),
    Quit(u32),
    StateUpdateOut(GameStateUpdate),
}
