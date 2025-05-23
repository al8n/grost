use crate::{
  decode::Decode,
  decode_owned_scalar,
  encode::Encode,
  flavors::{
    DefaultWireFormat,
    network::Context,
    selector::{
      DecodeError, EncodeError, Select, SelectorIdentifier, SelectorTag, SelectorWireType, Unknown,
      Zst,
    },
  },
  message, partial_encode_scalar, selectable,
};

selectable!(@scalar Select: bool);
partial_encode_scalar!(Select: bool as Zst);
decode_owned_scalar!(Select: bool as Zst);
message!(Select: bool as Zst);

impl DefaultWireFormat<Select> for bool {
  type Format = Zst;
}

impl Encode<Select, Zst> for bool {
  fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    if buf.is_empty() {
      return Err(EncodeError::insufficient_buffer(1, 0));
    }
    buf[0] = if *self {
      SelectorIdentifier::all()
    } else {
      SelectorIdentifier::none()
    }
    .as_u8();
    Ok(1)
  }

  fn encoded_len(&self, _: &Context) -> usize {
    1
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    <Self as Encode<Select, Zst>>::encoded_len(self, context)
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    <Self as Encode<Select, Zst>>::encode(self, context, buf)
  }

  fn encode_identified(
    &self,
    context: &Context,
    identifier: &SelectorIdentifier,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    let wt = identifier.wire_type();
    if identifier.wire_type() != SelectorWireType::Zst {
      return Err(EncodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wt,
      ));
    }

    let tag = identifier.tag();
    match tag {
      SelectorTag::All if *self => <Self as Encode<Select, Zst>>::encode(self, context, buf),
      SelectorTag::None if !*self => <Self as Encode<Select, Zst>>::encode(self, context, buf),
      SelectorTag::All => Err(EncodeError::identifier_mismatch(
        SelectorIdentifier::all(),
        *identifier,
      )),
      SelectorTag::None => Err(EncodeError::identifier_mismatch(
        SelectorIdentifier::none(),
        *identifier,
      )),
      tag => Err(EncodeError::unsupported_tag(
        core::any::type_name::<Self>(),
        tag,
      )),
    }
  }

  fn encoded_identified_len(&self, context: &Context, _: &SelectorIdentifier) -> usize {
    <Self as Encode<Select, Zst>>::encoded_len(self, context)
  }
}

impl<'de> Decode<'de, Select, Zst, Self> for bool {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<B>> + 'de,
  {
    if src.is_empty() {
      return Err(DecodeError::buffer_underflow());
    }

    let identifier = SelectorIdentifier::try_from_u8(src[0])?;

    let (wt, tag) = identifier.into_components();
    Ok(match (wt, tag) {
      (SelectorWireType::Zst, SelectorTag::None) => (1, false),
      (SelectorWireType::Zst, SelectorTag::All) => (1, true),
      _ => {
        return Err(DecodeError::unsupported_identifier(
          core::any::type_name::<Self>(),
          identifier,
        ));
      }
    })
  }

  fn decode_length_delimited<UB>(
    context: &Context,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<B>> + 'de,
  {
    <Self as Decode<'de, Select, Zst, Self>>::decode::<UB>(context, src)
  }
}
