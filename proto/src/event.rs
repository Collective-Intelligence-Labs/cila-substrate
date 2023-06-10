// Automatically generated rust module for 'event.proto' file

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
pub enum DomainEventType {
    UNSPECIFIED_EVENT = 0,
    NFT_MINTED = 1,
    NFT_TRANSFERED = 2,
}

impl Default for DomainEventType {
    fn default() -> Self {
        DomainEventType::UNSPECIFIED_EVENT
    }
}

impl From<i32> for DomainEventType {
    fn from(i: i32) -> Self {
        match i {
            0 => DomainEventType::UNSPECIFIED_EVENT,
            1 => DomainEventType::NFT_MINTED,
            2 => DomainEventType::NFT_TRANSFERED,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for DomainEventType {
    fn from(s: &'a str) -> Self {
        match s {
            "UNSPECIFIED_EVENT" => DomainEventType::UNSPECIFIED_EVENT,
            "NFT_MINTED" => DomainEventType::NFT_MINTED,
            "NFT_TRANSFERED" => DomainEventType::NFT_TRANSFERED,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DomainEvent<'a> {
    pub evnt_idx: u64,
    pub evnt_type: DomainEventType,
    pub evnt_payload: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for DomainEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.evnt_idx = r.read_uint64(bytes)?,
                Ok(16) => msg.evnt_type = r.read_enum(bytes)?,
                Ok(26) => msg.evnt_payload = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DomainEvent<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.evnt_idx == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.evnt_idx) as u64) }
        + if self.evnt_type == event::DomainEventType::UNSPECIFIED_EVENT { 0 } else { 1 + sizeof_varint(*(&self.evnt_type) as u64) }
        + if self.evnt_payload == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.evnt_payload).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.evnt_idx != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.evnt_idx))?; }
        if self.evnt_type != event::DomainEventType::UNSPECIFIED_EVENT { w.write_with_tag(16, |w| w.write_enum(*&self.evnt_type as i32))?; }
        if self.evnt_payload != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.evnt_payload))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct NFTMintedPayload<'a> {
    pub hash: Cow<'a, [u8]>,
    pub owner: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for NFTMintedPayload<'a> {
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

impl<'a> MessageWrite for NFTMintedPayload<'a> {
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
pub struct NFTTransferedPayload<'a> {
    pub hash: Cow<'a, [u8]>,
    pub from: Cow<'a, [u8]>,
    pub to: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for NFTTransferedPayload<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.from = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.to = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NFTTransferedPayload<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.hash == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.hash).len()) }
        + if self.from == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.from).len()) }
        + if self.to == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.to).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.hash != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.hash))?; }
        if self.from != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.from))?; }
        if self.to != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.to))?; }
        Ok(())
    }
}

