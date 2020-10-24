#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
use log::{info};

use frame_support::codec::{Decode, Encode};
use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch,
					traits::{Currency}};
use frame_system::ensure_signed;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait  {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Currency: Currency<Self::AccountId>;
}

pub type EraIndex = u32;

#[derive( Default, Clone, Encode, Decode)]
pub struct CreditScoreLedger<AccountId>
{
	delegated_account: AccountId,
	delegated_score: u64,
	withdraw_era: EraIndex
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	trait Store for Module<T: Trait> as Delegating  {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<u32>;

		DelegatedScore get(fn delegated): map hasher(blake2_128_concat) T::AccountId  => u64;

		// Delegators get(fn delegators): double_map hasher(twox_64_concat) EraIndex,
		//  hasher(twox_64_concat) T::AccountId => Option<CreditScoreLedger<AccountId, CreditScore>>;

		CreditLedger get(fn credit_ledger): map hasher(blake2_128_concat) T::AccountId
			=> Option<CreditScoreLedger<T::AccountId>>;

		pub CurrentEra get(fn current_era): Option<EraIndex>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		Delegated(AccountId, u64),
		UnDelegated(AccountId, u64),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		AlreadyDelegated,
		NotDelegate,

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

		#[weight = 10_000]
		//
		pub fn delegate(origin, score: u64) -> dispatch::DispatchResult {
			info!("[FLQ] will delegate score to validator ");

			let controller = ensure_signed(origin)?;

			if DelegatedScore::<T>::contains_key(controller.clone()) {

				Err(Error::<T>::AlreadyDelegated)?
			}

			DelegatedScore::<T>::insert(controller.clone(), score);

			let _ledger = CreditScoreLedger{
				delegated_account: controller.clone(),
				delegated_score: score,
				withdraw_era: 0,
			};

			CreditLedger::<T>::insert(controller.clone(), _ledger);

			if CreditLedger::<T>::contains_key(controller.clone()) {
				info!("[FLQ] insert to ledger success .")
			}


			Self::deposit_event(RawEvent::Delegated(controller, score));

			Ok(())
		}

		#[weight = 10_000]
		pub fn get_delegated_score(origin) -> dispatch::DispatchResult {
			info!("[FLQ] will get delegated score ");

			let controller = ensure_signed(origin)?;

			let score = DelegatedScore::<T>::get(controller.clone());

			Self::deposit_event(RawEvent::Delegated(controller, score));

			Ok(())
		}

		#[weight = 10_000]
		pub fn undelegate(origin) -> dispatch::DispatchResult {
			info!("[FLQ] will withdraw credit score ");

			let controller = ensure_signed(origin)?;

			let score = DelegatedScore::<T>::get(controller.clone());

			if !CreditLedger::<T>::contains_key(controller.clone()) {
				info!("[FLQ]  credit ledger not found . will return err ");
				Err(Error::<T>::NotDelegate)?
			}

			let current_era = CurrentEra::get().unwrap_or(0);
			// TODO 赎回周期为 当前周期 + 信誉分锁定周期数（）
			let withdraw_era = current_era + 1;

			let _ledger = CreditScoreLedger{
				delegated_account: controller.clone(),
				delegated_score: score,
				withdraw_era: withdraw_era,
			};

			CreditLedger::<T>::insert(controller.clone(),_ledger);

			Self::deposit_event(RawEvent::UnDelegated(controller, score));
			Ok(())
		}
	}
}
