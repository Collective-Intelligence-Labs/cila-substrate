#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
mod state;
mod aggregate;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_core::{H160};
	use cil_common::aggregate::{Aggregate, AggregateRepository};
	use cil_common::event_store::EventStore;
	use aggregate::NftsAggregate;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type EventStore: EventStore;
	}

	#[pallet::event]
	pub enum Event<T: Config> {
		CommandHandeled
	}
	
	#[pallet::error]
	pub enum Error<T> {
		UnknownCommand,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

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
}
