#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use frame_support::codec::{Decode, Encode};
use frame_support::traits::{Currency, ReservableCurrency, Time, Vec};
use frame_support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure};
use frame_system::{self, ensure_signed};
/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    type Timestamp: Time;
}
const MIN_LOCK_AMT: u32 = 100;
type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
pub struct Node<AccountId> {
    account_id: AccountId,
    ipv4: Vec<u8>,
    country: Option<Vec<u8>>,
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<AccountId> _parity_scale_codec::Decode for Node<AccountId>
    where
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
    {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            Ok(Node {
                account_id: {
                    let __codec_res_edqy = _parity_scale_codec::Decode::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(_) => return Err("Error decoding field Node.account_id".into()),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
                ipv4: {
                    let __codec_res_edqy = _parity_scale_codec::Decode::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(_) => return Err("Error decoding field Node.ipv4".into()),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
                country: {
                    let __codec_res_edqy = _parity_scale_codec::Decode::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(_) => return Err("Error decoding field Node.country".into()),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
            })
        }
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<AccountId> _parity_scale_codec::Encode for Node<AccountId>
    where
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
    {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            __codec_dest_edqy.push(&self.account_id);
            __codec_dest_edqy.push(&self.ipv4);
            __codec_dest_edqy.push(&self.country);
        }
    }
    impl<AccountId> _parity_scale_codec::EncodeLike for Node<AccountId>
    where
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
    {
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::default::Default> ::core::default::Default for Node<AccountId> {
    #[inline]
    fn default() -> Node<AccountId> {
        Node {
            account_id: ::core::default::Default::default(),
            ipv4: ::core::default::Default::default(),
            country: ::core::default::Default::default(),
        }
    }
}
/// [`RawEvent`] specialized for the configuration [`Trait`]
///
/// [`RawEvent`]: enum.RawEvent.html
/// [`Trait`]: trait.Trait.html
pub type Event<T> = RawEvent<<T as frame_system::Trait>::AccountId>;
/// Events for this module.
///
pub enum RawEvent<AccountId> {
    RegisterNode(AccountId, Vec<u8>, Option<Vec<u8>>),
    UnregisterNode(AccountId),
    ServerAdded(AccountId, Vec<u8>),
    ServerRemoved(AccountId, Vec<u8>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::clone::Clone> ::core::clone::Clone for RawEvent<AccountId> {
    #[inline]
    fn clone(&self) -> RawEvent<AccountId> {
        match (&*self,) {
            (&RawEvent::RegisterNode(ref __self_0, ref __self_1, ref __self_2),) => {
                RawEvent::RegisterNode(
                    ::core::clone::Clone::clone(&(*__self_0)),
                    ::core::clone::Clone::clone(&(*__self_1)),
                    ::core::clone::Clone::clone(&(*__self_2)),
                )
            }
            (&RawEvent::UnregisterNode(ref __self_0),) => {
                RawEvent::UnregisterNode(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&RawEvent::ServerAdded(ref __self_0, ref __self_1),) => RawEvent::ServerAdded(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
            ),
            (&RawEvent::ServerRemoved(ref __self_0, ref __self_1),) => RawEvent::ServerRemoved(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
            ),
        }
    }
}
impl<AccountId> ::core::marker::StructuralPartialEq for RawEvent<AccountId> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::cmp::PartialEq> ::core::cmp::PartialEq for RawEvent<AccountId> {
    #[inline]
    fn eq(&self, other: &RawEvent<AccountId>) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) };
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) };
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &RawEvent::RegisterNode(ref __self_0, ref __self_1, ref __self_2),
                        &RawEvent::RegisterNode(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                    }
                    (
                        &RawEvent::UnregisterNode(ref __self_0),
                        &RawEvent::UnregisterNode(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &RawEvent::ServerAdded(ref __self_0, ref __self_1),
                        &RawEvent::ServerAdded(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                    (
                        &RawEvent::ServerRemoved(ref __self_0, ref __self_1),
                        &RawEvent::ServerRemoved(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<AccountId>) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) };
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) };
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &RawEvent::RegisterNode(ref __self_0, ref __self_1, ref __self_2),
                        &RawEvent::RegisterNode(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                    }
                    (
                        &RawEvent::UnregisterNode(ref __self_0),
                        &RawEvent::UnregisterNode(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &RawEvent::ServerAdded(ref __self_0, ref __self_1),
                        &RawEvent::ServerAdded(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                    (
                        &RawEvent::ServerRemoved(ref __self_0, ref __self_1),
                        &RawEvent::ServerRemoved(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
impl<AccountId> ::core::marker::StructuralEq for RawEvent<AccountId> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::cmp::Eq> ::core::cmp::Eq for RawEvent<AccountId> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Vec<u8>>;
            let _: ::core::cmp::AssertParamIsEq<Option<Vec<u8>>>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Vec<u8>>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Vec<u8>>;
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<AccountId> _parity_scale_codec::Encode for RawEvent<AccountId>
    where
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
    {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                RawEvent::RegisterNode(ref aa, ref ba, ref ca) => {
                    __codec_dest_edqy.push_byte(0usize as u8);
                    __codec_dest_edqy.push(aa);
                    __codec_dest_edqy.push(ba);
                    __codec_dest_edqy.push(ca);
                }
                RawEvent::UnregisterNode(ref aa) => {
                    __codec_dest_edqy.push_byte(1usize as u8);
                    __codec_dest_edqy.push(aa);
                }
                RawEvent::ServerAdded(ref aa, ref ba) => {
                    __codec_dest_edqy.push_byte(2usize as u8);
                    __codec_dest_edqy.push(aa);
                    __codec_dest_edqy.push(ba);
                }
                RawEvent::ServerRemoved(ref aa, ref ba) => {
                    __codec_dest_edqy.push_byte(3usize as u8);
                    __codec_dest_edqy.push(aa);
                    __codec_dest_edqy.push(ba);
                }
                _ => (),
            }
        }
    }
    impl<AccountId> _parity_scale_codec::EncodeLike for RawEvent<AccountId>
    where
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<AccountId> _parity_scale_codec::Decode for RawEvent<AccountId>
    where
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
    {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match __codec_input_edqy.read_byte()? {
                __codec_x_edqy if __codec_x_edqy == 0usize as u8 => Ok(RawEvent::RegisterNode(
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: RegisterNode.0".into())
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: RegisterNode.1".into())
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: RegisterNode.2".into())
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                )),
                __codec_x_edqy if __codec_x_edqy == 1usize as u8 => Ok(RawEvent::UnregisterNode({
                    let __codec_res_edqy = _parity_scale_codec::Decode::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(_) => {
                            return Err("Error decoding field RawEvent :: UnregisterNode.0".into())
                        }
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                })),
                __codec_x_edqy if __codec_x_edqy == 2usize as u8 => Ok(RawEvent::ServerAdded(
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: ServerAdded.0".into())
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: ServerAdded.1".into())
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                )),
                __codec_x_edqy if __codec_x_edqy == 3usize as u8 => Ok(RawEvent::ServerRemoved(
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(_) => {
                                return Err(
                                    "Error decoding field RawEvent :: ServerRemoved.0".into()
                                )
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(_) => {
                                return Err(
                                    "Error decoding field RawEvent :: ServerRemoved.1".into()
                                )
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                )),
                _ => Err("No such variant in enum RawEvent".into()),
            }
        }
    }
};
impl<AccountId> core::fmt::Debug for RawEvent<AccountId>
where
    AccountId: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::RegisterNode(ref a0, ref a1, ref a2) => fmt
                .debug_tuple("RawEvent::RegisterNode")
                .field(a0)
                .field(a1)
                .field(a2)
                .finish(),
            Self::UnregisterNode(ref a0) => fmt
                .debug_tuple("RawEvent::UnregisterNode")
                .field(a0)
                .finish(),
            Self::ServerAdded(ref a0, ref a1) => fmt
                .debug_tuple("RawEvent::ServerAdded")
                .field(a0)
                .field(a1)
                .finish(),
            Self::ServerRemoved(ref a0, ref a1) => fmt
                .debug_tuple("RawEvent::ServerRemoved")
                .field(a0)
                .field(a1)
                .finish(),
            _ => Ok(()),
        }
    }
}
impl<AccountId> From<RawEvent<AccountId>> for () {
    fn from(_: RawEvent<AccountId>) -> () {
        ()
    }
}
impl<AccountId> RawEvent<AccountId> {
    #[allow(dead_code)]
    #[doc(hidden)]
    pub fn metadata() -> &'static [::frame_support::event::EventMetadata] {
        &[
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("RegisterNode"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "Vec<u8>",
                    "Option<Vec<u8>>",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("UnregisterNode"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&["AccountId"]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("ServerAdded"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "Vec<u8>",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("ServerRemoved"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "Vec<u8>",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[]),
            },
        ]
    }
}
use self::sp_api_hidden_includes_decl_storage::hidden_include::{
    IterableStorageDoubleMap as _, IterableStorageMap as _, StorageDoubleMap as _, StorageMap as _,
    StoragePrefixedMap as _, StorageValue as _,
};
#[doc(hidden)]
mod sp_api_hidden_includes_decl_storage {
    pub extern crate frame_support as hidden_include;
}
trait Store {
    type DeviceInfo;
    type ServersByCountry;
}
impl<T: Trait + 'static> Store for Module<T> {
    type DeviceInfo = DeviceInfo<T>;
    type ServersByCountry = ServersByCountry<T>;
}
impl<T: Trait + 'static> Module<T> {
    pub fn get_device_info<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<T::AccountId>,
    >(
        key: K,
    ) -> Node<T::AccountId> {
        < DeviceInfo < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < T :: AccountId , Node < T :: AccountId > > > :: get ( key )
    }
    pub fn get_servers_by_country<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<Vec<u8>>,
    >(
        key: K,
    ) -> Vec<T::AccountId> {
        < ServersByCountry < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < Vec < u8 > , Vec < T :: AccountId > > > :: get ( key )
    }
}
#[doc(hidden)]
pub struct __GetByteStructDeviceInfo<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_DeviceInfo:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructDeviceInfo<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_DeviceInfo
            .get_or_init(|| {
                let def_val: Node<T::AccountId> = Default::default();
                <Node<T::AccountId> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructDeviceInfo<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructDeviceInfo<T> {}
#[doc(hidden)]
pub struct __GetByteStructServersByCountry<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_ServersByCountry:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructServersByCountry<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_ServersByCountry
            .get_or_init(|| {
                let def_val: Vec<T::AccountId> = Default::default();
                <Vec<T::AccountId> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructServersByCountry<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructServersByCountry<T> {}
impl<T: Trait + 'static> Module<T> {
    #[doc(hidden)]
    pub fn storage_metadata(
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageMetadata { prefix : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Device" ) , entries : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "DeviceInfo" ) , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Identity , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::AccountId" ) , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Node<T::AccountId>" ) , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructDeviceInfo :: < T > ( self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData ) ) ) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "ServersByCountry" ) , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Identity , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Vec<u8>" ) , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Vec<T::AccountId>" ) , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructServersByCountry :: < T > ( self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData ) ) ) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } ] [ .. ] ) , }
    }
}
/// Hidden instance generated to be internally used when module is used without
/// instance.
#[doc(hidden)]
pub struct __InherentHiddenInstance;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for __InherentHiddenInstance {
    #[inline]
    fn clone(&self) -> __InherentHiddenInstance {
        match *self {
            __InherentHiddenInstance => __InherentHiddenInstance,
        }
    }
}
impl ::core::marker::StructuralEq for __InherentHiddenInstance {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for __InherentHiddenInstance {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
impl ::core::marker::StructuralPartialEq for __InherentHiddenInstance {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for __InherentHiddenInstance {
    #[inline]
    fn eq(&self, other: &__InherentHiddenInstance) -> bool {
        match *other {
            __InherentHiddenInstance => match *self {
                __InherentHiddenInstance => true,
            },
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for __InherentHiddenInstance {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
        }
    }
    impl _parity_scale_codec::EncodeLike for __InherentHiddenInstance {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for __InherentHiddenInstance {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            Ok(__InherentHiddenInstance)
        }
    }
};
impl core::fmt::Debug for __InherentHiddenInstance {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("__InherentHiddenInstance").finish()
    }
}
impl self::sp_api_hidden_includes_decl_storage::hidden_include::traits::Instance
    for __InherentHiddenInstance
{
    const PREFIX: &'static str = "Device";
}
struct DeviceInfo<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<
        Node<T::AccountId>,
    > for DeviceInfo<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ( )
    }
    fn storage_prefix() -> &'static [u8] {
        b"DeviceInfo"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::AccountId,
        Node<T::AccountId>,
    > for DeviceInfo<T>
{
    type Query = Node<T::AccountId>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Identity;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ( )
    }
    fn storage_prefix() -> &'static [u8] {
        b"DeviceInfo"
    }
    fn from_optional_value_to_query(v: Option<Node<T::AccountId>>) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<Node<T::AccountId>> {
        Some(v)
    }
}
struct ServersByCountry<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<
        Vec<T::AccountId>,
    > for ServersByCountry<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ( )
    }
    fn storage_prefix() -> &'static [u8] {
        b"ServersByCountry"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        Vec<u8>,
        Vec<T::AccountId>,
    > for ServersByCountry<T>
{
    type Query = Vec<T::AccountId>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Identity;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ( )
    }
    fn storage_prefix() -> &'static [u8] {
        b"ServersByCountry"
    }
    fn from_optional_value_to_query(v: Option<Vec<T::AccountId>>) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<Vec<T::AccountId>> {
        Some(v)
    }
}
pub struct Module<T: Trait>(::frame_support::sp_std::marker::PhantomData<(T,)>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::clone::Clone + Trait> ::core::clone::Clone for Module<T> {
    #[inline]
    fn clone(&self) -> Module<T> {
        match *self {
            Module(ref __self_0_0) => Module(::core::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::marker::Copy + Trait> ::core::marker::Copy for Module<T> {}
impl<T: Trait> ::core::marker::StructuralPartialEq for Module<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::PartialEq + Trait> ::core::cmp::PartialEq for Module<T> {
    #[inline]
    fn eq(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) => match *self {
                Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) => match *self {
                Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl<T: Trait> ::core::marker::StructuralEq for Module<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::Eq + Trait> ::core::cmp::Eq for Module<T> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<
                ::frame_support::sp_std::marker::PhantomData<(T,)>,
            >;
        }
    }
}
impl<T: Trait> core::fmt::Debug for Module<T>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("Module").field(&self.0).finish()
    }
}
impl<T: Trait> ::frame_support::traits::OnInitialize<T::BlockNumber> for Module<T> {}
impl<T: Trait> ::frame_support::traits::OnRuntimeUpgrade for Module<T> {}
impl<T: Trait> ::frame_support::traits::OnFinalize<T::BlockNumber> for Module<T> {}
impl<T: Trait> ::frame_support::traits::OffchainWorker<T::BlockNumber> for Module<T> {}
impl<T: Trait> Module<T> {
    /// Deposits an event using `frame_system::Module::deposit_event`.
    fn deposit_event(event: impl Into<<T as Trait>::Event>) {
        <frame_system::Module<T>>::deposit_event(event.into())
    }
}
#[cfg(feature = "std")]
impl<T: Trait> ::frame_support::traits::IntegrityTest for Module<T> {}
/// Can also be called using [`Call`].
///
/// [`Call`]: enum.Call.html
impl<T: Trait> Module<T> {
    ///
    /// NOTE: Calling this function will bypass origin filters.
    pub fn register_device(
        origin: T::Origin,
        ip: Vec<u8>,
        country: Option<Vec<u8>>,
    ) -> DispatchResult {
        let __tracing_span__ = {
            {
                if ::sp_tracing::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::sp_tracing::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                {
                    use ::tracing::__macro_support::*;
                    let callsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "register_device",
                                "pallet_deeper_node",
                                ::sp_tracing::tracing::Level::TRACE,
                                Some("pallets/deeper-node/src/lib.rs"),
                                Some(56u32),
                                Some("pallet_deeper_node"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        static CALLSITE: MacroCallsite = MacroCallsite::new(&META);
                        CALLSITE.register();
                        &CALLSITE
                    };
                    if callsite.is_enabled() {
                        let meta = callsite.metadata();
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        ::tracing::Span::none()
                    }
                } else {
                    ::tracing::Span::none()
                }
            }
        };
        let __tracing_guard__ = { __tracing_span__.enter() };
        let sender = ensure_signed(origin)?;
        {
            if !(ip.len() == 4) {
                {
                    return Err("IPv4 has 4 bytes".into());
                };
            }
        };
        let node = Node {
            account_id: sender.clone(),
            ipv4: ip.clone(),
            country: country.clone(),
        };
        T::Currency::reserve(&sender, BalanceOf::<T>::from(MIN_LOCK_AMT))?;
        <DeviceInfo<T>>::insert(sender.clone(), node);
        Self::deposit_event(RawEvent::RegisterNode(sender, ip, country));
        Ok(())
    }
    ///
    /// NOTE: Calling this function will bypass origin filters.
    pub fn unregister_device(origin: T::Origin) -> DispatchResult {
        let __tracing_span__ = {
            {
                if ::sp_tracing::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::sp_tracing::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                {
                    use ::tracing::__macro_support::*;
                    let callsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "unregister_device",
                                "pallet_deeper_node",
                                ::sp_tracing::tracing::Level::TRACE,
                                Some("pallets/deeper-node/src/lib.rs"),
                                Some(56u32),
                                Some("pallet_deeper_node"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        static CALLSITE: MacroCallsite = MacroCallsite::new(&META);
                        CALLSITE.register();
                        &CALLSITE
                    };
                    if callsite.is_enabled() {
                        let meta = callsite.metadata();
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        ::tracing::Span::none()
                    }
                } else {
                    ::tracing::Span::none()
                }
            }
        };
        let __tracing_guard__ = { __tracing_span__.enter() };
        let sender = ensure_signed(origin)?;
        {
            if !<DeviceInfo<T>>::contains_key(sender.clone()) {
                {
                    return Err("device not registered!".into());
                };
            }
        };
        let node = <DeviceInfo<T>>::get(sender.clone());
        if !node.country.is_none() {
            let country = node.country.unwrap();
            let _ = Self::try_remove_server(&sender, &country);
        }
        <DeviceInfo<T>>::remove(sender.clone());
        T::Currency::unreserve(&sender, BalanceOf::<T>::from(MIN_LOCK_AMT));
        Self::deposit_event(RawEvent::UnregisterNode(sender));
        Ok(())
    }
    ///
    /// NOTE: Calling this function will bypass origin filters.
    pub fn register_server(origin: T::Origin, country: Vec<u8>) -> DispatchResult {
        let __tracing_span__ = {
            {
                if ::sp_tracing::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::sp_tracing::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                {
                    use ::tracing::__macro_support::*;
                    let callsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "register_server",
                                "pallet_deeper_node",
                                ::sp_tracing::tracing::Level::TRACE,
                                Some("pallets/deeper-node/src/lib.rs"),
                                Some(56u32),
                                Some("pallet_deeper_node"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        static CALLSITE: MacroCallsite = MacroCallsite::new(&META);
                        CALLSITE.register();
                        &CALLSITE
                    };
                    if callsite.is_enabled() {
                        let meta = callsite.metadata();
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        ::tracing::Span::none()
                    }
                } else {
                    ::tracing::Span::none()
                }
            }
        };
        let __tracing_guard__ = { __tracing_span__.enter() };
        let sender = ensure_signed(origin)?;
        {
            if !<DeviceInfo<T>>::contains_key(sender.clone()) {
                {
                    return Err("sender device needs register first".into());
                };
            }
        };
        {
            if !(country.len() == 2) {
                {
                    return Err("Country code has 2 byte".into());
                };
            }
        };
        let _ = Self::try_add_server(&sender, &country);
        Ok(())
    }
    ///
    /// NOTE: Calling this function will bypass origin filters.
    pub fn unregister_server(origin: T::Origin, country: Vec<u8>) -> DispatchResult {
        let __tracing_span__ = {
            {
                if ::sp_tracing::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::sp_tracing::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                {
                    use ::tracing::__macro_support::*;
                    let callsite = {
                        use ::tracing::__macro_support::MacroCallsite;
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "unregister_server",
                                "pallet_deeper_node",
                                ::sp_tracing::tracing::Level::TRACE,
                                Some("pallets/deeper-node/src/lib.rs"),
                                Some(56u32),
                                Some("pallet_deeper_node"),
                                ::tracing_core::field::FieldSet::new(
                                    &[],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::SPAN,
                            )
                        };
                        static CALLSITE: MacroCallsite = MacroCallsite::new(&META);
                        CALLSITE.register();
                        &CALLSITE
                    };
                    if callsite.is_enabled() {
                        let meta = callsite.metadata();
                        ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                    } else {
                        ::tracing::Span::none()
                    }
                } else {
                    ::tracing::Span::none()
                }
            }
        };
        let __tracing_guard__ = { __tracing_span__.enter() };
        let sender = ensure_signed(origin)?;
        {
            if !<DeviceInfo<T>>::contains_key(sender.clone()) {
                {
                    return Err("sender device needs register first".into());
                };
            }
        };
        {
            if !(country.len() == 2) {
                {
                    return Err("Country code has 2 byte".into());
                };
            }
        };
        let _ = Self::try_remove_server(&sender, &country);
        Ok(())
    }
}
/// Dispatchable calls.
///
/// Each variant of this enum maps to a dispatchable function from the associated module.
pub enum Call<T: Trait> {
    #[doc(hidden)]
    #[codec(skip)]
    __PhantomItem(
        ::frame_support::sp_std::marker::PhantomData<(T,)>,
        ::frame_support::Never,
    ),
    #[allow(non_camel_case_types)]
    register_device(Vec<u8>, Option<Vec<u8>>),
    #[allow(non_camel_case_types)]
    unregister_device(),
    #[allow(non_camel_case_types)]
    register_server(Vec<u8>),
    #[allow(non_camel_case_types)]
    unregister_server(Vec<u8>),
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T: Trait> _parity_scale_codec::Encode for Call<T> {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                Call::register_device(ref aa, ref ba) => {
                    __codec_dest_edqy.push_byte(0usize as u8);
                    __codec_dest_edqy.push(aa);
                    __codec_dest_edqy.push(ba);
                }
                Call::unregister_device() => {
                    __codec_dest_edqy.push_byte(1usize as u8);
                }
                Call::register_server(ref aa) => {
                    __codec_dest_edqy.push_byte(2usize as u8);
                    __codec_dest_edqy.push(aa);
                }
                Call::unregister_server(ref aa) => {
                    __codec_dest_edqy.push_byte(3usize as u8);
                    __codec_dest_edqy.push(aa);
                }
                _ => (),
            }
        }
    }
    impl<T: Trait> _parity_scale_codec::EncodeLike for Call<T> {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T: Trait> _parity_scale_codec::Decode for Call<T> {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match __codec_input_edqy.read_byte()? {
                __codec_x_edqy if __codec_x_edqy == 0usize as u8 => Ok(Call::register_device(
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(_) => {
                                return Err("Error decoding field Call :: register_device.0".into())
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(_) => {
                                return Err("Error decoding field Call :: register_device.1".into())
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                )),
                __codec_x_edqy if __codec_x_edqy == 1usize as u8 => Ok(Call::unregister_device()),
                __codec_x_edqy if __codec_x_edqy == 2usize as u8 => Ok(Call::register_server({
                    let __codec_res_edqy = _parity_scale_codec::Decode::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(_) => {
                            return Err("Error decoding field Call :: register_server.0".into())
                        }
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                })),
                __codec_x_edqy if __codec_x_edqy == 3usize as u8 => Ok(Call::unregister_server({
                    let __codec_res_edqy = _parity_scale_codec::Decode::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(_) => {
                            return Err("Error decoding field Call :: unregister_server.0".into())
                        }
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                })),
                _ => Err("No such variant in enum Call".into()),
            }
        }
    }
};
impl<T: Trait> ::frame_support::dispatch::GetDispatchInfo for Call<T> {
    fn get_dispatch_info(&self) -> ::frame_support::dispatch::DispatchInfo {
        match *self {
            Call::register_device(ref ip, ref country) => {
                let base_weight = 10_000;
                let weight = < dyn :: frame_support :: dispatch :: WeighData < ( & Vec < u8 > , & Option < Vec < u8 > > ) > > :: weigh_data ( & base_weight , ( ip , country ) ) ;
                let class = <dyn ::frame_support::dispatch::ClassifyDispatch<(
                    &Vec<u8>,
                    &Option<Vec<u8>>,
                )>>::classify_dispatch(&base_weight, (ip, country));
                let pays_fee = < dyn :: frame_support :: dispatch :: PaysFee < ( & Vec < u8 > , & Option < Vec < u8 > > ) > > :: pays_fee ( & base_weight , ( ip , country ) ) ;
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::unregister_device() => {
                let base_weight = 10_000;
                let weight =
                    <dyn ::frame_support::dispatch::WeighData<()>>::weigh_data(&base_weight, ());
                let class =
                    <dyn ::frame_support::dispatch::ClassifyDispatch<()>>::classify_dispatch(
                        &base_weight,
                        (),
                    );
                let pays_fee =
                    <dyn ::frame_support::dispatch::PaysFee<()>>::pays_fee(&base_weight, ());
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::register_server(ref country) => {
                let base_weight = 10_000;
                let weight = <dyn ::frame_support::dispatch::WeighData<(&Vec<u8>,)>>::weigh_data(
                    &base_weight,
                    (country,),
                );
                let class = < dyn :: frame_support :: dispatch :: ClassifyDispatch < ( & Vec < u8 > , ) > > :: classify_dispatch ( & base_weight , ( country , ) ) ;
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(&Vec<u8>,)>>::pays_fee(
                    &base_weight,
                    (country,),
                );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::unregister_server(ref country) => {
                let base_weight = 10_000;
                let weight = <dyn ::frame_support::dispatch::WeighData<(&Vec<u8>,)>>::weigh_data(
                    &base_weight,
                    (country,),
                );
                let class = < dyn :: frame_support :: dispatch :: ClassifyDispatch < ( & Vec < u8 > , ) > > :: classify_dispatch ( & base_weight , ( country , ) ) ;
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(&Vec<u8>,)>>::pays_fee(
                    &base_weight,
                    (country,),
                );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::__PhantomItem(_, _) => {
                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::GetCallName for Call<T> {
    fn get_call_name(&self) -> &'static str {
        match *self {
            Call::register_device(ref ip, ref country) => {
                let _ = (ip, country);
                "register_device"
            }
            Call::unregister_device() => {
                let _ = ();
                "unregister_device"
            }
            Call::register_server(ref country) => {
                let _ = (country);
                "register_server"
            }
            Call::unregister_server(ref country) => {
                let _ = (country);
                "unregister_server"
            }
            Call::__PhantomItem(_, _) => {
                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
    fn get_call_names() -> &'static [&'static str] {
        &[
            "register_device",
            "unregister_device",
            "register_server",
            "unregister_server",
        ]
    }
}
impl<T: Trait> ::frame_support::dispatch::Clone for Call<T> {
    fn clone(&self) -> Self {
        match *self {
            Call::register_device(ref ip, ref country) => {
                Call::register_device((*ip).clone(), (*country).clone())
            }
            Call::unregister_device() => Call::unregister_device(),
            Call::register_server(ref country) => Call::register_server((*country).clone()),
            Call::unregister_server(ref country) => Call::unregister_server((*country).clone()),
            _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::register_device(ref ip, ref country) => {
                let self_params = (ip, country);
                if let Call::register_device(ref ip, ref country) = *_other {
                    self_params == (ip, country)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::unregister_device() => {
                let self_params = ();
                if let Call::unregister_device() = *_other {
                    self_params == ()
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::register_server(ref country) => {
                let self_params = (country,);
                if let Call::register_server(ref country) = *_other {
                    self_params == (country,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::unregister_server(ref country) => {
                let self_params = (country,);
                if let Call::unregister_server(ref country) = *_other {
                    self_params == (country,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::Eq for Call<T> {}
impl<T: Trait> ::frame_support::dispatch::fmt::Debug for Call<T> {
    fn fmt(
        &self,
        _f: &mut ::frame_support::dispatch::fmt::Formatter,
    ) -> ::frame_support::dispatch::result::Result<(), ::frame_support::dispatch::fmt::Error> {
        match *self {
            Call::register_device(ref ip, ref country) => {
                _f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"register_device", &(ip.clone(), country.clone())) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                        ],
                    },
                ))
            }
            Call::unregister_device() => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"unregister_device", &()) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            Call::register_server(ref country) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"register_server", &(country.clone(),)) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            Call::unregister_server(ref country) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"unregister_server", &(country.clone(),)) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Trait> ::frame_support::traits::UnfilteredDispatchable for Call<T> {
    type Origin = T::Origin;
    fn dispatch_bypass_filter(
        self,
        _origin: Self::Origin,
    ) -> ::frame_support::dispatch::DispatchResultWithPostInfo {
        match self {
            Call::register_device(ip, country) => {
                <Module<T>>::register_device(_origin, ip, country)
                    .map(Into::into)
                    .map_err(Into::into)
            }
            Call::unregister_device() => <Module<T>>::unregister_device(_origin)
                .map(Into::into)
                .map_err(Into::into),
            Call::register_server(country) => <Module<T>>::register_server(_origin, country)
                .map(Into::into)
                .map_err(Into::into),
            Call::unregister_server(country) => <Module<T>>::unregister_server(_origin, country)
                .map(Into::into)
                .map_err(Into::into),
            Call::__PhantomItem(_, _) => {
                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::Callable<T> for Module<T> {
    type Call = Call<T>;
}
impl<T: Trait> Module<T> {
    #[doc(hidden)]
    pub fn call_functions() -> &'static [::frame_support::dispatch::FunctionMetadata] {
        &[
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("register_device"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("ip"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("country"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("Option<Vec<u8>>"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("unregister_device"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("register_server"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("country"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("unregister_server"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("country"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            },
        ]
    }
}
impl<T: 'static + Trait> Module<T> {
    #[doc(hidden)]
    pub fn module_constants_metadata(
    ) -> &'static [::frame_support::dispatch::ModuleConstantMetadata] {
        &[]
    }
}
impl<T: Trait> ::frame_support::dispatch::ModuleErrorMetadata for Module<T> {
    fn metadata() -> &'static [::frame_support::dispatch::ErrorMetadata] {
        <&'static str as ::frame_support::dispatch::ModuleErrorMetadata>::metadata()
    }
}
impl<T: Trait> Module<T> {
    fn try_remove_server(sender: &T::AccountId, country: &Vec<u8>) -> DispatchResult {
        let mut server_list = <ServersByCountry<T>>::get(country.clone());
        if let Some(index) = server_list.iter().position(|x| *x == sender.clone()) {
            server_list.remove(index);
            <ServersByCountry<T>>::insert(country.clone(), server_list);
            Self::deposit_event(RawEvent::ServerRemoved(sender.clone(), country.clone()));
            let mut node = <DeviceInfo<T>>::get(sender.clone());
            node.country = None;
            <DeviceInfo<T>>::insert(sender.clone(), node);
        }
        Ok(())
    }
    fn try_add_server(sender: &T::AccountId, country: &Vec<u8>) -> DispatchResult {
        let mut server_list = <ServersByCountry<T>>::get(country.clone());
        let cloned_sender = sender.clone();
        for item in &server_list {
            {
                if !(*item != cloned_sender) {
                    {
                        return Err("double registration not allowed!".into());
                    };
                }
            };
        }
        server_list.push(cloned_sender);
        <ServersByCountry<T>>::insert(country.clone(), server_list);
        Self::deposit_event(RawEvent::ServerAdded(sender.clone(), country.clone()));
        let mut node = <DeviceInfo<T>>::get(sender.clone());
        node.country = Some(country.clone());
        <DeviceInfo<T>>::insert(sender.clone(), node);
        Ok(())
    }
}
