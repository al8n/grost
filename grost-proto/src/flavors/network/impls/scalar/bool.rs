use crate::{
  bridge, default_wire_format,
  flavors::network::{Fixed8, Network, Varint},
  referenceable_scalar, selectable_bridge,
};

default_wire_format!(Network: bool as Fixed8);

selectable_bridge!(Network:u8[bool]);
referenceable_scalar!(Network: bool as Fixed8, bool as Varint);

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
