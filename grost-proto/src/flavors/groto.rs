use bufkit::RefPeeker;
pub use context::Context;
pub use identifier::Identifier;
pub use impls::*;
pub use tag::Tag;
pub use wire_type::*;

/// The encode error for the Groto flavor.
pub type EncodeError = crate::error::EncodeError<Groto>;
/// The decode error for the Groto flavor.
pub type DecodeError = crate::error::DecodeError<Groto>;

use super::Flavor;

use crate::buffer::Chunk;

mod context;
mod identifier;
mod impls;
mod tag;
mod wire_type;

/// The groto flavor
#[derive(
  Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display,
)]
#[display("Groto")]
pub struct Groto;

impl Flavor for Groto {
  type Context = Context;
  type Identifier = Identifier;
  type WireType = WireType;
  type Tag = Tag;

  const NAME: &'static str = "Groto";

  fn peek_raw<B>(
    _: &Self::Context,
    wire_type: Self::WireType,
    buf: &B,
  ) -> Result<usize, DecodeError>
  where
    B: Chunk,
  {
    skip_helper(wire_type, buf.buffer())
  }
}

fn skip_helper<B>(wire_type: WireType, buf: &B) -> Result<usize, DecodeError>
where
  B: Chunk,
{
  let mut peeker = RefPeeker::new(buf);
  let buf_len = buf.len();

  macro_rules! try_skip_fixed {
    ($bytes:literal) => {{
      if buf_len < $bytes {
        return Err(DecodeError::insufficient_data_with_requested(
          buf_len, $bytes,
        ));
      }

      $bytes
    }};
  }

  Ok(match wire_type {
    WireType::Varint => varing::consume_varint(buf.buffer())?,
    WireType::LengthDelimited => {
      let (size_len, size) = varing::decode_u32_varint(buf.buffer())?;
      let size = size as usize;
      let total = size_len + size;
      if size > buf.len() {
        return Err(DecodeError::insufficient_data_with_requested(
          buf_len, total,
        ));
      }
      total
    }
    WireType::Nullable => {
      if buf_len <= 1 {
        return Err(DecodeError::insufficient_data(buf_len));
      }
      let marker = buf[0];
      if marker == 0 {
        // This is a zero byte indicating absence, so we skip it
        return Ok(1);
      }

      if buf_len <= 2 {
        return Err(DecodeError::insufficient_data(buf_len));
      }
      let wire_type = WireType::try_from_u8(buf[1]).map_err(|_| unknown_wire_type_value(buf[1]))?;
      if wire_type.is_nullable() {
        return Err(DecodeError::other("unexpected nested nullable wire type"));
      }
      return skip_helper(wire_type, &buf[2..]).map(|len| len + 2);
    }
    WireType::Fixed8 => try_skip_fixed!(1),
    WireType::Fixed16 => try_skip_fixed!(2),
    WireType::Fixed32 => try_skip_fixed!(4),
    WireType::Fixed64 => try_skip_fixed!(8),
    WireType::Fixed128 => try_skip_fixed!(16),
  })
}

#[cfg(any(feature = "std", feature = "alloc"))]
fn unknown_wire_type_value(wire_type: u8) -> DecodeError {
  DecodeError::other(std::format!("unknown wire type value {wire_type}"))
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
fn unknown_wire_type_value(_: u8) -> DecodeError {
  DecodeError::other("unknown wire type value")
}
