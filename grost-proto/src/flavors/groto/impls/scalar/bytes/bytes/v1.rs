use crate::{
  buffer::ReadBuf,
  convert::{PartialTransform, Transform},
  decode::BytesSlice,
  decode_bridge, default_wire_format, encode_bridge, flatten_state,
  flavors::groto::{Groto, LengthDelimited},
  partial_ref_state, partial_state, selectable,
};
use bytes_1::{Bytes, BytesMut};

default_wire_format!(Groto: Bytes as LengthDelimited; BytesMut as LengthDelimited);

selectable!(@scalar Groto: Bytes, BytesMut);

encode_bridge!(
  Groto: [u8] {
    Bytes as LengthDelimited {
      convert: Bytes::as_ref;
    },
    BytesMut as LengthDelimited {
      convert: BytesMut::as_ref;
    },
  },
);

decode_bridge!(
  Groto: &'de [u8] => BytesSlice<B> {
    Bytes as LengthDelimited {
      convert: |src: BytesSlice<B>| Bytes::copy_from_slice(src.as_ref());
    },
    BytesMut as LengthDelimited {
      convert: |src: BytesSlice<B>| {
        BytesMut::from(src.as_ref())
      };
    },
  },
);

partial_ref_state!(
  &'a Groto:
    Bytes as LengthDelimited => BytesSlice<__GROST_READ_BUF__>,
    BytesMut as LengthDelimited => BytesSlice<__GROST_READ_BUF__>,
);
partial_state!(
  Groto:
    Bytes => Bytes,
    BytesMut => BytesMut,
);
flatten_state!(Bytes, BytesMut);
bytes_bridge!(
  Groto: Bytes, BytesMut,
);

impl Transform<&[u8], Self, LengthDelimited, Groto> for Bytes {
  fn transform(input: &[u8]) -> Result<Self, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    Ok(Bytes::copy_from_slice(input))
  }
}

impl Transform<&[u8], Self, LengthDelimited, Groto> for BytesMut {
  fn transform(input: &[u8]) -> Result<Self, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    Ok(BytesMut::from(input))
  }
}

impl<B> Transform<BytesSlice<B>, Self, LengthDelimited, Groto> for Bytes
where
  B: ReadBuf,
{
  fn transform(input: BytesSlice<B>) -> Result<Self, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    Ok(Bytes::copy_from_slice(input.as_slice()))
  }
}

impl<B> Transform<BytesSlice<B>, Self, LengthDelimited, Groto> for BytesMut
where
  B: ReadBuf,
{
  fn transform(input: BytesSlice<B>) -> Result<Self, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    Ok(BytesMut::from(input.as_slice()))
  }
}

impl<B> PartialTransform<BytesSlice<B>, Option<Self>, LengthDelimited, Groto> for Bytes
where
  B: ReadBuf,
{
  fn partial_transform(
    input: BytesSlice<B>,
    selector: &bool,
  ) -> Result<Option<Self>, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    if *selector {
      <Self as Transform<BytesSlice<B>, Self, LengthDelimited, Groto>>::transform(input).map(Some)
    } else {
      Ok(None)
    }
  }
}

impl<B> PartialTransform<BytesSlice<B>, Option<Self>, LengthDelimited, Groto> for BytesMut
where
  B: ReadBuf,
{
  fn partial_transform(
    input: BytesSlice<B>,
    selector: &bool,
  ) -> Result<Option<Self>, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    if *selector {
      <Self as Transform<BytesSlice<B>, Self, LengthDelimited, Groto>>::transform(input).map(Some)
    } else {
      Ok(None)
    }
  }
}
