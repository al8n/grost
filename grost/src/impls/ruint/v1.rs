use ruint_1::Uint;

use crate::{
  Deserialize, DeserializeOwned, IntoTarget, Message, PartialSerialize, Serialize, TypeOwned,
  TypeRef, Wirable,
};

impl<const BITS: usize, const LMITS: usize> Wirable for Uint<BITS, LMITS> {
  wirable!(@varint);
}

impl<const BITS: usize, const LMITS: usize> Serialize for Uint<BITS, LMITS> {
  varint!(@serialize_impl);
}

impl<'de, const BITS: usize, const LMITS: usize> Deserialize<'de> for Uint<BITS, LMITS> {
  varint!(@deserialize_impl);
}

impl<const BITS: usize, const LMITS: usize> DeserializeOwned for Uint<BITS, LMITS> {
  varint!(@deserialize_owned_impl);
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

impl<const BITS: usize, const LMITS: usize> PartialSerialize for Uint<BITS, LMITS> {
  partial_serialize_primitives!(@impl);
}
