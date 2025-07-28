pub use decomposable::*;
pub use entry::{MapEntry, PartialMapEntry};
pub use packed_map_decoder::*;
pub use repeated_map_decoder::*;

use varing::decode_u32_varint;

use crate::{
  buffer::{ReadBuf, UnknownBuffer, WriteBuf},
  flavors::{
    Groto, RepeatedEntry, WireFormat,
    groto::{Context, Error, Identifier, Tag},
  },
};

#[cfg(any(feature = "std", feature = "alloc"))]
mod btreemap;
#[cfg(any(feature = "std", feature = "hashbrown_0_15"))]
mod hashmap;
#[cfg(feature = "indexmap_2")]
mod indexmap;

mod decomposable;
mod entry;
mod packed_map_decoder;
mod repeated_map_decoder;

fn packed_encoded_raw_len<'a, K, V, KW, VW, I, Item>(
  iter: I,
  encoded_len: impl Fn(Item, &Identifier, &Identifier) -> usize,
) -> usize
where
  K: 'a,
  V: 'a,
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  I: Iterator<Item = Item>,
{
  let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
  let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

  let mut len = 0;
  for item in iter {
    len += encoded_len(item, &ki, &vi);
  }

  len
}

fn packed_encode_raw<'a, K, V, KW, VW, I, Item, EFL, EF>(
  buf: &mut [u8],
  iter: I,
  encoded_raw_len: EFL,
  mut encode: EF,
) -> Result<usize, Error>
where
  K: 'a,
  V: 'a,
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  I: Iterator<Item = Item>,
  EFL: Fn() -> usize,
  EF: FnMut(Item, &Identifier, &Identifier, &mut [u8]) -> Result<usize, Error>,
{
  let encoded_len = encoded_raw_len();
  let buf_len = buf.len();
  if buf_len < encoded_len {
    return Err(Error::insufficient_buffer(encoded_len, buf_len));
  }

  let mut offset = 0;

  let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
  let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

  // encode the elements
  for item in iter {
    if offset >= buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    offset += encode(item, &ki, &vi, &mut buf[offset..])?;
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

fn packed_encode<'a, K: 'a, V: 'a, KW, VW, I, Item, EFL, EF>(
  buf: &mut [u8],
  num_elems: usize,
  iter: I,
  encoded_raw_len: EFL,
  encode: EF,
) -> Result<usize, Error>
where
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  I: Iterator<Item = Item>,
  EFL: Fn() -> usize,
  EF: Fn(Item, &Identifier, &Identifier, &mut [u8]) -> Result<usize, Error>,
{
  let encoded_raw_len = encoded_raw_len();
  let encoded_len = varing::encoded_u32_varint_len(encoded_raw_len as u32) + encoded_raw_len;

  let buf_len = buf.len();
  if buf_len < encoded_len {
    return Err(Error::insufficient_buffer(encoded_len, buf_len));
  }

  let mut offset = 0;

  // encode total bytes
  if encoded_len > u32::MAX as usize {
    return Err(Error::too_large(encoded_len, u32::MAX as usize));
  }

  let total_bytes = encoded_raw_len as u32;
  let total_bytes_size = varing::encode_u32_varint_to(total_bytes, buf)?;
  offset += total_bytes_size;

  // encode num of elements
  let num_elems_size = varing::encode_u32_varint_to(num_elems as u32, buf)?;
  offset += num_elems_size;

  let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
  let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

  // encode the elements
  for item in iter {
    if offset >= buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    offset += encode(item, &ki, &vi, &mut buf[offset..])?;
  }

  Ok(offset)
}

fn packed_decode<'a, K, KW, V, VW, T, RB>(
  context: &Context,
  src: RB,
  constructor: impl FnOnce(usize) -> Result<T, Error>,
  mut len: impl FnMut(&T) -> usize,
  mut merge_decode: impl FnMut(&mut T, &Identifier, &Identifier, RB) -> Result<usize, Error>,
) -> Result<(usize, T), Error>
where
  RB: ReadBuf,
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
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

  let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
  let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

  let mut map = constructor(num_elements as usize)?;
  while offset < bytes_len {
    offset += merge_decode(&mut map, &ki, &vi, src.slice(offset..))?;
  }

  let actual_num_elements = len(&map);
  if actual_num_elements != num_elements as usize && context.err_on_length_mismatch() {
    return Err(Error::custom(format!(
      "expected {num_elements} elements in map, but got {actual_num_elements} elements",
    )));
  }

  Ok((offset, map))
}

fn repeated_encoded_len<KW, VW, I, Item, const TAG: u32>(
  iter: I,
  encoded_len: impl Fn(Item, &Identifier, &Identifier, &Identifier) -> usize,
) -> usize
where
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  I: Iterator<Item = Item>,
{
  let ei = Identifier::new(RepeatedEntry::<KW, VW, TAG>::WIRE_TYPE, Tag::new(TAG));
  let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
  let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

  iter.map(|item| encoded_len(item, &ei, &ki, &vi)).sum()
}

fn repeated_encode<KW, VW, I, Item, const TAG: u32>(
  buf: &mut [u8],
  iter: I,
  encoded_raw_len: impl Fn() -> usize,
  mut encode: impl FnMut(Item, &Identifier, &Identifier, &Identifier, &mut [u8]) -> Result<usize, Error>,
) -> Result<usize, Error>
where
  KW: WireFormat<Groto>,
  VW: WireFormat<Groto>,
  I: Iterator<Item = Item>,
{
  let encoded_len = encoded_raw_len();
  let buf_len = buf.len();
  if buf_len < encoded_len {
    return Err(Error::insufficient_buffer(encoded_len, buf_len));
  }

  let ei = Identifier::new(RepeatedEntry::<KW, VW, TAG>::WIRE_TYPE, Tag::try_new(TAG)?);
  let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
  let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

  let mut offset = 0;
  for item in iter {
    if offset >= buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    offset += encode(item, &ei, &ki, &vi, &mut buf[offset..])?;
  }

  Ok(offset)
}

fn repeated_decode<'a, KW, VW, T, RB, const TAG: u32>(
  src: RB,
  mut merge_decode: impl FnMut(
    &Identifier,
    &Identifier,
    &Identifier,
    RB,
  ) -> Result<Option<usize>, Error>,
) -> Result<usize, Error>
where
  RB: ReadBuf,
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
{
  let ei = Identifier::new(RepeatedEntry::<KW, VW, TAG>::WIRE_TYPE, Tag::try_new(TAG)?);
  let ki = Identifier::new(KW::WIRE_TYPE, Tag::MAP_KEY);
  let vi = Identifier::new(VW::WIRE_TYPE, Tag::MAP_VALUE);

  let mut offset = 0;
  let buf_len = src.len();

  while offset < buf_len {
    match merge_decode(&ei, &ki, &vi, src.slice(offset..))? {
      Some(len) => {
        offset += len;
      }
      None => {
        // If merge_decode returns None, it means the next item is not the same type as the expected ones,
        break;
      }
    }
  }

  Ok(offset)
}

fn try_from<'a, K, V, KO, VO, KW, VW, RB, B, I, T>(
  map: &mut T,
  iter: I,
  check: impl FnOnce(&T) -> Result<(), Error>,
  mut insert: impl FnMut(&mut T, K, V) -> Result<(), Error>,
  mut from_key: impl FnMut(KO) -> Result<K, Error>,
  mut from_value: impl FnMut(VO) -> Result<V, Error>,
) -> Result<(), Error>
where
  KW: WireFormat<Groto> + 'a,
  VW: WireFormat<Groto> + 'a,
  K: 'a,
  V: 'a,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
  I: Iterator<Item = Result<(usize, PartialDecomposableMapEntry<KO, VO>), Error>>,
{
  for res in iter {
    let (_, item) = res?;
    let (k, v) = item
      .and_then(|k| from_key(k), |v| from_value(v))?
      .try_into_entry()?
      .into();
    insert(map, k, v)?;
  }

  check(map)
}
