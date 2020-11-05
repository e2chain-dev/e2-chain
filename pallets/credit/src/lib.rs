#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch, ensure};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


/// mircropayment number to credit factor: / 
pub const MICROPAYMENT_TO_CREDIT_SCORE_FACTOR: u64 = 1000;
/// Credit score threshold
pub const CREDIT_SCORE_THRESHOLD: u64 = 100;
/// credit score attenuation low threshold
pub const CREDIT_SCORE_ATTENUATION_LOW_THRESHOLD: u64 = 50;

// Credit score delegated threshold 
pub const CREDIT_SCORE_DELEGATED_PERMIT_THRESHOLD: u64 = 60;
/// per credit score vote weight
pub const TOKEN_PER_CREDIT_SCORE: u64 = 10_000_000;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    // A unique name is used to ensure that the pallet's storage items are isolated.
    // This name may be updated, but each pallet in the runtime must use a unique name.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Trait> as Credit {
        //store credit score using map
        pub UserCredit get(fn get_user_credit): map hasher(blake2_128_concat) T::AccountId => Option<u64>;
    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        MaxCredit(AccountId, u64),
        PassThreshold(AccountId, bool),
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        AlreadyInitilized,
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

        // An example dispatchable that takes a singles value as a parameter, writes the value to
        // storage and emits an event. This function must be dispatched by a signed extrinsic.

        // Check that the extrinsic was signed and get the signer.
        // This function will return an error if the extrinsic is not signed.
        // https://substrate.dev/docs/en/knowledgebase/runtime/origin

        // init credit score
        #[weight = 10_000]
        pub fn initilize_credit(origin, credit: u64) -> dispatch::DispatchResult{
            let sender = ensure_signed(origin)?;
            ensure!(!UserCredit::<T>::contains_key(sender.clone()), "Credit Score of AccountId  already Initilized");
            UserCredit::<T>::insert(sender.clone(), credit);
            Ok(())
        }

        // update credit score
        #[weight = 10_000]
        pub fn update_credit(origin, credit: u64) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(UserCredit::<T>::contains_key(sender.clone()), "AccountId is uninitilized");
            UserCredit::<T>::insert(sender.clone(), credit);
            Ok(())
        }

        // max credit
        #[weight = 10_000]
        pub fn max_credit(origin) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            Self::deposit_event(RawEvent::MaxCredit(sender, 100));
            Ok(())
        }

        // given type of threshold, determine whether a user's credit score has passed it
        #[weight = 10_000]
        pub fn pass_threshold(origin, ttype: u8) -> dispatch::DispatchResult{
            let sender = ensure_signed(origin)?;
            if let Some(score)= UserCredit::<T>::get(sender.clone()){
                let mut is_passed = false;
                if score > ttype as u64{
                    is_passed = true;
                }
                Self::deposit_event(RawEvent::PassThreshold(sender, is_passed));
            };
            Ok(())
        }

        // clear credit score
        #[weight = 10_000]
        pub fn kill_credit(origin) -> dispatch::DispatchResult{
            let sender = ensure_signed(origin)?;
            ensure!(UserCredit::<T>::contains_key(sender.clone()), "AccountId is not existed");
            UserCredit::<T>::remove(sender);
            Ok(())
        }
    }
}

pub trait CreditInterface<AccountId> {
    fn initilize_credit(account_id: AccountId, score: u64) -> bool;
    fn update_credit(account_id: AccountId, score: u64) -> bool;
    fn pass_threshold(account_id: AccountId, ttype: u8) -> bool;
    fn credit_attenuation(account_id: AccountId) -> bool;
    fn kill_credit(account_id: AccountId)-> bool;
}

impl<T: Trait> CreditInterface<T::AccountId> for Module<T> {
    fn initilize_credit(account_id: T::AccountId, score: u64) -> bool{
        if UserCredit::<T>::contains_key(account_id.clone()){
          false
        }else{
            UserCredit::<T>::insert(account_id, score);
            true
        }
    }
    fn update_credit(account_id: T::AccountId, score: u64) -> bool{
        if UserCredit::<T>::contains_key(account_id.clone()){
            UserCredit::<T>::insert(account_id, score);
            true
          }else{
              false
          }
    }
    fn pass_threshold(account_id: T::AccountId, ttype: u8)->bool{
        if UserCredit::<T>::contains_key(account_id.clone()){
            if let Some(score)= UserCredit::<T>::get(account_id){
                let mut is_passed = false;
                if score > ttype as u64{
                    is_passed = true;
                }
                return is_passed;
            }
        }
        false
    }
    fn credit_attenuation(account_id: T::AccountId) -> bool{
        if UserCredit::<T>::contains_key(account_id.clone()){
            if let Some(score) = UserCredit::<T>::get(account_id.clone()){
                if score > CREDIT_SCORE_ATTENUATION_LOW_THRESHOLD {
                    UserCredit::<T>::insert(account_id, score-1);
                    return true
                } 
            }
        }
        false
    }
    fn kill_credit(account_id: T::AccountId) -> bool{
        if UserCredit::<T>::contains_key(account_id.clone()){
            UserCredit::<T>::remove(account_id);
            return true
        }
        false
    }
}