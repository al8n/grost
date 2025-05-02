use crate::{
  decode::{Decode, DecodeOwned},
  decode_owned_scalar,
  encode::{Encode, PartialEncode},
  flavors::{
    network::{Context, DecodeError, EncodeError, Unknown, Zst}, Network, Selectable
  },
  partial_encode_scalar,
};

mod arbitrary_int;
mod bnum;
mod bool;
mod bytes;
mod char;
mod decimal;
mod f32;
mod f64;
mod half;
mod i128;
mod i16;
mod i32;
mod i64;
mod i8;
mod net;
// // mod num_complex;
// // mod num_rational;
mod ruint;
mod slice;
mod str;
mod string;
mod time;
mod u128;
mod u16;
mod u32;
mod u64;
mod u8;
mod uuid;

crate::network_zst!(());
crate::network_phantom!(core::marker::PhantomData<T>);
