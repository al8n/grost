use crate::{
  bridge, default_scalar_wire_format, flatten_state,
  flavors::groto::{Fixed32, Groto, Varint},
  groto_identity_transform, partial_ref_state, partial_state, selectable,
};

default_scalar_wire_format!(Groto: f32 as Fixed32);
selectable!(@scalar Groto:f32);
partial_ref_state!(@scalar &'a Groto:
  f32 as Fixed32,
  f32 as Varint,
);
partial_state!(@scalar Groto: f32);
flatten_state!(f32);
groto_identity_transform!(f32 as Fixed32, f32 as Varint,);
identity_partial_transform!(
  Groto {
    f32 as Fixed32,
    f32 as Varint,
  }
);

bridge!(
  Groto: u32 {
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
