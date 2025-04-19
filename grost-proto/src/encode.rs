use super::{flavors::Flavor, Wirable};

/// A trait for serializing data to binary format with support for various wire types.
///
/// This trait provides methods to encode data into binary representations,
/// calculate required buffer sizes, and handle length-delimited encoding.
pub trait Encode<F: Flavor>: Wirable<F> {
  /// Encodes the message into the provided buffer.
  ///
  /// Returns the number of bytes written to the buffer or an error if the operation fails.
  ///
  /// [`Encode::encoded_len`] can be used to determine the required buffer size.
  fn encode(&self, context: &F::Context, buf: &mut [u8]) -> Result<usize, F::EncodeError>;

  /// Returns the number of bytes needed to encode the message.
  ///
  /// This is used to determine the buffer size required for encoding.
  fn encoded_len(&self, context: &F::Context) -> usize;

  /// Encodes the message into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_vec(&self, context: &F::Context) -> Result<std::vec::Vec<u8>, F::EncodeError> {
    let mut buf = std::vec![0; self.encoded_len(context)];
    self.encode(context, &mut buf)?;
    Ok(buf)
  }

  /// Encodes the message into a [`Bytes`](::bytes::Bytes).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_bytes(&self, context: &F::Context) -> Result<super::bytes::Bytes, F::EncodeError> {
    self.encode_to_vec(context).map(Into::into)
  }
}

/// A trait for serializing data to binary format with support for various wire types.
///
/// This trait provides methods to encode data into binary representations,
/// calculate required buffer sizes, and handle length-delimited encoding.
pub trait PartialEncode<F: Flavor>: Wirable<F> {
  /// The selection type for the message, which determines which fields to include
  /// in the encoded output.
  type Selection;

  /// Encodes the message into the provided buffer.
  ///
  /// Returns the number of bytes written to the buffer or an error if the operation fails.
  ///
  /// [`Encode::encoded_len`] can be used to determine the required buffer size.
  fn partial_encode(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selection: &Self::Selection,
  ) -> Result<usize, F::EncodeError>;

  /// Returns the number of bytes needed to encode the message.
  ///
  /// This is used to determine the buffer size required for encoding.
  fn partial_encoded_len(&self, context: &F::Context, selection: &Self::Selection) -> usize;

  /// Encodes the message into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_to_vec(
    &self,
    context: &F::Context,
    selection: &Self::Selection,
  ) -> Result<std::vec::Vec<u8>, F::EncodeError> {
    let mut buf = std::vec![0; self.partial_encoded_len(context, selection)];
    self.partial_encode(context, &mut buf, selection)?;
    Ok(buf)
  }

  /// Encodes the message into a [`Bytes`](::bytes::Bytes).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_to_bytes(
    &self,
    context: &F::Context,
    selection: &Self::Selection,
  ) -> Result<super::bytes::Bytes, F::EncodeError> {
    self
      .partial_encode_to_vec(context, selection)
      .map(Into::into)
  }
}

impl<F, T> Wirable<F> for &T
where
  T: Wirable<F> + ?Sized,
  F: Flavor,
{
  const WIRE_TYPE: F::WireType = T::WIRE_TYPE;
}

impl<F, T> Wirable<F> for Option<T>
where
  T: Wirable<F>,
  F: Flavor,
{
  const WIRE_TYPE: F::WireType = T::WIRE_TYPE;
}

impl<F, T> Encode<F> for &T
where
  T: Encode<F> + ?Sized,
  F: Flavor,
{
  fn encode(&self, context: &F::Context, buf: &mut [u8]) -> Result<usize, F::EncodeError> {
    (*self).encode(context, buf)
  }

  fn encoded_len(&self, context: &F::Context) -> usize {
    (*self).encoded_len(context)
  }
}

impl<F, T> Encode<F> for Option<T>
where
  T: Encode<F>,
  F: Flavor,
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
}

impl<F, T> PartialEncode<F> for &T
where
  T: PartialEncode<F> + ?Sized,
  F: Flavor,
{
  type Selection = T::Selection;

  fn partial_encode(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selection: &Self::Selection,
  ) -> Result<usize, F::EncodeError> {
    (*self).partial_encode(context, buf, selection)
  }

  fn partial_encoded_len(&self, context: &F::Context, selection: &Self::Selection) -> usize {
    (*self).partial_encoded_len(context, selection)
  }
}

impl<F, T> PartialEncode<F> for Option<T>
where
  T: PartialEncode<F>,
  F: Flavor,
{
  type Selection = T::Selection;

  fn partial_encode(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selection: &Self::Selection,
  ) -> Result<usize, F::EncodeError> {
    if let Some(value) = self {
      value.partial_encode(context, buf, selection)
    } else {
      Ok(0)
    }
  }

  fn partial_encoded_len(&self, context: &F::Context, selection: &Self::Selection) -> usize {
    if let Some(value) = self {
      value.partial_encoded_len(context, selection)
    } else {
      0
    }
  }
}
