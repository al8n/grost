use crate::{
  bridge, decoded_state, default_wire_format, flatten_state,
  flavors::network::{Fixed64, Network, Varint},
  selectable,
};

default_wire_format!(Network: f64 as Fixed64);
selectable!(@scalar Network:f64);
decoded_state!(@scalar &'a Network: f64 as Fixed64, f64 as Varint);
flatten_state!(f64);

bridge!(
  Network: u64 {
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
