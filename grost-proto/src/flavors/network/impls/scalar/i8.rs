use crate::{
  bridge, decoded_state, default_wire_format, flatten_state,
  flavors::network::{DecodeError, Fixed8, Network, Varint},
  reflection::Type,
  selectable, try_from_bridge, type_reflection,
};
use core::num::NonZeroI8;

default_wire_format!(Network: i8 as Fixed8);
selectable!(@scalar Network:i8, NonZeroI8);
decoded_state!(@scalar &'a Network: i8 as Fixed8, NonZeroI8 as Fixed8, i8 as Varint, NonZeroI8 as Varint);
flatten_state!(i8, NonZeroI8);
type_reflection! {
  Network:
    i8 => Type::scalar("i8", "8-bit signed integer"),
    NonZeroI8 => Type::scalar("NonZeroI8", "Non-zero 8-bit signed integer"),
}

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
