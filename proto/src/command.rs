// Automatically generated rust module for 'command.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use alloc::vec::Vec;
use alloc::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
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
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Command<'a> {
    pub aggregate_id: Cow<'a, [u8]>,
    pub cmd_signature: Cow<'a, [u8]>,
    pub cmd_type: CommandType,
    pub cmd_payload: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for Command<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.aggregate_id = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.cmd_signature = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.cmd_type = r.read_enum(bytes)?,
                Ok(34) => msg.cmd_payload = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Command<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.aggregate_id == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.aggregate_id).len()) }
        + if self.cmd_signature == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.cmd_signature).len()) }
        + if self.cmd_type == command::CommandType::UNSPECIFIED_COMMAND { 0 } else { 1 + sizeof_varint(*(&self.cmd_type) as u64) }
        + if self.cmd_payload == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.cmd_payload).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.aggregate_id != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.aggregate_id))?; }
        if self.cmd_signature != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.cmd_signature))?; }
        if self.cmd_type != command::CommandType::UNSPECIFIED_COMMAND { w.write_with_tag(24, |w| w.write_enum(*&self.cmd_type as i32))?; }
        if self.cmd_payload != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.cmd_payload))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct MintNFTPayload<'a> {
    pub hash: Cow<'a, [u8]>,
    pub owner: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for MintNFTPayload<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.owner = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MintNFTPayload<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.hash == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.hash).len()) }
        + if self.owner == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.owner).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.hash != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.hash))?; }
        if self.owner != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.owner))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TransferNFTPayload<'a> {
    pub hash: Cow<'a, [u8]>,
    pub to: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for TransferNFTPayload<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.to = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TransferNFTPayload<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.hash == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.hash).len()) }
        + if self.to == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.to).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.hash != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.hash))?; }
        if self.to != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.to))?; }
        Ok(())
    }
}

