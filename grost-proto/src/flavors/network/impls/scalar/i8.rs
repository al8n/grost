use crate::{
  bridge, default_wire_format,
  flavors::network::{DecodeError, Fixed8, Network, Varint},
  referenceable_scalar, selectable_bridge, try_from_bridge,
};
use core::num::NonZeroI8;

default_wire_format!(Network: i8 as Fixed8);
selectable_bridge!(Network:u8[i8]);
referenceable_scalar!(Network: i8, NonZeroI8);
bridge!(
  Network: u8 {
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

selectable_bridge!(Network:i8[NonZeroI8]);
try_from_bridge!(
  Network: i8 {
    NonZeroI8 as Fixed8 {
      try_from: |v: i8| NonZeroI8::new(v).ok_or_else(|| DecodeError::custom("value cannot be zero"));
      to: |v: &NonZeroI8| v.get();
    },
    NonZeroI8 as Varint {
      try_from: |v: i8| NonZeroI8::new(v).ok_or_else(|| DecodeError::custom("value cannot be zero"));
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
