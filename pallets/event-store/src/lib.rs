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
	use cil_messages::event::{DomainEvent};
	use cil_common::event::{DomainEventProto};
	use sp_std::vec::{Vec};
	use sp_core::{H160};
	use cil_common::event_store::{EventStore};
	use quick_protobuf::{serialize_into_slice_without_len, deserialize_from_slice_without_len};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type EventMaxSize: Get<u32>;
	}

	pub type AggregateId = H160;
	pub type EventIndex = u64;

	#[pallet::storage]
	pub type Streams<T: Config> = 
		StorageDoubleMap<
			_, 
			Blake2_128Concat, 
			AggregateId, 
			Blake2_128Concat, 
			EventIndex, 
			BoundedVec<u8, T::EventMaxSize>, 
			ValueQuery
		>;
	
	#[pallet::storage]
	pub(super) type StreamsHeads<T: Config> = StorageMap<
		_,
		Blake2_128Concat, 
		AggregateId,
		EventIndex,
		ValueQuery
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		OmnichainEventStored { something: u32, who: T::AccountId },
	}
	
	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

	}

	impl<T: Config> EventStore for Pallet<T> {

		fn get(aggregate_id: H160) -> Vec<DomainEvent> {
			let mut evnts = Vec::new();

			if StreamsHeads::<T>::contains_key(aggregate_id) {
				let last_idx = StreamsHeads::<T>::get(aggregate_id);
				for idx in 0u64..=last_idx {
					let bounded_vec = Streams::<T>::get(aggregate_id, idx);
					let unbounded_vec: Vec<u8> = bounded_vec.into();
					let evnt = DomainEvent::deserialize(unbounded_vec);
					evnts.push(evnt);
				}
			};

			evnts
		}

		fn add(aggregate_id: H160, evnt: DomainEvent) {
			let idx = evnt.evnt_idx;
			let unbounded_vec = DomainEvent::serialize(evnt);
			let bounded_vec = BoundedVec::<u8, T::EventMaxSize>::try_from(unbounded_vec).unwrap();
			Streams::<T>::insert(aggregate_id, idx, bounded_vec);
			StreamsHeads::<T>::insert(aggregate_id, idx);
		}

	}
}
