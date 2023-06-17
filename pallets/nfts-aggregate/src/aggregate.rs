use sp_std::vec::Vec;
use cil_messages::command::{Command, CommandType, MintNFTPayload, TransferNFTPayload};
use cil_messages::event::{DomainEvent, DomainEventType, NFTMintedPayload, NFTTransferedPayload};
use cil_messages::utils::{MintNFTPayloadExt, NFTMintedPayloadExt, TransferNFTPayloadExt, NFTTransferedPayloadExt};
use cil_common::aggregate::Aggregate;
use crate::state::NftsAggregateState;


#[derive(Default)]
pub struct NftsAggregate {
    state: NftsAggregateState,
    changes: Vec<DomainEvent>,
    evnts_count: u64
}

impl Aggregate for NftsAggregate {

    type AggregateState = NftsAggregateState;

    fn get_state(&self) -> &NftsAggregateState {
        &self.state
    }

    fn get_state_mut(&mut self) -> &mut NftsAggregateState {
        &mut self.state
    }

    fn get_evnts_count(&self) -> u64 {
        self.evnts_count
    }

    fn inc_evnts_count(&mut self) {
        self.evnts_count = self.evnts_count + 1;
    }
    
    fn handle_command(&mut self, cmd: Command) {
        match cmd {
            Command { cmd_type: CommandType::MINT_NFT, cmd_payload, .. } => {
                let payload = MintNFTPayload::deserialize(cmd_payload);
                self.mint(payload).unwrap();
            },
            Command { cmd_type: CommandType::TRANSFER_NFT, cmd_payload, .. } => {
                let payload = TransferNFTPayload::deserialize(cmd_payload);
                self.transfer(payload).unwrap();
            },
            _ => {},
        }
    }

    fn pull_changes(&self) -> Vec<DomainEvent> {
        self.changes.clone()
    }

    fn put_change(&mut self, evnt: DomainEvent) {
        self.changes.push(evnt);
    }
}

impl NftsAggregate {
    fn mint(&mut self, payload: MintNFTPayload) -> Result<(), &str> {

        let nft_hash = payload.get_hash_h256().unwrap();
        if self.state.nfts.contains_key(&nft_hash) {
            return Err("NFT with such hash is already minted")
        };

        let evnt_payload = NFTMintedPayload {
            hash: payload.get_hash(),
            owner: payload.get_owner()
        };

        let evnt = DomainEvent {
            evnt_idx: self.get_evnts_count(),
            evnt_type: DomainEventType::NFT_MINTED,
            evnt_payload: NFTMintedPayload::serialize(evnt_payload)
        };

        self.apply_event(evnt);

        Ok(())
    }

    fn transfer(&mut self, payload: TransferNFTPayload) -> Result<(), &str> {
        let nft_hash = payload.get_hash_h256().unwrap();

        match self.state.nfts.get(&nft_hash) {
            None => Err("NFT with such hash does not exist"),
            Some(owner) => {
                let receiver = payload.get_receiver_h160().unwrap();
                if *owner == receiver {
                    Err("NFT can not be transferred to its current owner")
                } else {
                    let evnt_payload = NFTTransferedPayload {
                        hash: payload.get_hash(),
                        to: payload.get_receiver(),
                        // todo: validate the current owner from cmd signature
                        from: owner.to_fixed_bytes().to_vec()
                    };

                    let evnt = DomainEvent {
                        evnt_idx: self.get_evnts_count(),
                        evnt_type: DomainEventType::NFT_TRANSFERED,
                        evnt_payload: NFTTransferedPayload::serialize(evnt_payload)
                    };
        
                    self.apply_event(evnt);

                    Ok(())
                }
            }
        }
    }
}