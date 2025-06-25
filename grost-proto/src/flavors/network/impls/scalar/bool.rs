use crate::{
  bridge, default_wire_format, flatten_state, flavors::network::{Fixed8, Network, Varint}, identity_partial_transform, identity_transform, partial_ref_state, partial_state, selectable
};

default_wire_format!(Network: bool as Fixed8);

selectable!(@scalar Network:bool);
partial_ref_state!(@scalar &'a Network: bool as Fixed8, bool as Varint);
partial_state!(@scalar Network: bool as Fixed8, bool as Varint);
flatten_state!(bool);
identity_transform!(
  Network {
    bool as Fixed8,
    bool as Varint,
  }
);
identity_partial_transform!(
  Network { bool as Fixed8, bool as Varint }
);

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
