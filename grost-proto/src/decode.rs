use super::{
  buffer::{Buf, Buffer},
  error::Error,
  flavors::{Flavor, WireFormat},
  selection::Selectable,
};

/// A trait for decoding types from a byte slice, partially guided by a selector.
///
/// `PartialDecode` is designed for decoding a subset of the fields or structure
/// of a type based on a selector. This is useful when only a portion of a data
/// structure is needed, allowing for more efficient parsing.
///
/// ## Type Parameters
/// - `'de`: Lifetime of the input data.
/// - `F`: The decoding flavor (e.g., [`Network`](crate::flavors::Network) or other implementations) implementing the [`Flavor`] trait.
/// - `W`: The wire format strategy of the flavor.
/// - `O`: The output type resulting from decoding.
/// - `B`: The buffer implementation used to store the unknown data during decoding (defaults to `()`, will ignore the unknown data).
pub trait PartialDecode<'de, F, W, O, UB = ()>: Selectable<F, W>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Partially decodes an instance of the type from a byte buffer using a selector.
  ///
  /// ## Parameters
  /// - `context`: Contextual data used during decoding.
  /// - `src`: The input byte buffer to decode.
  /// - `selector`: A selector to guide the partial decoding process.
  ///
  /// ## Returns
  /// A tuple containing the number of bytes consumed and the decoded output.
  fn partial_decode<B>(
    context: &F::Context,
    src: B,
    selector: &Self::Selector,
  ) -> Result<(usize, Option<O>), F::Error>
  where
    O: Sized + 'de,
    B: Buf<'de>,
    UB: Buffer<F::Unknown<B>> + 'de;

  /// Partially decodes a length-delimited message using a selector.
  ///
  /// The input buffer is expected to be length-prefixed with a `u32` encoded in varint format.
  fn partial_decode_length_delimited<B>(
    context: &F::Context,
    src: B,
    selector: &Self::Selector,
  ) -> Result<(usize, Option<O>), F::Error>
  where
    O: Sized + 'de,
    B: Buf<'de>,
    UB: Buffer<F::Unknown<B>> + 'de,
  {
    let chunk = src.chunk();
    let (len_size, len) = varing::decode_u32_varint(chunk).map_err(Error::from)?;
    let src_len = src.len();
    let len = len as usize;
    let total = len_size + len;
    if total > src_len {
      return Err(Error::buffer_underflow().into());
    }

    if len_size >= src_len {
      return Err(Error::buffer_underflow().into());
    }

    Self::partial_decode(context, src.slice(len_size..total), selector)
  }

  /// Skips the type without actually decoding it. Returns the number of bytes skipped.
  fn skip<B>(context: &F::Context, src: B) -> Result<usize, F::Error>
  where
    O: Sized + 'de,
    B: Buf<'de>;
}

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
pub trait Decode<'de, F, W, O, UB = ()>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Decodes an instance from a raw byte slice.
  ///
  /// Returns a tuple with the number of bytes consumed and the decoded output.
  fn decode<B>(context: &F::Context, src: B) -> Result<(usize, O), F::Error>
  where
    O: Sized + 'de,
    B: Buf<'de>,
    UB: Buffer<F::Unknown<B>> + 'de;

  /// Decodes an instance of this type from a length-delimited byte buffer.
  ///
  /// The input buffer is expected to be length-prefixed with a `u32` encoded in varint format.
  fn decode_length_delimited<B>(context: &F::Context, src: B) -> Result<(usize, O), F::Error>
  where
    O: Sized + 'de,
    B: Buf<'de> + 'de,
    UB: Buffer<F::Unknown<B>> + 'de,
  {
    let chunk = src.chunk();
    let (len_size, len) = varing::decode_u32_varint(chunk).map_err(Error::from)?;
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

pub trait DecodeOwned<F, W, O, UB = ()>: for<'de> Decode<'de, F, W, O, UB>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
}

impl<F, W, O, UB, T> DecodeOwned<F, W, O, UB> for T
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
  T: for<'de> Decode<'de, F, W, O, UB>,
{
}

// /// A trait for fully decoding types into owned values from an owned byte buffer.
// ///
// /// Unlike [`Decode`], this trait allows decoding without borrowing the input buffer,
// /// making it suitable for owned deserialization where lifetimes must not overlap.
// ///
// /// See also [`Decode`] for more details.
// pub trait DecodeOwned<F, W, O, B = ()>: Decode<'static, F, W, O, B> + 'static
// where
//   F: Flavor + ?Sized,
//   W: WireFormat<F>,
// {
//   /// See [`Decode::decode`].
//   fn decode_owned<D>(context: &F::Context, src: D) -> Result<(usize, O), F::Error>
//   where
//     O: Sized + 'static,
//     D: BytesBuffer + 'static,
//     B: Buffer<F::Unknown<D>> + 'static;

//   /// See [`Decode::decode_length_delimited`].
//   fn decode_length_delimited_owned<D>(context: &F::Context, src: D) -> Result<(usize, O), F::Error>
//   where
//     O: Sized + 'static,
//     D: BytesBuffer + 'static,
//     B: Buffer<F::Unknown<D>> + 'static,
//   {
//     let bytes = src.as_bytes();
//     let (len_size, len) = varing::decode_u32_varint(bytes).map_err(Error::from)?;
//     let src_len = src.len();
//     let len = len as usize;
//     let total = len_size + len;
//     if total > src_len {
//       return Err(Error::buffer_underflow().into());
//     }

//     if len_size >= src_len {
//       return Err(Error::buffer_underflow().into());
//     }

//     Self::decode_owned::<D>(context, src.slice(len_size..total))
//   }
// }

#[cfg(any(feature = "std", feature = "alloc", feature = "triomphe_0_1"))]
macro_rules! deref_decode_impl {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<'de, F, W, O, UB, T> Decode<'de, F, W, O, UB> for $ty
      where
        F: Flavor + ?Sized,
        W: WireFormat<F>,
        T: Decode<'de, F, W, O, UB> + ?Sized,
      {
        fn decode<B>(context: &<F as Flavor>::Context, src: B) -> Result<(usize, O), <F as Flavor>::Error>
        where
          O: Sized + 'de,
          B: Buf<'de>,
          UB: Buffer<<F as Flavor>::Unknown<B>> + 'de
        {
          T::decode::<B>(context, src)
        }

        fn decode_length_delimited<B>(
          context: &<F as Flavor>::Context,
          src: B,
        ) -> Result<(usize, O), <F as Flavor>::Error>
        where
          O: Sized + 'de,
          B: Buf<'de>,
          UB: Buffer<<F as Flavor>::Unknown<B>> + 'de
        {
          T::decode_length_delimited::<B>(context, src)
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
