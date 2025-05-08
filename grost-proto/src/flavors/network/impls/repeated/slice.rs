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

macro_rules! repeated_impl {
  (@fixed $($size:literal::$wf:ty),+$(,)?) => {
    $(
      impl<V> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>> for [V]
      where
        V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
      {
        fn encode(&self, context: &$crate::__private::flavors::network::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
          let mut offset = 0;
          let buf_len = buf.len();
          let encoded_len = <Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encoded_len(self, context);
          if encoded_len > buf_len {
            return ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(encoded_len, buf_len));
          }

          for v in self {
            if offset >= buf_len {
              return ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(offset, buf_len));
            }
            offset += V::encode_length_delimited(v, context, &mut buf[offset..])
              .map_err(|e| e.update(<Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encoded_len(self, context), buf_len))?;
          }

          ::core::result::Result::Ok(offset)
        }

        fn encoded_len(&self, _: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
          self.len() * $size
        }

        fn encoded_length_delimited_len(&self, context: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
          let encoded_len = <Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encoded_len(self, context);
          let varint_len = varing::encoded_u32_varint_len(encoded_len as ::core::primitive::u32);
          encoded_len + varint_len
        }

        fn encode_length_delimited(
          &self,
          context: &$crate::__private::flavors::network::Context,
          buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
          let encoded_len = <Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encoded_len(self, context);
          let buf_len = buf.len();
          let offset = varing::encode_u32_varint_to(encoded_len as ::core::primitive::u32, buf)
            .map_err(|e| $crate::__private::flavors::network::EncodeError::from_varint_error(e)
              .update(<Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encoded_length_delimited_len(self, context), buf_len))?;
          let end = offset + encoded_len;
          if end > buf_len {
            return ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(end, buf_len));
          }

          <Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encode(self, context, &mut buf[offset..])
            .map(|write| {
              #[cfg(debug_assertions)]
              {
                $crate::debug_assert_write_eq::<Self>(write, encoded_len);
              }
              end
            })
            .map_err(|e| e.update(end, buf_len))
        }
      }
    )*
  };
  ($($wf:ty),+$(,)?) => {
    $(
      impl<V> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>> for [V]
      where
        V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
      {
        fn encode(&self, context: &$crate::__private::flavors::network::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
          let mut offset = 0;
          let buf_len = buf.len();
          let encoded_len = <Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encoded_len(self, context);
          if encoded_len > buf_len {
            return ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(encoded_len, buf_len));
          }

          for v in self {
            if offset >= buf_len {
              return ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(offset, buf_len));
            }
            offset += V::encode_length_delimited(v, context, &mut buf[offset..])
              .map_err(|e| e.update(<Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encoded_len(self, context), buf_len))?;
          }

          ::core::result::Result::Ok(offset)
        }

        fn encoded_len(&self, context: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
          self.iter().map(|v| v.encoded_length_delimited_len(context)).sum()
        }

        fn encoded_length_delimited_len(&self, context: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
          let encoded_len = <Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encoded_len(self, context);
          let varint_len = varing::encoded_u32_varint_len(encoded_len as ::core::primitive::u32);
          encoded_len + varint_len
        }

        fn encode_length_delimited(
          &self,
          context: &$crate::__private::flavors::network::Context,
          buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
          let encoded_len = <Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encoded_len(self, context);
          let buf_len = buf.len();
          let offset = varing::encode_u32_varint_to(encoded_len as ::core::primitive::u32, buf)
            .map_err(|e| $crate::__private::flavors::network::EncodeError::from_varint_error(e)
              .update(<Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encoded_length_delimited_len(self, context), buf_len))?;
          let end = offset + encoded_len;
          if end > buf_len {
            return ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(end, buf_len));
          }

          <Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encode(self, context, &mut buf[offset..])
            .map(|write| {
              #[cfg(debug_assertions)]
              {
                $crate::debug_assert_write_eq::<Self>(write, encoded_len);
              }
              end
            })
            .map_err(|e| e.update(end, buf_len))
        }
      }
    )*
  };
}

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

impl<V, W, const I: u32> Encode<Network, Repeated<Stream<W, I>>> for [V]
where
  Repeated<Stream<W, I>>: WireFormat<Network>,
  V: Encode<Network, W>,
  W: WireFormat<Network>,
{
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let encoded_identifier = Identifier::from_u32(I).encode();
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
    let encoded_identifier_len = Identifier::from_u32(I).encoded_len();

    encoded_identifier_len * self.len()
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

impl<V> Selectable<Network> for [V]
where
  V: Selectable<Network>,
{
  type Selector = V::Selector;
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
    let encoded_identifier = Identifier::from_u32(I).encode();
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
    let encoded_identifier_len = Identifier::from_u32(I).encoded_len();

    encoded_identifier_len * self.len()
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
