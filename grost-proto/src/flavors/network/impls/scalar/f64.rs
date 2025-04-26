use crate::{bridge, flavors::network::Network};

bridge!(
  Network: u64 {
    f64 {
      from: convert_u64_to_f64;
      to: convert_f64_to_u64;
    },
  },
);

#[inline]
const fn convert_f64_to_u64(v: &f64) -> u64 {
  u64::from_le_bytes(v.to_le_bytes())
}

#[inline]
const fn convert_u64_to_f64(v: u64) -> f64 {
  f64::from_le_bytes(v.to_le_bytes())
}
