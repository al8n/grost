use crate::{
  decode::{Decode, DecodeOwned},
  decode_owned_scalar,
  encode::{Encode, PartialEncode},
  flavors::{
    Network,
    network::{Context, DecodeError, EncodeError, Unknown, Zst},
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
// mod net;
// mod num_complex;
// mod num_rational;
mod ruint;
mod slice;
mod str;
// mod string;
mod time;
mod u128;
mod u16;
mod u32;
mod u64;
mod u8;
mod uuid;

// crate::network_zst!(());
// crate::network_phantom!(core::marker::PhantomData<T>);

impl<T> Encode<Network, Zst> for [T; 0] {
  fn encode(&self, _: &Context, _: &mut [u8]) -> Result<usize, EncodeError> {
    Ok(0)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    0
  }

  fn encoded_length_delimited_len(&self, _: &Context) -> usize {
    0
  }

  fn encode_length_delimited(&self, _: &Context, _: &mut [u8]) -> Result<usize, EncodeError> {
    Ok(0)
  }
}

impl<T> PartialEncode<Network, Zst> for [T; 0] {
  partial_encode_scalar!(@impl Network as Zst);
}

impl<'de, T> Decode<'de, Network, Zst, Self> for [T; 0] {
  fn decode<UB>(_: &Context, _: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
    Ok((0, []))
  }

  fn decode_length_delimited<UB>(
    ctx: &Context,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
    <Self as Decode<'de, Network, Zst, Self>>::decode::<UB>(ctx, src)
  }
}

impl DecodeOwned<Network, Zst, Self> for [u8; 0] {
  decode_owned_scalar!(@impl Network as Zst);
}
