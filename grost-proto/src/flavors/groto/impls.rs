use core::num::NonZeroUsize;

use bufkit::{ChunkMutExt, ChunkWriter, RefPeeker};
// mod map_selector;
pub use map::*;
pub use packed_decoder::*;
pub use repeated_decoder::*;
pub use set::*;
use varing::{encode_u32_varint_to, encoded_u32_varint_len};

use crate::{
  buffer::{Chunk, ChunkExt, ChunkMut, UnknownBuffer},
  decode::Decode,
  flavors::{
    Groto, Repeated, WireFormat,
    groto::{Context, DecodeError, EncodeError, Identifier, Tag},
  },
};

macro_rules! bidi_equivalent {
  ($($($($($lt:lifetime), +$(,)?)? :< $($tg:ident:$t:path $(: $ltb:lifetime)?),+$(,)? >:)? $([$(const $g:ident: $gty:ty), +$(,)?])? impl<$other:ty, $wf:ty> for <$this:ty, $this_wf:ty>),+$(,)?) => {
    bidi_equivalent!(@encode $($($($lt),*)? $(:< $($tg:$t),* >:)? $([$(const $g: $gty),*])? impl<$other, $wf> for <$this, $this_wf>),*);
    bidi_equivalent!(@partial_encode $($($($lt),*)? $(:< $($tg:$t),* >:)? $([$(const $g: $gty),*])? impl<$other, $wf> for <$this, $this_wf>),*);
  };
  (@encode $($($($lt:lifetime), +$(,)?)? $(:< $($tg:ident:$t:path $(: $ltb:lifetime)?),+$(,)? >:)? $([$(const $g:ident: $gty:ty), +$(,)?])? impl<$other:ty, $wf:ty> for <$this:ty, $this_wf:ty>),+$(,)?) => {
    $(
      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentEncode<$other, $wf, $crate::__private::flavors::Groto> for $this
      {

        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentEncode<$this, $this_wf, $crate::__private::flavors::Groto> for $other
      {

        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentEncode<&$other, $wf, $crate::__private::flavors::Groto> for &$this
      {

        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentEncode<&$this, $this_wf, $crate::__private::flavors::Groto> for &$other
      {

        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentEncode<&$other, $wf, $crate::__private::flavors::Groto> for $this
      {

        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentEncode<&$this, $this_wf, $crate::__private::flavors::Groto> for $other
      {

        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentEncode<$other, $wf, $crate::__private::flavors::Groto> for &$this
      {

        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentEncode<$this, $this_wf, $crate::__private::flavors::Groto> for &$other
      {

        type WireFormat = $wf;
      }
    )*
  };
  (@partial_encode $($($($lt:lifetime), +$(,)? )? $(:< $($tg:ident:$t:path $(: $ltb:lifetime)?),+$(,)? >:)? $([$(const $g:ident: $gty:ty), +$(,)?])? impl<$other:ty, $wf:ty> for <$this:ty, $this_wf:ty>),+$(,)?) => {
    $(
      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentPartialEncode<$other, $wf, $crate::__private::flavors::Groto> for $this
      {

        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentPartialEncode<$this, $this_wf, $crate::__private::flavors::Groto> for $other
      {

        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentPartialEncode<&$other, $wf, $crate::__private::flavors::Groto> for &$this
      {

        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentPartialEncode<&$this, $this_wf, $crate::__private::flavors::Groto> for &$other
      {

        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentPartialEncode<&$other, $wf, $crate::__private::flavors::Groto> for $this
      {

        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentPartialEncode<&$this, $this_wf, $crate::__private::flavors::Groto> for $other
      {

        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentPartialEncode<$other, $wf, $crate::__private::flavors::Groto> for &$this
      {

        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: $gty),*)?> $crate::__private::encode::EquivalentPartialEncode<$this, $this_wf, $crate::__private::flavors::Groto> for &$other
      {

        type WireFormat = $wf;
      }
    )*
  };
}

mod list;
mod map;
mod nullable;
mod packed_decoder;
mod repeated_decoder;
mod scalar;
mod set;
mod tuple;

macro_rules! default_wire_format_ref {
  ($($t:ident $(< $($p:ident),+$(,)? >)?),+$(,)?) => {
    $(
      impl<T> $crate::__private::flavors::$t<Groto> for &T
      where
        T: $crate::__private::flavors::$t<Groto> + ?Sized,
      {
        type Format $(< $($p),* >)? = T::Format $(< $($p),* > where $($p: $crate::__private::flavors::WireFormat<Groto> + 'static),*)?;
      }
    )*
  };
}

default_wire_format_ref!(
  DefaultBytesWireFormat,
  DefaultStringWireFormat,
  DefaultScalarWireFormat,
  DefaultObjectWireFormat,
  DefaultEnumWireFormat,
  DefaultUnionWireFormat,
  DefaultNullableWireFormat<V>,
  DefaultFlattenWireFormat<V>,
  DefaultListWireFormat<V>,
  DefaultSetWireFormat<K>,
  DefaultMapWireFormat<K, V>,
);

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

fn packed_encode_raw<'a, B, T: 'a, I, EFL, EF>(
  buf: ChunkWriter<B>,
  iter: I,
  encoded_raw_len: EFL,
  encode: EF,
) -> Result<usize, EncodeError>
where
  B: ChunkMut,
  I: Iterator<Item = &'a T>,
  EFL: Fn() -> usize,
  EF: Fn(&T, ChunkWriter<&'a mut B>) -> Result<usize, EncodeError>,
{
  let encoded_len = encoded_raw_len();
  let buf_len = buf.len();
  if buf_len < encoded_len {
    return Err(EncodeError::buffer_too_small(encoded_len, buf_len));
  }

  let mut offset = 0;

  // encode the elements
  for item in iter {
    let item_encoded_len =
      encode(item, buf.as_mut()).map_err(|e| e.propagate_buffer_info(encoded_len, buf_len))?;
    offset += item_encoded_len;
  }

  Ok(offset)
}

fn packed_encode<'a, C, T: 'a, I, EFL, EF>(
  mut buf: ChunkWriter<C>,
  num_elems: usize,
  iter: I,
  encoded_raw_len: EFL,
  encode: EF,
) -> Result<usize, EncodeError>
where
  C: ChunkMut,
  I: Iterator<Item = &'a T>,
  EFL: Fn() -> usize,
  EF: Fn(&T, ChunkWriter<&mut C>) -> Result<usize, EncodeError>,
{
  let elems_bytes = encoded_raw_len();
  let num_elems_size = encoded_u32_varint_len(num_elems as u32);
  let total_bytes = elems_bytes + num_elems_size;
  let encoded_len = encoded_u32_varint_len(total_bytes as u32) + total_bytes;

  let buf_len = buf.len();
  if buf_len < encoded_len {
    return Err(EncodeError::buffer_too_small(encoded_len, buf_len));
  }

  let mut offset = 0;

  offset += buf
    .write_varint(&(total_bytes as u32))
    .map_err(|e| EncodeError::from(e).propagate_buffer_info(encoded_len, buf_len))?;

  // encode num of elements
  offset += buf
    .write_varint(&(num_elems as u32))
    .map_err(|e| EncodeError::from(e).propagate_buffer_info(encoded_len, buf_len))?;

  // encode the elements
  for item in iter {
    offset +=
      encode(item, buf.as_mut()).map_err(|e| e.propagate_buffer_info(encoded_len, buf_len))?;
  }

  Ok(offset)
}

fn packed_encoded_len<'a, F>(num_elems: usize, encoded_raw_len: F) -> usize
where
  F: Fn() -> usize,
{
  let elems_bytes = encoded_raw_len();
  let num_elems_size = encoded_u32_varint_len(num_elems as u32);
  let total_bytes = elems_bytes + num_elems_size;
  encoded_u32_varint_len(total_bytes as u32) + total_bytes
}

fn packed_decode<'a, K, KW, T, RB>(
  context: &Context,
  mut src: RB,
  constructor: impl FnOnce(usize) -> Result<T, DecodeError>,
  mut len: impl FnMut(&T) -> usize,
  mut merge_decode: impl FnMut(&mut T, RB) -> Result<(usize, RB), DecodeError>,
) -> Result<(usize, T), DecodeError>
where
  RB: Chunk,
  KW: WireFormat<Groto> + 'a,
{
  // decode the total bytes
  let (mut offset, total_bytes) = src.read_varint::<u32>()?;

  // decode the number of elements
  let (num_elements_size, num_elements) = src.read_varint::<u32>()?;
  offset += num_elements_size;
  if num_elements == 0 {
    return Ok((offset, constructor(0)?));
  }

  let mut set = constructor(num_elements as usize)?;
  while offset < total_bytes as usize {
    let (read, remaining_src) = merge_decode(&mut set, src)?;
    offset += read;
    src = remaining_src;
  }

  context.err_length_mismatch(num_elements as usize, len(&set))?;

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

fn repeated_encode<'a, B, K: 'a, KW, I, const TAG: u32>(
  mut buf: ChunkWriter<B>,
  iter: impl Fn() -> I,
  encoded_len: impl Fn(&K) -> usize,
  mut encode: impl FnMut(&K, ChunkWriter<&mut B>) -> Result<usize, EncodeError>,
) -> Result<usize, EncodeError>
where
  I: Iterator<Item = &'a K>,
  KW: WireFormat<Groto>,
  B: ChunkMut,
{
  let identifier = Identifier::new(Repeated::<KW, TAG>::WIRE_TYPE, Tag::try_new(TAG)?);
  let encoded_identifier = identifier.encode();
  let encoded_identifier_len = encoded_identifier.len();
  let encoded_len = iter()
    .map(|k| encoded_identifier_len + encoded_len(k))
    .sum::<usize>();
  let buf_len = buf.remaining_mut();
  if encoded_len > buf_len {
    return Err(EncodeError::buffer_too_small(encoded_len, buf_len));
  }

  let mut offset = 0;
  for k in iter() {
    offset += buf
      .try_write_slice(&encoded_identifier)
      .map_err(|e| EncodeError::from(e).propagate_buffer_info(|| encoded_len, || buf_len))?;

    let k_len =
      encode(k, buf.as_mut()).map_err(|e| e.propagate_buffer_info(|| encoded_len, || buf_len))?;
    offset += k_len;
  }

  Ok(offset)
}

fn repeated_decode<'a, K: 'a, KW, T, RB, B, const TAG: u32>(
  this: &mut T,
  mut src: RB,
  mut merge_decode: impl FnMut(&mut T, RB) -> Result<(usize, RB), DecodeError>,
) -> Result<usize, DecodeError>
where
  RB: Chunk + 'a,
  KW: WireFormat<Groto> + 'a,
  K: Decode<'a, KW, RB, B, Groto>,
  T: Sized + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  let identifier = Identifier::new(Repeated::<KW, TAG>::WIRE_TYPE, Tag::try_new(TAG)?);
  let remaining = src.remaining();
  let mut offset = 0;

  // The following elements should be prefixed with the identifier.
  // | identifier | element | identifier | element | ...
  while src.has_remaining() {
    let (read, next_id) = src.read_varint::<u32>().map_err(|e| {
      DecodeError::from(e)
        .accumulate_requested(|| NonZeroUsize::new(offset))
        .update_available(|| remaining)
    })?;
    let next_id = Identifier::try_from_u32(next_id).map_err(DecodeError::from_parse_tag_error)?;

    // If the next identifier does not match the expected identifier, which means we have reached the end of the repeated elements.
    // We should stop decoding. We do not need to increment the offset here because we are not consuming the next identifier.
    if next_id != identifier {
      break;
    }

    // increment the offset by the size of the identifier
    offset += read;
    let (read, remaining_src) = merge_decode(this, src).map_err(|e| {
      e.accumulate_requested(|| NonZeroUsize::new(offset))
        .update_available(|| remaining)
    })?;

    offset += read;
    src = remaining_src;
  }

  Ok(offset)
}

fn try_from<'a, K, KO, KW, RB, B, I, T>(
  set: &mut T,
  iter: I,
  check: impl FnOnce(&T) -> Result<(), DecodeError>,
  mut insert: impl FnMut(&mut T, K) -> Result<(), DecodeError>,
  mut from_key: impl FnMut(KO) -> Result<K, DecodeError>,
) -> Result<(), DecodeError>
where
  KW: WireFormat<Groto> + 'a,
  K: 'a,
  RB: Chunk + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
  I: Iterator<Item = Result<(usize, KO), DecodeError>>,
{
  for res in iter {
    let (_, k) = res?;
    insert(set, from_key(k)?)?;
  }

  check(set)
}

fn decode_slice<'de, RB>(src: &'de mut RB) -> Result<(usize, &'de [u8]), DecodeError>
where
  RB: Chunk + 'de,
{
  let remaining = src.remaining();
  let (len_size, len) = src.read_varint::<u32>().map_err(DecodeError::from)?;

  let len = len as usize;
  let total = len_size + len;

  if len > src.remaining() {
    return Err(DecodeError::insufficient_data_with_requested(
      remaining, total,
    ));
  }

  Ok((total, src.prefix(len)))
}

fn decode_str<'de, RB>(src: &'de mut RB) -> Result<(usize, &'de str), DecodeError>
where
  RB: Chunk + 'de,
{
  let (total, bytes) = decode_slice(src)?;
  crate::utils::from_utf8(bytes)
    .map_err(|_| DecodeError::other("invalid UTF-8"))
    .map(|s| (total, s))
}
