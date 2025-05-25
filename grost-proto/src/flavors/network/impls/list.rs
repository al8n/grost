use crate::{
  Innermost, State,
  encode::{Encode, PartialEncode},
  flavors::{
    Network, WireFormat,
    network::{Borrowed, Context, Error, Flatten, LengthDelimited, Packed, WireType},
  },
  selection::{Selectable, Selector},
};

macro_rules! encode {
  (@encode_methods($ty:ty)) => {
    fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
      let mut offset = 0;
      let buf_len = buf.len();
      for value in self.iter() {
        if offset >= buf_len {
          return Err(Error::insufficient_buffer(
            <Self as $crate::__private::Encode<Network, $ty>>::encoded_len(self, context),
            buf_len,
          ));
        }

        offset += value.encode(context, &mut buf[offset..]).map_err(|e| {
          e.update(
            <Self as $crate::__private::Encode<Network, $ty>>::encoded_len(self, context),
            buf_len,
          )
        })?;
      }
      Ok(offset)
    }

    fn encoded_len(&self, context: &Context) -> usize {
      match W::WIRE_TYPE {
        WireType::Varint | WireType::LengthDelimited => {
          self.iter().map(|v| v.encoded_len(context)).sum()
        }
        WireType::Fixed8 => self.len(),
        WireType::Fixed16 => self.len() * 2,
        WireType::Fixed32 => self.len() * 4,
        WireType::Fixed64 => self.len() * 8,
        WireType::Fixed128 => self.len() * 16,
        WireType::Fixed256 => self.len() * 32,
      }
    }

    fn encoded_length_delimited_len(
      &self,
      context: &$crate::__private::flavors::network::Context,
    ) -> usize {
      let encoded_len =
        <Self as $crate::__private::Encode<Network, $ty>>::encoded_len(self, context);
      if encoded_len == 0 {
        return 0;
      }

      let len_size = varing::encoded_u32_varint_len(encoded_len as u32);
      len_size + encoded_len
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
      for value in self.iter() {
        if offset >= buf_len {
          return Err(Error::insufficient_buffer(
            <Self as $crate::__private::PartialEncode<Network, $ty>>::partial_encoded_len(
              self, context, selector,
            ),
            buf_len,
          ));
        }

        offset += value
          .partial_encode(context, &mut buf[offset..], selector)
          .map_err(|e| {
            e.update(
              <Self as $crate::__private::PartialEncode<Network, $ty>>::partial_encoded_len(
                self, context, selector,
              ),
              buf_len,
            )
          })?;
      }
      Ok(offset)
    }

    fn partial_encoded_len(
      &self,
      context: &<Network as crate::flavors::Flavor>::Context,
      selector: &Self::Selector,
    ) -> usize {
      match W::WIRE_TYPE {
        WireType::Varint | WireType::LengthDelimited => self
          .iter()
          .map(|v| v.partial_encoded_len(context, selector))
          .sum(),
        WireType::Fixed8 => self.len(),
        WireType::Fixed16 => self.len() * 2,
        WireType::Fixed32 => self.len() * 4,
        WireType::Fixed64 => self.len() * 8,
        WireType::Fixed128 => self.len() * 16,
        WireType::Fixed256 => self.len() * 32,
      }
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
  (@decoded_state $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<'a, T, W, __GROST_UNKNOWN_BUFFER__, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::State<$crate::__private::convert::Decoded<'a, $crate::__private::flavors::Network, W, __GROST_UNKNOWN_BUFFER__>> for $ty
      where
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      {
        type Input = &'a [u8];
        type Output = $crate::__private::flavors::network::PackedDecoder<'a, $ty, __GROST_UNKNOWN_BUFFER__, W>;
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
  (@selectable(packed) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, W, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::selection::Selectable<$crate::__private::flavors::Network, $crate::__private::flavors::network::Packed<W>> for $ty
      where
        T: $crate::__private::selection::Selectable<$crate::__private::flavors::Network, W>,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      {
        type Selector = T::Selector;
      }
    )*
  };
  (@selectable(borrowed) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<'a, T, W, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::selection::Selectable<$crate::__private::flavors::Network, $crate::__private::flavors::network::Borrowed<'a, $crate::__private::flavors::network::Packed<W>>> for $ty
      where
        T: $crate::__private::selection::Selectable<$crate::__private::flavors::Network, W> + ?::core::marker::Sized + 'a,
        W: $crate::__private::flavors::WireFormat<$crate::__private::flavors::Network>,
      {
        type Selector = T::Selector;
      }
    )*
  };
  (@selectable(bytes) $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<$($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::selection::Selectable<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty {
        type Selector = bool;
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
list!(@decoded_state [T; N] [const N: usize], [T]);
list!(@default_wire_format [T; N] [const N: usize], [T]);
list!(@selectable(packed) [T; N] [const N: usize], [T]);
list!(@selectable(borrowed) [&'a T; N] [const N: usize], [&'a T]);
list!(@selectable(bytes) [u8; N] [const N: usize], [u8]);
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

impl<T, N, W, I> Encode<Network, Flatten<W, I>> for [N]
where
  W: WireFormat<Network>,
  I: WireFormat<Network>,
  N: State<crate::convert::Flatten<Innermost>, Output = T> + Encode<Network, W>,
  T: Encode<Network, I> + ?Sized,
{
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let buf_len = buf.len();
    let this_len = self.len();
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(
        <Self as Encode<Network, Flatten<W, I>>>::encoded_len(self, context),
        buf_len,
      ));
    }

    let mut offset = 0;
    for value in self.iter() {
      if offset >= buf_len {
        return Err(Error::insufficient_buffer(
          <Self as Encode<Network, Flatten<W, I>>>::encoded_len(self, context),
          buf_len,
        ));
      }

      offset += value.encode(context, &mut buf[offset..])?;
    }
    Ok(offset)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    self
      .iter()
      .map(|n| <N as Encode<Network, W>>::encoded_len(n, context))
      .sum()
  }
}

impl<T, N, W, I> Selectable<Network, Flatten<W, I>> for [N]
where
  W: WireFormat<Network>,
  I: WireFormat<Network>,
  N: State<crate::convert::Flatten<Innermost>, Output = T>,
  T: Selectable<Network, I> + ?Sized,
{
  type Selector = T::Selector;
}

impl<T, N, W, I> PartialEncode<Network, Flatten<W, I>> for [N]
where
  W: WireFormat<Network>,
  I: WireFormat<Network>,
  N: State<crate::convert::Flatten<Innermost>, Output = T>
    + PartialEncode<Network, W, Selector = T::Selector>,
  T: PartialEncode<Network, I> + ?Sized,
{
  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    let buf_len = buf.len();
    let this_len = self.len();
    if buf_len < this_len {
      return Err(Error::insufficient_buffer(
        <Self as PartialEncode<Network, Flatten<W, I>>>::partial_encoded_len(
          self, context, selector,
        ),
        buf_len,
      ));
    }

    let mut offset = 0;
    for value in self.iter() {
      if offset >= buf_len {
        return Err(Error::insufficient_buffer(
          <Self as PartialEncode<Network, Flatten<W, I>>>::partial_encoded_len(
            self, context, selector,
          ),
          buf_len,
        ));
      }

      offset += value.partial_encode(context, &mut buf[offset..], selector)?;
    }
    Ok(offset)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    self
      .iter()
      .map(|n| <N as PartialEncode<Network, W>>::partial_encoded_len(n, context, selector))
      .sum()
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::vec::Vec;

  list!(@flatten_state Vec<T>);
  list!(@decoded_state Vec<T>);
  list!(@default_wire_format Vec<T>);
  list!(@selectable(packed) Vec<T>);
  list!(@selectable(borrowed) Vec<&'a T>);
  list!(@selectable(bytes) Vec<u8>);
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
  list!(@decoded_state SmallVec<[T; N]> [const N: usize]);
  list!(@default_wire_format SmallVec<[T; N]> [const N: usize]);
  list!(@selectable(packed) SmallVec<[T; N]> [const N: usize]);
  list!(@selectable(borrowed) SmallVec<[&'a T; N]> [const N: usize]);
  list!(@selectable(bytes) SmallVec<[u8; N]> [const N: usize]);
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
  list!(@decoded_state ArrayVec<T, N> [const N: usize]);
  list!(@default_wire_format ArrayVec<T, N> [const N: usize]);
  list!(@selectable(packed) ArrayVec<T, N> [const N: usize]);
  list!(@selectable(borrowed) ArrayVec<&'a T, N> [const N: usize]);
  list!(@selectable(bytes) ArrayVec<u8, N> [const N: usize]);
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

  list!(@flatten_state:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@decoded_state:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@default_wire_format:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@selectable(packed):<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@selectable(borrowed):<A: tinyvec_1::Array<Item = &'a T>>: ArrayVec<A>);
  list!(@selectable(bytes):<A: tinyvec_1::Array<Item = u8>>: ArrayVec<A>);
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
    list!(@decoded_state:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@default_wire_format:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@selectable(packed):<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@selectable(borrowed):<A: tinyvec_1::Array<Item = &'a T>>: TinyVec<A>);
    list!(@selectable(bytes):<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>);
    list!(
      @encode_as_slice:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>
    );
    list!(
      @encode_as_bytes:<A: tinyvec_1::Array<Item = u8>>: TinyVec<A>
    );
  };
};
