use crate::{
  buffer::{Buffer, ReadBuf},
  convert::{
    Partial, PartialIdentity, PartialRef, PartialTransform, PartialTryFromRef, Ref, State,
    Transform, TryFromPartial, TryFromPartialRef, TryFromRef,
  },
  decode::BytesSlice,
  decode_bridge, default_bytes_wire_format, encode_bridge, flatten_state,
  flavors::groto::{Error, Groto, LengthDelimited, Unknown},
  partial_ref_state, partial_state, ref_state, selectable,
};
use bytes_1::{Bytes, BytesMut};

default_bytes_wire_format!(Groto: Bytes as LengthDelimited; BytesMut as LengthDelimited);

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
  Groto: &'de [u8] => BytesSlice<RB> {
    Bytes as LengthDelimited {
      convert: |src: BytesSlice<RB>| Bytes::copy_from_slice(src.as_ref());
    },
    BytesMut as LengthDelimited {
      convert: |src: BytesSlice<RB>| {
        BytesMut::from(src.as_ref())
      };
    },
  },
);

ref_state!(
  &'a Groto:
    Bytes as LengthDelimited => BytesSlice<__GROST_READ_BUF__>,
    BytesMut as LengthDelimited => BytesSlice<__GROST_READ_BUF__>,
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
identity_partial_transform!(
  Groto {
    Bytes as LengthDelimited,
    BytesMut as LengthDelimited,
  }
);

impl TryFromPartial<LengthDelimited, Groto> for Bytes {
  fn try_from_partial(input: <Self as State<Partial<Groto>>>::Output) -> Result<Self, Error>
  where
    Self: Sized,
  {
    Ok(input)
  }
}

impl<'de, RB, B> TryFromPartialRef<'de, RB, B, LengthDelimited, Groto> for Bytes {
  fn try_from_partial_ref(
    input: <Self as State<PartialRef<'de, RB, B, LengthDelimited, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    RB: ReadBuf,
    B: Buffer<Unknown<RB>>,
  {
    Ok(input.into_inner().to_bytes())
  }
}

impl<'de, RB, B> TryFromRef<'de, RB, B, LengthDelimited, Groto> for Bytes {
  fn try_from_ref(
    input: <Self as State<Ref<'de, RB, B, LengthDelimited, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    RB: ReadBuf + 'de,
    B: Buffer<Unknown<RB>>,
  {
    Ok(input.into_inner().to_bytes())
  }
}

impl<'de, RB, B> PartialTryFromRef<'de, RB, B, LengthDelimited, Groto> for Bytes
where
  RB: ReadBuf,
  B: Buffer<Unknown<RB>> + 'de,
{
  fn partial_try_from_ref(
    input: <Self as State<PartialRef<'de, RB, B, LengthDelimited, Groto>>>::Output,
    _: &bool,
  ) -> Result<Self, Error>
  where
    Self: Sized,
  {
    Ok(input.into_inner().to_bytes())
  }
}

impl PartialIdentity<LengthDelimited, Groto> for Bytes {
  fn partial_identity(input: <Self as State<Partial<Groto>>>::Output, _: &bool) -> Self
  where
    Self: Sized,
  {
    input
  }
}

impl TryFromPartial<LengthDelimited, Groto> for BytesMut {
  fn try_from_partial(input: <Self as State<Partial<Groto>>>::Output) -> Result<Self, Error>
  where
    Self: Sized,
  {
    Ok(input)
  }
}

impl<'de, RB, B> TryFromPartialRef<'de, RB, B, LengthDelimited, Groto> for BytesMut {
  fn try_from_partial_ref(
    input: <Self as State<PartialRef<'de, RB, B, LengthDelimited, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    RB: ReadBuf,
    B: Buffer<Unknown<RB>>,
  {
    Ok(BytesMut::from(input.into_inner().to_bytes()))
  }
}

impl<'de, RB, B> TryFromRef<'de, RB, B, LengthDelimited, Groto> for BytesMut
where
  RB: ReadBuf,
  B: Buffer<Unknown<RB>> + 'de,
{
  fn try_from_ref(
    input: <Self as State<Ref<'de, RB, B, LengthDelimited, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
  {
    Ok(BytesMut::from(input.into_inner().to_bytes()))
  }
}

impl<'de, RB, B> PartialTryFromRef<'de, RB, B, LengthDelimited, Groto> for BytesMut
where
  RB: ReadBuf,
  B: Buffer<Unknown<RB>> + 'de,
{
  fn partial_try_from_ref(
    input: <Self as State<PartialRef<'de, RB, B, LengthDelimited, Groto>>>::Output,
    _: &bool,
  ) -> Result<Self, Error>
  where
    Self: Sized,
  {
    Ok(BytesMut::from(input.into_inner().to_bytes()))
  }
}

impl PartialIdentity<LengthDelimited, Groto> for BytesMut {
  fn partial_identity(input: <Self as State<Partial<Groto>>>::Output, _: &bool) -> Self
  where
    Self: Sized,
  {
    input
  }
}

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

impl<RB> Transform<BytesSlice<RB>, Self, LengthDelimited, Groto> for Bytes
where
  RB: ReadBuf,
{
  fn transform(input: BytesSlice<RB>) -> Result<Self, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    Ok(Bytes::copy_from_slice(input.as_slice()))
  }
}

impl<RB> Transform<BytesSlice<RB>, Self, LengthDelimited, Groto> for BytesMut
where
  RB: ReadBuf,
{
  fn transform(input: BytesSlice<RB>) -> Result<Self, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    Ok(BytesMut::from(input.as_slice()))
  }
}

impl<RB> PartialTransform<BytesSlice<RB>, Option<Self>, LengthDelimited, Groto> for Bytes
where
  RB: ReadBuf,
{
  fn partial_transform(
    input: BytesSlice<RB>,
    selector: &bool,
  ) -> Result<Option<Self>, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    if *selector {
      <Self as Transform<BytesSlice<RB>, Self, LengthDelimited, Groto>>::transform(input).map(Some)
    } else {
      Ok(None)
    }
  }
}

impl<RB> PartialTransform<BytesSlice<RB>, Option<Self>, LengthDelimited, Groto> for BytesMut
where
  RB: ReadBuf,
{
  fn partial_transform(
    input: BytesSlice<RB>,
    selector: &bool,
  ) -> Result<Option<Self>, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    if *selector {
      <Self as Transform<BytesSlice<RB>, Self, LengthDelimited, Groto>>::transform(input).map(Some)
    } else {
      Ok(None)
    }
  }
}

bidi_equivalent!(:<RB: ReadBuf>: impl<Bytes, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
bidi_equivalent!(impl <Bytes, LengthDelimited> for <[u8], LengthDelimited>);

bidi_equivalent!(:<RB: ReadBuf>: impl<BytesMut, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
bidi_equivalent!(impl <BytesMut, LengthDelimited> for <[u8], LengthDelimited>);
