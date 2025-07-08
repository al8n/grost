use super::{
  buffer::{Buffer, ReadBuf},
  error::Error,
  flavors::{Flavor, WireFormat},
};

pub use slice::BytesSlice;
pub use str::Str;

mod slice;
mod str;

/// A marker trait indicating that two types produce equivalent encoded output
/// despite potentially having different wire formats or internal representations.
///
/// ## Safety
///
/// This trait is unsafe because incorrect implementation can lead to data corruption
/// or incorrect behavior in systems that rely on encoding equivalence. Implementers
/// must ensure that:
///
/// 1. all methods of `Encode` for `Self` and `O` produce the same results for equivalent values
/// 2. The equivalence holds across all possible contexts (both `F::Context` and `<Self::Flavor as Flavor>::Context`)
///
/// # Example
///
/// ```ignore
/// struct MyStr(str);
///
/// unsafe impl EquivalentDecode<MyStr, LengthDelimited, Groto> for str {
///   type Flavor = Groto;
///   type WireFormat = LengthDelimited;
/// }
/// ```
pub unsafe trait EquivalentDecode<'a, O, Rhs, W, RB, B, F>
where
  Self: Decode<'a, O, Self::WireFormat, RB, B, Self::Flavor>,
  Rhs: Decode<'a, O, W, RB, B, F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  /// The wire format for Self
  type WireFormat: WireFormat<Self::Flavor>;

  /// The flavor for Self
  type Flavor: Flavor + ?Sized;
}

unsafe impl<'a, T, O, W, RB, B, F> EquivalentDecode<'a, O, T, W, RB, B, F> for T
where
  T: Decode<'a, O, W, RB, B, F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
  type Flavor = F;
}

unsafe impl<'a, T, O, W, RB, B, F> EquivalentDecode<'a, O, &T, W, RB, B, F> for T
where
  T: Decode<'a, O, W, RB, B, F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
  type Flavor = F;
}

unsafe impl<'a, T, O, W, RB, B, F> EquivalentDecode<'a, O, T, W, RB, B, F> for &T
where
  T: Decode<'a, O, W, RB, B, F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
  type Flavor = F;
}

/// A trait for fully decoding types from a borrowed byte slice.
///
/// `Decode` is intended for decoding entire data structures
/// from a buffer while borrowing the input data.
///
/// ## Type Parameters
///
/// - `'de`: Lifetime of the input data.
/// - `F`: The decoding flavor (e.g., [`Groto`](crate::flavors::Groto) or other implementations) implementing the [`Flavor`] trait.
/// - `W`: The wire format strategy of the flavor, which must implement [`WireFormat<F>`].
/// - `O`: The output type resulting from decoding.
/// - `RB`: The type of the read buffer used for decoding, which must implement [`ReadBuf`].
/// - `B`: The buffer implementation used to store the unknown data during decoding, which must implement [`Buffer`].
pub trait Decode<'de, O, W, RB, B, F>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Decodes an instance from a raw byte slice.
  ///
  /// Returns a tuple with the number of bytes consumed and the decoded output.
  fn decode(context: &'de F::Context, src: RB) -> Result<(usize, O), F::Error>
  where
    O: Sized + 'de,
    RB: ReadBuf + 'de,
    B: Buffer<F::Unknown<RB>> + 'de;

  /// Decodes an instance of this type from a length-delimited byte buffer.
  ///
  /// The input buffer is expected to be length-prefixed with a `u32` encoded in varint format.
  fn decode_length_delimited(context: &'de F::Context, src: RB) -> Result<(usize, O), F::Error>
  where
    O: Sized + 'de,
    RB: ReadBuf + 'de,
    B: Buffer<F::Unknown<RB>> + 'de,
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

impl<'de, O, W, RB, B, F, T> Decode<'de, O, W, RB, B, F> for &T
where
  T: Decode<'de, O, W, RB, B, F> + ?Sized,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  fn decode(context: &'de <F as Flavor>::Context, src: RB) -> Result<(usize, O), <F as Flavor>::Error>
  where
    O: Sized + 'de,
    RB: ReadBuf + 'de,
    B: Buffer<<F as Flavor>::Unknown<RB>> + 'de
  {
    <T as Decode<'de, O, W, RB, B, F>>::decode(context, src)
  }

  fn decode_length_delimited(context: &'de <F as Flavor>::Context, src: RB) -> Result<(usize, O), <F as Flavor>::Error>
  where
    O: Sized + 'de,
    RB: ReadBuf + 'de,
    B: Buffer<<F as Flavor>::Unknown<RB>> + 'de,
  {
    <T as Decode<'de, O, W, RB, B, F>>::decode_length_delimited(context, src)
  }
}

/// A data structure that can be deserialized without borrowing any data from the source buffer.
pub trait DecodeOwned<O, W, RB, B, F>: for<'de> Decode<'de, O, W, RB, B, F>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
}

impl<O, W, RB, B, F, T> DecodeOwned<O, W, RB, B, F> for T
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  T: for<'de> Decode<'de, O, W, RB, B, F>,
{
}

#[cfg(any(feature = "std", feature = "alloc", feature = "triomphe_0_1"))]
macro_rules! deref_decode_impl {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<'de, O, W, RB, B, F, T> Decode<'de, O, W, RB, B, F> for $ty
      where
        F: Flavor + ?Sized,
        W: WireFormat<F>,
        T: Decode<'de, O, W, RB, B, F> + ?Sized,
      {
        fn decode(context: &'de <F as Flavor>::Context, src: RB) -> Result<(usize, O), <F as Flavor>::Error>
        where
          O: Sized + 'de,
          RB: ReadBuf + 'de,
          B: Buffer<<F as Flavor>::Unknown<RB>> + 'de
        {
          T::decode(context, src)
        }

        fn decode_length_delimited(
          context: &'de <F as Flavor>::Context,
          src: RB,
        ) -> Result<(usize, O), <F as Flavor>::Error>
        where
          O: Sized + 'de,
          RB: ReadBuf + 'de,
          B: Buffer<<F as Flavor>::Unknown<RB>> + 'de
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
