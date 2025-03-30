use ruint_1::Uint;

use crate::{Deserialize, DeserializeOwned, OutputType, Serialize, TypeOwned, TypeRef, Wirable};

impl<const BITS: usize, const LMITS: usize> Wirable for Uint<BITS, LMITS> {
  wirable!(@varint);
}

impl<const BITS: usize, const LMITS: usize> Serialize for Uint<BITS, LMITS> {
  impl_varing_types!(@serialize);
}

impl<'de, const BITS: usize, const LMITS: usize> Deserialize<'de> for Uint<BITS, LMITS> {
  impl_varing_types!(@deserialize);
}

impl<const BITS: usize, const LMITS: usize> DeserializeOwned for Uint<BITS, LMITS> {
  impl_varing_types!(@deserialize_owned);
}

impl<const BITS: usize, const LMITS: usize> OutputType for Uint<BITS, LMITS> {
  impl_output_type_for_self!();
}

impl<const BITS: usize, const LMITS: usize> TypeRef<Self> for Uint<BITS, LMITS> {
  impl_varing_types!(@type_ref);
}

impl<const BITS: usize, const LMITS: usize> TypeOwned<Self> for Uint<BITS, LMITS> {
  impl_varing_types!(@type_owned);
}
