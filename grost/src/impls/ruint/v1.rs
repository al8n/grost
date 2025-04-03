use ruint_1::Uint;

use crate::{
  Decode, DecodeOwned, IntoTarget, Message, PartialEncode, Encode, TypeOwned,
  TypeRef, Wirable,
};

impl<const BITS: usize, const LMITS: usize> Wirable for Uint<BITS, LMITS> {
  wirable!(@varint);
}

impl<const BITS: usize, const LMITS: usize> Encode for Uint<BITS, LMITS> {
  varint!(@encode_impl);
}

impl<'de, const BITS: usize, const LMITS: usize> Decode<'de> for Uint<BITS, LMITS> {
  varint!(@decode_impl);
}

impl<const BITS: usize, const LMITS: usize> DecodeOwned for Uint<BITS, LMITS> {
  varint!(@decode_owned_impl);
}

impl<const BITS: usize, const LMITS: usize> Message for Uint<BITS, LMITS> {
  message!(@impl);
}

impl<const BITS: usize, const LMITS: usize> TypeRef<Self> for Uint<BITS, LMITS> {
  type_ref!(@copy_impl);
}

impl<const BITS: usize, const LMITS: usize> TypeOwned<Self> for Uint<BITS, LMITS> {
  type_owned!(@copy_impl);
}

impl<const BITS: usize, const LMITS: usize> IntoTarget<Self> for Uint<BITS, LMITS> {
  into_target!(@impl);
}

impl<const BITS: usize, const LMITS: usize> PartialEncode for Uint<BITS, LMITS> {
  partial_encode_primitives!(@impl);
}
