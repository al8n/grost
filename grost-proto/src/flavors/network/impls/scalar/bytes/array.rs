use crate::{
  decode::{Decode, DecodeOwned},
  decode_owned_scalar,
  encode::Encode,
  flavors::{
    Network,
    network::{
      Context, Error, Fixed8, Fixed16, Fixed32, Fixed64, Fixed128, LengthDelimited, Unknown,
    },
  },
};

// impl<'de, B, const N: usize> Decode<'de, Network, LengthDelimited, Self, B> for [u8; N] {
//   fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
//   where
//     Self: Sized + 'de,
//     UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
//   {
//     decode_to_array::<N>(src)
//   }

//   fn decode_length_delimited<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
//   where
//     Self: Sized + 'de,
//     UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
//   {
//     decode_length_delimited_to_array::<N>(src)
//   }
// }

// impl<const N: usize> DecodeOwned<Network, LengthDelimited, Self> for [u8; N] {
//   fn decode_owned<B, UB>(context: &Context, src: B) -> Result<(usize, Self), Error>
//   where
//     Self: Sized + 'static,
//     B: crate::buffer::BytesBuffer + 'static,
//     UB: crate::buffer::Buffer<Unknown<B>> + 'static,
//   {
//     Self::decode::<()>(context, src.as_bytes())
//   }

//   fn decode_length_delimited_owned<B, UB>(context: &Context, src: B) -> Result<(usize, Self), Error>
//   where
//     Self: Sized + 'static,
//     B: crate::buffer::BytesBuffer + 'static,
//     UB: crate::buffer::Buffer<Unknown<B>> + 'static,
//   {
//     Self::decode_length_delimited::<()>(context, src.as_bytes())
//   }
// }

macro_rules! encode_fixed {
  ($this:ident($buf:ident) as $fixed:literal) => {{
    let buf_len = $buf.len();
    if buf_len < $fixed {
      return Err(Error::insufficient_buffer($fixed, buf_len));
    }
    $buf[..$fixed].copy_from_slice($this.as_slice());
    $fixed
  }};
}

macro_rules! decode_fixed {
  ($size:literal:$src:ident) => {{
    if $src.len() < $size {
      return Err(Error::buffer_underflow());
    }
    ($size, $src[..$size].try_into().unwrap())
  }};
}

macro_rules! impl_fixed {
  ($($wt:ident($size:literal)),+$(,)?) => {
    $(
      impl Encode<Network, $wt> for [u8; $size] {
        fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
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
        ) -> Result<usize, Error> {
          <Self as Encode<Network, $wt>>::encode(self, ctx, buf)
        }
      }

      // partial_encode_scalar!(Network: [u8; $size] as $wt);

      impl<'de> Decode<'de, Network, $wt, Self> for [u8; $size] {
        fn decode<UB>(
          _: &Context,
          src: &'de [u8],
        ) -> Result<(usize, Self), Error>
        where
          Self: Sized + 'de,
          UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
        {
          Ok(decode_fixed!($size: src))
        }

        fn decode_length_delimited<UB>(
          ctx: &Context,
          src: &'de [u8],
        ) -> Result<(usize, Self), Error>
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

// impl_fixed!(Fixed8(1), Fixed16(2), Fixed32(4), Fixed64(8), Fixed128(16),);

#[inline]
fn decode_to_array<const N: usize>(src: &[u8]) -> Result<(usize, [u8; N]), Error> {
  if N == 0 {
    return Ok((0, [0u8; N]));
  }

  if src.len() < N {
    return Err(larger_than_array_capacity::<N>());
  }

  Ok((N, src[..N].try_into().unwrap()))
}

#[inline]
fn decode_length_delimited_to_array<const N: usize>(src: &[u8]) -> Result<(usize, [u8; N]), Error> {
  if N == 0 {
    return Ok((0, [0u8; N]));
  }

  let (size_len, size) = varing::decode_u32_varint(src)?;
  let end = size_len + size as usize;
  if end > src.len() {
    return Err(Error::buffer_underflow());
  }

  if end < N {
    return Err(larger_than_array_capacity::<N>());
  }

  Ok((end, src[size_len..end].try_into().unwrap()))
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
fn larger_than_array_capacity<const N: usize>() -> Error {
  Error::custom("cannot decode array with length greater than the capacity")
}

#[cfg(any(feature = "std", feature = "alloc"))]
fn larger_than_array_capacity<const N: usize>() -> Error {
  Error::custom(std::format!(
    "cannot decode array with length greater than the capacity {N}"
  ))
}
