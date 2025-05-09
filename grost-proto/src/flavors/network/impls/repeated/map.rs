// use crate::{
//   flavors::{
//     Network,
//     network::{Fixed8, Fixed16, Fixed32, Fixed64, Fixed128, LengthDelimited, Varint},
//   },
//   selector::Selectable,
// };

// macro_rules! map_impl {
//   (@encode $wf:ty) => {
//     fn encode(
//       &self,
//       context: &<$crate::__private::flavors::Network as $crate::flavors::Flavor>::Context,
//       buf: &mut [u8],
//     ) -> Result<usize, <$crate::__private::flavors::Network as $crate::flavors::Flavor>::EncodeError> {
//       let mut offset = 0;
//       let buf_len = buf.len();
//       let encoded_len = <Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encoded_len(self, context);
//       if encoded_len > buf_len {
//         return ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(encoded_len, buf_len));
//       }

//       for (k, v) in self {
//         if offset >= buf_len {
//           return ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(offset, buf_len));
//         }
//         offset += K::encode_length_delimited(k, context, &mut buf[offset..])
//           .map_err(|e| e.update(<Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encoded_len(self, context), buf_len))?;

//         if offset >= buf_len {
//           return ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(offset, buf_len));
//         }
//         offset += V::encode_length_delimited(v, context, &mut buf[offset..])
//           .map_err(|e| e.update(<Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encoded_len(self, context), buf_len))?;
//       }

//       #[cfg(debug_assertions)]
//       {
//         $crate::debug_assert_write_eq::<Self>(offset, <Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encoded_len(self, context));
//       }

//       ::core::result::Result::Ok(offset)
//     }
//   };
//   (@fixed_encode $wf:ty:$merged_wf:ty) => {
//     fn encode(
//       &self,
//       context: &<$crate::__private::flavors::Network as $crate::flavors::Flavor>::Context,
//       buf: &mut [u8],
//     ) -> Result<usize, <$crate::__private::flavors::Network as $crate::flavors::Flavor>::EncodeError> {
//       let mut offset = 0;
//       let buf_len = buf.len();
//       let encoded_len = <Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$merged_wf>>>::encoded_len(self, context);
//       if encoded_len > buf_len {
//         return ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(encoded_len, buf_len));
//       }

//       for (k, v) in self {
//         let mut start_offset = offset;
//         if offset >= buf_len {
//           return ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(offset, buf_len));
//         }
//         offset += K::encode_length_delimited(k, context, &mut buf[offset..])
//           .map_err(|e| e.update(<Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$merged_wf>>>::encoded_len(self, context), buf_len))?;

//         #[cfg(debug_assertions)]
//         {
//           assert_eq!(offset - start_offset, <$wf>::SIZE, "expected {} bytes written for each key", <$wf>::SIZE);
//         }

//         start_offset = offset;
//         if offset >= buf_len {
//           return ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(offset, buf_len));
//         }
//         offset += V::encode_length_delimited(v, context, &mut buf[offset..])
//           .map_err(|e| e.update(<Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$merged_wf>>>::encoded_len(self, context), buf_len))?;

//         #[cfg(debug_assertions)]
//         {
//           assert_eq!(offset - start_offset, <$wf>::SIZE, "expected {} bytes written for each value", <$wf>::SIZE);
//         }
//       }

//       #[cfg(debug_assertions)]
//       {
//         $crate::debug_assert_write_eq::<Self>(offset, <Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$merged_wf>>>::encoded_len(self, context));
//       }

//       ::core::result::Result::Ok(offset)
//     }
//   };
//   (@impl $wf:ty) => {
//     map_impl!(@encode $wf);

//     fn encoded_len(&self, ctx: &<$crate::__private::flavors::Network as $crate::flavors::Flavor>::Context) -> usize {
//       self.iter().map(|(k, v)| k.encoded_length_delimited_len(ctx) + v.encoded_length_delimited_len(ctx)).sum()
//     }

//     repeated_impl!(@encode_length_delimited $wf);
//   };
//   (@fixed_impl $wf:ty:$merged_wf:ty) => {
//     map_impl!(@fixed_encode $wf:$merged_wf);

//     fn encoded_len(&self, _: &<$crate::__private::flavors::Network as $crate::flavors::Flavor>::Context) -> usize {
//       self.len() * <$merged_wf>::SIZE
//     }

//     repeated_impl!(@encode_length_delimited $merged_wf);
//   };
//   (@fixed_kv $ty:ty:$(
//     $wf:ty:$merged_wf:ty
//   ),+$(,)?) => {
//     $(
//       impl<K, V> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$merged_wf>> for $ty
//       where
//         K: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//         V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//       {
//         map_impl!(@fixed_impl $wf:$merged_wf);
//       }
//     )*
//   };
//   (@fixed_array_kv $ty:ty:$(
//     $wf:ty:$merged_wf:ty
//   ),+$(,)?) => {
//     $(
//       impl<K, V, const N: usize> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$merged_wf>> for $ty
//       where
//         K: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//         V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//       {
//         map_impl!(@fixed_impl $wf:$merged_wf);
//       }
//     )*
//   };
//   (@kv $ty:ty:$(
//     $wf:ty
//   ),+$(,)?) => {
//     $(
//       impl<K, V> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>> for $ty
//       where
//         K: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//         V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//       {
//         map_impl!(@impl $wf);
//       }
//     )*
//   };
//   (@array_kv $ty:ty:$(
//     $wf:ty
//   ),+$(,)?) => {
//     $(
//       impl<K, V, const N: usize> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>> for $ty
//       where
//         K: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//         V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//       {
//         map_impl!(@impl $wf);
//       }
//     )*
//   };
//   (@fixed_kvh $ty:ty:$(
//     $wf:ty:$merged_wf:ty
//   ),+$(,)?) => {
//     $(
//       impl<K, V, S> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$merged_wf>> for $ty
//       where
//         K: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//         V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//       {
//         map_impl!(@fixed_impl $wf:$merged_wf);
//       }
//     )*
//   };
//   (@fixed_array_kvh $ty:ty:$(
//     $wf:ty:$merged_wf:ty
//   ),+$(,)?) => {
//     $(
//       impl<K, V, S, const N: usize> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$merged_wf>> for $ty
//       where
//         K: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//         V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//       {
//         map_impl!(@fixed_impl $wf:$merged_wf);
//       }
//     )*
//   };
//   (@kvh $ty:ty:$(
//     $wf:ty
//   ),+$(,)?) => {
//     $(
//       impl<K, V, S> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>> for $ty
//       where
//         K: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//         V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//       {
//         map_impl!(@impl $wf);
//       }
//     )*
//   };
//   (@array_kvh $ty:ty:$(
//     $wf:ty
//   ),+$(,)?) => {
//     $(
//       impl<K, V, S, const N: usize> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>> for $ty
//       where
//         K: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//         V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
//       {
//         map_impl!(@impl $wf);
//       }
//     )*
//   };
// }

// map_impl!(
//   @fixed_array_kv
//   [(K, V); N]:
//     Fixed8:Fixed16,
//     Fixed16:Fixed32,
//     Fixed32:Fixed64,
//     Fixed64:Fixed128,
// );

// map_impl!(
//   @array_kv
//   [(K, V); N]:
//     Varint,
//     LengthDelimited,
// );

// #[cfg(feature = "std")]
// const _: () = {
//   use std::collections::HashMap;

//   map_impl!(
//     @fixed_kvh HashMap<K, V, S>:
//       Fixed8:Fixed16,
//       Fixed16:Fixed32,
//       Fixed32:Fixed64,
//       Fixed64:Fixed128,
//   );

//   map_impl!(@kvh
//     HashMap<K, V, S>:
//       Varint,
//       LengthDelimited,
//   );
// };

// #[cfg(feature = "hashbrown_0_15")]
// const _: () = {
//   use hashbrown_0_15::HashMap;

//   map_impl!(
//     @fixed_kvh HashMap<K, V, S>:
//       Fixed8:Fixed16,
//       Fixed16:Fixed32,
//       Fixed32:Fixed64,
//       Fixed64:Fixed128,
//   );

//   map_impl!(@kvh
//     HashMap<K, V, S>:
//       Varint,
//       LengthDelimited,
//   );
// };

// #[cfg(feature = "indexmap_2")]
// const _: () = {
//   use indexmap_2::IndexMap;

//   map_impl!(
//     @fixed_kvh IndexMap<K, V, S>:
//       Fixed8:Fixed16,
//       Fixed16:Fixed32,
//       Fixed32:Fixed64,
//       Fixed64:Fixed128,
//   );

//   map_impl!(
//     @kvh IndexMap<K, V, S>:
//       Varint,
//       LengthDelimited,
//   );
// };

// #[cfg(any(feature = "std", feature = "alloc"))]
// const _: () = {
//   use std::{collections::BTreeMap, vec::Vec};

//   map_impl!(
//     @fixed_kv
//     BTreeMap<K, V>:
//       Fixed8:Fixed16,
//       Fixed16:Fixed32,
//       Fixed32:Fixed64,
//       Fixed64:Fixed128,
//   );

//   map_impl!(
//     @kv
//     BTreeMap<K, V>:
//       Varint,
//       LengthDelimited,
//   );

//   map_impl!(
//     @fixed_kv
//     Vec<(K, V)>:
//       Fixed8:Fixed16,
//       Fixed16:Fixed32,
//       Fixed32:Fixed64,
//       Fixed64:Fixed128,
//   );

//   map_impl!(
//     @kv
//     Vec<(K, V)>:
//       Varint,
//       LengthDelimited,
//   );
// };

// #[cfg(feature = "smallvec_1")]
// const _: () = {
//   use smallvec_1::SmallVec;

//   map_impl!(
//     @fixed_array_kv
//     SmallVec<[(K, V); N]>:
//       Fixed8:Fixed16,
//       Fixed16:Fixed32,
//       Fixed32:Fixed64,
//       Fixed64:Fixed128,
//   );

//   map_impl!(
//     @array_kv
//     SmallVec<[(K, V); N]>:
//       Varint,
//       LengthDelimited,
//   );
// };

// #[cfg(feature = "arrayvec_0_7")]
// const _: () = {
//   use arrayvec_0_7::ArrayVec;

//   map_impl!(
//     @fixed_array_kv
//     ArrayVec<(K, V), N>:
//       Fixed8:Fixed16,
//       Fixed16:Fixed32,
//       Fixed32:Fixed64,
//       Fixed64:Fixed128,
//   );

//   map_impl!(
//     @array_kv
//     ArrayVec<(K, V), N>:
//       Varint,
//       LengthDelimited,
//   );
// };

// #[cfg(feature = "tinyvec_1")]
// const _: () = {
//   use tinyvec_1::{Array, ArrayVec};

//   use crate::{
//     encode::Encode,
//     flavors::{Network, network::Repeated},
//   };

//   macro_rules! tinyvec_impl {
//     (@fixed $ty:ty:$($wf:ty:$merged_wf:ty),+$(,)?) => {
//       $(
//         impl<A, K, V> Encode<Network, Repeated<$merged_wf>> for $ty
//         where
//           K: Encode<Network, $wf>,
//           V: Encode<Network, $wf>,
//           A: Array<Item = (K, V)>,
//         {
//           map_impl!(@fixed_impl $wf:$merged_wf);
//         }
//       )*
//     };
//     ($ty:ty: $($wf:ty),+$(,)?) => {
//       $(
//         impl<A, K, V> Encode<Network, Repeated<$wf>> for $ty
//         where
//           K: Encode<Network, $wf>,
//           V: Encode<Network, $wf>,
//           A: Array<Item = (K, V)>,
//         {
//           map_impl!(@impl $wf);
//         }
//       )*
//     };
//   }

//   tinyvec_impl!(
//     @fixed ArrayVec<A>:
//     Fixed8:Fixed16,
//     Fixed16:Fixed32,
//     Fixed32:Fixed64,
//     Fixed64:Fixed128,
//   );

//   tinyvec_impl!(
//     ArrayVec<A>:
//       Varint,
//       LengthDelimited,
//   );

//   #[cfg(any(feature = "std", feature = "alloc"))]
//   const _: () = {
//     use tinyvec_1::TinyVec;

//     tinyvec_impl!(
//       @fixed TinyVec<A>:
//       Fixed8:Fixed16,
//       Fixed16:Fixed32,
//       Fixed32:Fixed64,
//       Fixed64:Fixed128,
//     );
//     tinyvec_impl!(
//       TinyVec<A>:
//       Varint,
//       LengthDelimited,
//     );
//   };
// };
