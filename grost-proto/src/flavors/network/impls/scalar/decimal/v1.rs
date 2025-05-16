use crate::{
  bridge, default_wire_format,
  flavors::network::{Fixed128, Network, Varint},
  selectable, state,
};
use rust_decimal_1::Decimal as f128;

default_wire_format!(Network: f128 as Fixed128);
selectable!(@scalar Network:f128);
state!(@scalar &'a Network: f128 as Fixed128, f128 as Varint);
bridge!(
  Network: u128 {
    f128 as Fixed128 {
      from: convert_u128_to_f128;
      to: convert_f128_to_u128;
    },
    f128 as Varint {
      from: convert_u128_to_f128;
      to: convert_f128_to_u128;
    },
  },
);

#[inline]
const fn convert_f128_to_u128(v: &f128) -> u128 {
  u128::from_le_bytes(v.serialize())
}

#[inline]
fn convert_u128_to_f128(v: u128) -> f128 {
  f128::deserialize(v.to_le_bytes())
}
