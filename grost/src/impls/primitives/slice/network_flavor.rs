use crate::{
  Decode, Encode, IntoTarget, TypeRef, Wirable,
  flavors::network::{Context, DecodeError, EncodeError, Network, WireType},
  unknown::UnknownBuffer,
};

impl Wirable<Network> for [u8] {
  const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

impl Encode<Network> for [u8] {
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    crate::__private::network::encode(
      context,
      self,
      buf,
      |val, buf| {
        let this_len = val.len();
        let buf_len = buf.len();

        if this_len > buf_len {
          return Err(EncodeError::insufficient_buffer(this_len, buf_len));
        }

        Ok(this_len)
      },
      |val| val.len(),
    )
  }

  #[inline]
  fn encoded_len(&self, context: &Context) -> usize {
    crate::__private::network::encoded_len(context, self, |val| val.len())
  }
}

partial_encode_primitives!(Network: [u8]);

impl<'de> Decode<'de, Network, Self> for &'de [u8] {
  fn decode<UB>(ctx: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: UnknownBuffer<Network, &'de [u8]> + 'de,
  {
    crate::__private::network::decode(ctx, src, |src| Ok((src.len(), src)))
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
