macro_rules! repeated_impl {
  (@fixed $ty:ty: $($wf:ty),+$(,)?) => {
    $(
      impl<V> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>> for $ty
      where
        V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
      {
        repeated_impl!(@encode $wf);

        fn encoded_len(&self, _: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
          self.len() * <$wf>::SIZE
        }

        repeated_impl!(@encode_length_delimited $wf);
      }

      impl<V>
        $crate::__private::PartialEncode<
          $crate::__private::flavors::Network,
          $crate::__private::flavors::network::Repeated<$wf>,
        > for $ty
      where
        V: $crate::__private::PartialEncode<crate::__private::flavors::Network, $wf>,
      {
        repeated_impl!(@partial_encode $wf);

        fn partial_encoded_len(
          &self,
          _: &<$crate::__private::flavors::Network as $crate::flavors::Flavor>::Context,
          selector: &Self::Selector,
        ) -> ::core::primitive::usize {
          if $crate::__private::selection::Selector::is_empty(selector) {
            return 0;
          }

          self.len() * <$wf>::SIZE
        }

        repeated_impl!(@partial_encode_length_delimited $wf);
      }
    )*
  };
  (@constn_fixed $ty:ty: $($wf:ty),+$(,)?) => {
    $(
      impl<V, const N: usize> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>> for $ty
      where
        V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
      {
        repeated_impl!(@encode $wf);

        fn encoded_len(&self, _: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
          self.len() * <$wf>::SIZE
        }

        repeated_impl!(@encode_length_delimited $wf);
      }

      impl<V, const N: usize>
        $crate::__private::PartialEncode<
          $crate::__private::flavors::Network,
          $crate::__private::flavors::network::Repeated<$wf>,
        > for $ty
      where
        V: $crate::__private::PartialEncode<crate::__private::flavors::Network, $wf>,
      {
        repeated_impl!(@partial_encode $wf);

        fn partial_encoded_len(
          &self,
          _: &<$crate::__private::flavors::Network as $crate::flavors::Flavor>::Context,
          selector: &Self::Selector,
        ) -> ::core::primitive::usize {
          if $crate::__private::selection::Selector::is_empty(selector) {
            return 0;
          }

          self.len() * <$wf>::SIZE
        }

        repeated_impl!(@partial_encode_length_delimited $wf);
      }
    )*
  };
  (@tinyvec_fixed $trait:ident:$ty:ty: $($wf:ty),+$(,)?) => {
    $(
      impl<V, A> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>> for $ty
      where
        V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
        A: $trait<Item = V>,
      {
        repeated_impl!(@encode $wf);

        fn encoded_len(&self, _: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
          self.len() * <$wf>::SIZE
        }

        repeated_impl!(@encode_length_delimited $wf);
      }

      impl<V, A>
        $crate::__private::PartialEncode<
          $crate::__private::flavors::Network,
          $crate::__private::flavors::network::Repeated<$wf>,
        > for $ty
      where
        V: $crate::__private::PartialEncode<crate::__private::flavors::Network, $wf>,
        A: $trait<Item = V>,
      {
        repeated_impl!(@partial_encode $wf);

        fn partial_encoded_len(
          &self,
          _: &<$crate::__private::flavors::Network as $crate::flavors::Flavor>::Context,
          selector: &Self::Selector,
        ) -> ::core::primitive::usize {
          if $crate::__private::selection::Selector::is_empty(selector) {
            return 0;
          }

          self.len() * <$wf>::SIZE
        }

        repeated_impl!(@partial_encode_length_delimited $wf);
      }
    )*
  };
  ($ty:ty: $($wf:ty),+$(,)?) => {
    $(
      impl<V> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>> for $ty
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
        > for $ty
      where
        V: $crate::__private::PartialEncode<$crate::__private::flavors::Network, $wf>,
      {
        repeated_impl!(@partial_encode $wf);

        fn partial_encoded_len(
          &self,
          context: &<$crate::__private::flavors::Network as $crate::flavors::Flavor>::Context,
          selector: &Self::Selector,
        ) -> ::core::primitive::usize {
          if $crate::__private::selection::Selector::is_empty(selector) {
            return 0;
          }

          self.iter().map(|v| v.partial_encoded_length_delimited_len(context, selector)).sum()
        }

        repeated_impl!(@partial_encode_length_delimited $wf);
      }
    )*
  };
  (@tinyvec $trait:ident:$ty:ty: $($wf:ty),+$(,)?) => {
    $(
      impl<V, A> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>> for $ty
      where
        V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
        A: $trait<Item = V>,
      {
        repeated_impl!(@encode $wf);

        fn encoded_len(&self, context: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
          self.iter().map(|v| v.encoded_length_delimited_len(context)).sum()
        }

        repeated_impl!(@encode_length_delimited $wf);
      }

      impl<V, A>
        $crate::__private::PartialEncode<
          $crate::__private::flavors::Network,
          $crate::__private::flavors::network::Repeated<$wf>,
        > for $ty
      where
        V: $crate::__private::PartialEncode<$crate::__private::flavors::Network, $wf>,
        A: $trait<Item = V>,
      {
        repeated_impl!(@partial_encode $wf);

        fn partial_encoded_len(
          &self,
          context: &<$crate::__private::flavors::Network as $crate::flavors::Flavor>::Context,
          selector: &Self::Selector,
        ) -> ::core::primitive::usize {
          if $crate::__private::selection::Selector::is_empty(selector) {
            return 0;
          }

          self.iter().map(|v| v.partial_encoded_length_delimited_len(context, selector)).sum()
        }

        repeated_impl!(@partial_encode_length_delimited $wf);
      }
    )*
  };
  (@constn $ty:ty: $($wf:ty),+$(,)?) => {
    $(
      impl<V, const N: usize> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$wf>> for $ty
      where
        V: $crate::__private::Encode<$crate::__private::flavors::Network, $wf>,
      {
        repeated_impl!(@encode $wf);

        fn encoded_len(&self, context: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
          self.iter().map(|v| v.encoded_length_delimited_len(context)).sum()
        }

        repeated_impl!(@encode_length_delimited $wf);
      }

      impl<V, const N: usize>
        $crate::__private::PartialEncode<
          $crate::__private::flavors::Network,
          $crate::__private::flavors::network::Repeated<$wf>,
        > for $ty
      where
        V: $crate::__private::PartialEncode<$crate::__private::flavors::Network, $wf>,
      {
        repeated_impl!(@partial_encode $wf);

        fn partial_encoded_len(
          &self,
          context: &<$crate::__private::flavors::Network as $crate::flavors::Flavor>::Context,
          selector: &Self::Selector,
        ) -> ::core::primitive::usize {
          if $crate::__private::selection::Selector::is_empty(selector) {
            return 0;
          }

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
      if $crate::selector::Selector::is_empty(selector) {
        return ::core::result::Result::Ok(0);
      }

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
      if $crate::selector::Selector::is_empty(selector) {
        return 0;
      }

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
      if $crate::selector::Selector::is_empty(selector) {
        return ::core::result::Result::Ok(0);
      }

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

macro_rules! impl_selectable_for_repeated {
  ($ty:ty as $wf:ty) => {
    impl<V>
      $crate::__private::selection::Selectable<
        $crate::__private::flavors::Network,
        $crate::__private::flavors::network::Repeated<$wf>,
      > for $ty
    where
      V: $crate::__private::selection::Selectable<$crate::__private::flavors::Network, $wf>,
    {
      type Selector = V::Selector;
    }
  };
  (@constn $ty:ty as $wf:ty) => {
    impl<V, const N: usize>
      $crate::__private::selection::Selectable<
        $crate::__private::flavors::Network,
        $crate::__private::flavors::network::Repeated<$wf>,
      > for $ty
    where
      V: $crate::__private::selection::Selectable<$crate::__private::flavors::Network, $wf>,
    {
      type Selector = V::Selector;
    }
  };
  (@tinyvec $trait:ident:$ty:ty as $wf:ty) => {
    impl<V, A>
      $crate::__private::selection::Selectable<
        $crate::__private::flavors::Network,
        $crate::__private::flavors::network::Repeated<$wf>,
      > for $ty
    where
      V: $crate::__private::selection::Selectable<$crate::__private::flavors::Network, $wf>,
      A: $trait<Item = V>,
    {
      type Selector = V::Selector;
    }
  };
}

macro_rules! impl_selectable {
  ($ty:ty: [ $($wf:ty),+$(,)? ]) => {
    $(
      impl_selectable_for_repeated!($ty as $wf);
    )*
  };
  (@constn $ty:ty: [ $($wf:ty),+$(,)? ]) => {
    $(
      impl_selectable_for_repeated!(@constn $ty as $wf);
    )*
  };
  (@tinyvec $trait:ident:$ty:ty: [ $($wf:ty),+$(,)? ]) => {
    $(
      impl_selectable_for_repeated!(@tinyvec $trait:$ty as $wf);
    )*
  };
}

macro_rules! repeated {
  ($ty:ty) => {
    repeated_impl!(
      @fixed $ty:
      $crate::__private::flavors::network::Fixed8,
      $crate::__private::flavors::network::Fixed16,
      $crate::__private::flavors::network::Fixed32,
      $crate::__private::flavors::network::Fixed64,
      $crate::__private::flavors::network::Fixed128,
    );

    repeated_impl!($ty: $crate::__private::flavors::network::LengthDelimited, $crate::__private::flavors::network::Varint,);

    impl_selectable!(
      $ty: [
        $crate::__private::flavors::network::Fixed8,
        $crate::__private::flavors::network::Fixed16,
        $crate::__private::flavors::network::Fixed32,
        $crate::__private::flavors::network::Fixed64,
        $crate::__private::flavors::network::Fixed128,
        $crate::__private::flavors::network::Varint,
        $crate::__private::flavors::network::LengthDelimited,
      ]
    );

    impl<V, W: ?::core::marker::Sized, const I: ::core::primitive::u32> $crate::__private::selection::Selectable<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$crate::__private::flavors::network::Stream<W, I>>> for $ty
    where
      V: $crate::__private::selection::Selectable<$crate::__private::flavors::Network, W>,
    {
      type Selector = V::Selector;
    }

    impl<V, W, const I: u32> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$crate::__private::flavors::network::Stream<W, I>>> for $ty
    where
      $crate::__private::flavors::network::Repeated<$crate::__private::flavors::network::Stream<W, I>>: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      V: $crate::__private::Encode<$crate::__private::flavors::Network, W>,
      W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
    {
      repeated!(@encode_impl);
    }

    impl<V, W, const I: u32> $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$crate::__private::flavors::network::Stream<W, I>>> for $ty
    where
      $crate::__private::flavors::network::Repeated<$crate::__private::flavors::network::Stream<W, I>>: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      V: $crate::__private::PartialEncode<$crate::__private::flavors::Network, W>,
      W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
    {
      repeated!(@partial_encode_impl);
    }
  };
  (@tinyvec $trait:ident:$ty:ty $([ $(const $g:ident usize),+$(,)? ])?) => {
    repeated_impl!(
      @tinyvec_fixed $trait:$ty:
      $crate::__private::flavors::network::Fixed8,
      $crate::__private::flavors::network::Fixed16,
      $crate::__private::flavors::network::Fixed32,
      $crate::__private::flavors::network::Fixed64,
      $crate::__private::flavors::network::Fixed128,
    );

    repeated_impl!(@tinyvec $trait:$ty: $crate::__private::flavors::network::LengthDelimited, $crate::__private::flavors::network::Varint,);

    impl_selectable!(
      @tinyvec
      $trait:$ty: [
        $crate::__private::flavors::network::Fixed8,
        $crate::__private::flavors::network::Fixed16,
        $crate::__private::flavors::network::Fixed32,
        $crate::__private::flavors::network::Fixed64,
        $crate::__private::flavors::network::Fixed128,
        $crate::__private::flavors::network::Varint,
        $crate::__private::flavors::network::LengthDelimited,
      ]
    );

    impl<V, A, W: ?::core::marker::Sized, const I: ::core::primitive::u32> $crate::__private::selection::Selectable<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$crate::__private::flavors::network::Stream<W, I>>> for $ty
    where
      V: $crate::__private::selection::Selectable<$crate::__private::flavors::Network, W>,
      A: $trait<Item = V>,
    {
      type Selector = V::Selector;
    }

    impl<V, A, W, const I: u32> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$crate::__private::flavors::network::Stream<W, I>>> for $ty
    where
      $crate::__private::flavors::network::Repeated<$crate::__private::flavors::network::Stream<W, I>>: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      V: $crate::__private::Encode<$crate::__private::flavors::Network, W>,
      W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      A: $trait<Item = V>,
    {
      repeated!(@encode_impl);
    }

    impl<V, A, W, const I: u32,> $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$crate::__private::flavors::network::Stream<W, I>>> for $ty
    where
      $crate::__private::flavors::network::Repeated<$crate::__private::flavors::network::Stream<W, I>>: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      V: $crate::__private::PartialEncode<$crate::__private::flavors::Network, W>,
      W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      A: $trait<Item = V>,
    {
      repeated!(@partial_encode_impl);
    }
  };
  (@constn $ty:ty $([ $(const $g:ident usize),+$(,)? ])?) => {
    repeated_impl!(
      @constn_fixed $ty:
      $crate::__private::flavors::network::Fixed8,
      $crate::__private::flavors::network::Fixed16,
      $crate::__private::flavors::network::Fixed32,
      $crate::__private::flavors::network::Fixed64,
      $crate::__private::flavors::network::Fixed128,
    );

    repeated_impl!(@constn $ty: $crate::__private::flavors::network::LengthDelimited, $crate::__private::flavors::network::Varint,);

    impl_selectable!(
      @constn
      $ty: [
        $crate::__private::flavors::network::Fixed8,
        $crate::__private::flavors::network::Fixed16,
        $crate::__private::flavors::network::Fixed32,
        $crate::__private::flavors::network::Fixed64,
        $crate::__private::flavors::network::Fixed128,
        $crate::__private::flavors::network::Varint,
        $crate::__private::flavors::network::LengthDelimited,
      ]
    );

    impl<V, W: ?::core::marker::Sized, const I: ::core::primitive::u32, const N: usize> $crate::__private::selection::Selectable<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$crate::__private::flavors::network::Stream<W, I>>> for $ty
    where
      V: $crate::__private::selection::Selectable<$crate::__private::flavors::Network, W>,
    {
      type Selector = V::Selector;
    }

    impl<V, W, const I: u32, const N: usize> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$crate::__private::flavors::network::Stream<W, I>>> for $ty
    where
      $crate::__private::flavors::network::Repeated<$crate::__private::flavors::network::Stream<W, I>>: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      V: $crate::__private::Encode<$crate::__private::flavors::Network, W>,
      W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
    {
      repeated!(@encode_impl);
    }

    impl<V, W, const I: u32, const N: usize> $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Repeated<$crate::__private::flavors::network::Stream<W, I>>> for $ty
    where
      $crate::__private::flavors::network::Repeated<$crate::__private::flavors::network::Stream<W, I>>: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      V: $crate::__private::PartialEncode<$crate::__private::flavors::Network, W>,
      W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
    {
      repeated!(@partial_encode_impl);
    }
  };
  (@encode_impl) => {
    fn encode(&self, context: &$crate::__private::flavors::network::Context, buf: &mut [u8]) -> Result<usize, $crate::__private::flavors::network::EncodeError> {
      let encoded_identifier = $crate::__private::flavors::network::Identifier::try_from_u32(I)
        .map_err($crate::__private::flavors::network::EncodeError::from)?
        .encode();
      let encoded_identifier_len = encoded_identifier.len();

      let mut offset = 0;
      let buf_len = buf.len();

      for item in self {
        if offset + encoded_identifier_len >= buf_len {
          return Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(
            self.encoded_len(context),
            buf_len,
          ));
        }
        buf[offset..offset + encoded_identifier_len].copy_from_slice(&encoded_identifier);
        offset += encoded_identifier_len;

        if offset >= buf_len {
          return Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(
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

    fn encoded_len(&self, context: &$crate::__private::flavors::network::Context) -> usize {
      varing::encoded_u32_varint_len(I) * self.len()
        + self
          .iter()
          .map(|v| v.encoded_length_delimited_len(context))
          .sum::<usize>()
    }

    fn encoded_length_delimited_len(&self, context: &$crate::__private::flavors::network::Context) -> usize {
      self.encoded_len(context)
    }

    fn encode_length_delimited(
      &self,
      context: &$crate::__private::flavors::network::Context,
      buf: &mut [u8],
    ) -> Result<usize, $crate::__private::flavors::network::EncodeError> {
      self.encode(context, buf)
    }
  };
  (@partial_encode_impl) => {
    fn partial_encode(
      &self,
      context: &$crate::__private::flavors::network::Context,
      buf: &mut [u8],
      selector: &Self::Selector,
    ) -> Result<usize, $crate::__private::flavors::network::EncodeError> {
      if $crate::__private::selection::Selector::is_empty(selector) {
        return Ok(0);
      }

      let encoded_identifier = $crate::__private::flavors::network::Identifier::try_from_u32(I)
        .map_err($crate::__private::flavors::network::EncodeError::from)?
        .encode();
      let encoded_identifier_len = encoded_identifier.len();

      let mut offset = 0;
      let buf_len = buf.len();

      for item in self {
        if offset + encoded_identifier_len >= buf_len {
          return Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(
            self.partial_encoded_len(context, selector),
            buf_len,
          ));
        }
        buf[offset..offset + encoded_identifier_len].copy_from_slice(&encoded_identifier);
        offset += encoded_identifier_len;

        if offset >= buf_len {
          return Err($crate::__private::flavors::network::EncodeError::insufficient_buffer(
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

    fn partial_encoded_len(&self, context: &$crate::__private::flavors::network::Context, selector: &Self::Selector) -> usize {
      if $crate::__private::selection::Selector::is_empty(selector) {
        return 0;
      }

      varing::encoded_u32_varint_len(I) * self.len()
        + self
          .iter()
          .map(|v| v.partial_encoded_length_delimited_len(context, selector))
          .sum::<usize>()
    }

    fn partial_encoded_length_delimited_len(
      &self,
      context: &$crate::__private::flavors::network::Context,
      selector: &Self::Selector,
    ) -> usize {
      self.partial_encoded_len(context, selector)
    }

    fn partial_encode_length_delimited(
      &self,
      context: &$crate::__private::flavors::network::Context,
      buf: &mut [u8],
      selector: &Self::Selector,
    ) -> Result<usize, $crate::__private::flavors::network::EncodeError> {
      self.partial_encode(context, buf, selector)
    }
  };
}

// mod map;

// mod arrayvec;
// mod repeated_decoder;
// mod slice;
// mod smallvec;
// mod tinyvec;

// #[cfg(any(feature = "std", feature = "alloc"))]
// mod vec;
