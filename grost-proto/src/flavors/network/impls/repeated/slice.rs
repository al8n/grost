use crate::{encode::Encode, flavors::{network::{Context, EncodeError, Fixed128, Fixed16, Fixed32, Fixed64, Fixed8, LengthDelimited, Repeated, Varint}, DefaultWireFormat, Network}};

macro_rules! repeated_fixed_impl {
  ($($size:literal::$wf:ty),+$(,)?) => {
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
}

repeated_fixed_impl!(
  1::Fixed8,
  2::Fixed16,
  4::Fixed32,
  8::Fixed64,
  16::Fixed128,
);

impl<V> DefaultWireFormat<Network> for [V]
where
  V: DefaultWireFormat<Network>,
  Self: Encode<Network, Repeated<V::Format>>,
{
  type Format = Repeated<V::Format>;
}

impl<V> Encode<Network, Repeated<Varint>> for [V]
where
  V: Encode<Network, Varint>,
{
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let mut offset = 0;
    let buf_len = buf.len();
    for v in self {
      if offset >= buf_len {
        return Err(EncodeError::insufficient_buffer(offset, buf_len));
      }
      offset += V::encode_length_delimited(v, context, &mut buf[offset..])
        .map_err(|e| e.update(self.encoded_len(context), buf_len))?;
    }

    Ok(offset)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    self.iter().map(|v| v.encoded_length_delimited_len(context)).sum()
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    let encoded_len = self.encoded_len(context);
    let varint_len = varing::encoded_u32_varint_len(encoded_len as u32);
    encoded_len + varint_len
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    let encoded_len = self.encoded_len(context);
    let buf_len = buf.len();
    let offset = varing::encode_u32_varint_to(encoded_len as u32, buf)
      .map_err(|e| EncodeError::from(e).update(self.encoded_length_delimited_len(context), buf_len))?;
    let end = offset + encoded_len;
    if end > buf_len {
      return Err(EncodeError::insufficient_buffer(end, buf_len));
    }

    self.encode(context, &mut buf[offset..])
      .map(|write| {
        #[cfg(debug_assertions)]
        {
          crate::debug_assert_write_eq::<Self>(write, encoded_len);
        }
        end
      })
      .map_err(|e| e.update(end, buf_len))
  }
}

impl<V> Encode<Network, Repeated<LengthDelimited>> for [V]
where
  V: Encode<Network, LengthDelimited>,
{
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let mut offset = 0;
    let buf_len = buf.len();
    for v in self {
      if offset >= buf_len {
        return Err(EncodeError::insufficient_buffer(offset, buf_len));
      }
      offset += V::encode_length_delimited(v, context, &mut buf[offset..])
        .map_err(|e| e.update(self.encoded_len(context), buf_len))?;
    }

    Ok(offset)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    self.iter().map(|v| v.encoded_length_delimited_len(context)).sum()
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    let encoded_len = self.encoded_len(context);
    let varint_len = varing::encoded_u32_varint_len(encoded_len as u32);
    encoded_len + varint_len
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    let encoded_len = self.encoded_len(context);
    let buf_len = buf.len();
    let offset = varing::encode_u32_varint_to(encoded_len as u32, buf)
      .map_err(|e| EncodeError::from(e).update(self.encoded_length_delimited_len(context), buf_len))?;
    let end = offset + encoded_len;
    if end > buf_len {
      return Err(EncodeError::insufficient_buffer(end, buf_len));
    }

    self.encode(context, &mut buf[offset..])
      .map(|write| {
        #[cfg(debug_assertions)]
        {
          crate::debug_assert_write_eq::<Self>(write, encoded_len);
        }
        end
      })
      .map_err(|e| e.update(end, buf_len))
  }
}



