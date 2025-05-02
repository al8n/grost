use super::flavors::{Flavor, Selectable, WireFormat};

pub use error::EncodeError;

mod error;

/// A trait for serializing data to binary format with support for various wire types.
///
/// This trait provides methods to encode data into binary representations,
/// calculate required buffer sizes, and handle length-delimited encoding.
///
/// ## About the arguments
///
/// All methods in this trait take `context` and `wire_type` as arguments.
///
/// - `context`: The context for encoding, which may include information about the encoding process,
///   e.g. the maximum encoded length allowed for a message.
/// - `wire_type`: The wire type used for encoding the message. This is a type that indicates how the data should be serialized.
///
///     **The implementor does not need to encode the `wire_type` to the buffer in `encode_*`
///   or including the encoded size of the `wire_type` in `*_len`, the wire type is
///   providing for deciding how to encode the message, because of some types may
///   support multiple wire types.**
///
///   For example, when using the [`Network`] flavor, `u16` can be encoded by LEB128 (varint) or fixed,
///   this is decided by how the users writing the schema.
///
///     - If in the schema code like the below example, then the `u16` will be encoded in LEB128 (varint):
///
///       ```graphql
///       type Foo {
///         bar: u16
///       }
///       ```
///   
///     - If in the schema code like the below example, then the `u16` will be encoded in fixed length (2 bytes):
///
///       ```graphql
///       type Foo {
///         bar: u16 @fixed
///       }
pub trait Encode<F: Flavor + ?Sized, W: WireFormat<F>> {
  /// Encodes the message into the provided buffer.
  ///
  /// Returns the number of bytes written to the buffer or an error if the operation fails.
  ///
  /// [`Encode::encoded_len`] can be used to determine the required buffer size.
  ///
  /// See also [ trait level documentation ](Encode) for more details about the arguments.
  fn encode(&self, context: &F::Context, buf: &mut [u8]) -> Result<usize, F::EncodeError>;

  /// Returns the number of bytes needed to encode the message. If the message cannot be encoded as the given wire type,
  /// then it will return `WireTypeNotSupported` err will be returned.
  ///
  /// This is used to determine the buffer size required for encoding.
  ///
  /// See also [ trait level documentation ](Encode) for more details about the arguments.
  fn encoded_len(&self, context: &F::Context) -> usize;

  /// Returns the number of bytes needed to encode the message with length-delimited.
  ///
  /// This is used to determine the buffer size required for encoding.
  ///
  /// See also [ trait level documentation ](Encode) for more details about the arguments.
  fn encoded_length_delimited_len(&self, context: &F::Context) -> usize;

  /// Encodes the message into the provided buffer with length-delimited. If the message cannot be encoded as the given wire type,
  /// then it will return `WireTypeNotSupported` err will be returned.
  ///
  /// Returns the number of bytes written to the buffer or an error if the operation fails.
  ///
  /// See also [ trait level documentation ](Encode) for more details about the arguments.
  fn encode_length_delimited(
    &self,
    context: &F::Context,
    buf: &mut [u8],
  ) -> Result<usize, F::EncodeError>;

  /// Encodes the message into a [`Vec`](std::vec::Vec).
  ///
  /// See also [ trait level documentation ](Encode) for more details about the arguments.
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_vec(&self, context: &F::Context) -> Result<std::vec::Vec<u8>, F::EncodeError> {
    let mut buf = std::vec![0; self.encoded_len(context)];
    self.encode(context, &mut buf)?;
    Ok(buf)
  }

  /// Encodes the message into a [`Bytes`](::bytes::Bytes).
  ///
  /// See also [ trait level documentation ](Encode) for more details about the arguments.
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_bytes(&self, context: &F::Context) -> Result<super::bytes::Bytes, F::EncodeError> {
    self.encode_to_vec(context).map(Into::into)
  }

  /// Encodes the message into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_length_delimited_to_vec(
    &self,
    context: &F::Context,
  ) -> Result<std::vec::Vec<u8>, F::EncodeError> {
    let mut buf = std::vec![0; self.encoded_len(context)];
    self.encode_length_delimited(context, &mut buf)?;
    Ok(buf)
  }

  /// Encodes the message into a [`Bytes`](::bytes::Bytes).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_length_delimited_to_bytes(
    &self,
    context: &F::Context,
  ) -> Result<super::bytes::Bytes, F::EncodeError> {
    self.encode_length_delimited_to_vec(context).map(Into::into)
  }
}

/// A trait for serializing data to binary format with support for various wire types.
///
/// This trait provides methods to encode data into binary representations,
/// calculate required buffer sizes, and handle length-delimited encoding.
///
/// ## About the arguments
///
/// All methods in this trait take `context` and `wire_type` as arguments.
///
/// - `context`: The context for encoding, which may include information about the encoding process,
///   e.g. the maximum encoded length allowed for a message.
/// - `selector`: The selector type for the message, which determines which fields to include
/// - `wire_type`: The wire type used for encoding the message. This is a type that indicates how the data should be serialized.
///
///     **The implementor does not need to encode the `wire_type` to the buffer in `encode_*`
///   or including the encoded size of the `wire_type` in `*_len`, the wire type is
///   providing for deciding how to encode the message, because of some types may
///   support multiple wire types.**
///
///   For example, when using the [`Network`] flavor, `u16` can be encoded by LEB128 (varint) or fixed,
///   this is decided by how the users writing the schema.
///
///     - If in the schema code like the below example, then the `u16` will be encoded in LEB128 (varint):
///
///       ```graphql
///       type Foo {
///         bar: u16
///       }
///       ```
///   
///     - If in the schema code like the below example, then the `u16` will be encoded in fixed length (2 bytes):
///
///       ```graphql
///       type Foo {
///         bar: u16 @fixed
///       }
pub trait PartialEncode<F: Flavor + ?Sized, W: WireFormat<F>>: super::flavors::Selectable {
  /// Encodes the message into the provided buffer.
  ///
  /// Returns the number of bytes written to the buffer or an error if the operation fails.
  ///
  /// [`Encode::encoded_len`] can be used to determine the required buffer size.
  fn partial_encode(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::EncodeError>;

  /// Returns the number of bytes needed to encode the message. If the message cannot be encoded as the given wire type,
  /// then it will return `WireTypeNotSupported` err will be returned.
  ///
  /// This is used to determine the buffer size required for encoding.
  fn partial_encoded_len(&self, context: &F::Context, selector: &Self::Selector) -> usize;

  /// Returns the number of bytes needed to encode the message with length-delimited.
  /// If the message cannot be encoded as the given wire type,
  /// then it will return `WireTypeNotSupported` err will be returned.
  ///
  /// This is used to determine the buffer size required for encoding.
  ///
  /// See also [ trait level documentation ](Encode) for more details about the arguments.
  fn partial_encoded_length_delimited_len(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> usize;

  /// Encodes the message into the provided buffer with length-delimited.
  ///
  /// Returns the number of bytes written to the buffer or an error if the operation fails.
  ///
  /// See also [ trait level documentation ](Encode) for more details about the arguments.
  fn partial_encode_length_delimited(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::EncodeError>;

  /// Encodes the message into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_to_vec(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> Result<std::vec::Vec<u8>, F::EncodeError> {
    let mut buf = std::vec![0; self.partial_encoded_len(context, selector)];
    self.partial_encode(context, &mut buf, selector)?;
    Ok(buf)
  }

  /// Encodes the message into a [`Bytes`](::bytes::Bytes).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_to_bytes(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> Result<super::bytes::Bytes, F::EncodeError> {
    self
      .partial_encode_to_vec(context, selector)
      .map(Into::into)
  }

  /// Encodes the message into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_length_delimited_to_vec(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> Result<std::vec::Vec<u8>, F::EncodeError> {
    let mut buf = std::vec![0; self.partial_encoded_len(context, selector)];
    self.partial_encode_length_delimited(context, &mut buf, selector)?;
    Ok(buf)
  }

  /// Encodes the message into a [`Bytes`](::bytes::Bytes).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_length_delimited_to_bytes(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> Result<super::bytes::Bytes, F::EncodeError> {
    self
      .partial_encode_length_delimited_to_vec(context, selector)
      .map(Into::into)
  }
}

impl<F, W, T> Encode<F, W> for &T
where
  T: Encode<F, W> + ?Sized,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  fn encode(&self, context: &F::Context, buf: &mut [u8]) -> Result<usize, F::EncodeError> {
    (*self).encode(context, buf)
  }

  fn encoded_len(&self, context: &F::Context) -> usize {
    (*self).encoded_len(context)
  }

  fn encode_length_delimited(
    &self,
    context: &F::Context,
    buf: &mut [u8],
  ) -> Result<usize, F::EncodeError> {
    (*self).encode_length_delimited(context, buf)
  }

  fn encoded_length_delimited_len(&self, context: &F::Context) -> usize {
    (*self).encoded_length_delimited_len(context)
  }
}

impl<F, W, T> Encode<F, W> for Option<T>
where
  T: Encode<F, W>,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  fn encode(&self, context: &F::Context, buf: &mut [u8]) -> Result<usize, F::EncodeError> {
    if let Some(value) = self {
      value.encode(context, buf)
    } else {
      Ok(0)
    }
  }

  fn encoded_len(&self, context: &F::Context) -> usize {
    if let Some(value) = self {
      value.encoded_len(context)
    } else {
      0
    }
  }

  fn encode_length_delimited(
    &self,
    context: &F::Context,
    buf: &mut [u8],
  ) -> Result<usize, F::EncodeError> {
    if let Some(value) = self {
      value.encode_length_delimited(context, buf)
    } else {
      Ok(0)
    }
  }

  fn encoded_length_delimited_len(&self, context: &F::Context) -> usize {
    if let Some(value) = self {
      value.encoded_length_delimited_len(context)
    } else {
      0
    }
  }
}

impl<F, W, T> PartialEncode<F, W> for &T
where
  T: PartialEncode<F, W> + Selectable + ?Sized,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  fn partial_encode(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::EncodeError> {
    (*self).partial_encode(context, buf, selector)
  }

  fn partial_encoded_len(&self, context: &F::Context, selector: &Self::Selector) -> usize {
    (*self).partial_encoded_len(context, selector)
  }

  fn partial_encode_length_delimited(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::EncodeError> {
    (*self).partial_encode_length_delimited(context, buf, selector)
  }

  fn partial_encoded_length_delimited_len(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> usize {
    (*self).partial_encoded_length_delimited_len(context, selector)
  }
}

impl<F, W, T> PartialEncode<F, W> for Option<T>
where
  T: PartialEncode<F, W> + Selectable,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  fn partial_encode(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::EncodeError> {
    if let Some(value) = self {
      value.partial_encode(context, buf, selector)
    } else {
      Ok(0)
    }
  }

  fn partial_encoded_len(&self, context: &F::Context, selector: &Self::Selector) -> usize {
    if let Some(value) = self {
      value.partial_encoded_len(context, selector)
    } else {
      0
    }
  }

  fn partial_encode_length_delimited(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::EncodeError> {
    if let Some(value) = self {
      value.partial_encode_length_delimited(context, buf, selector)
    } else {
      Ok(0)
    }
  }

  fn partial_encoded_length_delimited_len(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> usize {
    if let Some(value) = self {
      value.partial_encoded_length_delimited_len(context, selector)
    } else {
      0
    }
  }
}

#[cfg(any(feature = "std", feature = "alloc", feature = "triomphe_0_1"))]
macro_rules! deref_encode_impl {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<F, W, T> Encode<F, W> for $ty
      where
        T: Encode<F, W> + ?Sized,
        F: Flavor + ?Sized,
        W: WireFormat<F>,
      {
        fn encode(&self, context: &F::Context, buf: &mut [u8]) -> Result<usize, F::EncodeError> {
          (**self).encode(context, buf)
        }

        fn encoded_len(&self, context: &F::Context) -> usize {
          (**self).encoded_len(context)
        }

        fn encode_length_delimited(
          &self,
          context: &F::Context,
          buf: &mut [u8],
        ) -> Result<usize, F::EncodeError> {
          (**self).encode_length_delimited(context, buf)
        }

        fn encoded_length_delimited_len(&self, context: &F::Context) -> usize {
          (**self).encoded_length_delimited_len(context)
        }
      }
    )*
  };
}

#[cfg(any(feature = "std", feature = "alloc", feature = "triomphe_0_1"))]
macro_rules! deref_partial_encode_impl {
  ($($ty:ty),+$(,)?) => {{
    $(
      impl<T> Selectable for $ty
      where
        T: ?Sized + Selectable,
      {
        type Selector = T::Selector;
      }

      impl<F, W, T> PartialEncode<F, W> for $ty
      where
        T: PartialEncode<F, W> + Selectable + ?Sized,
        F: Flavor + ?Sized,
        W: WireFormat<F>,
      {
        fn partial_encode(
          &self,
          context: &F::Context,
          buf: &mut [u8],
          selector: &Self::Selector,
        ) -> Result<usize, F::EncodeError> {
          (**self).partial_encode(context, buf, selector)
        }

        fn partial_encoded_len(&self, context: &F::Context, selector: &Self::Selector) -> usize {
          (**self).partial_encoded_len(context, selector)
        }

        fn partial_encode_length_delimited(
          &self,
          context: &F::Context,
          buf: &mut [u8],
          selector: &Self::Selector,
        ) -> Result<usize, F::EncodeError> {
          (**self).partial_encode_length_delimited(context, buf, selector)
        }

        fn partial_encoded_length_delimited_len(
          &self,
          context: &F::Context,
          selector: &Self::Selector,
        ) -> usize {
          (**self).partial_encoded_length_delimited_len(context, selector)
        }
      }
    )*
  }};
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::{boxed::Box, rc::Rc, sync::Arc};

  deref_encode_impl!(Box<T>, Rc<T>, Arc<T>);
  deref_partial_encode_impl!(Box<T>, Rc<T>, Arc<T>);
};

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  use triomphe_0_1::Arc;

  deref_encode_impl!(Arc<T>);
  deref_partial_encode_impl!(Arc<T>);
};
