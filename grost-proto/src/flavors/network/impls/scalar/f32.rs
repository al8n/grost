use crate::{
  bridge, default_wire_format,
  flavors::network::{Fixed32, Network, Varint},
  referenceable_scalar, selectable_scalar,
};

default_wire_format!(Network: f32 as Fixed32);
selectable_scalar!(Network:f32);
referenceable_scalar!(Network: f32 as Fixed32, f32 as Varint);
bridge!(
  Network: u32 {
    f32 as Fixed32 {
      from: f32::from_bits;
      to: convert_f32_to_u32;
    },
    f32 as Varint {
      from: f32::from_bits;
      to: convert_f32_to_u32;
    },
  },
);

#[inline]
const fn convert_f32_to_u32(v: &f32) -> u32 {
  v.to_bits()
}
