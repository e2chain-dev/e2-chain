#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_support::traits::{Currency, ReservableCurrency, Vec};
use frame_support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure};
use frame_system::{self, ensure_signed};

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}

const MIN_LOCK_AMT: u64 = 60;

type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

// struct to store the registered Device Informatin
#[derive(Decode, Encode)]
pub struct DeviceInfo<AccountId> {
    AccountId: AccountId,
    IP: Vec<u8>, // IP will not be exposed in future version
}

// events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        //Balance = BalanceOf<T>,
    {
        // event for register an device
        Register(AccountId, Vec<u8>),
    }
);

// storage for this module
decl_storage! {
  trait Store for Module<T: Trait> as Device {

  }

}

// public interface for this runtime module
decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
      // initialize the default event for this module
      fn deposit_event() = default;
  }
}
