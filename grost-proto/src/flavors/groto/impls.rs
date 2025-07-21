// mod map_selector;
pub use map::*;
pub use packed_decoder::*;
pub use repeated_decoder::*;
pub use set::*;
use varing::{decode_u32_varint, encode_u32_varint_to, encoded_u32_varint_len};

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  convert::{PartialTransform, Transform},
  decode::Decode1,
  flavors::{
    Groto, Repeated, WireFormat,
    groto::{Context, Error, Identifier, Tag},
  },
  selection::Selectable,
};

macro_rules! identity_partial_transform {
  ($flavor:ty {
    $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $wf:ty), +$(,)?
  }) => {
    $(
      impl $(< $(const $g: ::core::primitive::usize),* > )? $crate::__private::convert::PartialTransform<$ty, ::core::option::Option<$ty>, $wf, $flavor> for $ty {
        fn partial_transform(input: Self, selector: &bool) -> ::core::result::Result<::core::option::Option<$ty>, <$flavor as $crate::__private::flavors::Flavor>::Error>
        {
          if $crate::__private::selection::Selector::<$flavor>::is_empty(selector) {
            return ::core::result::Result::Ok(::core::option::Option::None);
          }

          ::core::result::Result::Ok(::core::option::Option::Some(input))
        }
      }

      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::convert::PartialTransform<::core::option::Option<Self>, ::core::option::Option<Self>, $crate::__private::flavors::Nullable<$wf>, $flavor,> for $ty {
        fn partial_transform(input: ::core::option::Option<Self>, selector: &<Self as $crate::__private::selection::Selectable<$flavor>>::Selector) -> ::core::result::Result<::core::option::Option<Self>, <$flavor as $crate::__private::flavors::Flavor>::Error>
        {
          match input {
            ::core::option::Option::None => ::core::result::Result::Ok(::core::option::Option::None),
            ::core::option::Option::Some(input) => {
              if $crate::__private::selection::Selector::<$flavor>::is_empty(selector) {
                return ::core::result::Result::Ok(::core::option::Option::None);
              }

              ::core::result::Result::Ok(::core::option::Option::Some(input))
            }
          }
        }
      }
    )*
  };
}

macro_rules! bidi_equivalent {
  ($($($($($lt:lifetime), +$(,)?)? :< $($tg:ident:$t:path $(: $ltb:lifetime)?),+$(,)? >:)? $([$(const $g:ident: usize), +$(,)?])? impl<$other:ty, $wf:ty> for <$this:ty, $this_wf:ty>),+$(,)?) => {
    bidi_equivalent!(@encode $($($($lt),*)? $(:< $($tg:$t),* >:)? $([$(const $g: usize),*])? impl<$other, $wf> for <$this, $this_wf>),*);
    bidi_equivalent!(@partial_encode $($($($lt),*)? $(:< $($tg:$t),* >:)? $([$(const $g: usize),*])? impl<$other, $wf> for <$this, $this_wf>),*);
  };
  (@encode $($($($lt:lifetime), +$(,)?)? $(:< $($tg:ident:$t:path $(: $ltb:lifetime)?),+$(,)? >:)? $([$(const $g:ident: usize), +$(,)?])? impl<$other:ty, $wf:ty> for <$this:ty, $this_wf:ty>),+$(,)?) => {
    $(
      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<$other, $wf, $crate::__private::flavors::Groto> for $this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<$this, $this_wf, $crate::__private::flavors::Groto> for $other
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<&$other, $wf, $crate::__private::flavors::Groto> for &$this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<&$this, $this_wf, $crate::__private::flavors::Groto> for &$other
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<&$other, $wf, $crate::__private::flavors::Groto> for $this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<&$this, $this_wf, $crate::__private::flavors::Groto> for $other
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<$other, $wf, $crate::__private::flavors::Groto> for &$this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<$this, $this_wf, $crate::__private::flavors::Groto> for &$other
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $wf;
      }
    )*
  };
  (@partial_encode $($($($lt:lifetime), +$(,)? )? $(:< $($tg:ident:$t:path $(: $ltb:lifetime)?),+$(,)? >:)? $([$(const $g:ident: usize), +$(,)?])? impl<$other:ty, $wf:ty> for <$this:ty, $this_wf:ty>),+$(,)?) => {
    $(
      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<$other, $wf, $crate::__private::flavors::Groto> for $this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<$this, $this_wf, $crate::__private::flavors::Groto> for $other
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<&$other, $wf, $crate::__private::flavors::Groto> for &$this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<&$this, $this_wf, $crate::__private::flavors::Groto> for &$other
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<&$other, $wf, $crate::__private::flavors::Groto> for $this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<&$this, $this_wf, $crate::__private::flavors::Groto> for $other
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<$other, $wf, $crate::__private::flavors::Groto> for &$this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<$this, $this_wf, $crate::__private::flavors::Groto> for &$other
      {
        type Flavor = $crate::__private::flavors::Groto;
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

pub trait GrotoTransform<I, O, W>: Transform<I, O, W, Groto>
where
  W: WireFormat<Groto>,
{
}

impl<I, O, W, T> GrotoTransform<I, O, W> for T
where
  W: WireFormat<Groto>,
  T: Transform<I, O, W, Groto>,
{
}

pub trait GrotoPartialTransform<I, O, W>: PartialTransform<I, O, W, Groto>
where
  W: WireFormat<Groto>,
  I: Selectable<Groto, Selector = Self::Selector>,
  O: Selectable<Groto, Selector = Self::Selector>,
  Self: Selectable<Groto>,
{
}

impl<I, O, W, T> GrotoPartialTransform<I, O, W> for T
where
  W: WireFormat<Groto>,
  T: PartialTransform<I, O, W, Groto>,
  I: Selectable<Groto, Selector = T::Selector>,
  O: Selectable<Groto, Selector = T::Selector>,
  T: Selectable<Groto>,
{
}

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

fn packed_encode_raw<'a, T: 'a, I, EFL, EF>(
  buf: &mut [u8],
  iter: I,
  encoded_raw_len: EFL,
  encode: EF,
) -> Result<usize, Error>
where
  I: Iterator<Item = &'a T>,
  EFL: Fn() -> usize,
  EF: Fn(&T, &mut [u8]) -> Result<usize, Error>,
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

fn packed_encode<'a, T: 'a, I, EFL, EF>(
  buf: &mut [u8],
  num_elems: usize,
  iter: I,
  encoded_raw_len: EFL,
  encode: EF,
) -> Result<usize, Error>
where
  I: Iterator<Item = &'a T>,
  EFL: Fn() -> usize,
  EF: Fn(&T, &mut [u8]) -> Result<usize, Error>,
{
  let elems_bytes = encoded_raw_len();
  let num_elems_size = encoded_u32_varint_len(num_elems as u32);
  let total_bytes = elems_bytes + num_elems_size;
  let encoded_len = encoded_u32_varint_len(total_bytes as u32) + total_bytes;

  let buf_len = buf.len();
  if buf_len < encoded_len {
    return Err(Error::insufficient_buffer(encoded_len, buf_len));
  }

  let mut offset = 0;

  // encode total bytes
  if encoded_len > u32::MAX as usize {
    return Err(Error::too_large(encoded_len, u32::MAX as usize));
  }

  offset += encode_u32_varint_to(total_bytes as u32, buf)?;

  if offset + total_bytes > buf_len {
    return Err(Error::insufficient_buffer(encoded_len, buf_len));
  }

  // encode num of elements
  offset += encode_u32_varint_to(num_elems as u32, &mut buf[offset..])?;

  // encode the elements
  for item in iter {
    if offset >= buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    offset += encode(item, &mut buf[offset..])?;
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
