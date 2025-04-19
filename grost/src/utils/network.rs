use crate::{
  Wirable,
  flavors::network::{Context, DecodeError, EncodeError, Network, WireType},
};

pub(crate) mod fixed;
pub(crate) mod length_delimited;
pub(crate) mod varint;
pub(crate) mod zst;

pub fn encode<T, E, EL>(
  ctx: &Context,
  val: &T,
  dst: &mut [u8],
  encode_fn: E,
  encoded_len_fn: EL,
) -> Result<usize, EncodeError>
where
  T: Wirable<Network> + ?Sized,
  E: FnOnce(&T, &mut [u8]) -> Result<usize, EncodeError>,
  EL: FnOnce(&T) -> usize,
{
  match T::WIRE_TYPE {
    WireType::Zst => zst::encode_zst(ctx, dst),
    WireType::Varint => varint::encode_varint(ctx, val, dst, encode_fn, encoded_len_fn),
    WireType::LengthDelimited => {
      length_delimited::encode_length_delimiter(ctx, val, dst, encode_fn, encoded_len_fn)
    }
    WireType::Byte => fixed::encode_fixed::<_, _, 1>(ctx, val, dst, encode_fn),
    WireType::Fixed16 => fixed::encode_fixed::<_, _, 2>(ctx, val, dst, encode_fn),
    WireType::Fixed32 => fixed::encode_fixed::<_, _, 4>(ctx, val, dst, encode_fn),
    WireType::Fixed64 => fixed::encode_fixed::<_, _, 8>(ctx, val, dst, encode_fn),
    WireType::Fixed128 => fixed::encode_fixed::<_, _, 16>(ctx, val, dst, encode_fn),
  }
}

pub fn encoded_len<T, F>(ctx: &Context, val: &T, f: F) -> usize
where
  T: Wirable<Network> + ?Sized,
  F: FnOnce(&T) -> usize,
{
  match T::WIRE_TYPE {
    WireType::Zst => zst::encoded_zst_len(ctx),
    WireType::Varint => varint::encoded_varint_len(ctx, val, f),
    WireType::LengthDelimited => length_delimited::encoded_length_delimiter_len(ctx, val, f),
    WireType::Byte => fixed::encoded_fixed_len::<1>(ctx),
    WireType::Fixed16 => fixed::encoded_fixed_len::<2>(ctx),
    WireType::Fixed32 => fixed::encoded_fixed_len::<4>(ctx),
    WireType::Fixed64 => fixed::encoded_fixed_len::<8>(ctx),
    WireType::Fixed128 => fixed::encoded_fixed_len::<16>(ctx),
  }
}

pub fn decode<'a, T, F>(ctx: &Context, src: &'a [u8], f: F) -> Result<(usize, T), DecodeError>
where
  T: Wirable<Network>,
  F: FnOnce(&'a [u8]) -> Result<(usize, T), DecodeError>,
{
  match T::WIRE_TYPE {
    WireType::Zst => zst::decode_zst(ctx, src, || f(&[])),
    WireType::Varint => varint::decode_varint(ctx, src, f),
    WireType::LengthDelimited => length_delimited::decode_length_delimiter(ctx, src, f),
    WireType::Byte => fixed::decode_fixed::<_, _, 1>(ctx, src, f),
    WireType::Fixed16 => fixed::decode_fixed::<_, _, 2>(ctx, src, f),
    WireType::Fixed32 => fixed::decode_fixed::<_, _, 4>(ctx, src, f),
    WireType::Fixed64 => fixed::decode_fixed::<_, _, 8>(ctx, src, f),
    WireType::Fixed128 => fixed::decode_fixed::<_, _, 16>(ctx, src, f),
  }
}
