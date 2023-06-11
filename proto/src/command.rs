// Automatically generated rust module for 'command.proto' file

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandType {
    UNSPECIFIED_COMMAND = 0,
    MINT_NFT = 1,
    TRANSFER_NFT = 2,
}

impl Default for CommandType {
    fn default() -> Self {
        CommandType::UNSPECIFIED_COMMAND
    }
}

impl From<i32> for CommandType {
    fn from(i: i32) -> Self {
        match i {
            0 => CommandType::UNSPECIFIED_COMMAND,
            1 => CommandType::MINT_NFT,
            2 => CommandType::TRANSFER_NFT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for CommandType {
    fn from(s: &'a str) -> Self {
        match s {
            "UNSPECIFIED_COMMAND" => CommandType::UNSPECIFIED_COMMAND,
            "MINT_NFT" => CommandType::MINT_NFT,
            "TRANSFER_NFT" => CommandType::TRANSFER_NFT,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Command {
    pub aggregate_id: Vec<u8>,
    pub cmd_signature: Vec<u8>,
    pub cmd_type: CommandType,
    pub cmd_payload: Vec<u8>,
}

impl<'a> MessageRead<'a> for Command {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.aggregate_id = r.read_bytes(bytes)?.to_owned(),
                Ok(18) => msg.cmd_signature = r.read_bytes(bytes)?.to_owned(),
                Ok(24) => msg.cmd_type = r.read_enum(bytes)?,
                Ok(34) => msg.cmd_payload = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Command {
    fn get_size(&self) -> usize {
        0
        + if self.aggregate_id == Vec::<u8>::new() { 0 } else { 1 + sizeof_len(self.aggregate_id.len()) }
        + if self.cmd_signature == Vec::<u8>::new() { 0 } else { 1 + sizeof_len(self.cmd_signature.len()) }
        + if self.cmd_type == command::CommandType::UNSPECIFIED_COMMAND { 0 } else { 1 + sizeof_varint(*(&self.cmd_type) as u64) }
        + if self.cmd_payload == Vec::<u8>::new() { 0 } else { 1 + sizeof_len(self.cmd_payload.len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.aggregate_id != Vec::<u8>::new() { w.write_with_tag(10, |w| w.write_bytes(&self.aggregate_id))?; }
        if self.cmd_signature != Vec::<u8>::new() { w.write_with_tag(18, |w| w.write_bytes(&self.cmd_signature))?; }
        if self.cmd_type != command::CommandType::UNSPECIFIED_COMMAND { w.write_with_tag(24, |w| w.write_enum(*&self.cmd_type as i32))?; }
        if self.cmd_payload != Vec::<u8>::new() { w.write_with_tag(34, |w| w.write_bytes(&self.cmd_payload))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct MintNFTPayload {
    pub hash: Vec<u8>,
    pub owner: Vec<u8>,
}

impl<'a> MessageRead<'a> for MintNFTPayload {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hash = r.read_bytes(bytes)?.to_owned(),
                Ok(18) => msg.owner = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MintNFTPayload {
    fn get_size(&self) -> usize {
        0
        + if self.hash == Vec::<u8>::new() { 0 } else { 1 + sizeof_len(self.hash.len()) }
        + if self.owner == Vec::<u8>::new() { 0 } else { 1 + sizeof_len(self.owner.len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.hash != Vec::<u8>::new() { w.write_with_tag(10, |w| w.write_bytes(&self.hash))?; }
        if self.owner != Vec::<u8>::new() { w.write_with_tag(18, |w| w.write_bytes(&self.owner))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct TransferNFTPayload {
    pub hash: Vec<u8>,
    pub to: Vec<u8>,
}

impl<'a> MessageRead<'a> for TransferNFTPayload {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hash = r.read_bytes(bytes)?.to_owned(),
                Ok(18) => msg.to = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TransferNFTPayload {
    fn get_size(&self) -> usize {
        0
        + if self.hash == Vec::<u8>::new() { 0 } else { 1 + sizeof_len(self.hash.len()) }
        + if self.to == Vec::<u8>::new() { 0 } else { 1 + sizeof_len(self.to.len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.hash != Vec::<u8>::new() { w.write_with_tag(10, |w| w.write_bytes(&self.hash))?; }
        if self.to != Vec::<u8>::new() { w.write_with_tag(18, |w| w.write_bytes(&self.to))?; }
        Ok(())
    }
}

