pub use buffer::{DefaultPartialSetBuffer, PartialSetBuffer};
pub use packed_set_decoder::PackedSetDecoder;
use varing::decode_u32_varint;

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  decode::Decode1,
  flavors::{
    Groto, Repeated, WireFormat,
    groto::{Context, Error, Identifier, Tag},
  },
};

#[cfg(any(feature = "std", feature = "alloc"))]
mod btreeset;
#[cfg(any(feature = "std", feature = "hashbrown_0_15"))]
mod hashset;
#[cfg(feature = "indexmap_2")]
mod indexset;

mod buffer;
mod packed_set_decoder;

fn packed_encoded_raw_len<'a, K: 'a, KW, I, IEFL>(
  num_elems: usize,
  iter: I,
  item_encoded_len: IEFL,
) -> usize
where
  KW: WireFormat<Groto>,
  I: Iterator<Item = &'a K>,
  IEFL: Fn(&K) -> usize,
{
  let mut len = 0;
  if let Some(fixed_length) = KW::FIXED_LENGTH {
    len += fixed_length * num_elems;
  } else {
    for item in iter {
      len += item_encoded_len(item);
    }
  }
  len
}

fn packed_encode_raw<'a, K: 'a, I, EFL, EF>(
  buf: &mut [u8],
  iter: I,
  encoded_raw_len: EFL,
  encode: EF,
) -> Result<usize, Error>
where
  I: Iterator<Item = &'a K>,
  EFL: Fn() -> usize,
  EF: Fn(&K, &mut [u8]) -> Result<usize, Error>,
{
  let encoded_len = encoded_raw_len();
  let buf_len = buf.len();
  if buf_len < encoded_len {
    return Err(Error::insufficient_buffer(encoded_len, buf_len));
  }

  let mut offset = 0;

  // encode the elements
  for item in iter {
    if offset >= buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }
    let item_encoded_len = encode(item, &mut buf[offset..])?;
    offset += item_encoded_len;
  }

  Ok(offset)
}

fn packed_encode<'a, K: 'a, I, EFL, EF>(
  buf: &mut [u8],
  num_elems: usize,
  iter: I,
  encoded_raw_len: EFL,
  encode: EF,
) -> Result<usize, Error>
where
  I: Iterator<Item = &'a K>,
  EFL: Fn() -> usize,
  EF: Fn(&K, &mut [u8]) -> Result<usize, Error>,
{
  let elems_bytes = encoded_raw_len();
  let num_elems_size = varing::encoded_u32_varint_len(num_elems as u32);
  let total_bytes = elems_bytes + num_elems_size;
  let encoded_len = varing::encoded_u32_varint_len(total_bytes as u32) + total_bytes;

  let buf_len = buf.len();
  if buf_len < encoded_len {
    return Err(Error::insufficient_buffer(encoded_len, buf_len));
  }

  let mut offset = 0;

  // encode total bytes
  if encoded_len > u32::MAX as usize {
    return Err(Error::too_large(encoded_len, u32::MAX as usize));
  }

  let total_bytes_size = varing::encode_u32_varint_to(total_bytes as u32, buf)?;
  offset += total_bytes_size;

  if offset + total_bytes > buf_len {
    return Err(Error::insufficient_buffer(encoded_len, buf_len));
  }

  // encode num of elements
  let num_elems_size = varing::encode_u32_varint_to(num_elems as u32, &mut buf[offset..])?;
  offset += num_elems_size;

  // encode the elements
  for item in iter {
    if offset >= buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let item_encoded_len = encode(item, &mut buf[offset..])?;
    offset += item_encoded_len;
  }

  Ok(offset)
}

fn packed_encoded_len<'a, F>(num_elems: usize, encoded_raw_len: F) -> usize
where
  F: Fn() -> usize,
{
  let elems_bytes = encoded_raw_len();
  let num_elems_size = varing::encoded_u32_varint_len(num_elems as u32);
  let total_bytes = elems_bytes + num_elems_size;
  varing::encoded_u32_varint_len(total_bytes as u32) + total_bytes
}

fn packed_decode<'a, K, KW, T, RB>(
  context: &Context,
  src: RB,
  constructor: impl FnOnce(usize) -> Result<T, Error>,
  mut len: impl FnMut(&T) -> usize,
  mut merge_decode: impl FnMut(&mut T, RB) -> Result<usize, Error>,
) -> Result<(usize, T), Error>
where
  RB: ReadBuf,
  KW: WireFormat<Groto> + 'a,
{
  let bytes = src.as_bytes();
  let bytes_len = bytes.len();
  if bytes_len == 0 {
    return Err(Error::buffer_underflow());
  }

  // decode total bytes
  let (mut offset, total_bytes) = decode_u32_varint(bytes)?;

  if bytes_len < offset + total_bytes as usize {
    return Err(Error::buffer_underflow());
  }

  // decode the number of elements
  let (num_elements_size, num_elements) = decode_u32_varint(&bytes[offset..])?;
  offset += num_elements_size;
  if num_elements == 0 {
    return Ok((offset, constructor(0)?));
  }

  let mut set = constructor(num_elements as usize)?;
  while offset < bytes_len {
    offset += merge_decode(&mut set, src.slice(offset..))?;
  }

  let actual_num_elements = len(&set);
  if actual_num_elements != num_elements as usize && context.err_on_length_mismatch() {
    return Err(Error::custom(format!(
      "expected {num_elements} elements in set, but got {actual_num_elements} elements",
    )));
  }

  Ok((offset, set))
}

fn repeated_encoded_len<'a, K: 'a, KW, I, const TAG: u32>(
  iter: I,
  encoded_len: impl Fn(&K) -> usize,
) -> usize
where
  KW: WireFormat<Groto>,
  I: Iterator<Item = &'a K>,
{
  let identifier = Identifier::new(Repeated::<KW, TAG>::WIRE_TYPE, Tag::new(TAG));
  let encoded_identifier_len = identifier.encoded_len();
  iter.map(|k| encoded_identifier_len + encoded_len(k)).sum()
}

fn repeated_encode<'a, K: 'a, KW, I, const TAG: u32>(
  buf: &'a mut [u8],
  iter: impl Fn() -> I,
  encoded_len: impl Fn(&K) -> usize,
  mut encode: impl FnMut(&K, &mut [u8]) -> Result<usize, Error>,
) -> Result<usize, Error>
where
  I: Iterator<Item = &'a K>,
  KW: WireFormat<Groto>,
{
  let identifier = Identifier::new(Repeated::<KW, TAG>::WIRE_TYPE, Tag::try_new(TAG)?);
  let encoded_identifier = identifier.encode();
  let encoded_identifier_len = encoded_identifier.len();
  let encoded_len = iter()
    .map(|k| encoded_identifier_len + encoded_len(k))
    .sum::<usize>();
  let buf_len = buf.len();
  if encoded_len > buf_len {
    return Err(Error::insufficient_buffer(encoded_len, buf_len));
  }

  let mut offset = 0;
  for k in iter() {
    if offset + encoded_identifier_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    buf[offset..offset + encoded_identifier_len].copy_from_slice(&encoded_identifier);
    offset += encoded_identifier_len;

    if offset >= buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let k_len = encode(k, &mut buf[offset..]).map_err(|e| e.update(encoded_len, buf_len))?;
    offset += k_len;
  }

  Ok(offset)
}

fn repeated_decode<'a, K: 'a, KW, T, RB, B, const TAG: u32>(
  this: &mut T,
  src: RB,
  mut merge_decode: impl FnMut(&mut T, RB) -> Result<usize, Error>,
) -> Result<usize, Error>
where
  RB: ReadBuf + 'a,
  KW: WireFormat<Groto> + 'a,
  K: Decode1<'a, KW, RB, B, Groto>,
  T: Sized + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  let identifier = Identifier::new(Repeated::<KW, TAG>::WIRE_TYPE, Tag::try_new(TAG)?);
  let mut offset = 0;
  let buf = src.as_bytes();
  let buf_len = buf.len();
  if buf_len == 0 {
    return Err(Error::buffer_underflow());
  }

  // The following elements should be prefixed with the identifier.
  // | identifier | element | identifier | element | ...
  loop {
    if offset >= buf_len {
      break;
    }

    let (read, next_id) = Identifier::decode(&buf[offset..])?;

    // If the next identifier does not match the expected identifier, which means we have reached the end of the repeated elements.
    // We should stop decoding. We do not need to increment the offset here because we are not consuming the next identifier.
    if next_id != identifier {
      break;
    }

    // increment the offset by the size of the identifier
    offset += read;

    if offset >= buf_len {
      return Err(Error::buffer_underflow());
    }

    offset += merge_decode(this, src.slice(offset..))?;
  }

  Ok(offset)
}

fn try_from<'a, K, KO, KW, RB, B, I, T>(
  set: &mut T,
  iter: I,
  check: impl FnOnce(&T) -> Result<(), Error>,
  mut insert: impl FnMut(&mut T, K) -> Result<(), Error>,
  mut from_key: impl FnMut(KO) -> Result<K, Error>,
) -> Result<(), Error>
where
  KW: WireFormat<Groto> + 'a,
  K: 'a,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
  I: Iterator<Item = Result<(usize, KO), Error>>,
{
  for res in iter {
    let (_, k) = res?;
    insert(set, from_key(k)?)?;
  }

  check(set)
}

#[test]
fn t() {}
