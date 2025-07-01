use crate::{
  bridge, default_wire_format, flatten_state,
  flavors::groto::{Fixed64, Groto, Varint},
  groto_identity_transform, partial_ref_state, partial_state, selectable,
};

default_wire_format!(Groto: f64 as Fixed64);
selectable!(@scalar Groto:f64);
partial_ref_state!(@scalar &'a Groto:
  f64 as Fixed64,
  f64 as Varint,
);
partial_state!(@scalar Groto: f64);
flatten_state!(f64);
groto_identity_transform!(f64 as Fixed64, f64 as Varint,);
identity_partial_transform!(
  Groto {
    f64 as Fixed64,
    f64 as Varint,
  }
);

bridge!(
  Groto: u64 {
    f64 as Fixed64 {
      from: f64::from_bits;
      to: convert_f64_to_u64;
    },
    f64 as Varint {
      from: f64::from_bits;
      to: convert_f64_to_u64;
    },
  },
);

#[inline]
const fn convert_f64_to_u64(v: &f64) -> u64 {
  v.to_bits()
}
