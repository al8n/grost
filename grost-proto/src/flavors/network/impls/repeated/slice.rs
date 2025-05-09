use crate::{
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultWireFormat, Network, WireFormat,
    network::{
      Context, EncodeError, Fixed8, Fixed16, Fixed32, Fixed64, Fixed128, Identifier,
      LengthDelimited, Repeated, Stream, Varint,
    },
  },
  selector::Selectable,
};

repeated_impl!(
  @fixed
  1::Fixed8,
  2::Fixed16,
  4::Fixed32,
  8::Fixed64,
  16::Fixed128,
);

repeated_impl!(LengthDelimited, Varint,);

impl<V> DefaultWireFormat<Network> for [V]
where
  V: DefaultWireFormat<Network>,
  Repeated<V::Format>: WireFormat<Network>,
  Self: Encode<Network, Repeated<V::Format>>,
{
  type Format = Repeated<V::Format>;
}

impl<V> Selectable<Network> for [V]
where
  V: Selectable<Network>,
{
  type Selector = V::Selector;
}

impl<V, W, const I: u32> Encode<Network, Repeated<Stream<W, I>>> for [V]
where
  Repeated<Stream<W, I>>: WireFormat<Network>,
  V: Encode<Network, W>,
  W: WireFormat<Network>,
{
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let encoded_identifier = Identifier::try_from_u32(I)
      .map_err(|_| {
        // TODO(al8n): make this a proper error
        EncodeError::custom("invalid tag")
      })?
      .encode();
    let encoded_identifier_len = encoded_identifier.len();

    let mut offset = 0;
    let buf_len = buf.len();

    for item in self {
      if offset + encoded_identifier_len >= buf_len {
        return Err(EncodeError::insufficient_buffer(
          self.encoded_len(context),
          buf_len,
        ));
      }
      buf[offset..offset + encoded_identifier_len].copy_from_slice(&encoded_identifier);
      offset += encoded_identifier_len;

      if offset >= buf_len {
        return Err(EncodeError::insufficient_buffer(
          self.encoded_len(context),
          buf_len,
        ));
      }
      offset += item.encode_length_delimited(context, &mut buf[offset..])?;
    }

    #[cfg(debug_assertions)]
    {
      crate::debug_assert_write_eq::<Self>(offset, self.encoded_len(context));
    }

    Ok(offset)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    varing::encoded_u32_varint_len(I) * self.len()
      + self
        .iter()
        .map(|v| v.encoded_length_delimited_len(context))
        .sum::<usize>()
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    self.encoded_len(context)
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    self.encode(context, buf)
  }
}

impl<V, W, const I: u32> PartialEncode<Network, Repeated<Stream<W, I>>> for [V]
where
  Repeated<Stream<W, I>>: WireFormat<Network>,
  V: PartialEncode<Network, W>,
  W: WireFormat<Network>,
{
  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError> {
    let encoded_identifier = Identifier::try_from_u32(I)
      .map_err(|_| {
        // TODO(al8n): make this a proper error
        EncodeError::custom("invalid tag")
      })?
      .encode();
    let encoded_identifier_len = encoded_identifier.len();

    let mut offset = 0;
    let buf_len = buf.len();

    for item in self {
      if offset + encoded_identifier_len >= buf_len {
        return Err(EncodeError::insufficient_buffer(
          self.partial_encoded_len(context, selector),
          buf_len,
        ));
      }
      buf[offset..offset + encoded_identifier_len].copy_from_slice(&encoded_identifier);
      offset += encoded_identifier_len;

      if offset >= buf_len {
        return Err(EncodeError::insufficient_buffer(
          self.partial_encoded_len(context, selector),
          buf_len,
        ));
      }
      offset += item.partial_encode_length_delimited(context, &mut buf[offset..], selector)?;
    }

    #[cfg(debug_assertions)]
    {
      crate::debug_assert_write_eq::<Self>(offset, self.partial_encoded_len(context, selector));
    }

    Ok(offset)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    varing::encoded_u32_varint_len(I) * self.len()
      + self
        .iter()
        .map(|v| v.partial_encoded_length_delimited_len(context, selector))
        .sum::<usize>()
  }

  fn partial_encoded_length_delimited_len(
    &self,
    context: &Context,
    selector: &Self::Selector,
  ) -> usize {
    self.partial_encoded_len(context, selector)
  }

  fn partial_encode_length_delimited(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError> {
    self.partial_encode(context, buf, selector)
  }
}
