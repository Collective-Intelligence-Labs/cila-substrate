use sp_std::vec::{Vec};
use sp_core::{H160, H256};
use cil_messages::command::{MintNFTPayload, TransferNFTPayload};


pub struct MintNFTCmd(pub MintNFTPayload);
	
impl MintNFTCmd {

    pub fn get_hash(&self) -> Vec<u8> {
        self.0.hash.clone()
    }

    pub fn get_owner(&self) -> Vec<u8> {
        self.0.owner.clone()
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

pub struct TransferNFTCmd(pub TransferNFTPayload);

impl TransferNFTCmd {

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