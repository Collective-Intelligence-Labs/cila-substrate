// Automatically generated rust module for 'event.proto' file

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
#[derive(Default, Debug, PartialEq, Clone)]
pub struct DomainEvent {
    pub evnt_idx: u64,
    pub evnt_type: DomainEventType,
    pub evnt_payload: Vec<u8>,
}

impl<'a> MessageRead<'a> for DomainEvent {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.evnt_idx = r.read_uint64(bytes)?,
                Ok(16) => msg.evnt_type = r.read_enum(bytes)?,
                Ok(26) => msg.evnt_payload = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for DomainEvent {
    fn get_size(&self) -> usize {
        0
        + if self.evnt_idx == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.evnt_idx) as u64) }
        + if self.evnt_type == event::DomainEventType::UNSPECIFIED_EVENT { 0 } else { 1 + sizeof_varint(*(&self.evnt_type) as u64) }
        + if self.evnt_payload == Vec::<u8>::new() { 0 } else { 1 + sizeof_len(self.evnt_payload.len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.evnt_idx != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.evnt_idx))?; }
        if self.evnt_type != event::DomainEventType::UNSPECIFIED_EVENT { w.write_with_tag(16, |w| w.write_enum(*&self.evnt_type as i32))?; }
        if self.evnt_payload != Vec::<u8>::new() { w.write_with_tag(26, |w| w.write_bytes(&self.evnt_payload))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct NFTMintedPayload {
    pub hash: Vec<u8>,
    pub owner: Vec<u8>,
}

impl<'a> MessageRead<'a> for NFTMintedPayload {
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

impl MessageWrite for NFTMintedPayload {
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
pub struct NFTTransferedPayload {
    pub hash: Vec<u8>,
    pub from: Vec<u8>,
    pub to: Vec<u8>,
}

impl<'a> MessageRead<'a> for NFTTransferedPayload {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hash = r.read_bytes(bytes)?.to_owned(),
                Ok(18) => msg.from = r.read_bytes(bytes)?.to_owned(),
                Ok(26) => msg.to = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for NFTTransferedPayload {
    fn get_size(&self) -> usize {
        0
        + if self.hash == Vec::<u8>::new() { 0 } else { 1 + sizeof_len(self.hash.len()) }
        + if self.from == Vec::<u8>::new() { 0 } else { 1 + sizeof_len(self.from.len()) }
        + if self.to == Vec::<u8>::new() { 0 } else { 1 + sizeof_len(self.to.len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.hash != Vec::<u8>::new() { w.write_with_tag(10, |w| w.write_bytes(&self.hash))?; }
        if self.from != Vec::<u8>::new() { w.write_with_tag(18, |w| w.write_bytes(&self.from))?; }
        if self.to != Vec::<u8>::new() { w.write_with_tag(26, |w| w.write_bytes(&self.to))?; }
        Ok(())
    }
}

