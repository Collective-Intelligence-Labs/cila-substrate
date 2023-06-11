// Automatically generated rust module for 'operation.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use alloc::vec::Vec;
use alloc::borrow::ToOwned;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result, PackedFixed, PackedFixedIntoIter, PackedFixedRefIter};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Operation {
    pub router_id: Vec<u8>,
    pub commands: Vec<command::Command>,
}

impl<'a> MessageRead<'a> for Operation {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.router_id = r.read_bytes(bytes)?.to_owned(),
                Ok(18) => msg.commands.push(r.read_message::<command::Command>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Operation {
    fn get_size(&self) -> usize {
        0
        + if self.router_id == Vec::<u8>::new() { 0 } else { 1 + sizeof_len(self.router_id.len()) }
        + self.commands.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.router_id != Vec::<u8>::new() { w.write_with_tag(10, |w| w.write_bytes(&self.router_id))?; }
        for s in &self.commands { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

