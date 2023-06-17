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
	use sp_core::{H160, Bytes};
	use cil_common::event_store::{EventStore};
	use quick_protobuf::{serialize_into_slice_without_len, deserialize_from_slice_without_len};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type EventMaxSize: Get<u32>;
	}

	#[pallet::storage]
	pub(super) type Relays<T: Config> = StorageMap<
		_,
		Blake2_128Concat, 
		T::AccountId,
		bool,
		ValueQuery
	>;

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
		RelayAdded { relay: T::AccountId },
		RelayRemoved { relay: T::AccountId },
		OmnichainEventStored { event: BoundedVec<u8, T::EventMaxSize> },
	}
	
	#[pallet::error]
	pub enum Error<T> {
		NotAuthorizedRelay,
		EventIndexGapIsNotAllowed,
		UnexpectedEventIndex,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn add_relay(origin: OriginFor<T>, relay: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			<Relays<T>>::insert(relay.clone(), true);
			Self::deposit_event(Event::RelayAdded { relay });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn remove_relay(origin: OriginFor<T>, relay: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			<Relays<T>>::remove(relay.clone());
			Self::deposit_event(Event::RelayRemoved { relay });

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn push(origin: OriginFor<T>, aggregate_id: H160, start_idx: u64, evnts: Vec<Vec<u8>>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			if !<Relays<T>>::get(who) {
				return Err(Error::<T>::NotAuthorizedRelay.into());
			};

			if StreamsHeads::<T>::contains_key(aggregate_id) {
				let last_evnt_idx = StreamsHeads::<T>::get(aggregate_id);

				if start_idx > last_evnt_idx {
					return Err(Error::<T>::EventIndexGapIsNotAllowed.into());
				}

				for evnt_idx in start_idx..=last_evnt_idx {
					Streams::<T>::remove(aggregate_id, evnt_idx);
				}

				let evnts_len = evnts.len() as u64;
				for (idx, vec) in evnts.into_iter().enumerate() {
					let evnt = DomainEvent::deserialize(vec.clone());
					let evnt_idx = start_idx + idx as u64;

					if evnt.evnt_idx != evnt_idx {
						return Err(Error::<T>::UnexpectedEventIndex.into());
					}

					let stored_evnt = BoundedVec::<u8, T::EventMaxSize>::try_from(vec).unwrap();
					Streams::<T>::insert(aggregate_id, evnt_idx, stored_evnt.clone());
				}

				let pushed_last_evnt_idx = start_idx + (evnts_len - 1u64);
				StreamsHeads::<T>::insert(aggregate_id, pushed_last_evnt_idx);

			} else {

				if start_idx != 0u64 {
					return Err(Error::<T>::EventIndexGapIsNotAllowed.into());
				}

				let evnts_len = evnts.len() as u64;
				for (idx, vec) in evnts.into_iter().enumerate() {
					let evnt = DomainEvent::deserialize(vec.clone());
					let evnt_idx = idx as u64;

					if evnt.evnt_idx != evnt_idx {
						return Err(Error::<T>::UnexpectedEventIndex.into());
					}

					let stored_evnt = BoundedVec::<u8, T::EventMaxSize>::try_from(vec).unwrap();
					Streams::<T>::insert(aggregate_id, evnt_idx, stored_evnt.clone());
				}

				let pushed_last_evnt_idx = evnts_len - 1u64;
				StreamsHeads::<T>::insert(aggregate_id, pushed_last_evnt_idx);
			};

			Ok(())
		}

	}

	impl<T: Config> EventStore for Pallet<T> {

		fn get(aggregate_id: H160) -> Vec<DomainEvent> {
			let mut evnts = Vec::new();
			if StreamsHeads::<T>::contains_key(aggregate_id) {
				let last_evnt_idx = StreamsHeads::<T>::get(aggregate_id);
				for evnt_idx in 0u64..=last_evnt_idx {
					let stored_evnt = Streams::<T>::get(aggregate_id, evnt_idx);
					let vec: Vec<u8> = stored_evnt.into();
					let evnt = DomainEvent::deserialize(vec);
					evnts.push(evnt);
				}
			};

			evnts
		}

		fn add(aggregate_id: H160, evnt: DomainEvent) {
			let evnt_idx = evnt.evnt_idx;
			let vec = DomainEvent::serialize(evnt);
			let stored_evnt = BoundedVec::<u8, T::EventMaxSize>::try_from(vec).unwrap();
			Streams::<T>::insert(aggregate_id, evnt_idx, stored_evnt.clone());
			StreamsHeads::<T>::insert(aggregate_id, evnt_idx);
			Self::deposit_event(Event::OmnichainEventStored { event: stored_evnt });
		}

	}

}
