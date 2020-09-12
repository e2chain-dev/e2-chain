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

const MIN_LOCK_AMT: u32 = 100;

type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

// struct to store the registered Device Informatin
#[derive(Decode, Encode, Default)]
pub struct Node<AccountId> {
    account_id: AccountId,
    ipv4: Vec<u8>,            // IP will not be exposed in future version
    country: Option<Vec<u8>>, // country code, only server need this field
}

// events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        //Balance = BalanceOf<T>,
    {
        // register node: AccountId, ipv4
        RegisterNode(AccountId, Vec<u8>, Option<Vec<u8>>),
        UnregisterNode(AccountId),

        // add account into a country's server list
        ServerAdded(AccountId, Vec<u8>),
        // remove account from a country's server list
        ServerRemoved(AccountId, Vec<u8>),
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
      pub fn register_device(origin, ip: Vec<u8>, country: Option<Vec<u8>>) -> DispatchResult {
          let sender = ensure_signed(origin)?;
          ensure!(ip.len() == 4, "IPv4 has 4 bytes");
          let node = Node {
              account_id: sender.clone(),
              ipv4: ip.clone(),
              country: country.clone(),
          };
          T::Currency::reserve(&sender,BalanceOf::<T>::from(MIN_LOCK_AMT))?;
          <DeviceInfo<T>>::insert(sender.clone(), node);

          Self::deposit_event(RawEvent::RegisterNode(sender, ip, country));
          Ok(())
      }

      #[weight = 10_000]
      pub fn unregister_device(origin) -> DispatchResult {
          let sender = ensure_signed(origin)?;
          ensure!(<DeviceInfo<T>>::contains_key(sender.clone()), "device not registered!");
          let node = <DeviceInfo<T>>::get(sender.clone());
          if !node.country.is_none() {
              let country = node.country.unwrap();
              let _ = Self::try_remove_server(&sender, &country);
          }
          <DeviceInfo<T>>::remove(sender.clone());
          T::Currency::unreserve(&sender,BalanceOf::<T>::from(MIN_LOCK_AMT));
          Self::deposit_event(RawEvent::UnregisterNode(sender));
          Ok(())
      }


      #[weight = 10_000]
      pub fn register_server(origin, country: Vec<u8>) -> DispatchResult {
          let sender = ensure_signed(origin)?;
          ensure!(<DeviceInfo<T>>::contains_key(sender.clone()),
              "sender device needs register first");
          ensure!(country.len() == 2, "Country code has 2 byte");

          match Self::try_add_server(&sender, &country) {
              Ok(_) => { // update country in the node field
                  let mut node = <DeviceInfo<T>>::get(sender.clone());
                  node.country = Some(country.clone());
                  <DeviceInfo<T>>::insert(sender.clone(),node);
              },
              Err(err) => {
                  return Err(err);
              },
          }

          Ok(())
      }

      #[weight = 10_000]
      pub fn unregister_server(origin, country: Vec<u8>) -> DispatchResult {
          let sender = ensure_signed(origin)?;
          ensure!(<DeviceInfo<T>>::contains_key(sender.clone()),
              "sender device needs register first");
          ensure!(country.len() == 2, "Country code has 2 byte");
          let _ = Self::try_remove_server(&sender, &country);
          Ok(())
      }
  }
}

impl<T: Trait> Module<T> {
    // try to remove an account from a country's server list if exists
    fn try_remove_server(sender: &T::AccountId, country: &Vec<u8>) -> DispatchResult {
        let mut server_list = <ServersByCountry<T>>::get(country.clone());
        if let Some(index) = server_list.iter().position(|x| *x == sender.clone()) {
            server_list.remove(index);
            <ServersByCountry<T>>::insert(country.clone(), server_list);
            Self::deposit_event(RawEvent::ServerRemoved(sender.clone(), country.clone()));
        }
        Ok(())
    }

    // try to add an account to a country's server list; no double add
    fn try_add_server(sender: &T::AccountId, country: &Vec<u8>) -> DispatchResult {
        let mut server_list = <ServersByCountry<T>>::get(country.clone());

        let cloned_sender = sender.clone();
        for item in &server_list {
            ensure!(*item != cloned_sender, "double registration not allowed!");
        }

        server_list.push(cloned_sender);
        <ServersByCountry<T>>::insert(country.clone(), server_list);
        Self::deposit_event(RawEvent::ServerAdded(sender.clone(), country.clone()));
        Ok(())
    }
}
