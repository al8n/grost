use bstr_1::BStr;

use crate::{
  Decode, Encode, Wirable,
  flavors::network::{Context, DecodeError, EncodeError, Network, WireType},
  unknown::UnknownBuffer,
};

impl Wirable<Network> for BStr {
  const WIRE_TYPE: WireType = WireType::LengthDelimited;
}

impl Encode<Network> for BStr {
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    <[u8] as Encode<Network>>::encode(self, context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    <[u8] as Encode<Network>>::encoded_len(self, context)
  }
}

partial_encode_primitives!(Network: BStr);

impl<'de> Decode<'de, Network, Self> for &'de BStr {
  fn decode<UB>(context: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: UnknownBuffer<Network, &'de [u8]> + 'de,
  {
    <&'de [u8] as Decode<'de, Network, &'de [u8]>>::decode::<()>(context, src)
      .map(|(len, slice)| (len, BStr::new(slice)))
  }
}

#[cfg(all(any(feature = "std", feature = "alloc"), feature = "bytes_1"))]
const _: () = {
  use bstr_1::BString;
  use bytes_1::Bytes;

  bytes_bridge!(
    Network: (WireType::LengthDelimited) : BString {
      from_bytes: |s: &[u8]| {
        Ok(BString::new(s.to_vec()))
      };
      to_bytes: BString::as_ref;

      type EncodedOwned = Bytes {
        from_ref: |s: &Bytes| Ok(BString::new(s.to_vec()));
        from: |s: Bytes| Ok(BString::new(s.into()));
      }
    }
  );
};
