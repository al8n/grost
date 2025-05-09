macro_rules! repeated_impl {
  (@fixed $($size:literal::$wf:ty),+$(,)?) => {
    $(
      impl<V> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>> for [V]
      where
        V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
      {
        repeated_impl!(@encode $wf);

        fn encoded_len(&self, _: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
          self.len() * $size
        }

        repeated_impl!(@encode_length_delimited $wf);
      }

      impl<V>
        $crate::__private::PartialEncode<
          $crate::__private::flavors::Network,
          $crate::__private::flavors::network::Repeated<$wf>,
        > for [V]
      where
        V: $crate::__private::PartialEncode<crate::__private::flavors::Network, $wf>,
      {
        repeated_impl!(@partial_encode $wf);

        fn partial_encoded_len(
          &self,
          _: &<$crate::__private::flavors::Network as $crate::flavors::Flavor>::Context,
          _: &Self::Selector,
        ) -> ::core::primitive::usize {
          self.len() * $size
        }

        repeated_impl!(@partial_encode_length_delimited $wf);
      }
    )*
  };
  ($($wf:ty),+$(,)?) => {
    $(
      impl<V> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>> for [V]
      where
        V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
      {
        repeated_impl!(@encode $wf);

        fn encoded_len(&self, context: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
          self.iter().map(|v| v.encoded_length_delimited_len(context)).sum()
        }

        repeated_impl!(@encode_length_delimited $wf);
      }

      impl<V>
        $crate::__private::PartialEncode<
          $crate::__private::flavors::Network,
          $crate::__private::flavors::network::Repeated<$wf>,
        > for [V]
      where
        V: $crate::__private::PartialEncode<$crate::__private::flavors::Network, $wf>,
      {
        repeated_impl!(@partial_encode $wf);

        fn partial_encoded_len(
          &self,
          context: &<$crate::__private::flavors::Network as $crate::flavors::Flavor>::Context,
          selector: &Self::Selector,
        ) -> ::core::primitive::usize {
          self.iter().map(|v| v.partial_encoded_length_delimited_len(context, selector)).sum()
        }

        repeated_impl!(@partial_encode_length_delimited $wf);
      }
    )*
  };
  (@encode $wf:ty) => {
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

      #[cfg(debug_assertions)]
      {
        $crate::debug_assert_write_eq::<Self>(offset, <Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::encoded_len(self, context));
      }

      ::core::result::Result::Ok(offset)
    }
  };
  (@encode_length_delimited $wf:ty) => {
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
  };
  (@partial_encode $wf:ty) => {
    fn partial_encode(
      &self,
      context: &<$crate::__private::flavors::Network as $crate::flavors::Flavor>::Context,
      buf: &mut [::core::primitive::u8],
      selector: &Self::Selector,
    ) -> ::core::result::Result<::core::primitive::usize, <$crate::__private::flavors::Network as $crate::flavors::Flavor>::EncodeError>
    {
      let mut offset = 0;
      let buf_len = buf.len();
      let encoded_len = <Self as $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::partial_encoded_len(self, context, selector);
      if encoded_len > buf_len {
        return ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(encoded_len, buf_len));
      }

      for v in self {
        if offset >= buf_len {
          return ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(offset, buf_len));
        }
        offset += V::partial_encode_length_delimited(v, context, &mut buf[offset..], selector)
          .map_err(|e| e.update(<Self as $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::partial_encoded_len(self, context, selector), buf_len))?;
      }

      #[cfg(debug_assertions)]
      {
        $crate::debug_assert_write_eq::<Self>(offset, <Self as $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::partial_encoded_len(self, context, selector));
      }

      ::core::result::Result::Ok(offset)
    }
  };
  (@partial_encode_length_delimited $wf:ty) => {
    fn partial_encoded_length_delimited_len(
      &self,
      context: &<$crate::__private::flavors::Network as $crate::flavors::Flavor>::Context,
      selector: &Self::Selector,
    ) -> ::core::primitive::usize {
      let encoded_len = <Self as $crate::__private::PartialEncode<$crate::__private::flavors::Network,$crate::__private::flavors::network::Repeated<$wf>>>::partial_encoded_len(self, context, selector);
      let varint_len = varing::encoded_u32_varint_len(encoded_len as ::core::primitive::u32);
      encoded_len + varint_len
    }

    fn partial_encode_length_delimited(
      &self,
      context: &<$crate::__private::flavors::Network as $crate::flavors::Flavor>::Context,
      buf: &mut [u8],
      selector: &Self::Selector,
    ) -> ::core::result::Result<::core::primitive::usize, <$crate::__private::flavors::Network as $crate::flavors::Flavor>::EncodeError>
    {
      let encoded_len = <Self as $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::partial_encoded_len(self, context, selector);
      let buf_len = buf.len();
      let offset = varing::encode_u32_varint_to(encoded_len as ::core::primitive::u32, buf)
        .map_err(|e| $crate::__private::flavors::network::EncodeError::from_varint_error(e)
          .update(<Self as $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::partial_encoded_length_delimited_len(self, context, selector), buf_len))?;
      let end = offset + encoded_len;
      if end > buf_len {
        return ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(end, buf_len));
      }

      <Self as $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>>>::partial_encode(self, context, &mut buf[offset..], selector)
        .map(|write| {
          #[cfg(debug_assertions)]
          {
            $crate::debug_assert_write_eq::<Self>(write, encoded_len);
          }
          end
        })
        .map_err(|e| e.update(end, buf_len))
    }
  };
}

mod map;
mod slice;
