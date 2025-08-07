use crate::{
  buffer::{Buf, BufMut, UnknownBuffer},
  decode::Decode,
  flavors::{
    Groto, WireFormat,
    groto::{Context, Error, RepeatedDecoder, context::RepeatedDecodePolicy},
  },
};

fn repeated_decode_list<'a, K, KW, RB, B, T, const TAG: u32>(
  ctx: &'a Context,
  output: &mut T,
  src: RB,
  mut push: impl FnMut(&mut T, K) -> Result<(), Error>,
  reserve: impl FnOnce(&mut T, usize) -> Result<(), Error>,
) -> Result<usize, Error>
where
  K: Decode<'a, KW, RB, B, Groto> + 'a,
  T: 'a,
  KW: WireFormat<Groto> + 'a,
  RB: Buf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  match ctx.repeated_decode_policy() {
    RepeatedDecodePolicy::PreallocateCapacity => {
      let (read, decoder) = RepeatedDecoder::<K, RB, B, KW, TAG>::decode(ctx, src)?;
      reserve(output, decoder.capacity_hint())?;

      for item in decoder.iter() {
        let (_, ent) = item?;
        push(output, ent)?;
      }

      Ok(read)
    }
    RepeatedDecodePolicy::GrowIncrementally => {
      super::repeated_decode::<K, KW, T, RB, B, TAG>(output, src, |list, src| {
        let (read, item) = K::decode(ctx, src)?;
        push(list, item)?;

        Ok(read)
      })
    }
  }
}

macro_rules! partial_ref_state {
  (@bytes $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<'a, __GROST_READ_BUF__, __GROST_BUFFER__, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::state::State<$crate::__private::state::PartialRef<'a, $crate::__private::flavors::groto::LengthDelimited, __GROST_READ_BUF__, __GROST_BUFFER__, $crate::__private::flavors::Groto>> for $ty
      {
        type Output = $crate::__private::decode::BytesSlice<__GROST_READ_BUF__>;
      }
    )*
  };
  (@packed $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<'a, T, W, __GROST_READ_BUF__, __GROST_BUFFER__, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::state::State<
        $crate::__private::state::PartialRef<
          'a,
          $crate::__private::flavors::Packed<W>,
          __GROST_READ_BUF__,
          __GROST_BUFFER__,
          $crate::__private::flavors::Groto,
        >
      > for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        $crate::__private::flavors::Packed<W>: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        T: $crate::__private::state::State<$crate::__private::state::PartialRef<'a, W,  __GROST_READ_BUF__, __GROST_BUFFER__, $crate::__private::flavors::Groto>>,
        T::Output: Sized,
      {
        type Output = $crate::__private::flavors::groto::PackedDecoder<'a, T::Output, __GROST_READ_BUF__, __GROST_BUFFER__, W>;
      }
    )*
  };
  (@repeated $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<'a, T, W, __GROST_READ_BUF__, __GROST_BUFFER__, const TAG: ::core::primitive::u32, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::state::State<
        $crate::__private::state::PartialRef<
          'a,
          $crate::__private::flavors::Repeated<W, TAG>,
          __GROST_READ_BUF__,
          __GROST_BUFFER__,
          $crate::__private::flavors::Groto,
        >
      > for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        $crate::__private::flavors::Repeated<W, TAG>: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        T: $crate::__private::state::State<$crate::__private::state::PartialRef<'a, W,  __GROST_READ_BUF__, __GROST_BUFFER__, $crate::__private::flavors::Groto>>,
        T::Output: Sized,
      {
        type Output = $crate::__private::flavors::groto::RepeatedDecoderBuffer<'a, T::Output, __GROST_READ_BUF__, __GROST_BUFFER__, W, TAG>;
      }
    )*
  };
}

macro_rules! ref_state {
  (@bytes $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<'a, __GROST_READ_BUF__, __GROST_BUFFER__, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::state::State<$crate::__private::state::Ref<'a, $crate::__private::flavors::groto::LengthDelimited,  __GROST_READ_BUF__, __GROST_BUFFER__, $crate::__private::flavors::Groto>> for $ty
      {
        type Output = $crate::__private::decode::BytesSlice<__GROST_READ_BUF__>;
      }
    )*
  };
  (@packed $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<'a, T, W, __GROST_READ_BUF__, __GROST_BUFFER__, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::state::State<
        $crate::__private::state::Ref<
          'a,
          $crate::__private::flavors::Packed<W>,
          __GROST_READ_BUF__,
          __GROST_BUFFER__,
          $crate::__private::flavors::Groto,
        >
      > for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        $crate::__private::flavors::Packed<W>: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        T: $crate::__private::state::State<$crate::__private::state::Ref<'a, W, __GROST_READ_BUF__, __GROST_BUFFER__, $crate::__private::flavors::Groto>>,
        T::Output: Sized,
      {
        type Output = $crate::__private::flavors::groto::PackedDecoder<'a, T::Output, __GROST_READ_BUF__, __GROST_BUFFER__, W>;
      }
    )*
  };
  (@repeated $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<'a, T, W, __GROST_READ_BUF__, __GROST_BUFFER__, const TAG: ::core::primitive::u32, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::state::State<
        $crate::__private::state::Ref<
          'a,
          $crate::__private::flavors::Repeated<W, TAG>,
          __GROST_READ_BUF__,
          __GROST_BUFFER__,
          $crate::__private::flavors::Groto,
        >
      > for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        $crate::__private::flavors::Repeated<W, TAG>: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'a,
        T: $crate::__private::state::State<$crate::__private::state::Ref<'a, W, __GROST_READ_BUF__, __GROST_BUFFER__, $crate::__private::flavors::Groto>>,
        T::Output: Sized,
      {
        type Output = $crate::__private::flavors::groto::RepeatedDecoderBuffer<'a, T::Output, __GROST_READ_BUF__, __GROST_BUFFER__, W, TAG>;
      }
    )*
  };
}

macro_rules! default_wire_format {
  (@bytes $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<$($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::flavors::DefaultBytesWireFormat<$crate::__private::flavors::Groto> for $ty {
        type Format = $crate::__private::flavors::groto::LengthDelimited;
      }
    )*
  };
  (@packed $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::flavors::DefaultListWireFormat<$crate::__private::flavors::Groto> for $ty {
        type Format<V> = $crate::__private::flavors::Packed<V> where V: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'static;
      }
    )*
  };
  (@repeated $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::flavors::DefaultRepeatedWireFormat<$crate::__private::flavors::Groto> for $ty {
        type Format<V, const TAG: u32> = $crate::__private::flavors::Repeated<V, TAG> where V: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto> + 'static;
      }
    )*
  };
}

macro_rules! encode_list {
  (@bytes $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<$($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Encode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto> for $ty
      {
        encode_list!(@encode_bridge([u8] as $crate::__private::flavors::groto::LengthDelimited));
      }

      impl<$($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::PartialEncode<$crate::__private::flavors::groto::LengthDelimited, $crate::__private::flavors::Groto> for $ty
      {
        encode_list!(@partial_encode_bridge([u8] as $crate::__private::flavors::groto::LengthDelimited));
      }
    )*
  };
  (@packed $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, W, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Encode<$crate::__private::flavors::Packed<W>, $crate::__private::flavors::Groto> for $ty
      where
        T: $crate::__private::Encode<W, $crate::__private::flavors::Groto>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        encode_list!(@encode_bridge([T] as $crate::__private::flavors::Packed<W>));
      }

      impl<T, W, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::PartialEncode<$crate::__private::flavors::Packed<W>, $crate::__private::flavors::Groto> for $ty
      where
        T: $crate::__private::PartialEncode<W, $crate::__private::flavors::Groto>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        encode_list!(@partial_encode_bridge([T] as $crate::__private::flavors::Packed<W>));
      }
    )*
  };
  (@repeated $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, W $(, $($tg:$t)*)? $(, $(const $g: usize)*)?, const TAG: u32> $crate::__private::Encode<$crate::__private::flavors::Repeated<W, TAG>, $crate::__private::flavors::Groto> for $ty
      where
        T: $crate::__private::Encode<W, $crate::__private::flavors::Groto>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        encode_list!(@encode_bridge([T] as $crate::__private::flavors::Repeated<W, TAG>));
      }

      impl<T, W $(, $($tg:$t)*)? $(, $(const $g: usize)*)?, const TAG: u32> $crate::__private::PartialEncode<$crate::__private::flavors::Repeated<W, TAG>, $crate::__private::flavors::Groto> for $ty
      where
        T: $crate::__private::PartialEncode<W, $crate::__private::flavors::Groto>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        encode_list!(@partial_encode_bridge([T] as $crate::__private::flavors::Repeated<W, TAG>));
      }
    )*
  };
  (@borrow $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<'b, T: 'b, W, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Encode<
        $crate::__private::flavors::Borrowed<'b, $crate::__private::flavors::Packed<W>>,
        $crate::__private::flavors::Groto
      > for $ty
      where
        T: $crate::__private::Encode<W, $crate::__private::flavors::Groto>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        encode_list!(@encode_bridge([&'b T] as $crate::__private::flavors::Borrowed<'b, $crate::__private::flavors::Packed<W>>));
      }

      impl<'b, T: 'b, W, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::PartialEncode<
        $crate::__private::flavors::Borrowed<'b, $crate::__private::flavors::Packed<W>>,
        $crate::__private::flavors::Groto
      > for $ty
      where
        T: $crate::__private::PartialEncode<W, $crate::__private::flavors::Groto>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        encode_list!(@partial_encode_bridge([&'b T] as $crate::__private::flavors::Borrowed<'b, $crate::__private::flavors::Packed<W>>));
      }

      impl<'b, T: 'b, W, const TAG: u32, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Encode<
        $crate::__private::flavors::Borrowed<'b, $crate::__private::flavors::Repeated<W, TAG>>,
        $crate::__private::flavors::Groto
      > for $ty
      where
        T: $crate::__private::Encode<W, $crate::__private::flavors::Groto>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        encode_list!(@encode_bridge([&'b T] as $crate::__private::flavors::Borrowed<'b, $crate::__private::flavors::Repeated<W, TAG>>));
      }

      impl<'b, T: 'b, W, const TAG: u32, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::encode::PartialEncode<
        $crate::__private::flavors::Borrowed<'b, $crate::__private::flavors::Repeated<W, TAG>>,
        $crate::__private::flavors::Groto
      > for $ty
      where
        T: $crate::__private::encode::PartialEncode<W, $crate::__private::flavors::Groto>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Groto>,
      {
        encode_list!(@partial_encode_bridge([&'b T] as $crate::__private::flavors::Borrowed<'b, $crate::__private::flavors::Repeated<W, TAG>>));
      }
    )*
  };
  (@partial_encode_bridge($ty:ty as $wf:ty)) => {
    fn partial_encode_raw<WB>(
      &self,
      context: &$crate::__private::flavors::groto::Context,
      buf: impl ::core::convert::Into<$crate::__private::buffer::WriteBuf<WB>>,
      selector: &Self::Selector,
    ) -> ::core::result::Result<usize, $crate::__private::error::EncodeError<Groto>>
    where
      WB: $crate::__private::buffer::BufMut,
    {
      <$ty as $crate::__private::PartialEncode<$wf, $crate::__private::flavors::Groto>>::partial_encode_raw(self.as_ref(), context, buf, selector)
    }

    fn partial_encoded_raw_len(&self, context: &$crate::__private::flavors::groto::Context, selector: &Self::Selector) -> usize {
      <$ty as $crate::__private::PartialEncode<$wf, $crate::__private::flavors::Groto>>::partial_encoded_raw_len(self.as_ref(), context, selector)
    }

    fn partial_encode<WB>(
      &self,
      context: &$crate::__private::flavors::groto::Context,
      buf: impl ::core::convert::Into<$crate::__private::buffer::WriteBuf<WB>>,
      selector: &Self::Selector,
    ) -> ::core::result::Result<usize, $crate::__private::error::EncodeError<Groto>>
    where
      WB: $crate::__private::buffer::BufMut,
    {
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
  (@encode_bridge($ty:ty as $wf:ty)) => {
    fn encode_raw<WB>(
      &self,
      context: &$crate::__private::flavors::groto::Context,
      buf: impl ::core::convert::Into<$crate::__private::buffer::WriteBuf<WB>>,
    ) -> ::core::result::Result<usize, $crate::__private::error::EncodeError<Groto>>
    where
      WB: $crate::__private::buffer::BufMut,
    {
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

    fn encode<WB>(
      &self,
      context: &$crate::__private::flavors::groto::Context,
      buf: impl ::core::convert::Into<$crate::__private::buffer::WriteBuf<WB>>,
    ) -> ::core::result::Result<usize, $crate::__private::error::EncodeError<Groto>>
    where
      WB: $crate::__private::buffer::BufMut,
    {
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
}

macro_rules! flatten_state {
  ($($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )? > $crate::__private::state::State<
        $crate::__private::convert::Extracted
      > for $ty {
        type Output = $ty;
      }

      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::state::State<$crate::__private::convert::Extracted<$crate::__private::convert::Innermost>> for $ty
      where
        T: $crate::__private::state::State<$crate::__private::convert::Extracted<$crate::__private::convert::Innermost>>,
      {
        type Output = T::Output;
      }

      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::state::State<$crate::__private::convert::Extracted<$crate::__private::convert::Inner>> for $ty {
        type Output = T;
      }
    )*
  };
}

macro_rules! partial_state {
  ($($(:< $($tg:ident: $t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])? => $output:ty),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::state::State<$crate::__private::state::Partial<$crate::__private::flavors::Groto>> for $ty
      where
        T: $crate::__private::state::State<$crate::__private::state::Partial<$crate::__private::flavors::Groto>>,
        T::Output: Sized,
      {
        type Output = $output;
      }
    )*
  };
}

macro_rules! length {
  ($($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: $gty:ty),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $( $(const $g: $gty),* )? > $crate::__private::encode::Length for $ty {
        #[inline]
        fn length(&self) -> usize {
          <$ty>::len(self)
        }
      }
    )*
  };
}

macro_rules! selectable {
  ($($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::selection::Selectable<$crate::__private::flavors::Groto> for $ty
      where
        T: $crate::__private::selection::Selectable<$crate::__private::flavors::Groto>,
      {
        type Selector = T::Selector;
      }
    )*
  };
}

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

#[cfg(any(feature = "std", feature = "alloc"))]
mod vec;

mod arrayvec;
mod smallvec;
mod tinyvec;
