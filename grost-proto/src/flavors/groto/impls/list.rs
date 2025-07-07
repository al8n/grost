use crate::{
  buffer::ReadBuf,
  decode::BytesSlice,
  encode::{Encode, PartialEncode},
  flavors::{
    Borrowed, Groto, Packed, WireFormat,
    groto::{Context, Error, LengthDelimited, PackedDecoder},
  },
};

macro_rules! partial_encode {
  (@bridge($ty:ty as $wf:ty)) => {
    fn partial_encode_raw(
      &self,
      context: &$crate::__private::flavors::groto::Context,
      buf: &mut [u8],
      selector: &Self::Selector,
    ) -> ::core::result::Result<usize, $crate::__private::flavors::groto::Error> {
      <$ty as $crate::__private::PartialEncode<$wf, $crate::__private::flavors::Groto>>::partial_encode_raw(self.as_ref(), context, buf, selector)
    }

    fn partial_encoded_raw_len(&self, context: &$crate::__private::flavors::groto::Context, selector: &Self::Selector) -> usize {
      <$ty as $crate::__private::PartialEncode<$wf, $crate::__private::flavors::Groto>>::partial_encoded_raw_len(self.as_ref(), context, selector)
    }

    fn partial_encode(
      &self,
      context: &$crate::__private::flavors::groto::Context,
      buf: &mut [u8],
      selector: &Self::Selector,
    ) -> ::core::result::Result<usize, $crate::__private::flavors::groto::Error> {
      <$ty as $crate::__private::PartialEncode<$wf, $crate::__private::flavors::Groto>>::partial_encode(self.as_ref(), context, buf, selector)
    }

    fn partial_encoded_len(
      &self,
      context: &$crate::__private::flavors::groto::Context,
      selector: &Self::Selector,
    ) -> usize {
      <$ty as $crate::__private::PartialEncode<$wf, $crate::__private::flavors::Groto>>::partial_encoded_len(self.as_ref(), context, selector)
    }
  };
  (@impl($wf:ty)) => {
    fn partial_encode_raw(
      &self,
      context: &<Groto as crate::flavors::Flavor>::Context,
      buf: &mut [u8],
      selector: &Self::Selector,
    ) -> Result<usize, <Groto as crate::flavors::Flavor>::Error> {
      let mut offset = 0;
      let buf_len = buf.len();
      let encoded_len =
        <Self as PartialEncode<$wf, Groto>>::partial_encoded_raw_len(self, context, selector);

      if buf_len < encoded_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }

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

    fn partial_encoded_raw_len(
      &self,
      context: &<Groto as crate::flavors::Flavor>::Context,
      selector: &Self::Selector,
    ) -> usize {
      self
        .iter()
        .map(|v| v.partial_encoded_len(context, selector))
        .sum::<usize>()
    }

    fn partial_encode(
      &self,
      context: &<Groto as crate::flavors::Flavor>::Context,
      buf: &mut [u8],
      selector: &Self::Selector,
    ) -> Result<usize, <Groto as crate::flavors::Flavor>::Error> {
      let buf_len = buf.len();
      let body_size =
        <Self as PartialEncode<$wf, Groto>>::partial_encoded_raw_len(self, context, selector);
      let body_size_len = varing::encoded_u32_varint_len(body_size as u32);
      let encoded_len = body_size_len + body_size;
      let mut offset = varing::encode_u32_varint_to(body_size as u32, buf)
        .map_err(|e| Error::from_varint_encode_error(e).update(encoded_len, buf_len))?;
      if buf_len < encoded_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }

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
      let len =
        <Self as PartialEncode<$wf, Groto>>::partial_encoded_raw_len(self, context, selector);
      varing::encoded_u32_varint_len(len as u32) + len
    }
  };
}

macro_rules! encode {
  (@bridge($ty:ty as $wf:ty)) => {
    fn encode_raw(
      &self,
      context: &$crate::__private::flavors::groto::Context,
      buf: &mut [u8],
    ) -> ::core::result::Result<usize, $crate::__private::flavors::groto::Error> {
      <$ty as $crate::__private::Encode<$wf, $crate::__private::flavors::Groto>>::encode_raw(
        self.as_ref(),
        context,
        buf,
      )
    }

    fn encoded_raw_len(&self, context: &$crate::__private::flavors::groto::Context) -> usize {
      <$ty as $crate::__private::Encode<$wf, $crate::__private::flavors::Groto>>::encoded_raw_len(
        self.as_ref(),
        context,
      )
    }

    fn encode(
      &self,
      context: &$crate::__private::flavors::groto::Context,
      buf: &mut [u8],
    ) -> ::core::result::Result<usize, $crate::__private::flavors::groto::Error> {
      <$ty as $crate::__private::Encode<$wf, $crate::__private::flavors::Groto>>::encode(
        self.as_ref(),
        context,
        buf,
      )
    }

    fn encoded_len(&self, context: &$crate::__private::flavors::groto::Context) -> usize {
      <$ty as $crate::__private::Encode<$wf, $crate::__private::flavors::Groto>>::encoded_len(
        self.as_ref(),
        context,
      )
    }
  };
  (@impl($wf:ty)) => {
    fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
      let mut offset = 0;
      let buf_len = buf.len();

      let encoded_len = <Self as Encode<$wf, Groto>>::encoded_raw_len(self, context);
      if buf_len < encoded_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }

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

    fn encoded_raw_len(&self, context: &Context) -> usize {
      use $crate::__private::flavors::groto::WireType;

      match W::WIRE_TYPE {
        WireType::Varint | WireType::LengthDelimited | WireType::Nullable => {
          self.iter().map(|v| v.encoded_len(context)).sum()
        }
        WireType::Fixed8 => self.len(),
        WireType::Fixed16 => self.len() * 2,
        WireType::Fixed32 => self.len() * 4,
        WireType::Fixed64 => self.len() * 8,
        WireType::Fixed128 => self.len() * 16,
      }
    }

    fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
      let buf_len = buf.len();
      let body_size = <Self as Encode<$wf, Groto>>::encoded_raw_len(self, context);
      let body_size_len = varing::encoded_u32_varint_len(body_size as u32);
      let encoded_len = body_size_len + body_size;

      if buf_len < encoded_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }

      let mut offset = varing::encode_u32_varint_to(body_size as u32, buf)
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

    fn encoded_len(&self, ctx: &Context) -> usize {
      let len = <Self as Encode<$wf, Groto>>::encoded_raw_len(self, ctx);
      varing::encoded_u32_varint_len(len as u32) + len
    }
  };
}

macro_rules! decode {
  () => {
    fn decode(
      context: &'de <Groto as $crate::flavors::Flavor>::Context,
      src: RB,
    ) -> Result<(usize, PackedDecoder<'de, T, RB, B, W>), <Groto as $crate::flavors::Flavor>::Error>
    where
      PackedDecoder<'de, T, RB, B, W>: Sized,
      RB: $crate::buffer::ReadBuf + 'de,
      B: $crate::buffer::Buffer<<Groto as $crate::flavors::Flavor>::Unknown<RB>> + 'de,
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

      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::State<$crate::__private::convert::Flattened<$crate::__private::convert::Inner>> for $ty {
        type Output = T;
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
  (@default_wire_format(packed) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::flavors::DefaultListWireFormat<$crate::__private::flavors::Groto> for $ty {
        type Format<V> = $crate::__private::flavors::Packed<V> where V: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>;
      }
    )*
  };
  (@default_wire_format(repeated) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::flavors::DefaultRepeatedWireFormat<$crate::__private::flavors::Groto> for $ty {
        type Format<V, const TAG: u32> = $crate::__private::flavors::Repeated<V, TAG> where V: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>;
      }
    )*
  };
  (@default_wire_format(bytes) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<$($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::flavors::DefaultBytesWireFormat<$crate::__private::flavors::Groto> for $ty {
        type Format = $crate::__private::flavors::groto::LengthDelimited;
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
      impl<'a, T, W, TW, RB, B, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::Transform<$crate::__private::flavors::groto::PackedDecoder<'a, T, RB, B, TW>, $ty, W, $crate::__private::flavors::Groto> for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        TW: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        T: $crate::__private::convert::State<$crate::__private::convert::PartialRef<'a, RB, B, TW, $crate::__private::flavors::Groto>>
          + $crate::__private::decode::Decode<'a, T::Output, TW, RB, B, $crate::__private::flavors::Groto>
          + $crate::__private::convert::Transform<T::Output, T, TW, $crate::__private::flavors::Groto>
          + 'a,
        T::Output: Sized,
        B: $crate::__private::buffer::Buffer<$crate::__private::flavors::groto::Unknown<RB>> + 'a,
        RB: $crate::__private::buffer::ReadBuf + 'a,
      {
        fn transform(input: $crate::__private::flavors::groto::PackedDecoder<'a, T, RB, B, TW>) -> ::core::result::Result<Self, <$crate::__private::flavors::Groto as $crate::__private::flavors::Flavor>::Error>
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
        $crate::__private::flavors::Packed<W>,
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
      impl<'a, T, W, TW, RB, B, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::PartialTransform<PackedDecoder<'a, T, RB, B, TW>, ::core::option::Option<Self>, Packed<W>, $crate::__private::flavors::Groto> for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        TW: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        T: $crate::__private::convert::State<$crate::__private::convert::PartialRef<'a, RB, B, TW, $crate::__private::flavors::Groto>>
          + $crate::__private::convert::State<$crate::__private::convert::Flattened>
          + $crate::__private::selection::Selectable<$crate::__private::flavors::Groto>
          + $crate::__private::decode::Decode<'a,
              <T as $crate::__private::convert::State<
                $crate::__private::convert::PartialRef<
                  'a,
                  RB,
                  B,
                  TW,
                  $crate::__private::flavors::Groto
                >
              >>::Output,
              TW,
              RB,
              B,
              $crate::__private::flavors::Groto,
            >
          + $crate::__private::convert::PartialTransform<
              <T as $crate::__private::convert::State<
                $crate::__private::convert::PartialRef<
                  'a,
                  RB,
                  B,
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
        <T as $crate::__private::convert::State<$crate::__private::convert::PartialRef<'a, RB, B, TW, $crate::__private::flavors::Groto>>>::Output: Sized + $crate::__private::selection::Selectable<$crate::__private::flavors::Groto, Selector = T::Selector>,
        <T as $crate::__private::convert::State<$crate::__private::convert::Flattened>>::Output: Sized + $crate::__private::selection::Selectable<$crate::__private::flavors::Groto, Selector = T::Selector>,
        RB: $crate::__private::buffer::ReadBuf + 'a,
        B: $crate::__private::buffer::Buffer<$crate::__private::flavors::groto::Unknown<RB>> + 'a,
      {
        fn partial_transform(input: PackedDecoder<'a, T, RB, B, TW>, selector: &Self::Selector) -> ::core::result::Result<::core::option::Option<Self>, <$crate::__private::flavors::Groto as $crate::__private::flavors::Flavor>::Error> {
          if $crate::__private::selection::Selector::is_empty(selector) {
            return ::core::result::Result::Ok(::core::option::Option::None);
          }

          input.into_iter()
            .filter_map(|res| {
              match res.and_then(|(_, inp)| <
                T as $crate::__private::convert::PartialTransform<<T as $crate::__private::convert::State<$crate::__private::convert::PartialRef<'a, RB, B, TW, $crate::__private::flavors::Groto>>>::Output, ::core::option::Option<<T as $crate::__private::convert::State<$crate::__private::convert::Flattened>>::Output>, TW, $crate::__private::flavors::Groto>
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
      impl<'de, T, W, RB, B, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::decode::Decode<'de, PackedDecoder<'de, T, RB, B, W>, Packed<W>, RB, B, $crate::__private::flavors::Groto> for $ty
      where
        T: $crate::__private::convert::State<$crate::__private::convert::PartialRef<'de, RB, B, W, $crate::__private::flavors::Groto>>
          + $crate::__private::decode::Decode<'de, T::Output, W, RB, B, $crate::__private::flavors::Groto>,
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
      impl<'de, RB, B, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::decode::Decode<'de, Self, LengthDelimited, RB, B, $crate::__private::flavors::Groto, > for $ty {
        fn decode(context: &'de Context, src: RB) -> ::core::result::Result<(usize, Self), $crate::__private::flavors::groto::Error>
        where
          Self: ::core::marker::Sized,
          RB: $crate::__private::buffer::ReadBuf + 'de,
          B: $crate::__private::buffer::Buffer<$crate::__private::flavors::groto::Unknown<RB>> + 'de,
        {
          <[::core::primitive::u8] as $crate::__private::decode::Decode<'de, $crate::__private::decode::BytesSlice<RB>,  $crate::__private::flavors::groto::LengthDelimited, RB, B, $crate::__private::flavors::Groto>>::decode(context, src)
            .and_then(|(read, bytes)| {
              ($try_from)(bytes.as_ref())
                .map(|value| (read, value))
                .map_err(Into::into)
            })
        }
      }

      impl<'de, RB, B, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::decode::Decode<'de, $crate::__private::decode::BytesSlice<RB>, LengthDelimited, RB, B, $crate::__private::flavors::Groto, > for $ty {
        fn decode(context: &'de Context, src: RB) -> ::core::result::Result<(usize, $crate::__private::decode::BytesSlice<RB>), $crate::__private::flavors::groto::Error>
        where
          $crate::__private::decode::BytesSlice<B>: ::core::marker::Sized,
          RB: $crate::__private::buffer::ReadBuf + 'de,
          B: $crate::__private::buffer::Buffer<$crate::__private::flavors::groto::Unknown<RB>> + 'de,
        {
          <[::core::primitive::u8] as $crate::__private::decode::Decode<'de, $crate::__private::decode::BytesSlice<RB>, $crate::__private::flavors::groto::LengthDelimited, RB, B, $crate::__private::flavors::Groto>>::decode(context, src)
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
      impl<'de, RB, B, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::decode::Decode<'de, Self, LengthDelimited, RB, B, $crate::__private::flavors::Groto> for $ty {
        fn decode(context: &'de Context, src: RB) -> ::core::result::Result<(usize, Self), $crate::__private::flavors::groto::Error>
        where
          Self: ::core::marker::Sized,
          RB: $crate::__private::buffer::ReadBuf + 'de,
          B: $crate::__private::buffer::Buffer<$crate::__private::flavors::groto::Unknown<RB>> + 'de,
        {
          <[::core::primitive::u8] as $crate::__private::decode::Decode<'de, $crate::__private::decode::BytesSlice<RB>, $crate::__private::flavors::groto::LengthDelimited, RB, B, $crate::__private::flavors::Groto>>::decode(context, src)
            .map(|(read, bytes)| (read, ($from)(bytes.as_ref())))
        }
      }

      impl<'de, RB, B, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::decode::Decode<'de, $crate::__private::decode::BytesSlice<RB>, LengthDelimited, RB, B,$crate::__private::flavors::Groto> for $ty {
        fn decode(context: &'de Context, src: RB) -> ::core::result::Result<(usize, $crate::__private::decode::BytesSlice<RB>), $crate::__private::flavors::groto::Error>
        where
          $crate::__private::decode::BytesSlice<RB>: ::core::marker::Sized,
          RB: $crate::__private::buffer::ReadBuf + 'de,
          B: $crate::__private::buffer::Buffer<$crate::__private::flavors::groto::Unknown<RB>> + 'de,
        {
          <[::core::primitive::u8] as $crate::__private::decode::Decode<'de, $crate::__private::decode::BytesSlice<RB>, $crate::__private::flavors::groto::LengthDelimited, RB, B, $crate::__private::flavors::Groto>>::decode(context, src)
        }
      }
    )*
  };
  (@encode(bytes) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<$($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Encode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto> for $ty
      {
        encode!(@bridge([u8] as $crate::__private::flavors::groto::LengthDelimited));
      }

      impl<$($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::PartialEncode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto> for $ty
      {
        partial_encode!(@bridge([u8] as $crate::__private::flavors::groto::LengthDelimited));
      }
    )*
  };
  (@encode(packed) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, W, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Encode<$crate::__private::flavors::Packed<W>, $crate::__private::flavors::Groto> for $ty
      where
        T: $crate::__private::Encode<W, $crate::__private::flavors::Groto>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        encode!(@bridge([T] as $crate::__private::flavors::Packed<W>));
      }

      impl<T, W, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::PartialEncode<$crate::__private::flavors::Packed<W>, $crate::__private::flavors::Groto> for $ty
      where
        T: $crate::__private::PartialEncode<W, $crate::__private::flavors::Groto>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        partial_encode!(@bridge([T] as $crate::__private::flavors::Packed<W>));
      }
    )*
  };
  (@encode(borrow) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<'b, T: 'b, W, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Encode<
        $crate::__private::flavors::Borrowed<'b, $crate::__private::flavors::Packed<W>>,
        $crate::__private::flavors::Groto
      > for $ty
      where
        T: $crate::__private::Encode<W, $crate::__private::flavors::Groto>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        encode!(@bridge([&'b T] as $crate::__private::flavors::Borrowed<'b, $crate::__private::flavors::Packed<W>>));
      }

      impl<'b, T: 'b, W, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::PartialEncode<
        $crate::__private::flavors::Borrowed<'b, $crate::__private::flavors::Packed<W>>,
        $crate::__private::flavors::Groto
      > for $ty
      where
        T: $crate::__private::PartialEncode<W, $crate::__private::flavors::Groto>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        partial_encode!(@bridge([&'b T] as $crate::__private::flavors::Borrowed<'b, $crate::__private::flavors::Packed<W>>));
      }
    )*
  };
}

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
list!(@default_wire_format(packed) [T; N] [const N: usize], [T]);
list!(@default_wire_format(repeated) [T; N] [const N: usize], [T]);
list!(@default_wire_format(bytes) [u8; N] [const N: usize], [u8]);
list!(@selectable [T; N] [const N: usize], [T]);
list!(@decode_to_packed_decoder [T; N] [const N: usize], [T]);
list!(@decode_to_packed_decoder(try_from_bytes) [u8; N] [const N: usize] { |bytes| <[u8; N]>::try_from(bytes).map_err(|_| crate::__private::larger_than_array_capacity::<Groto, N>()) });
list!(
  @encode(packed) [T; N] [const N: usize]
);
list!(
  @encode(borrow) [&'b T; N] [const N: usize]
);
list!(
  @encode(bytes) [u8; N] [const N: usize]
);

// TODO(al8n): change this to single direction equivalent
bidi_equivalent!(:<RB: ReadBuf>: [const N: usize] impl<[u8; N], LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
bidi_equivalent!([const N: usize] impl <[u8; N], LengthDelimited> for <[u8], LengthDelimited>);

bidi_equivalent!(@encode :<T: Encode<W, Groto>, W: WireFormat<Groto>>:[const N: usize] impl <[T; N], Packed<W>> for <[T], Packed<W>>);
bidi_equivalent!(@partial_encode :<T: PartialEncode<W, Groto>, W: WireFormat<Groto>>:[const N: usize] impl <[T; N], Packed<W>> for <[T], Packed<W>>);

bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T; N], Borrowed<'a, Packed<W>>> for <[T], Packed<W>>);
bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T; N], Borrowed<'a, Packed<W>>> for <[T], Packed<W>>);

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::vec::Vec;

  list!(@flatten_state Vec<T>);
  list!(@partial_state Vec<T> => Vec<T::Output>);
  list!(@partial_ref_state(bytes) Vec<u8>);
  list!(@partial_ref_state(packed) Vec<T>);
  list!(@partial_ref_state(borrow) Vec<T>);
  list!(@default_wire_format(packed) Vec<T>);
  list!(@default_wire_format(repeated) Vec<T>);
  list!(@default_wire_format(bytes) Vec<u8>);
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
    @encode(packed) Vec<T>
  );
  list!(
    @encode(borrow) Vec<&'b T>
  );
  list!(
    @encode(bytes) Vec<u8>
  );
  // Vec<u8> is the same as encode BytesSlice<RB>
  bidi_equivalent!(:<RB: ReadBuf>: impl<Vec<u8>, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
  // Vec<u8> is the same as encode [u8]
  bidi_equivalent!(impl <Vec<u8>, LengthDelimited> for <[u8], LengthDelimited>);

  // Vec<T> is the same as encode [T]
  bidi_equivalent!(@encode :<T: Encode<W, Groto>, W: WireFormat<Groto>>: impl <Vec<T>, Packed<W>> for <[T], Packed<W>>);
  bidi_equivalent!(@partial_encode :<T: PartialEncode<W, Groto>, W: WireFormat<Groto>>: impl <Vec<T>, Packed<W>> for <[T], Packed<W>>);

  // Vec<T> is the same as encode [&'a T]
  bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>: impl <[&'a T], Borrowed<'a, Packed<W>>> for <Vec<T>, Packed<W>>);
  bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>: impl <[&'a T], Borrowed<'a, Packed<W>>> for <Vec<T>, Packed<W>>);

  // Vec<T> is the same as encode Vec<&'a T>
  bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>: impl <Vec<&'a T>, Borrowed<'a, Packed<W>>> for <Vec<T>, Packed<W>>);
  bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>: impl <Vec<&'a T>, Borrowed<'a, Packed<W>>> for <Vec<T>, Packed<W>>);
};

#[cfg(feature = "smallvec_1")]
const _: () = {
  use smallvec_1::SmallVec;

  list!(@flatten_state SmallVec<[T; N]> [const N: usize]);
  list!(@partial_state SmallVec<[T; N]> [const N: usize] => SmallVec<[T::Output; N]>);
  list!(@partial_ref_state(bytes) SmallVec<[u8; N]> [const N: usize]);
  list!(@partial_ref_state(packed) SmallVec<[T; N]> [const N: usize]);
  list!(@partial_ref_state(borrow) SmallVec<[T; N]> [const N: usize]);
  list!(@default_wire_format(packed) SmallVec<[T; N]> [const N: usize]);
  list!(@default_wire_format(repeated) SmallVec<[T; N]> [const N: usize]);
  list!(@default_wire_format(bytes) SmallVec<[u8; N]> [const N: usize]);
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
    @encode(packed) SmallVec<[T; N]> [const N: usize]
  );
  list!(
    @encode(borrow) SmallVec<[&'b T; N]> [const N: usize]
  );
  list!(
    @encode(bytes) SmallVec<[u8; N]> [const N: usize]
  );
  bidi_equivalent!(:<RB: ReadBuf>: [const N: usize] impl<SmallVec<[u8; N]>, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
  bidi_equivalent!([const N: usize] impl <SmallVec<[u8; N]>, LengthDelimited> for <[u8], LengthDelimited>);

  bidi_equivalent!(@encode :<T: Encode<W, Groto>, W: WireFormat<Groto>>:[const N: usize] impl <SmallVec<[T; N]>, Packed<W>> for <[T], Packed<W>>);
  bidi_equivalent!(@partial_encode :<T: PartialEncode<W, Groto>, W: WireFormat<Groto>>:[const N: usize] impl <SmallVec<[T; N]>, Packed<W>> for <[T], Packed<W>>);

  bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <SmallVec<[T; N]>, Packed<W>>);
  bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <SmallVec<[T; N]>, Packed<W>>);

  bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize] impl <SmallVec<[&'a T; N]>, Borrowed<'a, Packed<W>>> for <SmallVec<[T; N]>, Packed<W>>);
  bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize] impl <SmallVec<[&'a T; N]>, Borrowed<'a, Packed<W>>> for <SmallVec<[T; N]>, Packed<W>>);
};

#[cfg(feature = "arrayvec_0_7")]
const _: () = {
  use arrayvec_0_7::ArrayVec;

  list!(@flatten_state ArrayVec<T, N> [const N: usize]);
  list!(@partial_state ArrayVec<T, N> [const N: usize] => ArrayVec<T::Output, N>);
  list!(@partial_ref_state(bytes) ArrayVec<u8, N> [const N: usize]);
  list!(@partial_ref_state(packed) ArrayVec<T, N> [const N: usize]);
  list!(@partial_ref_state(borrow) ArrayVec<T, N> [const N: usize]);
  list!(@default_wire_format(packed) ArrayVec<T, N> [const N: usize]);
  list!(@default_wire_format(repeated) ArrayVec<T, N> [const N: usize]);
  list!(@default_wire_format(bytes) ArrayVec<u8, N> [const N: usize]);
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
    @encode(packed) ArrayVec<T, N> [const N: usize]
  );
  list!(
    @encode(borrow) ArrayVec<&'b T, N> [const N: usize]
  );
  list!(
    @encode(bytes) ArrayVec<u8, N> [const N: usize]
  );
  bidi_equivalent!(:<RB: ReadBuf>: [const N: usize] impl<ArrayVec<u8, N>, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
  bidi_equivalent!([const N: usize] impl <ArrayVec<u8, N>, LengthDelimited> for <[u8], LengthDelimited>);

  bidi_equivalent!(@encode :<T: Encode<W, Groto>, W: WireFormat<Groto>>:[const N: usize] impl <ArrayVec<T, N>, Packed<W>> for <[T], Packed<W>>);
  bidi_equivalent!(@partial_encode :<T: PartialEncode<W, Groto>, W: WireFormat<Groto>>:[const N: usize] impl <ArrayVec<T, N>, Packed<W>> for <[T], Packed<W>>);

  bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <ArrayVec<T, N>, Packed<W>>);
  bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <ArrayVec<T, N>, Packed<W>>);

  bidi_equivalent!(@encode 'a:<T: Encode<W, Groto>, W:WireFormat<Groto>:'a>:[const N: usize] impl <ArrayVec<&'a T, N>, Borrowed<'a, Packed<W>>> for <ArrayVec<T, N>, Packed<W>>);
  bidi_equivalent!(@partial_encode 'a:<T: PartialEncode<W, Groto>, W: WireFormat<Groto>:'a>:[const N: usize] impl <ArrayVec<&'a T, N>, Borrowed<'a, Packed<W>>> for <ArrayVec<T, N>, Packed<W>>);
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

  trait DefaultEncode<W: WireFormat<Groto>>: Encode<W, Groto> + Default {}

  impl<T, W> DefaultEncode<W> for T
  where
    T: Encode<W, Groto> + Default,
    W: WireFormat<Groto>,
  {
  }

  trait DefaultPartialEncode<W: WireFormat<Groto>>: PartialEncode<W, Groto> + Default {}

  impl<T, W> DefaultPartialEncode<W> for T
  where
    T: PartialEncode<W, Groto> + Default,
    W: WireFormat<Groto>,
  {
  }

  list!(@flatten_state:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@partial_ref_state(bytes):<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A>);
  list!(@partial_ref_state(packed):<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@partial_ref_state(borrow):<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@default_wire_format(packed):<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@default_wire_format(repeated):<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@default_wire_format(bytes):<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A>);
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
    @encode(packed):<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>
  );
  list!(
    @encode(borrow):<A: tinyvec_1::Array<Item = &'b T>>: ArrayVec<A>
  );
  list!(
    @encode(bytes):<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A>
  );
  bidi_equivalent!(:<RB: ReadBuf, A: tinyvec_1::Array<Item = u8>>: impl<ArrayVec<A>, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
  bidi_equivalent!(:<A: tinyvec_1::Array<Item = u8>>: impl <ArrayVec<A>, LengthDelimited> for <[u8], LengthDelimited>);

  bidi_equivalent!(@encode :<T: DefaultEncode<W>, W: WireFormat<Groto>>:[const N: usize] impl <ArrayVec<[T; N]>, Packed<W>> for <[T], Packed<W>>);
  bidi_equivalent!(@partial_encode :<T: DefaultPartialEncode<W>, W: WireFormat<Groto>>:[const N: usize] impl <ArrayVec<[T; N]>, Packed<W>> for <[T], Packed<W>>);

  bidi_equivalent!(@encode 'a:<T: DefaultEncode<W>, W:WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <ArrayVec<[T; N]>, Packed<W>>);
  bidi_equivalent!(@partial_encode 'a:<T: DefaultPartialEncode<W>, W: WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <ArrayVec<[T; N]>, Packed<W>>);

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
    list!(@default_wire_format(packed):<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@default_wire_format(bytes):<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>);
    list!(@default_wire_format(repeated):<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
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
      @encode(packed):<A: tinyvec_1::Array<Item = T>>: TinyVec<A>
    );
    list!(
      @encode(borrow):<A: tinyvec_1::Array<Item = &'b T>>: TinyVec<A>
    );
    list!(
      @encode(bytes):<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>
    );
    bidi_equivalent!(:<RB: ReadBuf, A: tinyvec_1::Array<Item = u8>>: impl<TinyVec<A>, LengthDelimited> for <BytesSlice<RB>, LengthDelimited>);
    bidi_equivalent!(:<A: tinyvec_1::Array<Item = u8>>: impl <TinyVec<A>, LengthDelimited> for <[u8], LengthDelimited>);

    bidi_equivalent!(@encode :<T: DefaultEncode<W>, W: WireFormat<Groto>>:[const N: usize] impl <TinyVec<[T; N]>, Packed<W>> for <[T], Packed<W>>);
    bidi_equivalent!(@partial_encode :<T: DefaultPartialEncode<W>, W: WireFormat<Groto>>:[const N: usize] impl <TinyVec<[T; N]>, Packed<W>> for <[T], Packed<W>>);

    bidi_equivalent!(@encode 'a:<T: DefaultEncode<W>, W:WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <TinyVec<[T; N]>, Packed<W>>);
    bidi_equivalent!(@partial_encode 'a:<T: DefaultPartialEncode<W>, W: WireFormat<Groto>:'a>:[const N: usize] impl <[&'a T], Borrowed<'a, Packed<W>>> for <TinyVec<[T; N]>, Packed<W>>);

    impl<T, const N: usize> State<Partial<Groto>> for TinyVec<[T; N]>
    where
      T: State<Partial<Groto>> + Default,
      T::Output: Sized + Default,
    {
      type Output = TinyVec<[T::Output; N]>;
    }
  };
};

// #[test]
// fn t() {
//   let a: &[&[u16]] = &[&[1u16, 2, 3], &[4, 5, 6]];
//   let flatten_a: &[u16] = &[1u16, 2, 3, 4, 5, 6];

//   let context = Context::default();
//   let encoded_len =
//     <[&[u16]] as Encode<Flatten<Borrowed<'_, Packed<Varint>>, Varint>, Groto>>::encoded_len(
//       a, &context,
//     );
//   let flatten_encoded_len =
//     <[u16] as Encode<Packed<Varint>, Groto>>::encoded_len(flatten_a, &context);
//   assert_eq!(encoded_len, flatten_encoded_len);

//   let mut buf = [0u8; 100];
//   let mut flatten_buf = [0u8; 100];

//   let encoded_len =
//     <[&[u16]] as Encode<Flatten<Borrowed<'_, Packed<Varint>>, Varint>, Groto>>::encode(
//       a, &context, &mut buf,
//     )
//     .unwrap();
//   let flatten_encoded_len =
//     <[u16] as Encode<Packed<Varint>, Groto>>::encode(flatten_a, &context, &mut flatten_buf)
//       .unwrap();
//   assert_eq!(encoded_len, flatten_encoded_len);
//   assert_eq!(&buf[..encoded_len], &flatten_buf[..flatten_encoded_len]);
// }

// #[test]
// fn t11() {
//   let a: &[&[&u16]] = &[&[&1u16, &2, &3], &[&4, &5, &6]];
//   let flatten_a: &[u16] = &[1u16, 2, 3, 4, 5, 6];

//   let context = Context::default();
//   let encoded_len =
//     <[&[&u16]] as Encode<Flatten<Borrowed<'_, Packed<Varint>>, Varint>, Groto>>::encoded_len(
//       a, &context,
//     );
//   let flatten_encoded_len =
//     <[u16] as Encode<Packed<Varint>, Groto>>::encoded_len(flatten_a, &context);
//   assert_eq!(encoded_len, flatten_encoded_len);

//   let mut buf = [0u8; 100];
//   let mut flatten_buf = [0u8; 100];

//   let encoded_len =
//     <[&[&u16]] as Encode<Flatten<Borrowed<'_, Packed<Varint>>, Varint>, Groto>>::encode(
//       a, &context, &mut buf,
//     )
//     .unwrap();
//   let flatten_encoded_len =
//     <[u16] as Encode<Packed<Varint>, Groto>>::encode(flatten_a, &context, &mut flatten_buf)
//       .unwrap();
//   assert_eq!(encoded_len, flatten_encoded_len);
//   assert_eq!(&buf[..encoded_len], &flatten_buf[..flatten_encoded_len]);
// }

// #[test]
// fn t1() {
//   let a: &[[u16; 3]] = &[[1u16, 2, 3], [4, 5, 6]];
//   let flatten_a: &[u16] = &[1u16, 2, 3, 4, 5, 6];

//   let context = Context::default();
//   let encoded_len =
//     <[[u16; 3]] as Encode<Flatten<Packed<Varint>, Varint>, Groto>>::encoded_len(a, &context);
//   let flatten_encoded_len =
//     <[u16] as Encode<Packed<Varint>, Groto>>::encoded_len(flatten_a, &context);
//   assert_eq!(encoded_len, flatten_encoded_len);

//   let mut buf = [0u8; 100];
//   let mut flatten_buf = [0u8; 100];

//   let encoded_len =
//     <[[u16; 3]] as Encode<Flatten<Packed<Varint>, Varint>, Groto>>::encode(a, &context, &mut buf)
//       .unwrap();
//   let flatten_encoded_len =
//     <[u16] as Encode<Packed<Varint>, Groto>>::encode(flatten_a, &context, &mut flatten_buf)
//       .unwrap();
//   assert_eq!(encoded_len, flatten_encoded_len);
//   assert_eq!(&buf[..encoded_len], &flatten_buf[..flatten_encoded_len]);
// }

// #[test]
// fn t2() {
//   let a: &[[&u16; 3]] = &[[&1u16, &2, &3], [&4, &5, &6]];
//   let flatten_a: &[u16] = &[1u16, 2, 3, 4, 5, 6];

//   let context = Context::default();
//   let encoded_len =
//     <[[&u16; 3]] as Encode<Flatten<Packed<Varint>, Varint>, Groto>>::encoded_len(a, &context);
//   let flatten_encoded_len =
//     <[u16] as Encode<Packed<Varint>, Groto>>::encoded_len(flatten_a, &context);
//   assert_eq!(encoded_len, flatten_encoded_len);

//   let mut buf = [0u8; 100];
//   let mut flatten_buf = [0u8; 100];

//   let encoded_len =
//     <[[&u16; 3]] as Encode<Flatten<Packed<Varint>, Varint>, Groto>>::encode(a, &context, &mut buf)
//       .unwrap();
//   let flatten_encoded_len =
//     <[u16] as Encode<Packed<Varint>, Groto>>::encode(flatten_a, &context, &mut flatten_buf)
//       .unwrap();
//   assert_eq!(encoded_len, flatten_encoded_len);
//   assert_eq!(&buf[..encoded_len], &flatten_buf[..flatten_encoded_len]);
// }

// #[test]
// fn t1() {
//   let a: &[&[u8]] = &[&[1u8, 2, 3], &[4, 5, 6]];
//   let flatten_a: &[u8] = &[1u8, 2, 3, 4, 5, 6];

//   let context = Context::default();
//   let encoded_len =
//     <[&[u8]] as Encode<Flatten<LengthDelimited, LengthDelimited>, Groto>>::encoded_len(a, &context);
//   let flatten_encoded_len =
//     <[u8] as Encode<LengthDelimited, Groto>>::encoded_len(flatten_a, &context);
//   assert_eq!(encoded_len, flatten_encoded_len);

//   let mut buf = [0u8; 100];
//   let mut flatten_buf = [0u8; 100];

//   let encoded_len = <[&[u8]] as Encode<Flatten<LengthDelimited, LengthDelimited>, Groto>>::encode(
//     a, &context, &mut buf,
//   )
//   .unwrap();
//   let flatten_encoded_len =
//     <[u8] as Encode<LengthDelimited, Groto>>::encode(flatten_a, &context, &mut flatten_buf)
//       .unwrap();
//   assert_eq!(encoded_len, flatten_encoded_len);
//   assert_eq!(&buf[..encoded_len], &flatten_buf[..flatten_encoded_len]);
// }

// #[test]
// fn t2() {
//   let a: &[&str] = &["hello", " world"];
//   let flatten_a: &str = "hello world";

//   let context = Context::default();
//   let encoded_len =
//     <[&str] as Encode<Flatten<LengthDelimited, LengthDelimited>, Groto>>::encoded_len(a, &context);
//   let flatten_encoded_len =
//     <str as Encode<LengthDelimited, Groto>>::encoded_len(flatten_a, &context);
//   assert_eq!(encoded_len, flatten_encoded_len);

//   let mut buf = [0u8; 100];
//   let mut flatten_buf = [0u8; 100];

//   let encoded_len = <[&str] as Encode<Flatten<LengthDelimited, LengthDelimited>, Groto>>::encode(
//     a, &context, &mut buf,
//   )
//   .unwrap();
//   let flatten_encoded_len =
//     <str as Encode<LengthDelimited, Groto>>::encode(flatten_a, &context, &mut flatten_buf).unwrap();
//   assert_eq!(encoded_len, flatten_encoded_len);
//   assert_eq!(&buf[..encoded_len], &flatten_buf[..flatten_encoded_len]);
// }

mod array;
mod slice;
