use crate::{
  decode::{Decode, DecodeOwned}, decode_owned_scalar, encode::Encode, flavors::{
    network::{
      Context, DecodeError, EncodeError, Fixed128, Fixed16, Fixed32, Fixed64, Fixed8, LengthDelimited, Unknown
    }, Network, Selectable
  }, partial_encode_scalar, IntoTarget, Message, PartialMessage, TypeRef
};

impl<const N: usize> Encode<Network, LengthDelimited> for [u8; N] {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let this_len = self.len();
    let buf_len = buf.len();
    if buf_len < this_len {
      return Err(EncodeError::insufficient_buffer(this_len, buf_len));
    }

    buf[..this_len].copy_from_slice(self.as_slice());
    Ok(this_len)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    self.len()
  }

  fn encoded_length_delimited_len(&self, _: &Context) -> usize {
    let this_len = self.len();
    let len_size = varing::encoded_u32_varint_len(this_len as u32);
    len_size + this_len
  }

  fn encode_length_delimited(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let this_len = self.len();
    let mut offset = varing::encode_u32_varint_to(this_len as u32, buf)?;
    let buf_len = buf.len();
    if buf_len < offset + this_len {
      return Err(EncodeError::insufficient_buffer(this_len + offset, buf_len));
    }

    buf[offset..offset + this_len].copy_from_slice(self.as_slice());
    offset += this_len;
    Ok(offset)
  }
}

impl<const N: usize> Selectable<Network> for [u8; N] {
  type Selector = bool;
}

partial_encode_scalar!(Network: [u8; N] [const N: usize] as LengthDelimited);

impl<'de, const N: usize> Decode<'de, Network, LengthDelimited, Self> for [u8; N] {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
    decode_to_array::<N>(src)
  }

  fn decode_length_delimited<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
    decode_length_delimited_to_array::<N>(src)
  }
}

impl<const N: usize> DecodeOwned<Network, LengthDelimited, Self> for [u8; N] {
  fn decode_owned<B, UB>(context: &Context, src: B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    Self::decode::<()>(context, src.as_bytes())
  }

  fn decode_length_delimited_owned<B, UB>(
    context: &Context,
    src: B,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    Self::decode_length_delimited::<()>(context, src.as_bytes())
  }
}

impl<const N: usize> PartialMessage<Network, LengthDelimited> for [u8; N] {
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

impl<const N: usize> Message<Network, LengthDelimited> for [u8; N] {
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

macro_rules! decode_fixed {
  ($size:literal:$src:ident) => {{
    if $src.len() < $size {
      return Err(DecodeError::buffer_underflow());
    }
    ($size, $src[..$size].try_into().unwrap())
  }};
}

macro_rules! impl_fixed {
  ($($wt:ident($size:literal)),+$(,)?) => {
    $(
      impl Encode<Network, $wt> for [u8; $size] {
        fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
          Ok(encode_fixed!(self(buf) as $size))
        }

        fn encoded_len(&self, _: &Context) -> usize {
          $size
        }

        fn encoded_length_delimited_len(
          &self,
          ctx: &Context,
        ) -> usize {
          <Self as Encode<Network, $wt>>::encoded_len(self, ctx)
        }

        fn encode_length_delimited(
          &self,
          ctx: &Context,
          buf: &mut [u8],
        ) -> Result<usize, EncodeError> {
          <Self as Encode<Network, $wt>>::encode(self, ctx, buf)
        }
      }

      partial_encode_scalar!(Network: [u8; $size] as $wt);

      impl<'de> Decode<'de, Network, $wt, Self> for [u8; $size] {
        fn decode<UB>(
          _: &Context,
          src: &'de [u8],
        ) -> Result<(usize, Self), DecodeError>
        where
          Self: Sized + 'de,
          UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
        {
          Ok(decode_fixed!($size: src))
        }

        fn decode_length_delimited<UB>(
          ctx: &Context,
          src: &'de [u8],
        ) -> Result<(usize, Self), DecodeError>
        where
          Self: Sized + 'de,
          UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
        {
          <Self as Decode<'de, Network, $wt, Self>>::decode::<UB>(ctx, src)
        }
      }

      decode_owned_scalar!(Network: [u8; $size] as $wt);
    )*
  };
}

impl_fixed!(Fixed8(1), Fixed16(2), Fixed32(4), Fixed64(8), Fixed128(16),);

impl<const N: usize> IntoTarget<Network, [u8; N]> for &[u8] {
  fn into_target(self) -> Result<[u8; N], DecodeError> {
    self.try_into().map_err(|_| DecodeError::buffer_underflow())
  }
}

impl<const N: usize> TypeRef<Network, [u8; N]> for &[u8] {
  fn to(&self) -> Result<[u8; N], DecodeError> {
    (*self).into_target()
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

#[cfg(not(any(feature = "std", feature = "alloc")))]
fn larger_than_array_capacity<const N: usize>() -> DecodeError {
  DecodeError::custom("cannot decode array with length greater than the capacity")
}

#[cfg(any(feature = "std", feature = "alloc"))]
fn larger_than_array_capacity<const N: usize>() -> DecodeError {
  DecodeError::custom(std::format!(
    "cannot decode array with length greater than the capacity {N}"
  ))
}

#[cfg(all(test, feature = "std"))]
mod tests {
  use super::*;

  macro_rules! quickcheck_fixed {
    ($input:ident($len:literal, $wt:ident)) => {
      paste::paste! {
        quickcheck::quickcheck! {
          fn [< fuzzy_encode_decode_array_ $len >](input: $input) -> bool {
            let input: [u8; $len] = input.to_le_bytes();
            let mut buf = [0u8; $len];
            let context = Context::new();

            let encoded = <[u8; $len] as Encode<Network, $wt>>::encode(&input, &context, &mut buf).unwrap();
            let (decoded_len, decoded) = <[u8; $len] as Decode<'_, Network, $wt, _>>::decode::<()>(&context, &buf).unwrap();
            let encoded_len = <[u8; $len] as Encode<Network, $wt>>::encoded_len(&input, &context);
            assert_eq!(encoded, encoded_len);
            assert_eq!(encoded, decoded_len);
            assert_eq!(input, decoded);

            let mut buf = [0; 512];
            let encoded = <[u8; $len] as Encode<Network, LengthDelimited>>::encode(&input, &context, &mut buf).unwrap();
            let (decoded_len, decoded) = <[u8; $len] as Decode<'_, Network, LengthDelimited, _>>::decode::<()>(&context, &buf).unwrap();
            let encoded_len = <[u8; $len] as Encode<Network, LengthDelimited>>::encoded_len(&input, &context,);
            assert_eq!(encoded, encoded_len);
            assert_eq!(encoded, decoded_len);
            assert_eq!(input, decoded);

            let encoded = <[u8; $len] as Encode<Network, $wt>>::encode_length_delimited(&input, &context, &mut buf).unwrap();
            let (decoded_len, decoded) = <[u8; $len] as Decode<'_, Network, $wt, _>>::decode_length_delimited::<()>(&context, &buf).unwrap();
            let encoded_len = <[u8; $len] as Encode<Network, LengthDelimited>>::encoded_length_delimited_len(&input, &context);
            assert_eq!(encoded, encoded_len);
            assert_eq!(encoded, decoded_len);
            assert_eq!(input, decoded);

            let encoded = <[u8; $len] as Encode<Network, LengthDelimited>>::encode_length_delimited(&input, &context, &mut buf).unwrap();
            let (decoded_len, decoded) = <[u8; $len] as Decode<'_, Network, LengthDelimited, _>>::decode_length_delimited::<()>(&context, &buf).unwrap();
            let encoded_len = <[u8; $len] as Encode<Network, LengthDelimited>>::encoded_length_delimited_len(&input, &context,);
            assert_eq!(encoded, encoded_len);
            assert_eq!(encoded, decoded_len);
            assert_eq!(input, decoded);

            true
          }
        }
      }
    };
  }

  quickcheck_fixed!(u8(1, Fixed8));
  quickcheck_fixed!(u16(2, Fixed16));
  quickcheck_fixed!(u32(4, Fixed32));
  quickcheck_fixed!(u64(8, Fixed64));
  quickcheck_fixed!(u128(16, Fixed128));
}
