#![cfg_attr(not(feature = "std"), no_std)]

/// A module for proof of existence
pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::{ensure_signed, pallet_prelude::*};
	use codec::{Encode,Decode}
	use sp_std::vec::Vec;

	#[derive(Encode,Decode)]
	pub struct Kitty(pub [u8;16])

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber)>;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
	}

	#[pallet::error]
	pub enum Error<T> {
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
	}
}
