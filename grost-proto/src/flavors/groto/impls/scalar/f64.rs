use crate::{
  bridge, default_scalar_wire_format, flatten_state,
  flavors::groto::{Fixed64, Groto, Varint},
  partial_identity, partial_ref_state, partial_state, ref_state, selectable,
};

default_scalar_wire_format!(Groto: f64 as Fixed64);
selectable!(@scalar Groto:f64);
ref_state!(@scalar &'a Groto:
  f64 as Fixed64,
  f64 as Varint,
);
partial_ref_state!(@scalar &'a Groto:
  f64 as Fixed64,
  f64 as Varint,
);
partial_state!(@scalar Groto: f64);
flatten_state!(f64);
partial_identity!(@scalar Groto: f64);

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
