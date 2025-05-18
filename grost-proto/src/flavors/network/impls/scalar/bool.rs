use crate::{
  bridge, decoded_state, default_wire_format, flatten_state,
  flavors::network::{Fixed8, Network, Varint},
  reflection::Type,
  selectable, type_reflection,
};

default_wire_format!(Network: bool as Fixed8);

selectable!(@scalar Network:bool);
decoded_state!(@scalar &'a Network: bool as Fixed8, bool as Varint);
flatten_state!(bool);
type_reflection! {
  Network:
    bool => Type::scalar("bool", "boolean"),
}
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
