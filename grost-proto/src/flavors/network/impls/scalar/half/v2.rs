use crate::{
  bridge, decoded_state, default_wire_format, flatten_state,
  flavors::network::{Fixed16, Network, Varint},
  reflection::Type,
  selectable, type_reflection,
};
use half_2::f16;

default_wire_format!(Network: f16 as Fixed16);

selectable!(@scalar Network:f16);
decoded_state!(@scalar &'a Network: f16 as Fixed16, f16 as Varint);
flatten_state!(f16);
type_reflection! {
  Network:
    f16 => Type::scalar("f16", "A 16-bit floating point type implementing the IEEE 754-2008 standard binary16 a.k.a \"half\" format."),
}
bridge!(
  Network: u16 {
    f16 as Fixed16 {
      from: f16::from_bits;
      to: convert_f16_to_u16;
    },
    f16 as Varint {
      from: f16::from_bits;
      to: convert_f16_to_u16;
    },
  },
);

#[inline]
const fn convert_f16_to_u16(v: &f16) -> u16 {
  v.to_bits()
}
