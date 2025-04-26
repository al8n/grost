use crate::{bridge, flavors::network::Network};
use half_2::f16;

bridge!(
  Network: u16 {
    f16 {
      from: convert_u16_to_f16;
      to: convert_f16_to_u16;
    },
  },
);

#[inline]
const fn convert_f16_to_u16(v: &f16) -> u16 {
  v.to_bits()
}

#[inline]
const fn convert_u16_to_f16(v: u16) -> f16 {
  f16::from_bits(v)
}
