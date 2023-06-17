use sp_std::collections::btree_map::BTreeMap;
use sp_core::{H160, H256};
use cil_messages::event::{DomainEvent, DomainEventType, NFTMintedPayload, NFTTransferedPayload};
use cil_messages::utils::{NFTMintedPayloadExt, NFTTransferedPayloadExt};
use cil_common::aggregate::AggregateState;


#[derive(Default)]
pub struct NftsAggregateState {
    pub nfts: BTreeMap<H256, H160>
}

impl AggregateState for NftsAggregateState {
    fn on_evnt(&mut self, evnt: DomainEvent) {
        match evnt {
            DomainEvent { evnt_type: DomainEventType::NFT_MINTED, evnt_payload, .. } => {
                let payload = NFTMintedPayload::deserialize(evnt_payload);
                self.on_minted(payload);
            },
            DomainEvent { evnt_type: DomainEventType::NFT_TRANSFERED, evnt_payload, .. } => {
                let payload = NFTTransferedPayload::deserialize(evnt_payload);
                self.on_transferred(payload);
            },
            _ => {},
        }
    }
}

impl NftsAggregateState {
    fn on_minted(&mut self, payload: NFTMintedPayload) {
        let nft_hash = payload.get_hash_h256().unwrap();
        let owner = payload.get_owner_h160().unwrap();
        self.nfts.insert(nft_hash, owner);
    }

    fn on_transferred(&mut self, payload: NFTTransferedPayload) {
        let nft_hash = payload.get_hash_h256().unwrap();
        let receiver = payload.get_receiver_h160().unwrap();
        self.nfts.insert(nft_hash, receiver);
    }
}