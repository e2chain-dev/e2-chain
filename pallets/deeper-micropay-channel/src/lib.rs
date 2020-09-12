#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_support::traits::{Currency, ExistenceRequirement::AllowDeath, Time, Vec};
use frame_support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure};
use frame_system::{self, ensure_signed};
use secp256k1;

mod hashing;

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
    expiration: Timestamp,
}

#[derive(Encode, Decode)]
pub struct Transfer<AccountId, Balance> {
    pub sender: AccountId,
    pub amount: Balance,
    pub nonce: u32,
}
#[derive(Encode, Decode)]
pub struct TransferData<AccountId, Balance> {
    pub data: Transfer<AccountId, Balance>,
    pub signature: Vec<u8>,
}

// events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        Timestamp = Moment<T>,
        Balance = BalanceOf<T>,
    {
        ChannelOpened(AccountId, AccountId, Timestamp),
        ChannelClosed(AccountId, AccountId, Timestamp),
        ClaimPayment(AccountId, AccountId, Balance),
    }
);

// storage for this module
decl_storage! {
  trait Store for Module<T: Trait> as Device {
      Channel get(fn get_channel): map hasher(blake2_128_concat) (T::AccountId, T::AccountId) => ChannelOf<T>;
      Nonce get(fn get_nonce): double_map hasher(blake2_128_concat) (T::AccountId, T::AccountId), hasher(blake2_128_concat) u32 => bool;
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
              expiration: time.clone(),
          };
          Channel::<T>::insert((sender.clone(),receiver.clone()), chan);
          Self::deposit_event(RawEvent::ChannelOpened(sender,receiver, time));
          Ok(())
      }

      #[weight = 10_000]
      pub fn claim_payment(origin, sender: T::AccountId, data: Vec<u8>) -> DispatchResult {
          let receiver = ensure_signed(origin)?;
          ensure!(Channel::<T>::contains_key((sender.clone(),receiver.clone())), "Channel not exists");

          let data = Decode::decode(&mut &data[..]);
          ensure!(data.is_ok(), "Bad transaction data");
          let transfer_data: TransferData<<T as frame_system::Trait>::AccountId, BalanceOf<T>> = data.unwrap();

          ensure!(!Nonce::<T>::contains_key((sender.clone(),receiver.clone()),transfer_data.data.nonce), "Nonce already consumed");
          Self::verify_signature(&sender, &transfer_data)?;

          T::Currency::transfer(&sender, &receiver, transfer_data.data.amount, AllowDeath)?; // TODO: check what is AllowDeath
          Nonce::<T>::insert((sender.clone(),receiver.clone()), transfer_data.data.nonce, true); // mark nonce as used
          Self::deposit_event(RawEvent::ClaimPayment(sender, receiver, transfer_data.data.amount));
          Ok(())
      }
  }
}

impl<T: Trait> Module<T> {
    // TODO: verify signature, signature is on hash of |receiver_addr|nonce|amount|
    // nonce represents session_id, during one session, a sender can send multiple accumulated
    // micropayments with the same nonce; the receiver can only claim one payment of the same
    // nonce, i.e. the latest accumulated micropayment.
    pub fn verify_signature(
        sender: &<T as frame_system::Trait>::AccountId,
        transfer_data: &TransferData<<T as frame_system::Trait>::AccountId, BalanceOf<T>>,
    ) -> DispatchResult {
        //let mut pk = [0u8; 33];
        //pk.copy_from_slice(&sender_pubkey);
        //let pk = secp256k1::PublicKey::from(sender).as_array_ref();
        //let pub_key = secp256k1::PublicKey::parse_compressed(&pk);
        //ensure!(pub_key.is_ok(), "Invalid Pubkey");

        /*
        let signature = secp256k1::Signature::parse_slice(&transfer_data.signature);
        ensure!(signature.is_ok(), "Invalid Signature");

        let msg_hash = hashing::blake2_256(&Encode::encode(&transfer_data.data));
        let mut buffer = [0u8; 32];
        buffer.copy_from_slice(&msg_hash);
        let message = secp256k1::Message::parse(&buffer);

        let verified = secp256k1::verify(&message, &signature.unwrap(), &pub_key.unwrap());
        ensure!(verified, "Fail to verify");
        */

        Ok(())
    }
}
