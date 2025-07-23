use super::{
  buffer::{ReadBuf, UnknownBuffer},
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
pub unsafe trait EquivalentDecode<'a, Rhs, W, RB, B, F>
where
  Self: Decode<'a, Self::WireFormat, RB, B, Self::Flavor>,
  Rhs: Decode<'a, W, RB, B, F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  /// The wire format for Self
  type WireFormat: WireFormat<Self::Flavor>;

  /// The flavor for Self
  type Flavor: Flavor + ?Sized;
}

unsafe impl<'a, T, W, RB, B, F> EquivalentDecode<'a, T, W, RB, B, F> for T
where
  T: Decode<'a, W, RB, B, F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
  type Flavor = F;
}

// unsafe impl<'a, T, W, RB, B, F> EquivalentDecode<'a, &T, W, RB, B, F> for T
// where
//   T: Decode<'a, W, RB, B, F> + ?Sized,
//   W: WireFormat<F>,
//   F: Flavor + ?Sized,
// {
//   type WireFormat = W;
//   type Flavor = F;
// }

// unsafe impl<'a, T, W, RB, B, F> EquivalentDecode<'a, T, W, RB, B, F> for &T
// where
//   T: Decode<'a, W, RB, B, F> + ?Sized,
//   W: WireFormat<F>,
//   F: Flavor + ?Sized,
// {
//   type WireFormat = W;
//   type Flavor = F;
// }

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
/// - `RB`: The type of the read buffer used for decoding, which must implement [`ReadBuf`].
/// - `B`: The buffer implementation used to store the unknown data during decoding, which must implement [`Buffer`].
pub trait Decode<'de, W, RB, B, F>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Decodes an instance from a raw byte slice.
  ///
  /// Returns a tuple with the number of bytes consumed and the decoded output.
  fn decode(context: &'de F::Context, src: RB) -> Result<(usize, Self), F::Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf + 'de,
    B: UnknownBuffer<RB, F> + 'de;

  /// Decodes an instance of this type from a length-delimited byte buffer.
  ///
  /// The input buffer is expected to be length-prefixed with a `u32` encoded in varint format.
  fn decode_length_delimited(context: &'de F::Context, src: RB) -> Result<(usize, Self), F::Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf + 'de,
    B: UnknownBuffer<RB, F> + 'de,
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

  /// Decodes an instance from a raw byte slice, merging the result into the current instance.
  fn merge_decode(&mut self, ctx: &'de F::Context, src: RB) -> Result<usize, F::Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf + 'de,
    B: UnknownBuffer<RB, F> + 'de,
  {
    let _ = ctx;
    let _ = src;
    Err(Error::unmergeable(core::any::type_name::<Self>(), W::WIRE_TYPE).into())
  }
}

/// A data structure that can be deserialized without borrowing any data from the source buffer.
pub trait DecodeOwned<W, RB, B, F>: for<'de> Decode<'de, W, RB, B, F>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
}

impl<W, RB, B, F, T> DecodeOwned<W, RB, B, F> for T
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  T: for<'de> Decode<'de, W, RB, B, F>,
{
}

#[cfg(any(feature = "std", feature = "alloc", feature = "triomphe_0_1"))]
macro_rules! deref_decode_impl {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<'de, W, RB, B, F, T> Decode<'de, W, RB, B, F> for $ty
      where
        F: Flavor + ?Sized,
        W: WireFormat<F>,
        T: Decode<'de, W, RB, B, F>,
      {
        fn decode(context: &'de <F as Flavor>::Context, src: RB) -> Result<(usize, Self), <F as Flavor>::Error>
        where
          Self: Sized + 'de,
          RB: ReadBuf + 'de,
          B: UnknownBuffer<RB, F> + 'de
        {
          T::decode(context, src).map(|(size, output)| (size, Self::new(output)))
        }

        fn decode_length_delimited(
          context: &'de <F as Flavor>::Context,
          src: RB,
        ) -> Result<(usize, Self), <F as Flavor>::Error>
        where
          Self: Sized + 'de,
          RB: ReadBuf + 'de,
          B: UnknownBuffer<RB, F> + 'de
        {
          T::decode_length_delimited(context, src).map(|(size, output)| (size, Self::new(output)))
        }

        fn merge_decode(&mut self, ctx: &'de <F as Flavor>::Context, src: RB) -> Result<usize, <F as Flavor>::Error>
        where
          Self: Sized + 'de,
          RB: ReadBuf + 'de,
          B: UnknownBuffer<RB, F> + 'de
        {
          if let Some(val) = <$ty>::get_mut(self) {
            return T::merge_decode(val, ctx, src);
          }

          Err(::core::convert::Into::into(
            $crate::error::Error::custom(concat!(
              "cannot merge decode into ",
              stringify!($ty),
              " as there are other references to the same allocation"
            ))
          ))
        }
      }
    )*
  };
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::{boxed::Box, rc::Rc, sync::Arc};

  impl<'de, W, RB, B, F, T> Decode<'de, W, RB, B, F> for Box<T>
  where
    F: Flavor + ?Sized,
    W: WireFormat<F>,
    T: Decode<'de, W, RB, B, F>,
  {
    fn decode(
      context: &'de <F as Flavor>::Context,
      src: RB,
    ) -> Result<(usize, Self), <F as Flavor>::Error>
    where
      Self: Sized + 'de,
      RB: ReadBuf + 'de,
      B: UnknownBuffer<RB, F> + 'de,
    {
      T::decode(context, src).map(|(size, output)| (size, Box::new(output)))
    }

    fn decode_length_delimited(
      context: &'de <F as Flavor>::Context,
      src: RB,
    ) -> Result<(usize, Self), <F as Flavor>::Error>
    where
      Self: Sized + 'de,
      RB: ReadBuf + 'de,
      B: UnknownBuffer<RB, F> + 'de,
    {
      T::decode_length_delimited(context, src).map(|(size, output)| (size, Box::new(output)))
    }

    fn merge_decode(
      &mut self,
      ctx: &'de <F as Flavor>::Context,
      src: RB,
    ) -> Result<usize, <F as Flavor>::Error>
    where
      Self: Sized + 'de,
      RB: ReadBuf + 'de,
      B: UnknownBuffer<RB, F> + 'de,
    {
      T::merge_decode(&mut **self, ctx, src)
    }
  }

  deref_decode_impl!(Arc<T>, Rc<T>);
};

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  use triomphe_0_1::Arc;

  deref_decode_impl!(Arc<T>);
};

#[test]
fn t() {}
