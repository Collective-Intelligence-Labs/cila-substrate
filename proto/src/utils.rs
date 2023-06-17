use crate::operation::{Operation};
use crate::command::{Command, MintNFTPayload, TransferNFTPayload};
use crate::event::{DomainEvent, NFTMintedPayload, NFTTransferedPayload};

use sp_core::{H160, H256};
use sp_std::vec::Vec;
use quick_protobuf::{serialize_into_slice_without_len, deserialize_from_slice_without_len, MessageWrite};


pub trait CommandExt {
    fn get_aggregate_id_h160(&self) -> Result<H160, &str>;
}

impl CommandExt for Command {
    fn get_aggregate_id_h160(&self) -> Result<H160, &str> {
        if self.aggregate_id.len() == 20 {
            let mut buf = [0u8; 20];
            buf.copy_from_slice(&self.aggregate_id);
            Ok(buf.into())
        } else {
            Err("Aggregate ID must be 20-bytes in length")
        }
    }
}

pub trait MintNFTPayloadExt {
    fn get_hash(&self) -> Vec<u8>;
    fn get_owner(&self) -> Vec<u8>;
    fn get_hash_h256(&self) -> Result<H256, &str>;
    fn get_owner_h160(&self) -> Result<H160, &str>;

    fn serialize(payload: MintNFTPayload) -> Vec<u8> {
        let payload_len = payload.get_size();
        let mut v : Vec<u8> = Vec::with_capacity(payload_len);
        v.resize(payload_len, 0);
        let payload_buf: &mut [u8] = &mut v[..];
        serialize_into_slice_without_len(&payload, payload_buf).unwrap();
        payload_buf.to_vec()
    }

    fn deserialize(bytes: Vec<u8>) -> MintNFTPayload {
        let payload: MintNFTPayload = deserialize_from_slice_without_len(&bytes).unwrap();
        payload
    }
}
	
impl MintNFTPayloadExt for MintNFTPayload {

    fn get_hash(&self) -> Vec<u8> {
        self.hash.clone()
    }

    fn get_owner(&self) -> Vec<u8> {
        self.owner.clone()
    }

    fn get_hash_h256(&self) -> Result<H256, &str> {
        if self.hash.len() == 32 {
            let mut buf = [0u8; 32];
            buf.copy_from_slice(&self.hash);
            Ok(buf.into())
        } else {
            Err("NFT hash must be 32-bytes in length")
        }
    }

    fn get_owner_h160(&self) -> Result<H160, &str> {
        if self.owner.len() == 20 {
            let mut buf = [0u8; 20];
            buf.copy_from_slice(&self.owner);
            Ok(buf.into())
        } else {
            Err("NFT owner must be 20-bytes in length")
        }
    }
}

pub trait TransferNFTPayloadExt {
    fn get_hash(&self) -> Vec<u8>;
    fn get_hash_h256(&self) -> Result<H256, &str>;
    fn get_receiver(&self) -> Vec<u8>;
    fn get_receiver_h160(&self) -> Result<H160, &str>;

    fn serialize(payload: TransferNFTPayload) -> Vec<u8> {
        let payload_len = payload.get_size();
        let mut v : Vec<u8> = Vec::with_capacity(payload_len);
        v.resize(payload_len, 0);
        let payload_buf: &mut [u8] = &mut v[..];
        serialize_into_slice_without_len(&payload, payload_buf).unwrap();
        payload_buf.to_vec()
    }

    fn deserialize(bytes: Vec<u8>) -> TransferNFTPayload {
        let payload: TransferNFTPayload = deserialize_from_slice_without_len(&bytes).unwrap();
        payload
    }
}

impl TransferNFTPayloadExt for TransferNFTPayload {

    fn get_hash(&self) -> Vec<u8> {
        self.hash.clone()
    }

    fn get_hash_h256(&self) -> Result<H256, &str> {
        if self.hash.len() == 32 {
            let mut buf = [0u8; 32];
            buf.copy_from_slice(&self.hash);
            Ok(buf.into())
        } else {
            Err("NFT hash must be 32-bytes in length")
        }
    }

    fn get_receiver(&self) -> Vec<u8> {
        self.to.clone()
    }

    fn get_receiver_h160(&self) -> Result<H160, &str> {
        if self.to.len() == 20 {
            let mut buf = [0u8; 20];
            buf.copy_from_slice(&self.to);
            Ok(buf.into())
        } else {
            Err("NFT owner must be 20-bytes in length")
        }
    }
}


pub trait NFTMintedPayloadExt {
    fn get_hash(&self) -> Vec<u8>;
    fn get_hash_h256(&self) -> Result<H256, &str>;
    fn get_owner(&self) -> Vec<u8>;
    fn get_owner_h160(&self) -> Result<H160, &str>;

    fn serialize(payload: NFTMintedPayload) -> Vec<u8> {
        let payload_len = payload.get_size();
        let mut v : Vec<u8> = Vec::with_capacity(payload_len);
        v.resize(payload_len, 0);
        let payload_buf: &mut [u8] = &mut v[..];
        serialize_into_slice_without_len(&payload, payload_buf).unwrap();
        payload_buf.to_vec()
    }

    fn deserialize(bytes: Vec<u8>) -> NFTMintedPayload {
        let payload: NFTMintedPayload = deserialize_from_slice_without_len(&bytes).unwrap();
        payload
    }
}

impl NFTMintedPayloadExt for NFTMintedPayload {

    fn get_hash(&self) -> Vec<u8> {
        self.hash.clone()
    }
    
    fn get_hash_h256(&self) -> Result<H256, &str> {
        if self.hash.len() == 32 {
            let mut buf = [0u8; 32];
            buf.copy_from_slice(&self.hash);
            Ok(buf.into())
        } else {
            Err("NFT hash must be 32-bytes in length")
        }
    }
    
    fn get_owner(&self) -> Vec<u8> {
        self.owner.clone()
    }
    
    fn get_owner_h160(&self) -> Result<H160, &str> {
        if self.owner.len() == 20 {
            let mut buf = [0u8; 20];
            buf.copy_from_slice(&self.owner);
            Ok(buf.into())
        } else {
            Err("NFT owner must be 20-bytes in length")
        }
    }
}

pub trait NFTTransferedPayloadExt {
    fn get_hash(&self) -> Vec<u8>;
    fn get_hash_h256(&self) -> Result<H256, &str>;
    fn get_sender(&self) -> Vec<u8>;
    fn get_sender_h160(&self) -> Result<H160, &str>;
    fn get_receiver(&self) -> Vec<u8>;
    fn get_receiver_h160(&self) -> Result<H160, &str>;

    fn serialize(payload: NFTTransferedPayload) -> Vec<u8> {
        let payload_len = payload.get_size();
        let mut v : Vec<u8> = Vec::with_capacity(payload_len);
        v.resize(payload_len, 0);
        let payload_buf: &mut [u8] = &mut v[..];
        serialize_into_slice_without_len(&payload, payload_buf).unwrap();
        payload_buf.to_vec()
    }

    fn deserialize(bytes: Vec<u8>) -> NFTTransferedPayload {
        let payload: NFTTransferedPayload = deserialize_from_slice_without_len(&bytes).unwrap();
        payload
    }
}


impl NFTTransferedPayloadExt for NFTTransferedPayload {

    fn get_hash(&self) -> Vec<u8> {
        self.hash.clone()
    }

    fn get_hash_h256(&self) -> Result<H256, &str> {
        if self.hash.len() == 32 {
            let mut buf = [0u8; 32];
            buf.copy_from_slice(&self.hash);
            Ok(buf.into())
        } else {
            Err("NFT hash must be 32-bytes in length")
        }
    }

    fn get_sender(&self) -> Vec<u8> {
        self.from.clone()
    }

    fn get_sender_h160(&self) -> Result<H160, &str> {
        if self.from.len() == 20 {
            let mut buf = [0u8; 20];
            buf.copy_from_slice(&self.from);
            Ok(buf.into())
        } else {
            Err("NFT owner must be 20-bytes in length")
        }
    }

    fn get_receiver(&self) -> Vec<u8> {
        self.to.clone()
    }

    fn get_receiver_h160(&self) -> Result<H160, &str> {
        if self.to.len() == 20 {
            let mut buf = [0u8; 20];
            buf.copy_from_slice(&self.to);
            Ok(buf.into())
        } else {
            Err("NFT owner must be 20-bytes in length")
        }
    }

}


pub trait DomainEventExt {

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

impl DomainEventExt for DomainEvent {}


pub trait OperationExt {
    
    fn serialize(op: Operation) -> Vec<u8> {
        let op_len = op.get_size();
        let mut v : Vec<u8> = Vec::with_capacity(op_len);
        v.resize(op_len, 0);
        let op_buf: &mut [u8] = &mut v[..];
        serialize_into_slice_without_len(&op, op_buf).unwrap();
        op_buf.to_vec()
    }

    fn deserialize(bytes: Vec<u8>) -> Operation {
        let op: Operation = deserialize_from_slice_without_len(&bytes).unwrap();
        op
    }

}

impl OperationExt for Operation {}