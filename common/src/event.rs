use sp_std::vec::{Vec};
use sp_core::{H160, H256};
use cil_messages::event::{DomainEvent, DomainEventType, NFTMintedPayload, NFTTransferedPayload};
use quick_protobuf::{serialize_into_slice_without_len, deserialize_from_slice_without_len, MessageWrite};

pub struct NFTMintedEvent(pub NFTMintedPayload);

impl NFTMintedEvent {

    pub fn get_hash(&self) -> Vec<u8> {
        self.0.hash.clone()
    }

    pub fn get_hash_h256(&self) -> Result<H256, &str> {
        if self.0.hash.len() == 32 {
            let mut buf = [0u8; 32];
            buf.copy_from_slice(&self.0.hash);
            Ok(buf.into())
        } else {
            Err("NFT hash must be 32-bytes in length")
        }
    }

    pub fn get_owner(&self) -> Vec<u8> {
        self.0.owner.clone()
    }

    pub fn get_owner_h160(&self) -> Result<H160, &str> {
        if self.0.owner.len() == 20 {
            let mut buf = [0u8; 20];
            buf.copy_from_slice(&self.0.owner);
            Ok(buf.into())
        } else {
            Err("NFT owner must be 20-bytes in length")
        }
    }
}

pub trait NFTMintedPayloadSerializer {
    fn serialize(payload: NFTMintedPayload) -> Vec<u8> {
        let payload_len = payload.get_size();
        let mut v : Vec<u8> = Vec::with_capacity(payload_len);
        v.resize(payload_len, 0);
        let payload_buf: &mut [u8] = &mut v[..];
        serialize_into_slice_without_len(&payload, payload_buf).unwrap();
        payload_buf.to_vec()
    }
}

impl NFTMintedPayloadSerializer for NFTMintedPayload {}


pub struct NFTTransferredEvent(pub NFTTransferedPayload);

impl NFTTransferredEvent {

    pub fn get_hash(&self) -> Vec<u8> {
        self.0.hash.clone()
    }

    pub fn get_hash_h256(&self) -> Result<H256, &str> {
        if self.0.hash.len() == 32 {
            let mut buf = [0u8; 32];
            buf.copy_from_slice(&self.0.hash);
            Ok(buf.into())
        } else {
            Err("NFT hash must be 32-bytes in length")
        }
    }

    pub fn get_sender(&self) -> Vec<u8> {
        self.0.from.clone()
    }

    pub fn get_sender_H160(&self) -> Result<H160, &str> {
        if self.0.from.len() == 20 {
            let mut buf = [0u8; 20];
            buf.copy_from_slice(&self.0.from);
            Ok(buf.into())
        } else {
            Err("NFT owner must be 20-bytes in length")
        }
    }

    pub fn get_receiver(&self) -> Vec<u8> {
        self.0.to.clone()
    }

    pub fn get_receiver_H160(&self) -> Result<H160, &str> {
        if self.0.to.len() == 20 {
            let mut buf = [0u8; 20];
            buf.copy_from_slice(&self.0.to);
            Ok(buf.into())
        } else {
            Err("NFT owner must be 20-bytes in length")
        }
    }

}

pub trait NFTTransferredPayloadSerializer {
    fn serialize(payload: NFTTransferedPayload) -> Vec<u8> {
        let payload_len = payload.get_size();
        let mut v : Vec<u8> = Vec::with_capacity(payload_len);
        v.resize(payload_len, 0);
        let payload_buf: &mut [u8] = &mut v[..];
        serialize_into_slice_without_len(&payload, payload_buf).unwrap();
        payload_buf.to_vec()
    }
}

impl NFTTransferredPayloadSerializer for NFTTransferedPayload {}

pub trait DomainEventProto {
    fn serialize(evnt: DomainEvent) -> Vec<u8> {
        let evnt_len = evnt.get_size();
        let mut v : Vec<u8> = Vec::with_capacity(evnt_len);
        v.resize(evnt_len, 0);
        let evnt_buf: &mut [u8] = &mut v[..];
        serialize_into_slice_without_len(&evnt, evnt_buf).unwrap();
        evnt_buf.to_vec()
    }

    fn deserialize(bytes: Vec<u8>) -> DomainEvent {
        let evnt: DomainEvent = deserialize_from_slice_without_len(&bytes).unwrap();
        evnt
    }
}

impl DomainEventProto for DomainEvent {}