use crate::flavors::WireFormat;

use super::{
  buffer::{Buffer, BytesBuffer},
  flavors::Flavor,
};

pub use error::DecodeError;

mod error;

/// A trait for types that can be decoded from bytes with a lifetime.
///
/// This trait provides methods to decode data from byte slices,
/// with support for both direct and length-prefixed decoding.
///
/// * `'de` - The lifetime of the input data
pub trait Decode<'de, F, W, O>
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Decodes an instance of this type from a byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode<UB>(context: &F::Context, src: &'de [u8]) -> Result<(usize, O), F::DecodeError>
  where
    O: Sized + 'de,
    UB: Buffer<F::Unknown<&'de [u8]>> + 'de;

  /// Decodes an instance of this type from a length-delimited byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode_length_delimited<UB>(
    context: &F::Context,
    src: &'de [u8],
  ) -> Result<(usize, O), F::DecodeError>
  where
    O: Sized + 'de,
    UB: Buffer<F::Unknown<&'de [u8]>> + 'de;
}

/// A trait for types that can be decoded without borrowing data.
///
/// Types implementing this trait can be decoded into owned values
/// without maintaining a borrow of the original data.
///
/// This is useful for deserialization scenarios where the input data
/// may not outlive the decoded value.
pub trait DecodeOwned<F, W, O>: Decode<'static, F, W, O> + 'static
where
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  /// Decodes an instance of this type from a byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode_owned<B, UB>(context: &F::Context, src: B) -> Result<(usize, O), F::DecodeError>
  where
    O: Sized + 'static,
    B: BytesBuffer + 'static,
    UB: Buffer<F::Unknown<B>> + 'static;

  /// Decodes an instance of this type from a length-delimited byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode_length_delimited_owned<B, UB>(
    context: &F::Context,
    src: B,
  ) -> Result<(usize, O), F::DecodeError>
  where
    O: Sized + 'static,
    B: BytesBuffer + 'static,
    UB: Buffer<F::Unknown<B>> + 'static;
}

pub struct Decoded<T, B, UB> {
  data: T,
  unknown: Option<UB>,
  _m: core::marker::PhantomData<B>,
}

impl<T, B, UB> Decoded<T, B, UB> {
  /// Creates a new `Decoded` instance with the given data.
  #[inline]
  pub const fn new(data: T) -> Self {
    Self {
      data,
      unknown: None,
      _m: core::marker::PhantomData,
    }
  }

  /// Gets the data of the decoded message.
  #[inline]
  pub const fn data(&self) -> &T {
    &self.data
  }

  /// Gets the unknown data of the decoded message.
  #[inline]
  pub const fn unknown_ref(&self) -> Option<&UB> {
    self.unknown.as_ref()
  }

  /// Returns the mutable reference to the unknown data of the decoded message.
  #[inline]
  pub const fn unknown_mut(&mut self) -> Option<&mut UB> {
    self.unknown.as_mut()
  }

  /// Sets the unknown data of the decoded message.
  #[inline]
  pub fn set_unknown(&mut self, unknown: UB) -> &mut Self {
    self.unknown = Some(unknown);
    self
  }

  /// Sets the unknown data of the decoded message to `None`.
  #[inline]
  pub fn with_unknown(mut self) -> Self {
    self.unknown = None;
    self
  }

  /// Clears the unknown data of the decoded message.
  #[inline]
  pub fn clear_unknown(&mut self) -> &mut Self {
    self.unknown = None;
    self
  }

  /// Clears the unknown data of the decoded message.
  #[inline]
  pub fn without_unknown(mut self) -> Self {
    self.unknown = None;
    self
  }

  /// Clears the unknown data of the decoded message and returns the data.
  #[inline]
  pub const fn take_unknown(&mut self) -> Option<UB> {
    self.unknown.take()
  }
}

#[cfg(any(feature = "std", feature = "alloc", feature = "triomphe_0_1"))]
macro_rules! deref_decode_impl {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<'de, F, W, O, T> Decode<'de, F, W, O> for $ty
      where
        F: Flavor + ?Sized,
        W: WireFormat<F>,
        T: Decode<'de, F, W, O> + ?Sized,
      {
        fn decode<UB>(context: &<F as Flavor>::Context, src: &'de [u8]) -> Result<(usize, O), <F as Flavor>::DecodeError>
        where
          O: Sized + 'de,
          UB: Buffer<<F as Flavor>::Unknown<&'de [u8]>> + 'de
        {
          T::decode::<UB>(context, src)
        }

        fn decode_length_delimited<UB>(
          context: &<F as Flavor>::Context,
          src: &'de [u8],
        ) -> Result<(usize, O), <F as Flavor>::DecodeError>
        where
          O: Sized + 'de,
          UB: Buffer<<F as Flavor>::Unknown<&'de [u8]>> + 'de
        {
          T::decode_length_delimited::<UB>(context, src)
        }
      }
    )*
  };
}

#[cfg(any(feature = "std", feature = "alloc", feature = "triomphe_0_1"))]
macro_rules! deref_decode_owned_impl {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<F, W, O, T> DecodeOwned<F, W, O> for $ty
      where
        F: Flavor + ?Sized,
        W: WireFormat<F>,
        T: DecodeOwned<F, W, O> + ?Sized,
      {
        fn decode_owned<B, UB>(context: &<F as Flavor>::Context, src: B) -> Result<(usize, O), <F as Flavor>::DecodeError>
        where
          O: Sized + 'static,
          B: BytesBuffer + 'static,
          UB: Buffer<<F as Flavor>::Unknown<B>> + 'static
        {
          T::decode_owned::<B, UB>(context, src)
        }

        fn decode_length_delimited_owned<B, UB>(
          context: &<F as Flavor>::Context,
          src: B,
        ) -> Result<(usize, O), <F as Flavor>::DecodeError>
        where
          O: Sized + 'static,
          B: BytesBuffer + 'static,
          UB: Buffer<<F as Flavor>::Unknown<B>> + 'static
        {
          T::decode_length_delimited_owned::<B, UB>(context, src)
        }
      }
    )*
  };
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::{boxed::Box, rc::Rc, sync::Arc};

  deref_decode_impl!(Box<T>, Rc<T>, Arc<T>);
  deref_decode_owned_impl!(Box<T>, Rc<T>, Arc<T>);
};

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  use triomphe_0_1::Arc;

  deref_decode_impl!(Arc<T>);
  deref_decode_owned_impl!(Arc<T>);
};
