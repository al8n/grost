use crate::{
  bridge, default_scalar_wire_format, flatten_state,
  flavors::groto::{Fixed128, Groto, Varint},
  groto_identity_transform, partial_ref_state, partial_state, selectable,
};
use rust_decimal_1::Decimal as f128;

default_scalar_wire_format!(Groto: f128 as Fixed128);
selectable!(@scalar Groto:f128);
partial_ref_state!(@scalar &'a Groto:
  f128 as Fixed128,
  f128 as Varint,
);
partial_state!(@scalar Groto: f128);
flatten_state!(f128);
groto_identity_transform!(f128 as Fixed128, f128 as Varint,);
identity_partial_transform!(
  Groto {
    f128 as Fixed128,
    f128 as Varint,
  }
);
bridge!(
  Groto: u128 {
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
