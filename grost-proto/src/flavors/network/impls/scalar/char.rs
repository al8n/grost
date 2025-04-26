use crate::{
  flavors::network::{DecodeError, Network},
  try_from_bridge,
};

try_from_bridge!(
  Network: u32 {
    char {
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
fn convert_u32_to_char(v: u32) -> Result<char, DecodeError> {
  match char::from_u32(v) {
    Some(c) => Ok(c),
    None => Err(DecodeError::custom("invalid char value")),
  }
}
