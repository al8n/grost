use crate::selection::Selectable;

use super::{
  buffer::{Buffer, ReadBuf},
  error::Error,
  flavors::{Flavor, WireFormat},
};

/// A trait for fully decoding types from a borrowed byte slice.
///
/// `Decode` is intended for decoding entire data structures
/// from a buffer while borrowing the input data.
///
/// ## Type Parameters
///
/// - `'de`: Lifetime of the input data.
/// - `F`: The decoding flavor (e.g., [`Network`](crate::flavors::Network) or other implementations) implementing the [`Flavor`] trait.
/// - `W`: The wire format strategy of the flavor.
/// - `O`: The output type resulting from decoding.
/// - `B`: The buffer implementation used to store the unknown data during decoding (defaults to `()`, will ignore the unknown data).
pub trait Decode<'de, F, W, O, B = &'de [u8], UB = ()>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Decodes an instance from a raw byte slice.
  ///
  /// Returns a tuple with the number of bytes consumed and the decoded output.
  fn decode(context: &'de F::Context, src: B) -> Result<(usize, O), F::Error>
  where
    O: Sized + 'de,
    B: ReadBuf<'de>,
    UB: Buffer<F::Unknown<B>> + 'de;

  /// Decodes an instance of this type from a length-delimited byte buffer.
  ///
  /// The input buffer is expected to be length-prefixed with a `u32` encoded in varint format.
  fn decode_length_delimited(context: &'de F::Context, src: B) -> Result<(usize, O), F::Error>
  where
    O: Sized + 'de,
    B: ReadBuf<'de> + 'de,
    UB: Buffer<F::Unknown<B>> + 'de,
  {
    let as_bytes = src.as_bytes();
    let (len_size, len) = varing::decode_u32_varint(as_bytes).map_err(Error::from)?;
    let src_len = src.len();
    let len = len as usize;
    let total = len_size + len;
    if total > src_len {
      return Err(Error::buffer_underflow().into());
    }

    if len_size >= src_len {
      return Err(Error::buffer_underflow().into());
    }

    Self::decode(context, src.slice(len_size..total))
  }
}

/// A data structure that can be deserialized without borrowing any data from the source buffer.
pub trait DecodeOwned<F, W, O, B, UB>: for<'de> Decode<'de, F, W, O, B, UB>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
}

impl<F, W, O, B, UB, T> DecodeOwned<F, W, O, B, UB> for T
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  T: for<'de> Decode<'de, F, W, O, B, UB>,
{
}

/// A trait for transforming the input type `I` into the current type `Self`.
pub trait Transform<F, W, I>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Transforms from the input type `I` into the current type `Self`.
  fn transform(input: I) -> Result<Self, F::Error>
  where
    Self: Sized;
}

/// A trait for partially transforming the input type `I` into the current type `Self`.
pub trait PartialTransform<F, W, I>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  I: Selectable<F, Selector = Self::Selector>,
  Self: Selectable<F>,
{
  /// Partially transforms from the input type `I` into the current type `Self`.
  ///
  /// If there is nothing selected, it returns `Ok(None)`.
  fn partial_transform(input: I, selector: &I::Selector) -> Result<Option<Self>, F::Error>
  where
    Self: Sized;
}

/// A trait for transforming the current type into another type `O`.
pub trait TransformInto<F, W, O>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Transforms the current type into the output type `O`.
  fn transform_into(self) -> Result<O, F::Error>
  where
    Self: Sized;
}

impl<F, W, I, T> TransformInto<F, W, T> for I
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  T: Transform<F, W, I> + Sized,
{
  fn transform_into(self) -> Result<T, F::Error> {
    T::transform(self)
  }
}

/// A trait for partially transforming the current type into another type `O`.
pub trait PartialTransformInto<F, W, O>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  O: Selectable<F, Selector = Self::Selector>,
  Self: Selectable<F>,
{
  /// Partially transforms the current type into the output type `O`.
  ///
  /// If there is nothing selected, it returns `Ok(None)`.
  fn partial_transform_into(self, selector: &Self::Selector) -> Result<Option<O>, F::Error>
  where
    Self: Sized;
}

impl<F, W, I, T> PartialTransformInto<F, W, T> for I
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  T: PartialTransform<F, W, I> + Sized + Selectable<F>,
  I: Selectable<F, Selector = <T as Selectable<F>>::Selector>,
{
  fn partial_transform_into(
    self,
    selector: &<T as Selectable<F>>::Selector,
  ) -> Result<Option<T>, <F as Flavor>::Error>
  where
    Self: Sized + Selectable<F, Selector = <T as Selectable<F>>::Selector>,
  {
    T::partial_transform(self, selector)
  }
}

#[cfg(any(feature = "std", feature = "alloc", feature = "triomphe_0_1"))]
macro_rules! deref_decode_impl {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<'de, F, W, O, B, UB, T> Decode<'de, F, W, O, B, UB> for $ty
      where
        F: Flavor + ?Sized,
        W: WireFormat<F>,
        T: Decode<'de, F, W, O, B, UB> + ?Sized,
      {
        fn decode(context: &'de <F as Flavor>::Context, src: B) -> Result<(usize, O), <F as Flavor>::Error>
        where
          O: Sized + 'de,
          B: ReadBuf<'de>,
          UB: Buffer<<F as Flavor>::Unknown<B>> + 'de
        {
          T::decode(context, src)
        }

        fn decode_length_delimited(
          context: &'de <F as Flavor>::Context,
          src: B,
        ) -> Result<(usize, O), <F as Flavor>::Error>
        where
          O: Sized + 'de,
          B: ReadBuf<'de>,
          UB: Buffer<<F as Flavor>::Unknown<B>> + 'de
        {
          T::decode_length_delimited(context, src)
        }
      }

      impl<F, W, I, T> Transform<F, W, I> for $ty
      where
        F: Flavor + ?Sized,
        W: WireFormat<F>,
        T: Transform<F, W, I> + Sized,
      {
        fn transform(input: I) -> Result<$ty, F::Error> {
          T::transform(input).map(Into::into)
        }
      }
    )*
  };
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::{boxed::Box, rc::Rc, sync::Arc};

  deref_decode_impl!(Box<T>, Rc<T>, Arc<T>);
};

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  use triomphe_0_1::Arc;

  deref_decode_impl!(Arc<T>);
};
