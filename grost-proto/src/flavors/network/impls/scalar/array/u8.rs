
use crate::{
  IntoTarget, Message, PartialMessage, TypeOwned, TypeRef,
  decode::{Decode, DecodeOwned},
  encode::Encode,
  flavors::{
    Network,
    network::{Context, DecodeError, EncodeError, Unknown, WireType},
  },
};

use super::larger_than_array_capacity;

macro_rules! encode_fixed {
  ($this:ident($buf:ident) as $fixed:literal) => {{
    let buf_len = $buf.len();
    if buf_len < $fixed {
      return Err(EncodeError::insufficient_buffer($fixed, buf_len));
    }
    $buf[..$fixed].copy_from_slice($this.as_slice());
    $fixed
  }};
}

impl<const N: usize> Encode<Network> for [u8; N] {
  fn encode(
    &self,
    _: &Context,
    wire_type: WireType,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    Ok(match wire_type {
      WireType::Zst if N == 0 => 0,
      WireType::Byte if N == 1 => {
        if buf.is_empty() {
          return Err(EncodeError::insufficient_buffer(1, 0));
        }
        buf[0] = self[0];
        1
      },
      WireType::Fixed16 if N == 2 => encode_fixed!(self(buf) as 2),
      WireType::Fixed32 if N == 4 => encode_fixed!(self(buf) as 4),
      WireType::Fixed64 if N == 8 => encode_fixed!(self(buf) as 8),
      WireType::Fixed128 if N == 16 => encode_fixed!(self(buf) as 16),
      WireType::LengthDelimited => {
        let this_len = self.len();
        let buf_len = buf.len();
        if buf_len < this_len {
          return Err(EncodeError::insufficient_buffer(this_len, buf_len));
        }

        buf[..this_len].copy_from_slice(self.as_slice());
        this_len
      },
      val => return Err(EncodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        val,
      )),
    })
  }

  fn encoded_len(&self, _: &Context, wire_type: WireType) -> Result<usize, EncodeError> {
    Ok(match wire_type {
      WireType::Zst if N == 0 => 0,
      WireType::Byte if N == 1 => 1,
      WireType::Fixed16 if N == 2 => 2,
      WireType::Fixed32 if N == 4 => 4,
      WireType::Fixed64 if N == 8 => 8,
      WireType::Fixed128 if N == 16 => 16,
      WireType::LengthDelimited => self.len(),
      val => return Err(EncodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        val,
      )),
    })
  }

  fn encoded_length_delimited_len(
    &self,
    _: &Context,
    wire_type: WireType,
  ) -> Result<usize, EncodeError> {
    Ok(match wire_type {
      WireType::Zst if N == 0 => 0,
      WireType::Byte if N == 1 => 1,
      WireType::Fixed16 if N == 2 => 2,
      WireType::Fixed32 if N == 4 => 4,
      WireType::Fixed64 if N == 8 => 8,
      WireType::Fixed128 if N == 16 => 16,
      WireType::LengthDelimited => {
        let this_len = self.len();
        let len_size = varing::encoded_u32_varint_len(this_len as u32);
        len_size + this_len
      },
      val => return Err(EncodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        val,
      )),
    })
  }

  fn encode_length_delimited(
    &self,
    _: &Context,
    wire_type: WireType,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    Ok(match wire_type {
      WireType::Zst if N == 0 => 0,
      WireType::Fixed16 if N == 2 => encode_fixed!(self(buf) as 2),
      WireType::Fixed32 if N == 4 => encode_fixed!(self(buf) as 4),
      WireType::Fixed64 if N == 8 => encode_fixed!(self(buf) as 8),
      WireType::Fixed128 if N == 16 => encode_fixed!(self(buf) as 16),
      WireType::LengthDelimited => {
        let this_len = self.len();
        let mut offset = varing::encode_u32_varint_to(this_len as u32, buf)?;
        let buf_len = buf.len();
        if buf_len < offset + this_len {
          return Err(EncodeError::insufficient_buffer(
            this_len + offset,
            buf_len,
          ));
        }

        buf[offset..offset + this_len].copy_from_slice(self.as_slice());
        offset += this_len;
        offset
      },
      _ => return Err(EncodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wire_type,
      )),
    })
  }
}

macro_rules! decode_fixed {
  ($src:ident) => {{
    if $src.len() < N {
      return Err(DecodeError::buffer_underflow());
    }
    (N, $src[..N].try_into().unwrap())
  }};
}

impl<'de, const N: usize> Decode<'de, Network, Self> for [u8; N] {
  fn decode<UB>(
    _: &Context,
    wire_type: WireType,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
    Ok(match wire_type {
      WireType::Zst if N == 0 => (0, [0u8; N]),
      WireType::Byte if N == 1 => {
        if src.is_empty() {
          return Err(DecodeError::buffer_underflow());
        }
        let this_len = 1;
        let mut buf = [0u8; N];
        buf[0] = src[0];
        (this_len, buf)
      },
      WireType::Fixed16 if N == 2 => decode_fixed!(src),
      WireType::Fixed32 if N == 4 => decode_fixed!(src),
      WireType::Fixed64 if N == 8 => decode_fixed!(src),
      WireType::Fixed128 if N == 16 => decode_fixed!(src),
      WireType::LengthDelimited => return decode_to_array::<N>(src),
      wt => return Err(DecodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wt,
      )),
    })
  }

  fn decode_length_delimited<UB>(
    _: &Context,
    wire_type: WireType,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
    Ok(match wire_type {
      WireType::Zst if N == 0 => (0, [0u8; N]),
      WireType::Byte if N == 1 => {
        if src.is_empty() {
          return Err(DecodeError::buffer_underflow());
        }
        let this_len = 1;
        let mut buf = [0u8; N];
        buf[0] = src[0];
        (this_len, buf)
      },
      WireType::Fixed16 if N == 2 => decode_fixed!(src),
      WireType::Fixed32 if N == 4 => decode_fixed!(src),
      WireType::Fixed64 if N == 8 => decode_fixed!(src),
      WireType::Fixed128 if N == 16 => decode_fixed!(src),
      WireType::LengthDelimited => return decode_length_delimited_to_array::<N>(src),
      _ => return Err(DecodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wire_type,
      )),
    })
  }
}

impl<const N: usize> DecodeOwned<Network, Self> for [u8; N] {
  fn decode_owned<B, UB>(
    context: &Context,
    wire_type: WireType,
    src: B,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    Self::decode::<()>(context, wire_type, src.as_bytes())
  }

  fn decode_length_delimited_owned<B, UB>(
    context: &Context,
    wire_type: WireType,
    src: B,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    Self::decode_length_delimited::<()>(context, wire_type, src.as_bytes())
  }
}

impl<const N: usize> PartialMessage<Network> for [u8; N] {
  type UnknownBuffer<B> = ();

  type Encoded<'a>
    = &'a [u8]
  where
    Self: Sized + 'a;

  type Borrowed<'a>
    = &'a Self
  where
    Self: 'a;

  type EncodedOwned
    = Self
  where
    Self: Sized + 'static;
}

impl<const N: usize> Message<Network> for [u8; N] {
  type Partial = Self;

  type Encoded<'a>
    = &'a [u8]
  where
    Self: Sized + 'a;

  type Borrowed<'a>
    = &'a Self
  where
    Self: 'a;

  type EncodedOwned
    = Self
  where
    Self: Sized + 'static;
}

impl<const N: usize> IntoTarget<Network, Self> for [u8; N]
{
  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}

impl<const N: usize> TypeOwned<Network, Self> for [u8; N]
{
  fn to(&self) -> Result<Self, DecodeError> {
    Ok(*self)
  }
}

impl<const N: usize> IntoTarget<Network, [u8; N]> for &[u8] {
  fn into_target(self) -> Result<[u8; N], DecodeError> {
    self.try_into().map_err(|_| DecodeError::buffer_underflow())
  }
}

impl<const N: usize> TypeRef<Network, [u8; N]> for &[u8] {
  fn to(&self) -> Result<[u8; N], DecodeError> {
    self.into_target()
  }
}

#[inline]
fn decode_to_array<const N: usize>(src: &[u8]) -> Result<(usize, [u8; N]), DecodeError> {
  if N == 0 {
    return Ok((0, [0u8; N]));
  }

  if src.len() < N {
    return Err(larger_than_array_capacity::<N>());
  }

  Ok((N, src[..N].try_into().unwrap()))
}

#[inline]
fn decode_length_delimited_to_array<const N: usize>(
  src: &[u8],
) -> Result<(usize, [u8; N]), DecodeError> {
  if N == 0 {
    return Ok((0, [0u8; N]));
  }

  let (size_len, size) = varing::decode_u32_varint(src)?;
  let end = size_len + size as usize;
  if end > src.len() {
    return Err(DecodeError::buffer_underflow());
  }

  if end < N {
    return Err(larger_than_array_capacity::<N>());
  }

  Ok((end, src[size_len..end].try_into().unwrap()))
}