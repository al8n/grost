use grost_types::{EncodeError, Identifier, WireType};

use crate::Context;

pub fn encode_length_delimiter<V, F, EL>(
  ctx: &Context,
  val: &V,
  buf: &mut [u8],
  encode: F,
  encoded_len: EL,
) -> Result<usize, EncodeError>
where
  V: ?Sized,
  F: FnOnce(&V, &mut [u8]) -> Result<usize, EncodeError>,
  EL: FnOnce(&V) -> usize,
{
  macro_rules! insufficient_buffer {
    ($required:expr, $buf_len:ident) => {{ EncodeError::insufficient_buffer($required, $buf_len) }};
    (@update $e:ident, $required:expr, $buf_len:ident) => {{ $e.update($required, $buf_len) }};
  }

  macro_rules! check_bound {
    ($offset:ident, $required:expr, $buf_len:ident) => {{
      if $offset >= $buf_len {
        return Err(insufficient_buffer!($required, $buf_len));
      }
    }};
  }

  match (ctx.tag(), ctx.length_delimiter()) {
    (Some(tag), _) => {
      let identifier = Identifier::new(WireType::LengthDelimited, tag);
      let buf_len = buf.len();
      let identifier_len = match identifier.encode_to(buf) {
        Ok(write) => write,
        Err(e) => {
          return Err(insufficient_buffer!(@update e, {
            let len = encoded_len(val);
            identifier.encoded_len() + varing::encoded_u32_varint_len(len as u32) + len
          }, buf_len));
        }
      };

      let mut offset = identifier_len;
      let len = encoded_len(val);
      let len_size = varing::encoded_u32_varint_len(len as u32);
      check_bound!(offset, offset + len_size + len, buf_len);

      offset += varing::encode_u32_varint_to(len as u32, &mut buf[offset..])
        .map_err(|_| insufficient_buffer!(offset + len_size + len, buf_len))?;

      check_bound!(offset, offset + len, buf_len);
      offset += encode(val, &mut buf[offset..])
        .map_err(|e| insufficient_buffer!(@update e, offset + len, buf_len))?;

      #[cfg(debug_assertions)]
      crate::debug_assert_write_eq::<V>(offset, identifier_len + len_size + len);

      Ok(offset)
    }
    (None, true) => {
      let len = encoded_len(val);
      let len_size = varing::encoded_u32_varint_len(len as u32);
      let buf_len = buf.len();
      let mut offset = 0;
      offset += varing::encode_u32_varint_to(len as u32, &mut buf[offset..]).map_err(|_| {
        insufficient_buffer!(
          { varing::encoded_u32_varint_len(len as u32) + len },
          buf_len
        )
      })?;

      check_bound!(
        offset,
        varing::encoded_u32_varint_len(len as u32) + len,
        buf_len
      );
      offset += encode(val, &mut buf[offset..])
        .map_err(|e| insufficient_buffer!(@update e, len_size, buf_len))?;
      #[cfg(debug_assertions)]
      crate::debug_assert_write_eq::<V>(offset, len_size + len);

      Ok(offset)
    }
    _ => encode(val, buf),
  }
}

pub fn encoded_length_delimiter_len<V, F>(ctx: &Context, val: &V, f: F) -> usize
where
  V: ?Sized,
  F: FnOnce(&V) -> usize,
{
  match (ctx.tag(), ctx.length_delimiter()) {
    (Some(tag), _) => {
      let identifier = Identifier::new(WireType::LengthDelimited, tag);
      let identifier_len = identifier.encoded_len();
      let len = f(val);
      let len_size = varing::encoded_u32_varint_len(len as u32);
      identifier_len + len_size + len
    }
    (None, true) => {
      let len = f(val);
      let len_size = varing::encoded_u32_varint_len(len as u32);
      len_size + len
    }
    _ => f(val),
  }
}

pub fn decode_length_delimiter<'a, V, F>(
  ctx: &Context,
  src: &'a [u8],
  f: F,
) -> Result<(usize, V), grost_types::DecodeError>
where
  V: Sized,
  F: FnOnce(&'a [u8]) -> Result<(usize, V), grost_types::DecodeError>,
{
  match (ctx.tag(), ctx.length_delimiter()) {
    (Some(tag), _) => {
      let identifier = Identifier::new(WireType::LengthDelimited, tag);
      let (offset, decoded_identifier) = Identifier::decode(src)?;
      if identifier != decoded_identifier {
        return Err(grost_types::DecodeError::identifier_mismatch(
          identifier,
          decoded_identifier,
        ));
      }
      let (offset, len) = varing::decode_u32_varint(src[offset..].as_ref())?;
      let (offset, val) = f(&src[offset..])?;
      Ok((offset + len as usize, val))
    }
    (None, true) => {
      let (offset, len) = varing::decode_u32_varint(src)?;
      let (offset, val) = f(&src[offset..])?;
      Ok((offset + len as usize, val))
    }
    _ => f(src),
  }
}
