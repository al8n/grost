use crate::{
  Decode, DecodeOwned, Encode, IntoTarget, Message, PartialMessage, TypeOwned, TypeRef, Wirable,
  buffer::{Buffer, BytesBuffer},
  flavors::network::{Context, DecodeError, EncodeError, Network, WireType},
  unknown::Unknown,
};

impl<const N: usize> Wirable<Network> for [u8; N] {
  const WIRE_TYPE: WireType = {
    match N {
      0 => WireType::Zst,
      1 => WireType::Byte,
      2 => WireType::Fixed16,
      4 => WireType::Fixed32,
      8 => WireType::Fixed64,
      16 => WireType::Fixed128,
      _ => WireType::LengthDelimited,
    }
  };
}

impl<const N: usize> Encode<Network> for [u8; N] {
  fn encode(&self, ctx: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    if N == 0 {
      return Ok(0);
    }

    crate::__private::network::encode(
      ctx,
      self,
      buf,
      |val, buf| {
        if buf.len() < N {
          return Err(EncodeError::insufficient_buffer(N, buf.len()));
        }

        buf[..N].copy_from_slice(val.as_slice());
        Ok(N)
      },
      |_| N,
    )
  }

  #[inline]
  fn encoded_len(&self, ctx: &Context) -> usize {
    if N == 0 {
      return 0;
    }
    crate::__private::network::encoded_len(ctx, self, |_| N)
  }
}

impl<'de, const N: usize> Decode<'de, Network, Self> for [u8; N] {
  fn decode<UB>(ctx: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: Buffer<Unknown<Network, &'de [u8]>>,
  {
    if N == 0 {
      return Ok((0, [0; N]));
    }

    crate::__private::network::decode(ctx, src, decode)
  }
}

impl<const N: usize> DecodeOwned<Network, Self> for [u8; N] {
  fn decode_owned<B, UB>(context: &Context, src: B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: BytesBuffer + 'static,
    UB: Buffer<Unknown<Network, B>> + 'static,
  {
    <Self as Decode<'_, Network, Self>>::decode::<()>(context, src.as_bytes())
  }
}

impl<const N: usize> IntoTarget<Network, Self> for [u8; N] {
  fn into_target(self) -> Result<Self, DecodeError> {
    Ok(self)
  }
}

impl<const N: usize> TypeRef<Network, Self> for [u8; N] {
  fn to(&self) -> Result<Self, DecodeError> {
    Ok(*self)
  }
}

impl<const N: usize> TypeOwned<Network, Self> for [u8; N] {
  fn to(&self) -> Result<Self, DecodeError> {
    Ok(*self)
  }
}

impl<const N: usize> PartialMessage<Network> for [u8; N] {
  type UnknownBuffer<B> = ();

  type Encoded<'a>
    = &'a [u8]
  where
    Self: Sized + 'a;

  type Borrowed<'a>
    = &'a [u8; N]
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
    = &'a [u8; N]
  where
    Self: 'a;

  type EncodedOwned
    = Self
  where
    Self: Sized + 'static;
}

#[inline]
fn decode<const N: usize>(src: &[u8]) -> Result<(usize, [u8; N]), DecodeError> {
  if N == 0 {
    return Ok((0, [0; N]));
  }

  src[..N]
    .try_into()
    .map(|arr| (N, arr))
    .map_err(|_| DecodeError::buffer_underflow())
}
