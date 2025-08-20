use crate::{
  buffer::{Chunk, ChunkWriter, UnknownBuffer},
  convert::{PartialIdentity, TryFromPartialRef, TryFromRef},
  decode::{BytesSlice, Decode},
  default_bytes_wire_format, encode_bridge, flatten_state,
  flavors::groto::{Context, DecodeError, EncodeError, Groto, LengthDelimited},
  partial_ref_state, partial_state, ref_state, selectable,
  state::{Partial, PartialRef, Ref, State},
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

impl<'de, RB, B> TryFromPartialRef<'de, LengthDelimited, RB, B, Groto> for Bytes {
  fn try_from_partial_ref(
    _: &'de Context,
    input: <Self as State<PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output,
  ) -> Result<Self, DecodeError>
  where
    Self: Sized,
    RB: Chunk,
    B: UnknownBuffer<RB, Groto>,
  {
    Ok(input.into_inner().to_bytes())
  }
}

impl<'de, RB, B> TryFromRef<'de, LengthDelimited, RB, B, Groto> for Bytes {
  fn try_from_ref(
    _: &'de Context,
    input: <Self as State<Ref<'de, LengthDelimited, RB, B, Groto>>>::Output,
  ) -> Result<Self, DecodeError>
  where
    Self: Sized,
    RB: Chunk + 'de,
    B: UnknownBuffer<RB, Groto>,
  {
    Ok(input.into_inner().to_bytes())
  }
}

impl PartialIdentity<Groto> for Bytes {
  fn partial_identity<'a>(
    input: &'a mut <Self as State<Partial<Groto>>>::Output,
    _: &'a bool,
  ) -> &'a mut Self
  where
    Self: Sized,
  {
    input
  }
}

impl<'de, RB, B> TryFromPartialRef<'de, LengthDelimited, RB, B, Groto> for BytesMut {
  fn try_from_partial_ref(
    _: &'de Context,
    input: <Self as State<PartialRef<'de, LengthDelimited, RB, B, Groto>>>::Output,
  ) -> Result<Self, DecodeError>
  where
    Self: Sized,
    RB: Chunk,
    B: UnknownBuffer<RB, Groto>,
  {
    Ok(BytesMut::from(input.into_inner().to_bytes()))
  }
}

impl<'de, RB, B> TryFromRef<'de, LengthDelimited, RB, B, Groto> for BytesMut {
  fn try_from_ref(
    _: &'de Context,
    input: <Self as State<Ref<'de, LengthDelimited, RB, B, Groto>>>::Output,
  ) -> Result<Self, DecodeError>
  where
    Self: Sized,
    <Self as State<Ref<'de, LengthDelimited, RB, B, Groto>>>::Output: Sized,
    RB: Chunk + 'de,
    B: UnknownBuffer<RB, Groto>,
  {
    Ok(BytesMut::from(input.into_inner().to_bytes()))
  }
}

impl PartialIdentity<Groto> for BytesMut {
  fn partial_identity<'a>(
    input: &'a mut <Self as State<Partial<Groto>>>::Output,
    _: &'a bool,
  ) -> &'a mut Self
  where
    Self: Sized,
  {
    input
  }
}

bidi_equivalent!(:<RB: Chunk>: impl<Bytes, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
bidi_equivalent!(impl <Bytes, LengthDelimited> for <[u8], LengthDelimited>);

bidi_equivalent!(:<RB: Chunk>: impl<BytesMut, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
bidi_equivalent!(impl <BytesMut, LengthDelimited> for <[u8], LengthDelimited>);

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for Bytes {
  fn decode(context: &'de Context, src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Chunk + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    <BytesSlice<RB> as Decode<'de, LengthDelimited, RB, B, Groto>>::decode(context, src)
      .map(|(n, v)| (n, v.into_inner().to_bytes()))
  }
}

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for BytesMut {
  fn decode(context: &'de Context, src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Chunk + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    <BytesSlice<RB> as Decode<'de, LengthDelimited, RB, B, Groto>>::decode(context, src)
      .map(|(n, v)| (n, BytesMut::from(v.into_inner().to_bytes_mut())))
  }
}
