use crate::{encode::{Encode, PartialEncode}, flavors::{network::{Context, EncodeError, LengthDelimited, WireType}, DefaultWireFormat, Network, WireFormat}, reflection::{Reflectable, Type, TypeReflection}, selection::Selector};

macro_rules! list {
  (@flatten_state $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )? > $crate::__private::convert::State<
        $crate::__private::convert::Flatten
      > for $ty {
        type Input = $ty;
        type Output = $ty;
      }

      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::convert::State<$crate::__private::convert::Flatten<$crate::__private::convert::Base>> for $ty
      where
        T: $crate::__private::convert::State<$crate::__private::convert::Flatten<$crate::__private::convert::Base>>,
      {
        type Input = T::Input;
        type Output = T::Output;
      }
    )*
  };
  (@wire_format $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::flavors::DefaultWireFormat<$crate::__private::flavors::Network> for $ty
      where
        T: $crate::__private::flavors::DefaultWireFormat<$crate::__private::flavors::Network>,
        $crate::__private::reflection::TypeReflection<T>: $crate::__private::reflection::Reflectable<T, Reflection = $crate::__private::reflection::Type>,
      {
        type Format = $crate::__private::flavors::network::LengthDelimited;
      }
    )*
  };
  (@selectable $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::selection::Selectable<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty
      where
        T: $crate::__private::selection::Selectable<$crate::__private::flavors::Network, T::Format> + $crate::__private::flavors::DefaultWireFormat<$crate::__private::flavors::Network>,
        $crate::__private::reflection::TypeReflection<T>: $crate::__private::reflection::Reflectable<T, Reflection = $crate::__private::reflection::Type>,
      {
        type Selector = T::Selector;
      }
    )*
  };
  (@encode_as_slice $($(:< $($tg:ident:$t:path),+$(,)? >:)? $ty:ty $([ $(const $g:ident: usize),+$(,)? ])?),+$(,)?) => {
    $(
      impl<T, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty
      where
        T: $crate::__private::flavors::DefaultWireFormat<$crate::__private::flavors::Network> + $crate::__private::Encode<$crate::__private::flavors::Network, T::Format>,
        $crate::__private::reflection::TypeReflection<T>: $crate::__private::reflection::Reflectable<T, Reflection = Type>,
      {
        fn encode(&self, context: &$crate::__private::flavors::network::Context, buf: &mut [u8]) -> ::core::result::Result<usize, $crate::__private::flavors::network::EncodeError> {
          <[T] as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::encode(self.as_ref(), context, buf)
        }

        fn encoded_len(&self, context: &$crate::__private::flavors::network::Context) -> usize {
          <[T] as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::encoded_len(self.as_ref(), context)
        }

        fn encoded_length_delimited_len(&self, context: &$crate::__private::flavors::network::Context) -> usize {
          <[T] as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::encoded_length_delimited_len(self.as_ref(), context)
        }

        fn encode_length_delimited(
          &self,
          context: &$crate::__private::flavors::network::Context,
          buf: &mut [u8],
        ) -> ::core::result::Result<usize, $crate::__private::flavors::network::EncodeError> {
          <[T] as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::encode_length_delimited(self.as_ref(), context, buf)
        }
      }

      impl<T, $($($tg:$t),*)? $($(const $g: usize),*)?> $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited> for $ty
      where
        T: $crate::__private::flavors::DefaultWireFormat<$crate::__private::flavors::Network> + $crate::__private::PartialEncode<$crate::__private::flavors::Network, T::Format>,
        $crate::__private::reflection::TypeReflection<T>: $crate::__private::reflection::Reflectable<T, Reflection = Type>,
      {
        fn partial_encode(
          &self,
          context: &$crate::__private::flavors::network::Context,
          buf: &mut [u8],
          selector: &Self::Selector,
        ) -> ::core::result::Result<usize, $crate::__private::flavors::network::EncodeError> {
          <[T] as $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::partial_encode(self.as_ref(), context, buf, selector)
        }

        fn partial_encoded_len(
          &self,
          context: &$crate::__private::flavors::network::Context,
          selector: &Self::Selector,
        ) -> usize {
          <[T] as $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::partial_encoded_len(self.as_ref(), context, selector)
        }

        fn partial_encoded_length_delimited_len(
          &self,
          context: &$crate::__private::flavors::network::Context,
          selector: &Self::Selector,
        ) -> usize {
          <[T] as $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::partial_encoded_length_delimited_len(self.as_ref(), context, selector)
        }

        fn partial_encode_length_delimited(
          &self,
          context: &$crate::__private::flavors::network::Context,
          buf: &mut [u8],
          selector: &Self::Selector,
        ) -> ::core::result::Result<usize, $crate::__private::flavors::network::EncodeError> {
          <[T] as $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::LengthDelimited>>::partial_encode_length_delimited(self.as_ref(), context, buf, selector)
        }
      }
    )*
  };
}

list!(@flatten_state [T; N] [const N: usize], [T]);
list!(@wire_format [T; N] [const N: usize], [T]);
list!(@selectable [T; N] [const N: usize], [T]);
list!(
  @encode_as_slice [T; N] [const N: usize]
);

impl<T> Encode<Network, LengthDelimited> for [T]
where
  T: DefaultWireFormat<Network> + Encode<Network, T::Format>,
  TypeReflection<T>: Reflectable<T, Reflection = Type>,
{
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    match <T::Format as WireFormat<Network>>::WIRE_TYPE {
      WireType::Zst => Ok(0),
      WireType::LengthDelimited => {
        let mut offset = 0;
        let buf_len = buf.len();
        for value in self.iter() {
          if offset >= buf_len {
            return Err(EncodeError::insufficient_buffer(self.encoded_len(context), buf_len));
          }

          offset += value.encode_length_delimited(context, &mut buf[offset..])
            .map_err(|e| e.update(self.encoded_len(context), buf_len))?;
        }
        Ok(offset)
      },
      _ => {
        let mut offset = 0;
        let buf_len = buf.len();
        for value in self.iter() {
          if offset >= buf_len {
            return Err(EncodeError::insufficient_buffer(self.encoded_len(context), buf_len));
          }

          offset += value.encode(context, &mut buf[offset..])
            .map_err(|e| e.update(self.encoded_len(context), buf_len))?;
        }
        Ok(offset)
      }
    }
  }

  fn encoded_len(&self, context: &Context) -> usize {
    match <T::Format as WireFormat<Network>>::WIRE_TYPE {
      WireType::Zst => 0,
      WireType::Varint => self.iter().map(|v| v.encoded_len(context)).sum(),
      WireType::LengthDelimited => self.iter().map(|v| v.encoded_length_delimited_len(context)).sum(),
      WireType::Fixed8 => self.len(),
      WireType::Fixed16 => self.len() * 2,
      WireType::Fixed32 => self.len() * 4,
      WireType::Fixed64 => self.len() * 8,
      WireType::Fixed128 => self.len() * 16,
    }
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    let encoded_len = self.encoded_len(context);
    if encoded_len == 0 {
      return 0;
    }

    let len_size = varing::encoded_u32_varint_len(encoded_len as u32);
    len_size + encoded_len
  }

  fn encode_length_delimited(
    &self,
    context: &Context,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    let encoded_len = self.encoded_len(context);
    let buf_len = buf.len();
    let offset = varing::encode_u32_varint_to(encoded_len as u32, buf)
      .map_err(|e| EncodeError::from_varint_error(e).update(self.encoded_length_delimited_len(context), buf_len))?;

    let required = encoded_len + offset;
    if offset + encoded_len > buf_len {
      return Err(EncodeError::insufficient_buffer(required, buf_len));
    }

    if offset >= buf_len {
      return Err(EncodeError::insufficient_buffer(encoded_len, buf_len));
    }

    self.encode(context, &mut buf[offset..])
      .map(|v| {
        #[cfg(debug_assertions)]
        {
          crate::debug_assert_write_eq::<Self>(v, encoded_len);
        }

        required
      })
      .map_err(|e| e.update(self.encoded_length_delimited_len(context), buf_len))
  }
}

impl<T> PartialEncode<Network, LengthDelimited> for [T]
where
  T: DefaultWireFormat<Network> + PartialEncode<Network, T::Format>,
  TypeReflection<T>: Reflectable<T, Reflection = Type>,
{
  fn partial_encode(
    &self,
    context: &<Network as crate::flavors::Flavor>::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, <Network as crate::flavors::Flavor>::EncodeError> {
    // If the selector is empty, we can return 0 immediately.
    if selector.is_empty() {
      return Ok(0);
    }

    match <T::Format as WireFormat<Network>>::WIRE_TYPE {
      WireType::Zst => Ok(0),
      WireType::LengthDelimited => {
        let mut offset = 0;
        let buf_len = buf.len();
        for value in self.iter() {
          if offset >= buf_len {
            return Err(EncodeError::insufficient_buffer(self.partial_encoded_len(context, selector), buf_len));
          }

          offset += value.partial_encode_length_delimited(context, &mut buf[offset..], selector)
            .map_err(|e| e.update(self.partial_encoded_len(context, selector), buf_len))?;
        }
        Ok(offset)
      },
      _ => {
        let mut offset = 0;
        let buf_len = buf.len();
        for value in self.iter() {
          if offset >= buf_len {
            return Err(EncodeError::insufficient_buffer(self.partial_encoded_len(context, selector), buf_len));
          }

          offset += value.partial_encode(context, &mut buf[offset..], selector)
            .map_err(|e| e.update(self.partial_encoded_len(context, selector), buf_len))?;
        }
        Ok(offset)
      }
    }
  }

  fn partial_encoded_len(&self, context: &<Network as crate::flavors::Flavor>::Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    match <T::Format as WireFormat<Network>>::WIRE_TYPE {
      WireType::Zst => 0,
      WireType::Varint => self.iter().map(|v| v.partial_encoded_len(context, selector)).sum(),
      WireType::LengthDelimited => self.iter().map(|v| v.partial_encoded_length_delimited_len(context, selector)).sum(),
      WireType::Fixed8 => self.len(),
      WireType::Fixed16 => self.len() * 2,
      WireType::Fixed32 => self.len() * 4,
      WireType::Fixed64 => self.len() * 8,
      WireType::Fixed128 => self.len() * 16,
    }
  }

  fn partial_encoded_length_delimited_len(
    &self,
    context: &<Network as crate::flavors::Flavor>::Context,
    selector: &Self::Selector,
  ) -> usize {
    if selector.is_empty() {
      return 0;
    }

    let encoded_len = self.partial_encoded_len(context, selector);
    if encoded_len == 0 {
      return 0;
    }

    let len_size = varing::encoded_u32_varint_len(encoded_len as u32);
    len_size + encoded_len
  }

  fn partial_encode_length_delimited(
    &self,
    context: &<Network as crate::flavors::Flavor>::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, <Network as crate::flavors::Flavor>::EncodeError> {
    if selector.is_empty() {
      return Ok(0);
    }

    let encoded_len = self.partial_encoded_len(context, selector);
    if encoded_len == 0 {
      return Ok(0);
    }

    let buf_len = buf.len();
    let offset = varing::encode_u32_varint_to(encoded_len as u32, buf)
      .map_err(|e| EncodeError::from_varint_error(e).update(self.partial_encoded_length_delimited_len(context, selector), buf_len))?;

    let required = encoded_len + offset;
    if offset + encoded_len > buf_len {
      return Err(EncodeError::insufficient_buffer(required, buf_len));
    }

    if offset >= buf_len {
      return Err(EncodeError::insufficient_buffer(encoded_len, buf_len));
    }

    self.partial_encode(context, &mut buf[offset..], selector)
      .map(|v| {
        #[cfg(debug_assertions)]
        {
          crate::debug_assert_write_eq::<Self>(v, encoded_len);
        }

        required
      })
      .map_err(|e| e.update(self.partial_encoded_length_delimited_len(context, selector), buf_len))
  }
}


#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::vec::Vec;

  list!(@flatten_state Vec<T>);
  list!(@wire_format Vec<T>);
  list!(@selectable Vec<T>);
  list!(
    @encode_as_slice Vec<T>
  );
};

#[cfg(feature = "smallvec_1")]
const _: () = {
  use smallvec_1::SmallVec;

  list!(@flatten_state SmallVec<[T; N]> [const N: usize]);
  list!(@wire_format SmallVec<[T; N]> [const N: usize]);
  list!(@selectable SmallVec<[T; N]> [const N: usize]);
  list!(
    @encode_as_slice SmallVec<[T; N]> [const N: usize]
  );
};

#[cfg(feature = "arrayvec_0_7")]
const _: () = {
  use arrayvec_0_7::ArrayVec;

  list!(@flatten_state ArrayVec<T, N> [const N: usize]);
  list!(@wire_format ArrayVec<T, N> [const N: usize]);
  list!(@selectable ArrayVec<T, N> [const N: usize]);
  list!(
    @encode_as_slice ArrayVec<T, N> [const N: usize]
  );
};

#[cfg(feature = "tinyvec_1")]
const _: () = {
  use tinyvec_1::ArrayVec;

  list!(@flatten_state:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@wire_format:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);
  list!(@selectable:<A: tinyvec_1::Array<Item = T>>: ArrayVec<A>);

  #[cfg(any(feature = "std", feature = "alloc"))]
  const _: () = {
    use tinyvec_1::TinyVec;

    list!(@flatten_state:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@wire_format:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
    list!(@selectable:<A: tinyvec_1::Array<Item = T>>: TinyVec<A>);
  };
};
