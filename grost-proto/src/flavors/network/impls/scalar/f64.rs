use crate::{
  bridge,
  flavors::network::{Fixed64, Network, Varint},
};

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
