use super::{
  buffer::{Buffer, ReadBuf},
  error::Error,
  flavors::{Flavor, WireFormat},
};

pub use slice::BytesSlice;
pub use str::Str;

mod slice;
mod str;

/// A trait for fully decoding types from a borrowed byte slice.
///
/// `Decode` is intended for decoding entire data structures
/// from a buffer while borrowing the input data.
///
/// ## Type Parameters
///
/// - `'de`: Lifetime of the input data.
/// - `F`: The decoding flavor (e.g., [`Groto`](crate::flavors::Groto) or other implementations) implementing the [`Flavor`] trait.
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
    B: ReadBuf + 'de,
    UB: Buffer<F::Unknown<B>> + 'de;

  /// Decodes an instance of this type from a length-delimited byte buffer.
  ///
  /// The input buffer is expected to be length-prefixed with a `u32` encoded in varint format.
  fn decode_length_delimited(context: &'de F::Context, src: B) -> Result<(usize, O), F::Error>
  where
    O: Sized + 'de,
    B: ReadBuf + 'de,
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

/// A trait for merging the input type `I` into the current type `Self`.
pub trait Merge<'de, F, W, I>: Default
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Merges the input type `I` into the current type `Self`.
  fn merge(&mut self, other: I) -> Result<(), F::Error>;
}

impl<'de, F, W, I, T> Merge<'de, F, W, Option<I>> for Option<T>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  T: Merge<'de, F, W, I>,
{
  fn merge(&mut self, other: Option<I>) -> Result<(), <F as Flavor>::Error> {
    match other {
      Some(value) => {
        if let Some(inner) = self {
          inner.merge(value)
        } else {
          let mut this = T::default();
          this.merge(value).map(|_| {
            *self = Some(this);
          })
        }
      }
      None => Ok(()),
    }
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
          B: ReadBuf + 'de,
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
          B: ReadBuf + 'de,
          UB: Buffer<<F as Flavor>::Unknown<B>> + 'de
        {
          T::decode_length_delimited(context, src)
        }
      }

      impl<F, W, I, O, T> $crate::__private::convert::Transform<I, O, W, F> for $ty
      where
        F: Flavor + ?Sized,
        W: WireFormat<F>,
        T: $crate::__private::convert::Transform<I, O, W, F> + Sized,
      {
        fn transform(input: I) -> Result<O, F::Error> {
          T::transform(input)
        }
      }

      impl<F, W, I, O, T> $crate::__private::convert::PartialTransform<I, O, W, F> for $ty
      where
        F: Flavor + ?Sized,
        W: WireFormat<F>,
        I: $crate::__private::selection::Selectable<F, Selector = T::Selector>,
        O: $crate::__private::selection::Selectable<F, Selector = T::Selector>,
        T: $crate::__private::convert::PartialTransform<I, O, W, F> + $crate::__private::selection::Selectable<F>,
      {
        fn partial_transform(input: I, selector: &Self::Selector) -> Result<O, F::Error> {
          T::partial_transform(input, selector)
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
