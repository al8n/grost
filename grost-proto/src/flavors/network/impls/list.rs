use crate::{
  encode::{Encode, PartialEncode},
  flavors::{
    Network, WireFormat,
    network::{Borrowed, Context, Error, LengthDelimited, Packed, PackedDecoder, WireType},
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
      context: &<Network as crate::flavors::Flavor>::Context,
      buf: &mut [u8],
      selector: &Self::Selector,
    ) -> Result<usize, <Network as crate::flavors::Flavor>::Error> {
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
      context: &<Network as crate::flavors::Flavor>::Context,
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
      context: &'de <Network as $crate::flavors::Flavor>::Context,
      src: B,
    ) -> Result<(usize, PackedDecoder<'de, T, B, UB, W>), <Network as $crate::flavors::Flavor>::Error>
    where
      PackedDecoder<'de, T, B, UB, W>: Sized,
      B: $crate::buffer::ReadBuf,
      UB: $crate::buffer::Buffer<<Network as $crate::flavors::Flavor>::Unknown<B>> + 'de,
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
        $crate::__private::convert::Flatten
      > for $ty {
        type Input = $ty;
        type Output = $ty;
      }

      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::State<$crate::__private::convert::Flatten<$crate::__private::convert::Innermost>> for $ty
      where
        T: $crate::__private::convert::State<$crate::__private::convert::Flatten<$crate::__private::convert::Innermost>>,
      {
        type Input = T::Input;
        type Output = T::Output;
      }
    )*
  };
  (@decoded_state(bytes) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<'a, __GROST_READ_BUF__, __GROST_UNKNOWN_BUFFER__, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::State<$crate::__private::convert::Decoded<'a, $crate::__private::flavors::Network, LengthDelimited, __GROST_READ_BUF__, __GROST_UNKNOWN_BUFFER__>> for $ty
      {
        type Input = $crate::__private::decode::BytesSlice<__GROST_READ_BUF__>;
        type Output = $crate::__private::decode::BytesSlice<__GROST_READ_BUF__>;
      }
    )*
  };
  (@decoded_state(packed) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<'a, T, W, __GROST_READ_BUF__, __GROST_UNKNOWN_BUFFER__, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::State<
        $crate::__private::convert::Decoded<
          'a,
          $crate::__private::flavors::Network,
          Packed<W>,
          __GROST_READ_BUF__,
          __GROST_UNKNOWN_BUFFER__,
        >
      > for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      {
        type Input = &'a [u8];
        type Output = $crate::__private::flavors::network::PackedDecoder<'a, T, __GROST_READ_BUF__, __GROST_UNKNOWN_BUFFER__, W>;
      }
    )*
  };
  (@decoded_state(borrow) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<'a, T, W, __GROST_READ_BUF__, __GROST_UNKNOWN_BUFFER__, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::State<
        $crate::__private::convert::Decoded<
          'a,
          $crate::__private::flavors::Network,
          Borrowed<'a, Packed<W>>,
          __GROST_READ_BUF__,
          __GROST_UNKNOWN_BUFFER__,
        >
      > for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      {
        type Input = &'a [u8];
        type Output = $crate::__private::flavors::network::PackedDecoder<'a, T, __GROST_READ_BUF__, __GROST_UNKNOWN_BUFFER__, W>;
      }
    )*
  };
  (@default_wire_format $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::flavors::DefaultWireFormat<$crate::__private::flavors::Network> for $ty
      where
        T: $crate::__private::flavors::DefaultWireFormat<$crate::__private::flavors::Network>,
      {
        type Format = $crate::__private::flavors::network::Packed<T::Format>;
      }
    )*
  };
  (@identity_transform $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, W, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::Transform<$crate::__private::flavors::Network, W, Self> for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      {
        fn transform(input: Self) -> ::core::result::Result<Self, $crate::__private::flavors::network::Error> {
          ::core::result::Result::Ok(input)
        }
      }
    )*
  };
  // TODO(al8n): remove the wire format generic from `Selectable`?
  // (@identity_partial_transform $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
  //   $(
  //     impl<T, W, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::PartialTransform<$crate::__private::flavors::Network, W, Self> for $ty
  //     where
  //       W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
  //       T: $crate::__private::selection::Selectable<$crate::__private::flavors::Network, W>
  //         + $crate::__private::PartialTransform<$crate::__private::flavors::Network, W, Self>,
  //       Self: $crate::__private::selection::Selectable<$crate::__private::flavors::Network, W, Selector = T::Selector>,
  //     {
  //       fn partial_transform(input: Self, selector: &T::Selector) -> ::core::result::Result<::core::option::Option<Self>, $crate::__private::flavors::network::Error> {
  //         if $crate::__private::selection::Selector::is_empty(selector) {
  //           return ::core::result::Result::Ok(::core::option::Option::None);
  //         }

  //         input.into_iter()
  //           .filter_map(|val| {
  //             match T::partial_transform(val, selector) {
  //               ::core::result::Result::Ok(val) => val.map(|val| ::core::result::Result::Ok(val)),
  //               ::core::result::Result::Err(e) => ::core::option::Option::Some(Err(e)),
  //             }
  //           })
  //           .collect::<::core::result::Result<$ty, _>>()
  //           .map(|val| if val.is_empty() {
  //             ::core::option::Option::None
  //           } else {
  //             ::core::option::Option::Some(val)
  //           })
  //       }
  //     }
  //   )*
  // };
  (@packed_transform $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<'a, T, W, TW, B, UB, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::Transform<$crate::__private::flavors::Network, W, $crate::__private::flavors::network::PackedDecoder<'a, T, B, UB, TW>> for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network> + 'a,
        TW: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network> + 'a,
        T: $crate::__private::convert::State<$crate::__private::convert::Decoded<'a, $crate::__private::flavors::Network, TW, B, UB>, Input = &'a [u8]>
          + $crate::__private::decode::Decode<'a, $crate::__private::flavors::Network, TW, T::Output, B, UB>
          + $crate::__private::decode::Transform<$crate::__private::flavors::Network, TW, T::Output>
          + 'a,
        T::Output: Sized,
        UB: $crate::__private::buffer::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'a,
        B: $crate::__private::buffer::ReadBuf + 'a,
      {
        fn transform(input: $crate::__private::flavors::network::PackedDecoder<'a, T, B, UB, TW>) -> ::core::result::Result<Self, <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::Error>
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

      impl<'a, T, W, TW, B, UB, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::PartialTransform<$crate::__private::flavors::Network, W, $crate::__private::flavors::network::PackedDecoder<'a, T, B, UB, TW>> for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network> + 'a,
        TW: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network> + 'a,
        T: $crate::__private::convert::State<$crate::__private::convert::Decoded<'a, $crate::__private::flavors::Network, TW, B, UB>, Input = &'a [u8]>
          + $crate::__private::decode::Decode<'a, $crate::__private::flavors::Network, TW, T::Output, B, UB>
          + $crate::__private::selection::Selectable<$crate::__private::flavors::Network>
          + $crate::__private::decode::PartialTransform<$crate::__private::flavors::Network, TW, T::Output>
          + 'a,
        T::Output: Sized + $crate::__private::selection::Selectable<$crate::__private::flavors::Network, Selector = T::Selector>,
        UB: $crate::__private::buffer::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'a,
        B: $crate::__private::buffer::ReadBuf + 'a,
        Self: $crate::__private::selection::Selectable<$crate::__private::flavors::Network, Selector = T::Selector>,
      {
        fn partial_transform(
          input: $crate::__private::flavors::network::PackedDecoder<'a, T, B, UB, TW>,
          selector: &T::Selector,
        ) -> ::core::result::Result<::core::option::Option<Self>, <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::Error>
        where
          Self: Sized
        {
          if $crate::__private::selection::Selector::is_empty(selector) {
            return ::core::result::Result::Ok(::core::option::Option::None);
          }

          input.into_iter()
            .filter_map(|res| {
              match res.and_then(|(_, inp)| T::partial_transform(inp, selector)) {
                ::core::result::Result::Ok(val) => val.map(|val| ::core::result::Result::Ok(val)),
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

      impl<T, W, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::PartialTransform<$crate::__private::flavors::Network, $crate::__private::flavors::network::Packed<W>, $ty> for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
        T: $crate::__private::selection::Selectable<$crate::__private::flavors::Network>
          + $crate::__private::decode::PartialTransform<$crate::__private::flavors::Network, W, T>,
        Self: $crate::__private::selection::Selectable<$crate::__private::flavors::Network, Selector = T::Selector>,
      {
        fn partial_transform(
          input: $ty,
          selector: &T::Selector,
        ) -> ::core::result::Result<::core::option::Option<Self>, <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::Error>
        where
          Self: Sized
        {
          if $crate::__private::selection::Selector::is_empty(selector) {
            return ::core::result::Result::Ok(::core::option::Option::None);
          }

          input.into_iter()
            .filter_map(|res| {
              match T::partial_transform(res, selector) {
                ::core::result::Result::Ok(val) => val.map(|val| ::core::result::Result::Ok(val)),
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
  (@selectable $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::selection::Selectable<$crate::__private::flavors::Network> for $ty
      where
        T: $crate::__private::selection::Selectable<$crate::__private::flavors::Network>,
      {
        type Selector = T::Selector;
      }
    )*
  };
  (@decode_to_packed_decoder $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<'de, T, W, B, UB, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Decode<'de, $crate::__private::flavors::Network, Packed<W>, PackedDecoder<'de, T, B, UB, W>, B, UB> for $ty
      where
        T: $crate::__private::convert::State<$crate::__private::convert::Decoded<'de, $crate::__private::flavors::Network, W, B, UB>, Input = &'de [u8]>
          + $crate::__private::Decode<'de, $crate::__private::flavors::Network, W, T::Output, B, UB>,
        T::Output: ::core::marker::Sized,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network> + 'de,
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
      impl<'de, B, UB, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Decode<'de, $crate::__private::flavors::Network, LengthDelimited, Self, B, UB> for $ty {
        fn decode(context: &'de Context, src: B) -> ::core::result::Result<(usize, Self), $crate::__private::flavors::network::Error>
        where
          Self: ::core::marker::Sized,
          B: $crate::__private::buffer::ReadBuf + 'de,
          UB: $crate::__private::buffer::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'de,
        {
          <[::core::primitive::u8] as $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, $crate::__private::decode::BytesSlice<B>, B, UB>>::decode(context, src)
            .and_then(|(read, bytes)| {
              ($try_from)(bytes.as_ref())
                .map(|value| (read, value))
                .map_err(Into::into)
            })
        }
      }

      impl<'de, B, UB, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Decode<'de, $crate::__private::flavors::Network, LengthDelimited, $crate::__private::decode::BytesSlice<B>, B, UB> for $ty {
        fn decode(context: &'de Context, src: B) -> ::core::result::Result<(usize, $crate::__private::decode::BytesSlice<B>), $crate::__private::flavors::network::Error>
        where
          $crate::__private::decode::BytesSlice<B>: ::core::marker::Sized,
          B: $crate::__private::buffer::ReadBuf + 'de,
          UB: $crate::__private::buffer::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'de,
        {
          <[::core::primitive::u8] as $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, $crate::__private::decode::BytesSlice<B>, B, UB>>::decode(context, src)
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
      impl<'de, B, UB, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Decode<'de, $crate::__private::flavors::Network, LengthDelimited, Self, B, UB> for $ty {
        fn decode(context: &'de Context, src: B) -> ::core::result::Result<(usize, Self), $crate::__private::flavors::network::Error>
        where
          Self: ::core::marker::Sized,
          B: $crate::__private::buffer::ReadBuf + 'de,
          UB: $crate::__private::buffer::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'de,
        {
          <[::core::primitive::u8] as $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, $crate::__private::decode::BytesSlice<B>, B, UB>>::decode(context, src)
            .map(|(read, bytes)| (read, ($from)(bytes.as_ref())))
        }
      }

      impl<'de, B, UB, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Decode<'de, $crate::__private::flavors::Network, LengthDelimited, $crate::__private::decode::BytesSlice<B>, B, UB> for $ty {
        fn decode(context: &'de Context, src: B) -> ::core::result::Result<(usize, $crate::__private::decode::BytesSlice<B>), $crate::__private::flavors::network::Error>
        where
          $crate::__private::decode::BytesSlice<B>: ::core::marker::Sized,
          B: $crate::__private::buffer::ReadBuf + 'de,
          UB: $crate::__private::buffer::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'de,
        {
          <[::core::primitive::u8] as $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited, $crate::__private::decode::BytesSlice<B>, B, UB>>::decode(context, src)
        }
      }
    )*
  };
  (@encode_as_slice $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, W, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Packed<W>> for $ty
      where
        T: $crate::__private::Encode<$crate::__private::flavors::Network, W>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      {
        fn encode(&self, context: &$crate::__private::flavors::network::Context, buf: &mut [u8]) -> ::core::result::Result<usize, $crate::__private::flavors::network::Error> {
          <[T] as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Packed<W>>>::encode(self.as_ref(), context, buf)
        }

        fn encoded_len(&self, context: &$crate::__private::flavors::network::Context) -> usize {
          <[T] as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Packed<W>>>::encoded_len(self.as_ref(), context)
        }
      }

      impl<T, W, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Packed<W>> for $ty
      where
        T: $crate::__private::PartialEncode<$crate::__private::flavors::Network, W>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      {
        fn partial_encode(
          &self,
          context: &$crate::__private::flavors::network::Context,
          buf: &mut [u8],
          selector: &Self::Selector,
        ) -> ::core::result::Result<usize, $crate::__private::flavors::network::Error> {
          <[T] as $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Packed<W>>>::partial_encode(self.as_ref(), context, buf, selector)
        }

        fn partial_encoded_len(
          &self,
          context: &$crate::__private::flavors::network::Context,
          selector: &Self::Selector,
        ) -> usize {
          <[T] as $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Packed<W>>>::partial_encoded_len(self.as_ref(), context, selector)
        }
      }
    )*
  };
  (@encode_as_bytes $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<$($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty
      {
        fn encode(&self, context: &$crate::__private::flavors::network::Context, buf: &mut [u8]) -> ::core::result::Result<usize, $crate::__private::flavors::network::Error> {
          <[u8] as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::encode(self.as_ref(), context, buf)
        }

        fn encoded_len(&self, context: &$crate::__private::flavors::network::Context) -> usize {
          <[u8] as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::encoded_len(self.as_ref(), context)
        }
      }

      impl<$($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty
      {
        fn partial_encode(
          &self,
          context: &$crate::__private::flavors::network::Context,
          buf: &mut [u8],
          selector: &Self::Selector,
        ) -> ::core::result::Result<usize, $crate::__private::flavors::network::Error> {
          <[u8] as $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::partial_encode(self.as_ref(), context, buf, selector)
        }

        fn partial_encoded_len(
          &self,
          context: &$crate::__private::flavors::network::Context,
          selector: &Self::Selector,
        ) -> usize {
          <[u8] as $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::partial_encoded_len(self.as_ref(), context, selector)
        }
      }
    )*
  };
}

list!(@flatten_state [T; N] [const N: usize], [T]);
list!(@decoded_state(bytes) [u8; N] [const N: usize], [u8]);
list!(@decoded_state(packed) [T; N] [const N: usize], [T]);
list!(@decoded_state(borrow) [T; N] [const N: usize], [T]);
list!(@identity_transform [T; N] [const N: usize]);
list!(@default_wire_format [T; N] [const N: usize], [T]);
list!(@selectable [T; N] [const N: usize], [T]);
list!(@decode_to_packed_decoder [T; N] [const N: usize], [T]);
list!(@decode_to_packed_decoder(try_from_bytes) [u8; N] [const N: usize] { |bytes| <[u8; N]>::try_from(bytes).map_err(|_| crate::__private::larger_than_array_capacity::<Network, N>()) });
list!(
  @encode_as_slice [T; N] [const N: usize]
);

impl<'a, T, W> Encode<Network, Borrowed<'a, Packed<W>>> for [&'a T]
where
  T: Encode<Network, W> + ?Sized,
  W: WireFormat<Network>,
{
  encode!(@encode_methods(Borrowed<'a, Packed<W>>));
}

impl<'a, T, W> PartialEncode<Network, Borrowed<'a, Packed<W>>> for [&'a T]
where
  T: PartialEncode<Network, W> + ?Sized,
  W: WireFormat<Network>,
{
  encode!(@partial_encode_methods(Borrowed<'a, Packed<W>>));
}

impl<T, W> Encode<Network, Packed<W>> for [T]
where
  T: Encode<Network, W>,
  W: WireFormat<Network>,
{
  encode!(@encode_methods(Packed<W>));
}

impl<T, W> PartialEncode<Network, Packed<W>> for [T]
where
  T: PartialEncode<Network, W>,
  W: WireFormat<Network>,
{
  encode!(@partial_encode_methods(Packed<W>));
}

impl Encode<Network, LengthDelimited> for [u8] {
  #[inline]
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let buf_len = buf.len();
    let this_len = self.len();
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(
        <Self as crate::encode::Encode<Network, LengthDelimited>>::encoded_len(self, context),
        buf_len,
      ));
    }

    let len_size = varing::encode_u32_varint_to(this_len as u32, buf).map_err(|e| {
      Error::from_varint_encode_error(e).update(
        <Self as Encode<Network, LengthDelimited>>::encoded_len(self, context),
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

impl PartialEncode<Network, LengthDelimited> for [u8] {
  #[inline]
  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    _: &Self::Selector,
  ) -> Result<usize, Error> {
    <Self as Encode<Network, LengthDelimited>>::encode(self, context, buf)
  }

  #[inline]
  fn partial_encoded_len(&self, context: &Context, _: &Self::Selector) -> usize {
    <Self as Encode<Network, LengthDelimited>>::encoded_len(self, context)
  }
}

// // TODO: fix encode impl
// impl<T, N, W, I> Encode<Network, Flatten<W, I>> for [N]
// where
//   W: WireFormat<Network>,
//   I: WireFormat<Network>,
//   N: State<crate::convert::Flatten<Innermost>, Output = T> + Encode<Network, W>,
//   T: Encode<Network, I> + ?Sized,
// {
//   fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
//     let buf_len = buf.len();
//     let this_len = self.len();
//     if buf_len < this_len {
//       return Err(Error::insufficient_buffer(
//         <Self as Encode<Network, Flatten<W, I>>>::encoded_len(self, context),
//         buf_len,
//       ));
//     }

//     let mut offset = 0;
//     for value in self.iter() {
//       if offset >= buf_len {
//         return Err(Error::insufficient_buffer(
//           <Self as Encode<Network, Flatten<W, I>>>::encoded_len(self, context),
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
//       .map(|n| <N as Encode<Network, W>>::encoded_len(n, context))
//       .sum()
//   }
// }

// impl<T, N, W, I> Selectable<Network, Flatten<W, I>> for [N]
// where
//   W: WireFormat<Network>,
//   I: WireFormat<Network>,
//   N: State<crate::convert::Flatten<Innermost>, Output = T>,
//   T: Selectable<Network, I> + ?Sized,
// {
//   type Selector = T::Selector;
// }

// // TODO: fix encode impl
// impl<T, N, W, I> PartialEncode<Network, Flatten<W, I>> for [N]
// where
//   W: WireFormat<Network>,
//   I: WireFormat<Network>,
//   N: State<crate::convert::Flatten<Innermost>, Output = T>
//     + PartialEncode<Network, W, Selector = T::Selector>,
//   T: PartialEncode<Network, I> + ?Sized,
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
//         <Self as PartialEncode<Network, Flatten<W, I>>>::partial_encoded_len(
//           self, context, selector,
//         ),
//         buf_len,
//       ));
//     }

//     let mut offset = 0;
//     for value in self.iter() {
//       if offset >= buf_len {
//         return Err(Error::insufficient_buffer(
//           <Self as PartialEncode<Network, Flatten<W, I>>>::partial_encoded_len(
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
//       .map(|n| <N as PartialEncode<Network, W>>::partial_encoded_len(n, context, selector))
//       .sum()
//   }
// }

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::vec::Vec;

  list!(@flatten_state Vec<T>);
  list!(@decoded_state(bytes) Vec<u8>);
  list!(@decoded_state(packed) Vec<T>);
  list!(@decoded_state(borrow) Vec<T>);
  list!(@default_wire_format Vec<T>);
  list!(@identity_transform Vec<T>);
  list!(@packed_transform Vec<T>);
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
  list!(@decoded_state(bytes) SmallVec<[u8; N]> [const N: usize]);
  list!(@decoded_state(packed) SmallVec<[T; N]> [const N: usize]);
  list!(@decoded_state(borrow) SmallVec<[T; N]> [const N: usize]);
  list!(@default_wire_format SmallVec<[T; N]> [const N: usize]);
  list!(@identity_transform SmallVec<[T; N]> [const N: usize]);
  list!(@packed_transform SmallVec<[T; N]> [const N: usize]);
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
  list!(@decoded_state(bytes) ArrayVec<u8, N> [const N: usize]);
  list!(@decoded_state(packed) ArrayVec<T, N> [const N: usize]);
  list!(@decoded_state(borrow) ArrayVec<T, N> [const N: usize]);
  list!(@default_wire_format ArrayVec<T, N> [const N: usize]);
  list!(@identity_transform ArrayVec<T, N> [const N: usize]);
  list!(@packed_transform ArrayVec<T, N> [const N: usize]);
  list!(@selectable ArrayVec<T, N> [const N: usize]);
  list!(
    @decode_to_packed_decoder ArrayVec<T, N> [const N: usize]
  );
  list!(
    @decode_to_packed_decoder(try_from_bytes) ArrayVec<u8, N> [const N: usize] {
      |bytes| ArrayVec::try_from(bytes).map_err(|_| crate::__private::larger_than_array_capacity::<Network, N>())
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
  list!(@decoded_state(bytes):<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A>);
  list!(@decoded_state(packed):<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@decoded_state(borrow):<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@default_wire_format:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@identity_transform:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@packed_transform:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
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

  #[cfg(any(feature = "std", feature = "alloc"))]
  const _: () = {
    use tinyvec_1::TinyVec;

    list!(@flatten_state:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@decoded_state(bytes):<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>);
    list!(@decoded_state(packed):<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@decoded_state(borrow):<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@default_wire_format:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@identity_transform:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@packed_transform:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
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
  };
};
