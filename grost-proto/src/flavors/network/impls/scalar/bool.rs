use crate::{
  bridge,
  flavors::network::{Fixed8, Network, Varint},
};

bridge!(
  Network: u8 {
    bool as Fixed8 {
      from: convert_u8_to_bool;
      to: convert_bool_to_u8;
    },
    bool as Varint {
      from: convert_u8_to_bool;
      to: convert_bool_to_u8;
    },
  },
);

#[inline]
const fn convert_bool_to_u8(v: &bool) -> u8 {
  *v as u8
}

#[inline]
const fn convert_u8_to_bool(v: u8) -> bool {
  v != 0
}
