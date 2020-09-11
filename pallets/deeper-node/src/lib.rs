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
#[derive(Decode, Encode, Default)]
pub struct Node<AccountId> {
    account_id: AccountId,
    ipv4: Vec<u8>, // IP will not be exposed in future version
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
      DeviceInfo get(fn get_device_info): map hasher(identity) T::AccountId => Node<T::AccountId>;
      ServersByCountry get(fn get_servers_by_country): map hasher(identity) Vec<u8> => Vec<T::AccountId>;
  }

}

// public interface for this runtime module
decl_module! {
  pub struct Module<T: Trait> for enum Call where origin: T::Origin {
      // initialize the default event for this module
      fn deposit_event() = default;

      #[weight = 10_000]
      pub fn register_device(origin, ip: Vec<u8>) -> DispatchResult {
          let sender = ensure_signed(origin)?;
          ensure!(ip.len() == 4, "IPv4 has 4 bytes");
          let node = Node {
              account_id: sender.clone(),
              ipv4: ip,
          };
          // TODO: lock some tokens
          <DeviceInfo<T>>::insert(sender, node);
          Ok(())
      }

      #[weight = 10_000]
      pub fn register_server(origin, country: Vec<u8>) -> DispatchResult {
          let sender = ensure_signed(origin)?;
          ensure!(<DeviceInfo<T>>::contains_key(sender.clone()),"sender device needs register first");
          ensure!(country.len() == 1, "Country code has 1 byte");

          // TODO: think of more efficient way
          let mut server_list = <ServersByCountry<T>>::get(country.clone());
          // TODO: check repeat registration and slash?
          server_list.push(sender);
          <ServersByCountry<T>>::insert(country,server_list);
          Ok(())
      }
  }
}
