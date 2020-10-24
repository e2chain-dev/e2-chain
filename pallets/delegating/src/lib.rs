#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
use log::{info,error};

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
		Something get(fn something): Option<u32>;

		DelegatedScore get(fn delegated): map hasher(blake2_128_concat) T::AccountId  => u64;

		CreditLedger get(fn credit_ledger): map hasher(blake2_128_concat) T::AccountId
			=> CreditScoreLedger<T::AccountId>;

		//	TODO should be update when era change
		pub CurrentEra get(fn current_era): Option<EraIndex>;

		Delegators get(fn delegators): double_map hasher(blake2_128_concat) EraIndex,
		 hasher(blake2_128_concat) T::AccountId => CreditScoreLedger<T::AccountId>;
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
		WithdrawCredit(AccountId, u64),
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
		NoCreditLedgerData,
		NotRightEra,
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

		///
		/// TODO score 参数不需要， 实际 信誉分 通过 credit pallet 模块获取
		#[weight = 10_000]
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

			CreditLedger::<T>::insert(controller.clone(), _ledger.clone());

			if CreditLedger::<T>::contains_key(controller.clone()) {
				info!("[FLQ] insert to ledger success .")
			}

			//
			let current_era = CurrentEra::get().unwrap_or(0);
			Delegators::<T>::insert(current_era, controller.clone(), _ledger.clone());

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
			info!("[FLQ] will undelegate credit score ");

			// 合法性检查
			let controller = ensure_signed(origin)?;

			let score = DelegatedScore::<T>::get(controller.clone());

			if !CreditLedger::<T>::contains_key(controller.clone()) {
				error!("[FLQ]  credit ledger not found . will return err ");
				Err(Error::<T>::NotDelegate)?
			}

			let current_era = CurrentEra::get().unwrap_or(0);
			// TODO 赎回锁定周期为 当前ERA_index + 信誉分锁定周期数（不要使用固定值 2）
			info!("[FLQ] current era index : {:?}", current_era);
			let withdraw_era = current_era + 2;

			let _ledger = CreditScoreLedger{
				delegated_account: controller.clone(),
				delegated_score: score,
				withdraw_era: withdraw_era,
			};
			CreditLedger::<T>::insert(controller.clone(),_ledger);
			// 从 delegators 中删除当前账户的

			Self::deposit_event(RawEvent::UnDelegated(controller, score));
			Ok(())
		}

		///
		///
		/// [storage]
		/// Read  :  Delegated, CreditLedger, CurrentEra
		/// Write :  Delegated, CreditLedger, Delegator
		///
		#[weight = 10_000]
		pub fn withdraw_credit_score(origin) -> dispatch::DispatchResult{
			info!("[FLQ] withdraw credit score");

			let controller = ensure_signed(origin)?;

			// 检查该账户是否存在 delegate
			if !DelegatedScore::<T>::contains_key(controller.clone()) {
				error!("[FLQ] you have not delegated credit score");
				Err(Error::<T>::NotDelegate)?
			}

			// 检查该账户是否存在 待赎回的 credit score
			if !CreditLedger::<T>::contains_key(controller.clone()) {
				error!("[FLQ] can't found credit ledger for your account ");
				Err(Error::<T>::NoCreditLedgerData)?
			}

			// 检查 credit score 是否达到可赎回的条件（是否过了锁定期）
			let _ledger = CreditLedger::<T>::get(controller.clone());
			let current_era = CurrentEra::get().unwrap_or(0);
			if _ledger.withdraw_era > current_era {
				error!("[FLQ] can't withdraw credit score because it's not the right time yet ");
				Err(Error::<T>::NotRightEra)?
			}

			// 删除该账户对应的 Delegated
			DelegatedScore::<T>::remove(controller.clone());

			// 删除该账户对应的 CreditLedger
			CreditLedger::<T>::remove(controller.clone());

			Self::deposit_event(RawEvent::WithdrawCredit(controller, _ledger.delegated_score));

			Ok(())
		}

		///	用户重新委托信誉分（针对： 已经发起 undelegate，信誉分还处于锁定状态的情况）
		///
		///
		/// TODO 待实现
		#[weight = 10_000]
		pub fn redelegate(origin) -> dispatch::DispatchResult {
			info!("[FLQ] redelegate credit score ");


			Ok(())
		}

		#[weight = 10_000]
		pub fn getdelegators(origin, era_index: u32) -> dispatch::DispatchResult{
			info!("[FLQ] get delegators for era_index : {:?}", era_index);
			// 查看指定 era 周期内对应的 delegators
			// TODO 待实现

			Ok(())
		}


		// #[weight = 10_000]
		// pub fn vote_validators(origin, validators: Vec<T::AccountId>)  -> dispatch::DispatchResult {
		// 	info!("[FLQ] vote validator ");
		// 	// 选择将信誉分 委托给哪些 validator
		// 	// TODO 待实现
		//
		//
		// 	Ok(())
		// }
	}
}
