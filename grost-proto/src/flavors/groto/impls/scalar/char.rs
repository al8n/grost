use crate::{
  default_scalar_wire_format, flatten_state,
  flavors::groto::{Error, Fixed32, Groto, Varint},
  partial_ref_state, partial_state, ref_state, selectable, try_from_bridge,
};

default_scalar_wire_format!(Groto: char as Fixed32);
selectable!(@scalar Groto:char);
ref_state!(@scalar &'a Groto: char as Fixed32, char as Varint);
partial_ref_state!(@scalar &'a Groto: char as Fixed32, char as Varint);
partial_state!(@scalar Groto: char);
flatten_state!(char);

try_from_bridge!(
  Groto: u32 {
    char as Fixed32 {
      try_from: convert_u32_to_char;
      to: convert_char_to_u32;
    },
    char as Varint {
      try_from: convert_u32_to_char;
      to: convert_char_to_u32;
    },
  },
);

#[inline]
const fn convert_char_to_u32(v: &char) -> u32 {
  *v as u32
}

#[inline]
fn convert_u32_to_char(v: u32) -> Result<char, Error> {
  match char::from_u32(v) {
    Some(c) => Ok(c),
    None => Err(Error::custom("invalid char value")),
  }
}
