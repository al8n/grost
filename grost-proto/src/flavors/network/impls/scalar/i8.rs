use crate::{bridge, flavors::network::Network, try_from_bridge};
use core::num::NonZeroI8;

bridge!(
  Network: u8 {
    i8 {
      from: convert_u8_to_i8;
      to: convert_i8_to_u8;
    },
  },
);

try_from_bridge!(
  Network: i8 {
    NonZeroI8 {
      try_from: |v: i8| NonZeroI8::new(v).ok_or_else(|| crate::error::DecodeError::custom("value cannot be zero"));
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
