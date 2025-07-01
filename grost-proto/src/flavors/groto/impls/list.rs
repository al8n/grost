use crate::{
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, WireFormat,
    groto::{Borrowed, Context, Error, LengthDelimited, Packed, PackedDecoder, WireType},
  },
};

mod array;

macro_rules! encode {
  (@encode_methods($ty:ty)) => {
    fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
      let mut offset = 0;
      let buf_len = buf.len();
      let body_size = match W::WIRE_TYPE {
        WireType::Varint | WireType::LengthDelimited => {
          self.iter().map(|v| v.encoded_len(context)).sum()
        }
        WireType::Fixed8 => self.len(),
        WireType::Fixed16 => self.len() * 2,
        WireType::Fixed32 => self.len() * 4,
        WireType::Fixed64 => self.len() * 8,
        WireType::Fixed128 => self.len() * 16,
        WireType::Fixed256 => self.len() * 32,
      };
      let body_size_len = varing::encoded_u32_varint_len(body_size as u32);
      let encoded_len = body_size_len + body_size;

      if buf_len < encoded_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }

      offset += varing::encode_u32_varint_to(body_size as u32, buf)
        .map_err(|e| Error::from_varint_encode_error(e).update(encoded_len, buf_len))?;

      for value in self.iter() {
        if offset >= buf_len {
          return Err(Error::insufficient_buffer(encoded_len, buf_len));
        }

        offset += value
          .encode(context, &mut buf[offset..])
          .map_err(|e| e.update(encoded_len, buf_len))?;
      }
      Ok(offset)
    }

    fn encoded_len(&self, context: &Context) -> usize {
      let len = match W::WIRE_TYPE {
        WireType::Varint | WireType::LengthDelimited => {
          self.iter().map(|v| v.encoded_len(context)).sum()
        }
        WireType::Fixed8 => self.len(),
        WireType::Fixed16 => self.len() * 2,
        WireType::Fixed32 => self.len() * 4,
        WireType::Fixed64 => self.len() * 8,
        WireType::Fixed128 => self.len() * 16,
        WireType::Fixed256 => self.len() * 32,
      };
      varing::encoded_u32_varint_len(len as u32) + len
    }
  };
  (@partial_encode_methods($ty:ty)) => {
    fn partial_encode(
      &self,
      context: &<Groto as crate::flavors::Flavor>::Context,
      buf: &mut [u8],
      selector: &Self::Selector,
    ) -> Result<usize, <Groto as crate::flavors::Flavor>::Error> {
      let mut offset = 0;
      let buf_len = buf.len();
      let body_size = self
        .iter()
        .map(|v| v.partial_encoded_len(context, selector))
        .sum::<usize>();
      let body_size_len = varing::encoded_u32_varint_len(body_size as u32);
      let encoded_len = body_size_len + body_size;

      if buf_len < encoded_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }

      offset += varing::encode_u32_varint_to(body_size as u32, buf)
        .map_err(|e| Error::from_varint_encode_error(e).update(encoded_len, buf_len))?;

      for value in self.iter() {
        if offset >= buf_len {
          return Err(Error::insufficient_buffer(encoded_len, buf_len));
        }

        offset += value
          .partial_encode(context, &mut buf[offset..], selector)
          .map_err(|e| e.update(encoded_len, buf_len))?;
      }
      Ok(offset)
    }

    fn partial_encoded_len(
      &self,
      context: &<Groto as crate::flavors::Flavor>::Context,
      selector: &Self::Selector,
    ) -> usize {
      let len = self
        .iter()
        .map(|v| v.partial_encoded_len(context, selector))
        .sum::<usize>();
      varing::encoded_u32_varint_len(len as u32) + len
    }
  };
}

macro_rules! decode {
  () => {
    fn decode(
      context: &'de <Groto as $crate::flavors::Flavor>::Context,
      src: B,
    ) -> Result<(usize, PackedDecoder<'de, T, B, UB, W>), <Groto as $crate::flavors::Flavor>::Error>
    where
      PackedDecoder<'de, T, B, UB, W>: Sized,
      B: $crate::buffer::ReadBuf,
      UB: $crate::buffer::Buffer<<Groto as $crate::flavors::Flavor>::Unknown<B>> + 'de,
    {
      let buf = src.as_bytes();
      let buf_len = buf.len();
      if buf_len == 0 {
        return Err(Error::buffer_underflow());
      }

      let (len, data_len) = varing::decode_u32_varint(buf)?;
      let total = len + data_len as usize;
      if total > buf_len {
        return Err(Error::buffer_underflow());
      }

      Ok((total, PackedDecoder::new(context, src, len)))
    }
  };
}

macro_rules! list {
  (@flatten_state $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )? > $crate::__private::convert::State<
        $crate::__private::convert::Flattened
      > for $ty {
        type Output = $ty;
      }

      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::State<$crate::__private::convert::Flattened<$crate::__private::convert::Innermost>> for $ty
      where
        T: $crate::__private::convert::State<$crate::__private::convert::Flattened<$crate::__private::convert::Innermost>>,
      {
        type Output = T::Output;
      }
    )*
  };
  (@partial_state $($(:< $($tg:ident: $t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])? => $output:ty),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::State<$crate::__private::convert::Partial<$crate::__private::flavors::Groto>> for $ty
      where
        T: $crate::__private::convert::State<$crate::__private::convert::Partial<$crate::__private::flavors::Groto>>,
        T::Output: Sized,
      {
        type Output = $output;
      }
    )*
  };
  (@partial_ref_state(bytes) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<'a, __GROST_READ_BUF__, __GROST_BUFFER__, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::State<$crate::__private::convert::PartialRef<'a, __GROST_READ_BUF__, __GROST_BUFFER__, LengthDelimited, $crate::__private::flavors::Groto>> for $ty
      {
        type Output = $crate::__private::decode::BytesSlice<__GROST_READ_BUF__>;
      }
    )*
  };
  (@partial_ref_state(packed) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<'a, T, W, __GROST_READ_BUF__, __GROST_BUFFER__, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::State<
        $crate::__private::convert::PartialRef<
          'a,
          __GROST_READ_BUF__,
          __GROST_BUFFER__,
          Packed<W>,
          $crate::__private::flavors::Groto,
        >
      > for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        type Output = $crate::__private::flavors::groto::PackedDecoder<'a, T, __GROST_READ_BUF__, __GROST_BUFFER__, W>;
      }
    )*
  };
  (@partial_ref_state(borrow) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<'a, T, W, __GROST_READ_BUF__, __GROST_BUFFER__, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::State<
        $crate::__private::convert::PartialRef<
          'a,
          __GROST_READ_BUF__,
          __GROST_BUFFER__,
          Borrowed<'a, Packed<W>>,
          $crate::__private::flavors::Groto,
        >
      > for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        type Output = $crate::__private::flavors::groto::PackedDecoder<'a, T, __GROST_READ_BUF__, __GROST_BUFFER__, W>;
      }
    )*
  };
  (@default_wire_format $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::flavors::DefaultWireFormat<$crate::__private::flavors::Groto> for $ty
      where
        T: $crate::__private::flavors::DefaultWireFormat<$crate::__private::flavors::Groto>,
      {
        type Format = $crate::__private::flavors::groto::Packed<T::Format>;
      }
    )*
  };
  (@identity_transform $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, W, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::Transform<Self, Self, W, $crate::__private::flavors::Groto> for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        fn transform(input: Self) -> ::core::result::Result<Self, $crate::__private::flavors::groto::Error> {
          ::core::result::Result::Ok(input)
        }
      }

      impl<T, W, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::Transform<Self, ::core::option::Option<Self>, W, $crate::__private::flavors::Groto> for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        fn transform(input: Self) -> ::core::result::Result<::core::option::Option<Self>, $crate::__private::flavors::groto::Error> {
          ::core::result::Result::Ok(::core::option::Option::Some(input))
        }
      }

      impl<T, W, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::Transform<$ty, ::core::option::Option<$ty>, W, $crate::__private::flavors::Groto> for ::core::option::Option<$ty>
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        fn transform(input: $ty) -> ::core::result::Result<Self, $crate::__private::flavors::groto::Error> {
          ::core::result::Result::Ok(::core::option::Option::Some(input))
        }
      }
    )*
  };
  (@identity_partial_transform(bytes) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<$($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::PartialTransform<Self, ::core::option::Option<Self>, $crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto> for $ty
      {
        fn partial_transform(input: Self, selector: &Self::Selector) -> ::core::result::Result<::core::option::Option<Self>, $crate::__private::flavors::groto::Error> {
          if <Self::Selector as $crate::__private::selection::Selector<$crate::__private::flavors::Groto>>::is_empty(selector) {
            return ::core::result::Result::Ok(::core::option::Option::None);
          }

          ::core::result::Result::Ok(::core::option::Option::Some(input))
        }
      }
    )*
  };
  (@transform(from_bytes) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<B: $crate::__private::buffer::ReadBuf, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::Transform<$crate::__private::decode::BytesSlice<B>, Self, $crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto> for $ty
      {
        fn transform(input: $crate::__private::decode::BytesSlice<B>) -> ::core::result::Result<Self, $crate::__private::flavors::groto::Error> {
          ::core::result::Result::Ok(Self::from(input.as_slice()))
        }
      }
    )*
  };
  (@transform(try_from_bytes) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])? { $try_from:expr }),+$(,)?) => {
    $(
      impl<B: $crate::__private::buffer::ReadBuf, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::Transform<$crate::__private::decode::BytesSlice<B>, Self, $crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto> for $ty
      {
        fn transform(input: $crate::__private::decode::BytesSlice<B>) -> ::core::result::Result<Self, $crate::__private::flavors::groto::Error> {
          ($try_from)(input.as_slice())
        }
      }
    )*
  };
  (@transform(packed) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<'a, T, W, TW, B, UB, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::Transform<$crate::__private::flavors::groto::PackedDecoder<'a, T, B, UB, TW>, $ty, W, $crate::__private::flavors::Groto> for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        TW: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        T: $crate::__private::convert::State<$crate::__private::convert::PartialRef<'a, B, UB, TW, $crate::__private::flavors::Groto>>
          + $crate::__private::decode::Decode<'a, $crate::__private::flavors::Groto, TW, T::Output, B, UB>
          + $crate::__private::convert::Transform<T::Output, T, TW, $crate::__private::flavors::Groto>
          + 'a,
        T::Output: Sized,
        UB: $crate::__private::buffer::Buffer<$crate::__private::flavors::groto::Unknown<B>> + 'a,
        B: $crate::__private::buffer::ReadBuf + 'a,
      {
        fn transform(input: $crate::__private::flavors::groto::PackedDecoder<'a, T, B, UB, TW>) -> ::core::result::Result<Self, <$crate::__private::flavors::Groto as $crate::__private::flavors::Flavor>::Error>
        where
          Self: Sized
        {
          input.into_iter()
            .map(|res| {
              res.and_then(|(_, inp)| T::transform(inp))
            })
            .collect()
        }
      }
    )*
  };
  (@partial_transform(from_bytes) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<B: $crate::__private::buffer::ReadBuf, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::PartialTransform<$crate::__private::decode::BytesSlice<B>, ::core::option::Option<Self>, $crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto> for $ty
      {
        fn partial_transform(input: $crate::__private::decode::BytesSlice<B>, selector: &Self::Selector) -> ::core::result::Result<::core::option::Option<Self>, $crate::__private::flavors::groto::Error> {
          if <Self::Selector as $crate::__private::selection::Selector<$crate::__private::flavors::Groto>>::is_empty(selector) {
            return ::core::result::Result::Ok(::core::option::Option::None);
          }

          ::core::result::Result::Ok(::core::option::Option::Some(Self::from(input.as_slice())))
        }
      }
    )*
  };
  (@partial_transform(try_from_bytes) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])? { $try_from:expr }),+$(,)?) => {
    $(
      impl<B: $crate::__private::buffer::ReadBuf, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::PartialTransform<$crate::__private::decode::BytesSlice<B>, ::core::option::Option<Self>, $crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto> for $ty
      {
        fn partial_transform(input: $crate::__private::decode::BytesSlice<B>, selector: &Self::Selector) -> ::core::result::Result<::core::option::Option<Self>, $crate::__private::flavors::groto::Error> {
          if <Self::Selector as $crate::__private::selection::Selector<$crate::__private::flavors::Groto>>::is_empty(selector) {
            return ::core::result::Result::Ok(::core::option::Option::None);
          }

          ($try_from)(input.as_slice())
            .map(::core::option::Option::Some)
            .map_err(|e| $crate::__private::flavors::groto::Error::from(e))
        }
      }
    )*
  };
  (@partial_transform(identity) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, W, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::PartialTransform<
        Self,
        ::core::option::Option<Self>,
        $crate::__private::flavors::groto::Packed<W>,
        $crate::__private::flavors::Groto,
      > for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
        T: $crate::__private::convert::PartialTransform<
            T,
            ::core::option::Option<<T as $crate::__private::convert::State<$crate::__private::convert::Flattened>>::Output>,
            W,
            $crate::__private::flavors::Groto,
          >
          + $crate::__private::convert::State<$crate::__private::convert::Flattened>
          + $crate::__private::convert::State<$crate::__private::convert::Partial<$crate::__private::flavors::Groto>>
          + $crate::__private::selection::Selectable<$crate::__private::flavors::Groto>
          + $crate::__private::convert::Transform<
              <T as $crate::__private::convert::State<$crate::__private::convert::Flattened>>::Output,
              T,
              W,
              $crate::__private::flavors::Groto
            >,
        <T as $crate::__private::State<$crate::__private::convert::Partial<$crate::__private::flavors::Groto>>>::Output: Sized +  $crate::__private::selection::Selectable<$crate::__private::flavors::Groto, Selector = T::Selector>,
        <T as $crate::__private::convert::State<$crate::__private::convert::Flattened>>::Output: Sized + $crate::__private::selection::Selectable<$crate::__private::flavors::Groto, Selector = T::Selector>,
      {
        fn partial_transform(
          input: Self,
          selector: &T::Selector,
        ) -> ::core::result::Result<::core::option::Option<Self>, <$crate::__private::flavors::Groto as $crate::__private::flavors::Flavor>::Error>
        {
          if $crate::__private::selection::Selector::is_empty(selector) {
            return ::core::result::Result::Ok(::core::option::Option::None);
          }

          input.into_iter()
            .filter_map(|res| {
              match T::partial_transform(res, selector) {
                ::core::result::Result::Ok(val) => {
                  val.and_then(|val| if $crate::__private::selection::Selectable::is_empty(&val) {
                    ::core::option::Option::None
                  } else {
                    ::core::option::Option::Some(<T as $crate::__private::convert::Transform<
                      <T as $crate::__private::convert::State<$crate::__private::convert::Flattened>>::Output,
                      T,
                      W,
                      $crate::__private::flavors::Groto
                    >>::transform(val))
                  })
                },
                ::core::result::Result::Err(e) => ::core::option::Option::Some(Err(e)),
              }
            })
            .collect::<::core::result::Result<$ty, _>>()
            .map(|val| if val.is_empty() {
              ::core::option::Option::None
            } else {
              ::core::option::Option::Some(val)
            })
        }
      }
    )*
  };
  (@partial_transform(packed) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<'a, T, W, TW, B, UB, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::PartialTransform<PackedDecoder<'a, T, B, UB, TW>, ::core::option::Option<Self>, Packed<W>, $crate::__private::flavors::Groto> for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        TW: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        T: $crate::__private::convert::State<$crate::__private::convert::PartialRef<'a, B, UB, TW, $crate::__private::flavors::Groto>>
          + $crate::__private::convert::State<$crate::__private::convert::Flattened>
          + $crate::__private::selection::Selectable<$crate::__private::flavors::Groto>
          + $crate::__private::decode::Decode<'a,
              $crate::__private::flavors::Groto,
              TW,
              <T as $crate::__private::convert::State<
                $crate::__private::convert::PartialRef<
                  'a,
                  B,
                  UB,
                  TW,
                  $crate::__private::flavors::Groto
                >
              >>::Output,
              B,
              UB,
            >
          + $crate::__private::convert::PartialTransform<
              <T as $crate::__private::convert::State<
                $crate::__private::convert::PartialRef<
                  'a,
                  B,
                  UB,
                  TW,
                  $crate::__private::flavors::Groto
                >
              >>::Output,
              ::core::option::Option<<T as $crate::__private::convert::State<$crate::__private::convert::Flattened>>::Output>,
              TW,
              $crate::__private::flavors::Groto
            >
          + $crate::__private::convert::Transform<
              <T as $crate::__private::convert::State<$crate::__private::convert::Flattened>>::Output,
              T,
              TW,
              $crate::__private::flavors::Groto
            >
          + 'a,
        <T as $crate::__private::convert::State<$crate::__private::convert::PartialRef<'a, B, UB, TW, $crate::__private::flavors::Groto>>>::Output: Sized + $crate::__private::selection::Selectable<$crate::__private::flavors::Groto, Selector = T::Selector>,
        <T as $crate::__private::convert::State<$crate::__private::convert::Flattened>>::Output: Sized + $crate::__private::selection::Selectable<$crate::__private::flavors::Groto, Selector = T::Selector>,
        B: $crate::__private::buffer::ReadBuf + 'a,
        UB: $crate::__private::buffer::Buffer<$crate::__private::flavors::groto::Unknown<B>> + 'a,
      {
        fn partial_transform(input: PackedDecoder<'a, T, B, UB, TW>, selector: &Self::Selector) -> ::core::result::Result<::core::option::Option<Self>, <$crate::__private::flavors::Groto as $crate::__private::flavors::Flavor>::Error> {
          if $crate::__private::selection::Selector::is_empty(selector) {
            return ::core::result::Result::Ok(::core::option::Option::None);
          }

          input.into_iter()
            .filter_map(|res| {
              match res.and_then(|(_, inp)| <
                T as $crate::__private::convert::PartialTransform<<T as $crate::__private::convert::State<$crate::__private::convert::PartialRef<'a, B, UB, TW, $crate::__private::flavors::Groto>>>::Output, ::core::option::Option<<T as $crate::__private::convert::State<$crate::__private::convert::Flattened>>::Output>, TW, $crate::__private::flavors::Groto>
              >::partial_transform(inp, selector)) {
                ::core::result::Result::Ok(val) => match val {
                  ::core::option::Option::Some(val) => ::core::option::Option::Some(
                    <T as $crate::__private::convert::Transform<
                      <T as $crate::__private::convert::State<$crate::__private::convert::Flattened>>::Output,
                      T,
                      TW,
                      $crate::__private::flavors::Groto
                    >>::transform(val)
                  ),
                  ::core::option::Option::None => ::core::option::Option::None,
                },
                ::core::result::Result::Err(e) => ::core::option::Option::Some(::core::result::Result::Err(e)),
              }
            })
            .collect::<::core::result::Result<$ty, _>>()
            .map(|val| if val.is_empty() {
              ::core::option::Option::None
            } else {
              ::core::option::Option::Some(val)
            })
        }
      }
    )*
  };
  (@selectable $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::selection::Selectable<$crate::__private::flavors::Groto> for $ty
      where
        T: $crate::__private::selection::Selectable<$crate::__private::flavors::Groto>,
      {
        type Selector = T::Selector;

        fn is_empty(&self) -> bool {
          let this: &[T] = self.as_ref();
          this.is_empty()
        }
      }
    )*
  };
  (@decode_to_packed_decoder $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<'de, T, W, B, UB, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::decode::Decode<'de, $crate::__private::flavors::Groto, Packed<W>, PackedDecoder<'de, T, B, UB, W>, B, UB> for $ty
      where
        T: $crate::__private::convert::State<$crate::__private::convert::PartialRef<'de, B, UB, W, $crate::__private::flavors::Groto>>
          + $crate::__private::decode::Decode<'de, $crate::__private::flavors::Groto, W, T::Output, B, UB>,
        T::Output: ::core::marker::Sized,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'de,
      {
        decode!();
      }
    )*
  };
  (@decode_to_packed_decoder(try_from_bytes) $(
    $(:< $($tg:ident:$t:path),+$(,)? >:)?
    $ty:ty
    $([ $(const $g:ident: usize),+$(,)? ])?
    { $try_from:expr }
  ),+$(,)?) => {
    $(
      impl<'de, B, UB, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::decode::Decode<'de, $crate::__private::flavors::Groto, LengthDelimited, Self, B, UB> for $ty {
        fn decode(context: &'de Context, src: B) -> ::core::result::Result<(usize, Self), $crate::__private::flavors::groto::Error>
        where
          Self: ::core::marker::Sized,
          B: $crate::__private::buffer::ReadBuf + 'de,
          UB: $crate::__private::buffer::Buffer<$crate::__private::flavors::groto::Unknown<B>> + 'de,
        {
          <[::core::primitive::u8] as $crate::__private::decode::Decode<'de, $crate::__private::flavors::Groto, $crate::__private::flavors::groto::LengthDelimited, $crate::__private::decode::BytesSlice<B>, B, UB>>::decode(context, src)
            .and_then(|(read, bytes)| {
              ($try_from)(bytes.as_ref())
                .map(|value| (read, value))
                .map_err(Into::into)
            })
        }
      }

      impl<'de, B, UB, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::decode::Decode<'de, $crate::__private::flavors::Groto, LengthDelimited, $crate::__private::decode::BytesSlice<B>, B, UB> for $ty {
        fn decode(context: &'de Context, src: B) -> ::core::result::Result<(usize, $crate::__private::decode::BytesSlice<B>), $crate::__private::flavors::groto::Error>
        where
          $crate::__private::decode::BytesSlice<B>: ::core::marker::Sized,
          B: $crate::__private::buffer::ReadBuf + 'de,
          UB: $crate::__private::buffer::Buffer<$crate::__private::flavors::groto::Unknown<B>> + 'de,
        {
          <[::core::primitive::u8] as $crate::__private::decode::Decode<'de, $crate::__private::flavors::Groto, $crate::__private::flavors::groto::LengthDelimited, $crate::__private::decode::BytesSlice<B>, B, UB>>::decode(context, src)
        }
      }
    )*
  };
  (@decode_to_packed_decoder(from_bytes) $(
    $(:< $($tg:ident:$t:path),+$(,)? >:)?
    $ty:ty
    $([ $(const $g:ident: usize),+$(,)? ])?
    { $from:expr }
  ),+$(,)?) => {
    $(
      impl<'de, B, UB, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::decode::Decode<'de, $crate::__private::flavors::Groto, LengthDelimited, Self, B, UB> for $ty {
        fn decode(context: &'de Context, src: B) -> ::core::result::Result<(usize, Self), $crate::__private::flavors::groto::Error>
        where
          Self: ::core::marker::Sized,
          B: $crate::__private::buffer::ReadBuf + 'de,
          UB: $crate::__private::buffer::Buffer<$crate::__private::flavors::groto::Unknown<B>> + 'de,
        {
          <[::core::primitive::u8] as $crate::__private::decode::Decode<'de, $crate::__private::flavors::Groto, $crate::__private::flavors::groto::LengthDelimited, $crate::__private::decode::BytesSlice<B>, B, UB>>::decode(context, src)
            .map(|(read, bytes)| (read, ($from)(bytes.as_ref())))
        }
      }

      impl<'de, B, UB, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::decode::Decode<'de, $crate::__private::flavors::Groto, LengthDelimited, $crate::__private::decode::BytesSlice<B>, B, UB> for $ty {
        fn decode(context: &'de Context, src: B) -> ::core::result::Result<(usize, $crate::__private::decode::BytesSlice<B>), $crate::__private::flavors::groto::Error>
        where
          $crate::__private::decode::BytesSlice<B>: ::core::marker::Sized,
          B: $crate::__private::buffer::ReadBuf + 'de,
          UB: $crate::__private::buffer::Buffer<$crate::__private::flavors::groto::Unknown<B>> + 'de,
        {
          <[::core::primitive::u8] as $crate::__private::decode::Decode<'de, $crate::__private::flavors::Groto, $crate::__private::flavors::groto::LengthDelimited, $crate::__private::decode::BytesSlice<B>, B, UB>>::decode(context, src)
        }
      }
    )*
  };
  (@encode_as_slice $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, W, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Encode<$crate::__private::flavors::Groto, $crate::__private::flavors::groto::Packed<W>> for $ty
      where
        T: $crate::__private::Encode<$crate::__private::flavors::Groto, W>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        fn encode(&self, context: &$crate::__private::flavors::groto::Context, buf: &mut [u8]) -> ::core::result::Result<usize, $crate::__private::flavors::groto::Error> {
          <[T] as $crate::__private::Encode<$crate::__private::flavors::Groto, $crate::__private::flavors::groto::Packed<W>>>::encode(self.as_ref(), context, buf)
        }

        fn encoded_len(&self, context: &$crate::__private::flavors::groto::Context) -> usize {
          <[T] as $crate::__private::Encode<$crate::__private::flavors::Groto, $crate::__private::flavors::groto::Packed<W>>>::encoded_len(self.as_ref(), context)
        }
      }

      impl<T, W, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::PartialEncode<$crate::__private::flavors::Groto, $crate::__private::flavors::groto::Packed<W>> for $ty
      where
        T: $crate::__private::PartialEncode<$crate::__private::flavors::Groto, W>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        fn partial_encode(
          &self,
          context: &$crate::__private::flavors::groto::Context,
          buf: &mut [u8],
          selector: &Self::Selector,
        ) -> ::core::result::Result<usize, $crate::__private::flavors::groto::Error> {
          <[T] as $crate::__private::PartialEncode<$crate::__private::flavors::Groto, $crate::__private::flavors::groto::Packed<W>>>::partial_encode(self.as_ref(), context, buf, selector)
        }

        fn partial_encoded_len(
          &self,
          context: &$crate::__private::flavors::groto::Context,
          selector: &Self::Selector,
        ) -> usize {
          <[T] as $crate::__private::PartialEncode<$crate::__private::flavors::Groto, $crate::__private::flavors::groto::Packed<W>>>::partial_encoded_len(self.as_ref(), context, selector)
        }
      }
    )*
  };
  (@encode_as_bytes $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<$($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Encode<$crate::__private::flavors::Groto, $crate::__private::flavors::groto::LengthDelimited> for $ty
      {
        fn encode(&self, context: &$crate::__private::flavors::groto::Context, buf: &mut [u8]) -> ::core::result::Result<usize, $crate::__private::flavors::groto::Error> {
          <[u8] as $crate::__private::Encode<$crate::__private::flavors::Groto, $crate::__private::flavors::groto::LengthDelimited>>::encode(self.as_ref(), context, buf)
        }

        fn encoded_len(&self, context: &$crate::__private::flavors::groto::Context) -> usize {
          <[u8] as $crate::__private::Encode<$crate::__private::flavors::Groto, $crate::__private::flavors::groto::LengthDelimited>>::encoded_len(self.as_ref(), context)
        }
      }

      impl<$($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::PartialEncode<$crate::__private::flavors::Groto, $crate::__private::flavors::groto::LengthDelimited> for $ty
      {
        fn partial_encode(
          &self,
          context: &$crate::__private::flavors::groto::Context,
          buf: &mut [u8],
          selector: &Self::Selector,
        ) -> ::core::result::Result<usize, $crate::__private::flavors::groto::Error> {
          <[u8] as $crate::__private::PartialEncode<$crate::__private::flavors::Groto, $crate::__private::flavors::groto::LengthDelimited>>::partial_encode(self.as_ref(), context, buf, selector)
        }

        fn partial_encoded_len(
          &self,
          context: &$crate::__private::flavors::groto::Context,
          selector: &Self::Selector,
        ) -> usize {
          <[u8] as $crate::__private::PartialEncode<$crate::__private::flavors::Groto, $crate::__private::flavors::groto::LengthDelimited>>::partial_encoded_len(self.as_ref(), context, selector)
        }
      }
    )*
  };
}

impl<'a, T, W> Encode<Groto, Borrowed<'a, Packed<W>>> for [&'a T]
where
  T: Encode<Groto, W> + ?Sized,
  W: WireFormat<Groto>,
{
  encode!(@encode_methods(Borrowed<'a, Packed<W>>));
}

impl<'a, T, W> PartialEncode<Groto, Borrowed<'a, Packed<W>>> for [&'a T]
where
  T: PartialEncode<Groto, W> + ?Sized,
  W: WireFormat<Groto>,
{
  encode!(@partial_encode_methods(Borrowed<'a, Packed<W>>));
}

impl<T, W> Encode<Groto, Packed<W>> for [T]
where
  T: Encode<Groto, W>,
  W: WireFormat<Groto>,
{
  encode!(@encode_methods(Packed<W>));
}

impl<T, W> PartialEncode<Groto, Packed<W>> for [T]
where
  T: PartialEncode<Groto, W>,
  W: WireFormat<Groto>,
{
  encode!(@partial_encode_methods(Packed<W>));
}

impl Encode<Groto, LengthDelimited> for [u8] {
  #[inline]
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let buf_len = buf.len();
    let this_len = self.len();
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(
        <Self as crate::encode::Encode<Groto, LengthDelimited>>::encoded_len(self, context),
        buf_len,
      ));
    }

    let len_size = varing::encode_u32_varint_to(this_len as u32, buf).map_err(|e| {
      Error::from_varint_encode_error(e).update(
        <Self as Encode<Groto, LengthDelimited>>::encoded_len(self, context),
        buf_len,
      )
    })?;

    let total = len_size + this_len;
    if total > buf_len {
      return Err(Error::insufficient_buffer(total, buf_len));
    }

    buf[len_size..total].copy_from_slice(self);
    Ok(total)
  }

  #[inline]
  fn encoded_len(&self, _: &Context) -> usize {
    let len = self.len();
    let len_size = varing::encoded_u32_varint_len(len as u32);
    len_size + len
  }
}

impl PartialEncode<Groto, LengthDelimited> for [u8] {
  #[inline]
  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    _: &Self::Selector,
  ) -> Result<usize, Error> {
    <Self as Encode<Groto, LengthDelimited>>::encode(self, context, buf)
  }

  #[inline]
  fn partial_encoded_len(&self, context: &Context, _: &Self::Selector) -> usize {
    <Self as Encode<Groto, LengthDelimited>>::encoded_len(self, context)
  }
}

// // TODO: fix encode impl
// impl<T, N, W, I> Encode<Groto, Flattened<W, I>> for [N]
// where
//   W: WireFormat<Groto>,
//   I: WireFormat<Groto>,
//   N: State<crate::convert::Flattened<Innermost>, Output = T> + Encode<Groto, W>,
//   T: Encode<Groto, I> + ?Sized,
// {
//   fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
//     let buf_len = buf.len();
//     let this_len = self.len();
//     if buf_len < this_len {
//       return Err(Error::insufficient_buffer(
//         <Self as Encode<Groto, Flattened<W, I>>>::encoded_len(self, context),
//         buf_len,
//       ));
//     }

//     let mut offset = 0;
//     for value in self.iter() {
//       if offset >= buf_len {
//         return Err(Error::insufficient_buffer(
//           <Self as Encode<Groto, Flattened<W, I>>>::encoded_len(self, context),
//           buf_len,
//         ));
//       }

//       offset += value.encode(context, &mut buf[offset..])?;
//     }
//     Ok(offset)
//   }

//   fn encoded_len(&self, context: &Context) -> usize {
//     self
//       .iter()
//       .map(|n| <N as Encode<Groto, W>>::encoded_len(n, context))
//       .sum()
//   }
// }

// impl<T, N, W, I> Selectable<Groto, Flattened<W, I>> for [N]
// where
//   W: WireFormat<Groto>,
//   I: WireFormat<Groto>,
//   N: State<crate::convert::Flattened<Innermost>, Output = T>,
//   T: Selectable<Groto, I> + ?Sized,
// {
//   type Selector = T::Selector;
// }

// // TODO: fix encode impl
// impl<T, N, W, I> PartialEncode<Groto, Flattened<W, I>> for [N]
// where
//   W: WireFormat<Groto>,
//   I: WireFormat<Groto>,
//   N: State<crate::convert::Flattened<Innermost>, Output = T>
//     + PartialEncode<Groto, W, Selector = T::Selector>,
//   T: PartialEncode<Groto, I> + ?Sized,
// {
//   fn partial_encode(
//     &self,
//     context: &Context,
//     buf: &mut [u8],
//     selector: &Self::Selector,
//   ) -> Result<usize, Error> {
//     let buf_len = buf.len();
//     let this_len = self.len();
//     if buf_len < this_len {
//       return Err(Error::insufficient_buffer(
//         <Self as PartialEncode<Groto, Flattened<W, I>>>::partial_encoded_len(
//           self, context, selector,
//         ),
//         buf_len,
//       ));
//     }

//     let mut offset = 0;
//     for value in self.iter() {
//       if offset >= buf_len {
//         return Err(Error::insufficient_buffer(
//           <Self as PartialEncode<Groto, Flattened<W, I>>>::partial_encoded_len(
//             self, context, selector,
//           ),
//           buf_len,
//         ));
//       }

//       offset += value.partial_encode(context, &mut buf[offset..], selector)?;
//     }
//     Ok(offset)
//   }

//   fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
//     self
//       .iter()
//       .map(|n| <N as PartialEncode<Groto, W>>::partial_encoded_len(n, context, selector))
//       .sum()
//   }
// }

list!(@flatten_state [T; N] [const N: usize], [T]);
list!(@partial_state [T; N] [const N: usize] => [T::Output; N], [T] => [T::Output]);
list!(@partial_ref_state(bytes) [u8; N] [const N: usize], [u8]);
list!(@partial_ref_state(packed) [T; N] [const N: usize], [T]);
list!(@partial_ref_state(borrow) [T; N] [const N: usize], [T]);
list!(@identity_transform [T; N] [const N: usize]);
list!(@identity_partial_transform(bytes) [u8; N] [const N: usize]);
list!(@transform(try_from_bytes) [u8; N] [const N: usize] {
  |s: &[u8]| {
    <[u8; N]>::try_from(s).map_err(|_| crate::__private::larger_than_array_capacity::<Groto, N>().into())
  }
});
list!(@partial_transform(try_from_bytes) [u8; N] [const N: usize] {
  |s: &[u8]| {
    <[u8; N]>::try_from(s).map_err(|_| crate::__private::larger_than_array_capacity::<Groto, N>())
  }
});
list!(@default_wire_format [T; N] [const N: usize], [T]);
list!(@selectable [T; N] [const N: usize], [T]);
list!(@decode_to_packed_decoder [T; N] [const N: usize], [T]);
list!(@decode_to_packed_decoder(try_from_bytes) [u8; N] [const N: usize] { |bytes| <[u8; N]>::try_from(bytes).map_err(|_| crate::__private::larger_than_array_capacity::<Groto, N>()) });
list!(
  @encode_as_slice [T; N] [const N: usize]
);

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::vec::Vec;

  list!(@flatten_state Vec<T>);
  list!(@partial_state Vec<T> => Vec<T::Output>);
  list!(@partial_ref_state(bytes) Vec<u8>);
  list!(@partial_ref_state(packed) Vec<T>);
  list!(@partial_ref_state(borrow) Vec<T>);
  list!(@default_wire_format Vec<T>);
  list!(@identity_transform Vec<T>);
  list!(@identity_partial_transform(bytes) Vec<u8>);
  list!(@transform(packed) Vec<T>);
  list!(@transform(from_bytes) Vec<u8>);
  list!(@partial_transform(from_bytes) Vec<u8>);
  list!(@partial_transform(packed) Vec<T>);
  list!(@partial_transform(identity) Vec<T>);
  list!(@selectable Vec<T>);
  list!(@decode_to_packed_decoder Vec<T>);
  list!(@decode_to_packed_decoder(from_bytes) Vec<u8> {
    Vec::from
  });
  list!(
    @encode_as_slice Vec<T>
  );
  list!(
    @encode_as_bytes Vec<u8>
  );
};

#[cfg(feature = "smallvec_1")]
const _: () = {
  use smallvec_1::SmallVec;

  list!(@flatten_state SmallVec<[T; N]> [const N: usize]);
  list!(@partial_state SmallVec<[T; N]> [const N: usize] => SmallVec<[T::Output; N]>);
  list!(@partial_ref_state(bytes) SmallVec<[u8; N]> [const N: usize]);
  list!(@partial_ref_state(packed) SmallVec<[T; N]> [const N: usize]);
  list!(@partial_ref_state(borrow) SmallVec<[T; N]> [const N: usize]);
  list!(@default_wire_format SmallVec<[T; N]> [const N: usize]);
  list!(@identity_transform SmallVec<[T; N]> [const N: usize]);
  list!(@identity_partial_transform(bytes) SmallVec<[u8; N]> [const N: usize]);
  list!(@transform(packed) SmallVec<[T; N]> [const N: usize]);
  list!(@transform(from_bytes) SmallVec<[u8; N]> [const N: usize]);
  list!(@partial_transform(from_bytes) SmallVec<[u8; N]> [const N: usize]);
  list!(@partial_transform(packed) SmallVec<[T; N]> [const N: usize]);
  list!(@partial_transform(identity) SmallVec<[T; N]> [const N: usize]);
  list!(@selectable SmallVec<[T; N]> [const N: usize]);
  list!(
    @decode_to_packed_decoder SmallVec<[T; N]> [const N: usize]
  );
  list!(
    @decode_to_packed_decoder(from_bytes) SmallVec<[u8; N]> [const N: usize] {
      SmallVec::from
    }
  );
  list!(
    @encode_as_slice SmallVec<[T; N]> [const N: usize]
  );
  list!(
    @encode_as_bytes SmallVec<[u8; N]> [const N: usize]
  );
};

#[cfg(feature = "arrayvec_0_7")]
const _: () = {
  use arrayvec_0_7::ArrayVec;

  list!(@flatten_state ArrayVec<T, N> [const N: usize]);
  list!(@partial_state ArrayVec<T, N> [const N: usize] => ArrayVec<T::Output, N>);
  list!(@partial_ref_state(bytes) ArrayVec<u8, N> [const N: usize]);
  list!(@partial_ref_state(packed) ArrayVec<T, N> [const N: usize]);
  list!(@partial_ref_state(borrow) ArrayVec<T, N> [const N: usize]);
  list!(@default_wire_format ArrayVec<T, N> [const N: usize]);
  list!(@identity_transform ArrayVec<T, N> [const N: usize]);
  list!(@identity_partial_transform(bytes) ArrayVec<u8, N> [const N: usize]);
  list!(@transform(packed) ArrayVec<T, N> [const N: usize]);
  list!(@transform(try_from_bytes) ArrayVec<u8, N> [const N: usize] {
    |s: &[u8]| {
      ArrayVec::try_from(s).map_err(|_| crate::__private::larger_than_array_capacity::<Groto, N>().into())
    }
  });
  list!(@partial_transform(try_from_bytes) ArrayVec<u8, N> [const N: usize] {
    |s: &[u8]| {
      ArrayVec::try_from(s).map_err(|_| crate::__private::larger_than_array_capacity::<Groto, N>())
    }
  });
  list!(@partial_transform(identity) ArrayVec<T, N> [const N: usize]);
  list!(@partial_transform(packed) ArrayVec<T, N> [const N: usize]);
  list!(@selectable ArrayVec<T, N> [const N: usize]);
  list!(
    @decode_to_packed_decoder ArrayVec<T, N> [const N: usize]
  );
  list!(
    @decode_to_packed_decoder(try_from_bytes) ArrayVec<u8, N> [const N: usize] {
      |bytes| ArrayVec::try_from(bytes).map_err(|_| crate::__private::larger_than_array_capacity::<Groto, N>())
    }
  );
  list!(
    @encode_as_slice ArrayVec<T, N> [const N: usize]
  );
  list!(
    @encode_as_bytes ArrayVec<u8, N> [const N: usize]
  );
};

#[cfg(feature = "tinyvec_1")]
const _: () = {
  use tinyvec_1::ArrayVec;

  use crate::convert::{Partial, State};

  #[cfg(not(any(feature = "std", feature = "alloc")))]
  pub fn larger_than_array_capacity<A>() -> Error
  where
    A: tinyvec_1::Array<Item = u8>,
  {
    Error::custom("cannot decode array with length greater than the capacity")
  }

  #[cfg(any(feature = "std", feature = "alloc"))]
  pub fn larger_than_array_capacity<A>() -> Error
  where
    A: tinyvec_1::Array<Item = u8>,
  {
    Error::custom(std::format!(
      "cannot decode array with length greater than the capacity {}",
      A::CAPACITY
    ))
  }

  list!(@flatten_state:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@partial_ref_state(bytes):<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A>);
  list!(@partial_ref_state(packed):<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@partial_ref_state(borrow):<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@default_wire_format:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@identity_transform:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@identity_partial_transform(bytes):<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A>);
  list!(@transform(packed):<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@transform(try_from_bytes):<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A> {
    |s: &[u8]| {
      ArrayVec::try_from(s).map_err(|_| larger_than_array_capacity::<A>())
    }
  });
  list!(@partial_transform(try_from_bytes):<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A> {
    |s: &[u8]| {
      ArrayVec::try_from(s).map_err(|_| larger_than_array_capacity::<A>())
    }
  });
  list!(@partial_transform(packed):<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@partial_transform(identity):<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@selectable:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(
    @decode_to_packed_decoder:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>
  );
  list!(
    @decode_to_packed_decoder(try_from_bytes):<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A> {
      |bytes| ArrayVec::try_from(bytes).map_err(|_| larger_than_array_capacity::<A>())
    }
  );
  list!(
    @encode_as_slice:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>
  );
  list!(
    @encode_as_bytes:<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A>
  );

  impl<T, const N: usize> State<Partial<Groto>> for ArrayVec<[T; N]>
  where
    T: State<Partial<Groto>>,
    T::Output: Sized,
  {
    type Output = ArrayVec<[T::Output; N]>;
  }

  #[cfg(any(feature = "std", feature = "alloc"))]
  const _: () = {
    use tinyvec_1::TinyVec;

    list!(@flatten_state:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@partial_ref_state(bytes):<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>);
    list!(@partial_ref_state(packed):<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@partial_ref_state(borrow):<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@default_wire_format:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@identity_transform:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@identity_partial_transform(bytes):<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>);
    list!(@transform(from_bytes):<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>);
    list!(@transform(packed):<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@partial_transform(from_bytes):<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>);
    list!(@partial_transform(identity):<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@partial_transform(packed):<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@selectable:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(
      @decode_to_packed_decoder:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>
    );
    list!(
      @decode_to_packed_decoder(from_bytes):<A: tinyvec_1::Array<Item = u8>>: TinyVec<A> {
        TinyVec::from
      }
    );
    list!(
      @encode_as_slice:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>
    );
    list!(
      @encode_as_bytes:<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>
    );

    impl<T, const N: usize> State<Partial<Groto>> for TinyVec<[T; N]>
    where
      T: State<Partial<Groto>> + Default,
      T::Output: Sized + Default,
    {
      type Output = TinyVec<[T::Output; N]>;
    }
  };
};
