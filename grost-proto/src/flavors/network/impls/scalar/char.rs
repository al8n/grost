use crate::{
  decoded_state, default_wire_format, flatten_state,
  flavors::network::{Error, Fixed32, Network, Varint},
  identity_partial_transform, identity_transform, selectable, try_from_bridge,
};

default_wire_format!(Network: char as Fixed32);
selectable!(@scalar Network:char);
decoded_state!(@scalar &'a Network: char as Fixed32, char as Varint);
flatten_state!(char);
identity_transform!(
  Network {
    char as Fixed32,
    char as Varint,
  }
);
identity_partial_transform!(
  Network {
    char as Fixed32,
    char as Varint,
  }
);

try_from_bridge!(
  Network: u32 {
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
