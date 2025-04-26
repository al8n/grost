use crate::{bridge, flavors::network::Network};

bridge!(
  Network: u8 {
    i8 {
      from: convert_u8_to_i8;
      to: convert_i8_to_u8;
    },
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
