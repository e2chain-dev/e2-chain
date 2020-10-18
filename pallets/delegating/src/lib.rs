#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
use log::{info};

use frame_support::{decl_module, decl_storage, decl_event, decl_error, Parameter, dispatch, traits::Get};
use frame_system::ensure_signed;
// use sp_std::fmt::Debug;
// use sp_runtime::{
// 	RuntimeDebug, Perbill, DispatchError, Either, generic,
// 	traits::{
// 		self, CheckEqual, AtLeast32Bit, Zero, Lookup, LookupError,
// 		SimpleBitOps, Hash, Member, MaybeDisplay, BadOrigin,
// 		MaybeSerialize, MaybeSerializeDeserialize, MaybeMallocSizeOf, StaticLookup, One, Bounded,
// 		Dispatchable, AtLeast32BitUnsigned
// 	},
// 	offchain::storage_lock::BlockNumberProvider,
// };

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait  {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

pub type EraIndex = u32;
pub type CreditScore = u64;

struct CreditScoreLedger<AccountId, CreditScore>

{
	account_id: AccountId,
	credit_score: CreditScore,
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<u32>;

		// Delegators get(fn delegators): double_map hasher(twox_64_concat) EraIndex,
		//  hasher(twox_64_concat) T::AccountId => Option<CreditScoreLedger<AccountId, CreditScore>>;

		// RewordInEra get(fn reword):
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, AccountId),
		Delegated(AccountId, u32),
		UnDelegated(AccountId, u32),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn delegate_score(origin, score: u32) -> dispatch::DispatchResult {
			info!("[FLQ] will delegate score to validator ");

			let stasher = ensure_signed(origin)?;

			Something::put(score);

			Self::deposit_event(RawEvent::Delegated(stasher, 186));

			Ok(())
		}

		#[weight = 10_000]
		pub fn get_delegated_score(origin) -> dispatch::DispatchResult {
			info!("[FLQ] will get delegated score ");

			let stasher = ensure_signed(origin)?;

			let score = Something::get().unwrap_or(0);

			Self::deposit_event(RawEvent::Delegated(stasher, score));

			Ok(())
		}
	}
}
