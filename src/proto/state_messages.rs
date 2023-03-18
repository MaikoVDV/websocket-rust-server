// Automatically generated rust module for 'state-messages.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct GameStateUpdate {
    pub players: Vec<generic_protobufs::Player>,
    pub bodies: Vec<generic_protobufs::Body>,
}

impl<'a> MessageRead<'a> for GameStateUpdate {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.players.push(r.read_message::<generic_protobufs::Player>(bytes)?),
                Ok(18) => msg.bodies.push(r.read_message::<generic_protobufs::Body>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GameStateUpdate {
    fn get_size(&self) -> usize {
        0
        + self.players.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.bodies.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.players { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.bodies { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct InitialState {
    pub client_id: u32,
    pub full_state: Option<GameStateUpdate>,
}

impl<'a> MessageRead<'a> for InitialState {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.client_id = r.read_uint32(bytes)?,
                Ok(18) => msg.full_state = Some(r.read_message::<GameStateUpdate>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for InitialState {
    fn get_size(&self) -> usize {
        0
        + if self.client_id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.client_id) as u64) }
        + self.full_state.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.client_id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.client_id))?; }
        if let Some(ref s) = self.full_state { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

