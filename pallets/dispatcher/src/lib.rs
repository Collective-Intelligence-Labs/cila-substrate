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

	use cil_messages::operation::{Operation};
	use cil_messages::utils::{OperationExt, CommandExt};
	use cil_common::aggregate::{AggregateRepository, Aggregate};

	use sp_std::vec::{Vec};

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type NftsAggregateRepository: AggregateRepository;
	}

	#[pallet::storage]
	pub(super) type Routers<T: Config> = StorageMap<
		_,
		Blake2_128Concat, 
		T::AccountId,
		bool,
		ValueQuery
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RouterAdded { router: T::AccountId },
		RouterRemoved { router: T::AccountId },
	}
	
	#[pallet::error]
	pub enum Error<T> {
		NotAuthorizedRouter,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn add_router(origin: OriginFor<T>, router: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			<Routers<T>>::insert(router.clone(), true);
			Self::deposit_event(Event::RouterAdded { router });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn remove_router(origin: OriginFor<T>, router: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;

			<Routers<T>>::remove(router.clone());
			Self::deposit_event(Event::RouterRemoved { router });

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn dispatch(origin: OriginFor<T>, op_bytes: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			if !<Routers<T>>::get(who) {
				return Err(Error::<T>::NotAuthorizedRouter.into());
			};
			
			let op: Operation = Operation::deserialize(op_bytes);
			let aggregate_id = op.commands.get(0).unwrap().get_aggregate_id_h160().unwrap();
			let mut aggregate = T::NftsAggregateRepository::get_aggregate(aggregate_id);

			for cmd in op.commands {
				// todo: check command signature
				aggregate.handle_command(cmd);
			}

			T::NftsAggregateRepository::save_aggregate(aggregate_id, aggregate);

			Ok(())
		}

	}
}