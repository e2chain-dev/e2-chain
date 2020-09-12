#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_support::traits::{Currency, Time, Vec};
use frame_support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure};
use frame_system::{self, ensure_signed};

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Currency: Currency<Self::AccountId>;
    type Timestamp: Time;
}

type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

type Moment<T> = <<T as Trait>::Timestamp as Time>::Moment;

type ChannelOf<T> = Chan<<T as frame_system::Trait>::AccountId, Moment<T>>;

// struct to store the registered Device Informatin
#[derive(Decode, Encode, Default)]
pub struct Chan<AccountId, Timestamp> {
    sender: AccountId,
    receiver: AccountId,
    timestamp: Timestamp,
}

// events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        Timestamp = Moment<T>,
        //Balance = BalanceOf<T>,
    {
        ChannelOpened(AccountId, AccountId, Timestamp),
        ChannelClosed(AccountId, AccountId, Timestamp),
    }
);

// storage for this module
decl_storage! {
  trait Store for Module<T: Trait> as Device {
      Channel get(fn get_channel): map hasher(blake2_128_concat) (T::AccountId, T::AccountId) => ChannelOf<T>;
  }

}

// public interface for this runtime module
decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
      // initialize the default event for this module
      fn deposit_event() = default;

      #[weight = 10_000]
      pub fn open_channel(origin, receiver: T::AccountId) -> DispatchResult {
          let sender = ensure_signed(origin)?;
          ensure!(!Channel::<T>::contains_key((sender.clone(),receiver.clone())), "Channel already opened");
          let time = T::Timestamp::now();
          let chan = ChannelOf::<T>{
              sender: sender.clone(),
              receiver: receiver.clone(),
              timestamp: time.clone(),
          };
          Channel::<T>::insert((sender.clone(),receiver.clone()), chan);
          Self::deposit_event(RawEvent::ChannelOpened(sender,receiver, time));
          Ok(())
      }
  }
}
