use crate::{
  bridge, decoded_state, default_wire_format, flatten_state,
  flavors::network::{Fixed32, Network, Varint},
  identity_transform, selectable,
};

default_wire_format!(Network: f32 as Fixed32);
selectable!(@scalar Network:f32);
decoded_state!(@scalar &'a Network: f32 as Fixed32, f32 as Varint);
flatten_state!(f32);
identity_transform!(
  Network {
    f32 as Fixed32,
    f32 as Varint,
  }
);

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
