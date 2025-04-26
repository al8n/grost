use crate::{bridge, flavors::network::Network};

bridge!(
  Network: u32 {
    f32 {
      from: convert_u32_to_f32;
      to: convert_f32_to_u32;
    },
  },
);

#[inline]
const fn convert_f32_to_u32(v: &f32) -> u32 {
  u32::from_le_bytes(v.to_le_bytes())
}

#[inline]
const fn convert_u32_to_f32(v: u32) -> f32 {
  f32::from_le_bytes(v.to_le_bytes())
}
