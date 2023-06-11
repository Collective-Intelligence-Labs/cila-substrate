#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::{Vec};
	use sp_std::collections::btree_map::{BTreeMap};
	use sp_core::{H160, H256};
	use quick_protobuf::{serialize_into_slice_without_len, deserialize_from_slice_without_len, MessageWrite};

	use cil_messages::command::{Command, CommandType, MintNFTPayload, TransferNFTPayload};
	use cil_messages::event::{DomainEvent, DomainEventType, NFTMintedPayload, NFTTransferedPayload};
	use cil_common::event::{NFTMintedEvent, NFTTransferredEvent, NFTMintedPayloadSerializer, NFTTransferredPayloadSerializer};
	use cil_common::command::{MintNFTCmd, TransferNFTCmd};
	use cil_common::aggregate::{Aggregate, AggregateState, AggregateRepository};
	use cil_common::event_store::{EventStore};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type EventStore: EventStore;
	}

	#[pallet::event]
	pub enum Event<T: Config> {
		MintNftHandled { magic_num: u32 },
		TransferNftHandled { magic_num: u32 },
	}
	
	#[pallet::error]
	pub enum Error<T> {
		UnknownCommand,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[derive(Default)]
	pub struct NftsAggregateState {
		nfts: BTreeMap<H256, H160>
	}

	#[derive(Default)]
	pub struct NftsAggregate {
		state: NftsAggregateState,
		changes: Vec<DomainEvent>,
		evnts_count: u64
	}

	impl<T: Config> AggregateRepository for Pallet<T> {
		type Aggregate = NftsAggregate;
		
		fn get_aggregate(aggregate_id: H160) -> NftsAggregate {
			let mut aggregate = NftsAggregate::default();
			let evnts = T::EventStore::get(aggregate_id);
			aggregate.replay_events(evnts);
			aggregate
		}

		fn save_aggregate(aggregate_id: H160, aggregate: NftsAggregate) {
			let changes = aggregate.pull_changes();
			for change in changes.into_iter() {
				T::EventStore::add(aggregate_id, change);
			}
		}
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
					let payload: MintNFTPayload = deserialize_from_slice_without_len(&cmd_payload).unwrap();
					self.mint(MintNFTCmd(payload)).unwrap();
				},
				Command { cmd_type: CommandType::TRANSFER_NFT, cmd_payload, .. } => {
					let payload: TransferNFTPayload = deserialize_from_slice_without_len(&cmd_payload).unwrap();
					self.transfer(TransferNFTCmd(payload)).unwrap();
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

		fn mint(&mut self, cmd: MintNFTCmd) -> Result<(), &str> {

			let nft_hash = cmd.get_hash_h256().unwrap();
			if self.state.nfts.contains_key(&nft_hash) {
				return Err("NFT with such hash is already minted")
			};

			let evnt_payload = NFTMintedPayload {
				hash: cmd.get_hash(),
				owner: cmd.get_owner()
			};

			let evnt = DomainEvent {
				evnt_idx: self.get_evnts_count(),
				evnt_type: DomainEventType::NFT_MINTED,
				evnt_payload: NFTMintedPayload::serialize(evnt_payload)
			};

			self.apply_event(evnt);

			Ok(())
		}

		fn transfer(&mut self, cmd: TransferNFTCmd) -> Result<(), &str> {
			let nft_hash = cmd.get_hash_h256().unwrap();

			match self.state.nfts.get(&nft_hash) {
				None => Err("NFT with such hash does not exist"),
				Some(owner) => {
					let receiver = cmd.get_receiver_H160().unwrap();
					if *owner == receiver {
						Err("NFT can not be transferred to its current owner")
					} else {
						let evnt_payload = NFTTransferedPayload {
							hash: cmd.get_hash(),
							to: cmd.get_receiver(),
							// validation for the current owner needs to be performed using the tx signature and will be added in the next steps
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


	impl AggregateState for NftsAggregateState {

		fn on_evnt(&mut self, evnt: DomainEvent) {
			match evnt {
				DomainEvent { evnt_type: DomainEventType::NFT_MINTED, evnt_payload, .. } => {
					let payload: NFTMintedPayload = deserialize_from_slice_without_len(&evnt_payload).unwrap();
					self.on_minted(NFTMintedEvent(payload));
				},
				DomainEvent { evnt_type: DomainEventType::NFT_TRANSFERED, evnt_payload, .. } => {
					let payload: NFTTransferedPayload = deserialize_from_slice_without_len(&evnt_payload).unwrap();
					self.on_transferred(NFTTransferredEvent(payload));
				},
				_ => {},
			}
		}

	}

	impl NftsAggregateState {

		fn on_minted(&mut self, evnt: NFTMintedEvent) {
			let nft_hash = evnt.get_hash_h256().unwrap();
			let owner = evnt.get_owner_h160().unwrap();
			self.nfts.insert(nft_hash, owner);
		}
	
		fn on_transferred(&mut self, evnt: NFTTransferredEvent) {
			let nft_hash = evnt.get_hash_h256().unwrap();
			let receiver = evnt.get_receiver_H160().unwrap();
			self.nfts.insert(nft_hash, receiver);
		}

	}
	
}
