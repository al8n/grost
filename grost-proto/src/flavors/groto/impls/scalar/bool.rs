use crate::{
  bridge, default_scalar_wire_format, flatten_state,
  flavors::groto::{Fixed8, Groto, Varint},
  partial_ref_state, partial_state, ref_state, selectable,
};

default_scalar_wire_format!(Groto: bool as Fixed8);

selectable!(@scalar Groto:bool);
ref_state!(@scalar &'a Groto:
  bool as Fixed8,
  bool as Varint,
);
partial_ref_state!(@scalar &'a Groto:
  bool as Fixed8,
  bool as Varint,
);
partial_state!(@scalar Groto: bool);
flatten_state!(bool);

bridge!(
  Groto: u8 {
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
