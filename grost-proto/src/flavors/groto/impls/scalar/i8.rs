use crate::{
  bridge, default_scalar_wire_format, flatten_state,
  flavors::groto::{DecodeError, Fixed8, Groto, Varint},
  partial_identity, partial_ref_state, partial_state, ref_state, selectable, try_from_bridge,
};
use core::num::NonZeroI8;

default_scalar_wire_format!(Groto: i8 as Fixed8; NonZeroI8 as Fixed8);
selectable!(@scalar Groto:i8, NonZeroI8);
ref_state!(@scalar &'a Groto:
  i8 as Fixed8,
  NonZeroI8 as Fixed8,
  i8 as Varint,
  NonZeroI8 as Varint,
);
partial_ref_state!(@scalar &'a Groto:
  i8 as Fixed8,
  NonZeroI8 as Fixed8,
  i8 as Varint,
  NonZeroI8 as Varint,
);
partial_state!(@scalar Groto: i8, NonZeroI8);
flatten_state!(i8, NonZeroI8);
partial_identity!(@scalar Groto: i8, NonZeroI8);

bridge!(
  Groto: u8 {
    i8 as Fixed8 {
      from: convert_u8_to_i8;
      to: convert_i8_to_u8;
    },
    i8 as Varint {
      from: convert_u8_to_i8;
      to: convert_i8_to_u8;
    },
  },
);

try_from_bridge!(
  Groto: i8 {
    NonZeroI8 as Fixed8 {
      try_from: |v: i8| NonZeroI8::new(v).ok_or_else(|| DecodeError::other("value cannot be zero"));
      to: |v: &NonZeroI8| v.get();
    },
    NonZeroI8 as Varint {
      try_from: |v: i8| NonZeroI8::new(v).ok_or_else(|| DecodeError::other("value cannot be zero"));
      to: |v: &NonZeroI8| v.get();
    }
  },
);

#[inline]
const fn convert_i8_to_u8(v: &i8) -> u8 {
  *v as u8
}

#[inline]
const fn convert_u8_to_i8(v: u8) -> i8 {
  v as i8
}
