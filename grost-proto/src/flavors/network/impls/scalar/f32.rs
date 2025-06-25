use crate::{
  bridge, default_wire_format, flatten_state, flavors::network::{Fixed32, Network, Varint}, identity_partial_transform, identity_transform, partial_ref_state, partial_state, selectable
};

default_wire_format!(Network: f32 as Fixed32);
selectable!(@scalar Network:f32);
partial_ref_state!(@scalar &'a Network: f32 as Fixed32, f32 as Varint);
partial_state!(@scalar Network: f32 as Fixed32, f32 as Varint);
flatten_state!(f32);
identity_transform!(
  Network {
    f32 as Fixed32,
    f32 as Varint,
  }
);
identity_partial_transform!(
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
