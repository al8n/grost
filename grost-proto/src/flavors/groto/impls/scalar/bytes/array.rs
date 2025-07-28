use crate::{
  decode::{Decode, DecodeOwned},
  encode::Encode,
  flavors::{
    Groto,
    groto::{Context, Error, Fixed8, Fixed16, Fixed32, Fixed64, Fixed128, LengthDelimited},
  },
};

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
      impl Encode<Groto, $wt> for [u8; $size] {
        fn encode<WB>(&self, _: &Context, buf: &mut WB) -> Result<usize, Error> where WB: WriteBuf + ?Sized {
          Ok(encode_fixed!(self(buf) as $size))
        }

        fn encoded_len(&self, _: &Context) -> usize {
          $size
        }

        fn encoded_length_delimited_len(
          &self,
          ctx: &Context,
        ) -> usize {
          <Self as Encode<Groto, $wt>>::encoded_len(self, ctx)
        }

        fn encode_length_delimited(
          &self,
          ctx: &Context,
          buf: &mut [u8],
        ) -> Result<usize, Error> {
          <Self as Encode<Groto, $wt>>::encode(self, ctx, buf)
        }
      }

      impl<'de> Decode<'de, Groto, $wt, Self> for [u8; $size] {
        fn decode<UB>(
          _: &Context,
          src: &'de [u8],
        ) -> Result<(usize, Self), Error>
        where
          Self: Sized + 'de,
          UB: crate::buffer::Buffer<Unknown<B>> + 'de,
        {
          Ok(decode_fixed!($size: src))
        }

        fn decode_length_delimited<UB>(
          ctx: &Context,
          src: &'de [u8],
        ) -> Result<(usize, Self), Error>
        where
          Self: Sized + 'de,
          UB: crate::buffer::Buffer<Unknown<B>> + 'de,
        {
          <Self as Decode<'de, Groto, $wt, Self>>::decode::<UB>(ctx, src)
        }
      }

      decode_owned_scalar!(Groto: [u8; $size] as $wt);
    )*
  };
}

// impl_fixed!(Fixed8(1), Fixed16(2), Fixed32(4), Fixed64(8), Fixed128(16),);

// #[inline]
// fn decode_to_array<const N: usize>(src: &[u8]) -> Result<(usize, [u8; N]), Error> {
//   if N == 0 {
//     return Ok((0, [0u8; N]));
//   }

//   if src.len() < N {
//     return Err(larger_than_array_capacity::<N>());
//   }

//   Ok((N, src[..N].try_into().unwrap()))
// }

// #[inline]
// fn decode_length_delimited_to_array<const N: usize>(src: &[u8]) -> Result<(usize, [u8; N]), Error> {
//   if N == 0 {
//     return Ok((0, [0u8; N]));
//   }

//   let (size_len, size) = varing::decode_u32_varint(src)?;
//   let end = size_len + size as usize;
//   if end > src.len() {
//     return Err(Error::buffer_underflow());
//   }

//   if end < N {
//     return Err(larger_than_array_capacity::<N>());
//   }

//   Ok((end, src[size_len..end].try_into().unwrap()))
// }
