use super::{Context, Wirable};
use grost_types::{EncodeError, WireType};

/// A trait for serializing data to binary format with support for various wire types.
///
/// This trait provides methods to encode data into binary representations,
/// calculate required buffer sizes, and handle length-delimited encoding.
pub trait Encode: Wirable {
  /// Encodes the message into the provided buffer.
  ///
  /// Returns the number of bytes written to the buffer or an error if the operation fails.
  ///
  /// [`Encode::encoded_len`] can be used to determine the required buffer size.
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, EncodeError>;

  /// Returns the number of bytes needed to encode the message.
  ///
  /// This is used to determine the buffer size required for encoding.
  fn encoded_len(&self, context: &Context) -> usize;

  // /// Returns the encoded length of the data including the identifier.
  // ///
  // /// For `WireType::LengthDelimited`, this includes the varint-encoded length
  // /// prefix followed by the actual data length. For other wire types, this is
  // /// equivalent to [`Encode::encoded_len`].
  // fn encoded_len_with_identifier(&self, identifier: Identifier) -> usize {
  //   let len = self.encoded_len();
  //   let identifier_len = identifier.encoded_len();
  //   match Self::WIRE_TYPE {
  //     WireType::LengthDelimited => {
  //       identifier_len + varing::encoded_u32_varint_len(len as u32) + len
  //     }
  //     _ => identifier_len + len,
  //   }
  // }

  /// Encodes the message into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_vec(&self, context: &Context) -> Result<std::vec::Vec<u8>, EncodeError> {
    let mut buf = std::vec![0; self.encoded_len(context)];
    self.encode(context, &mut buf)?;
    Ok(buf)
  }

  /// Encodes the message into a [`Bytes`](::bytes::Bytes).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_bytes(&self, context: &Context) -> Result<super::bytes::Bytes, EncodeError> {
    self.encode_to_vec(context).map(Into::into)
  }
}

/// A trait for serializing data to binary format with support for various wire types.
///
/// This trait provides methods to encode data into binary representations,
/// calculate required buffer sizes, and handle length-delimited encoding.
pub trait PartialEncode: Wirable {
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
    context: &Context,
    buf: &mut [u8],
    selection: &Self::Selection,
  ) -> Result<usize, EncodeError>;

  /// Returns the number of bytes needed to encode the message.
  ///
  /// This is used to determine the buffer size required for encoding.
  fn partial_encoded_len(&self, context: &Context, selection: &Self::Selection) -> usize;

  // /// Returns the encoded length of the data including the identifier.
  // ///
  // /// For `WireType::LengthDelimited`, this includes the varint-encoded length
  // /// prefix followed by the actual data length. For other wire types, this is
  // /// equivalent to [`PartialEncode::partial_encoded_len`].
  // fn partial_encoded_len_with_identifier(&self, identifier: Identifier, selection: &Self::Selection) -> usize {
  //   let len = self.partial_encoded_len(selection);
  //   let identifier_len = identifier.encoded_len();
  //   match Self::WIRE_TYPE {
  //     WireType::LengthDelimited => identifier_len + varing::encoded_u32_varint_len(len as u32) + len,
  //     _ => identifier_len + len,
  //   }
  // }

  /// Encodes the message into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_to_vec(
    &self,
    context: &Context,
    selection: &Self::Selection,
  ) -> Result<std::vec::Vec<u8>, EncodeError> {
    let mut buf = std::vec![0; self.partial_encoded_len(context, selection)];
    self.partial_encode(context, &mut buf, selection)?;
    Ok(buf)
  }

  /// Encodes the message into a [`Bytes`](::bytes::Bytes).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_to_bytes(
    &self,
    context: &Context,
    selection: &Self::Selection,
  ) -> Result<super::bytes::Bytes, EncodeError> {
    self
      .partial_encode_to_vec(context, selection)
      .map(Into::into)
  }

  // /// Encodes the message with an identifier to a buffer.
  // ///
  // /// For `WireType::LengthDelimited`, this prepends a varint-encoded length
  // /// before the message data. For other wire types, this behaves the same as [`Encode::encode`].
  // ///
  // /// An error will be returned if the buffer does not have sufficient capacity.
  // fn partial_encode_with_identifier(
  //   &self,
  //   identifier: Identifier,
  //   buf: &mut [u8],
  //   selection: &Self::Selection,
  // ) -> Result<usize, EncodeError> {
  //   macro_rules! insufficient_buffer {
  //     ($this:ident($identifier:ident, $buf_len:ident)) => {{
  //       EncodeError::insufficient_buffer(
  //         $this.partial_encoded_len_with_identifier($identifier, selection),
  //         $buf_len,
  //       )
  //     }};
  //     (@update $this:ident($e:ident, $identifier:ident, $buf_len:ident)) => {{
  //       $e.update($this.partial_encoded_len_with_identifier($identifier, selection), $buf_len)
  //     }};
  //   }

  //   macro_rules! check_bound {
  //     ($this:ident($identifier:ident, $offset:ident, $buf_len:ident)) => {{
  //       if $offset >= $buf_len {
  //         return Err(insufficient_buffer!($this($identifier, $buf_len)));
  //       }
  //     }};
  //   }

  //   if Self::WIRE_TYPE != WireType::LengthDelimited {
  //     return identifier.encode_to(buf).and_then(|written| {
  //       let buf_len = buf.len();
  //       if written > buf.len() {
  //         Err(insufficient_buffer!(self(identifier, buf_len)))
  //       } else {
  //         self.partial_encode(&mut buf[written..], selection)
  //       }
  //     });
  //   }

  //   let len = self.partial_encoded_len(selection);
  //   if len > u32::MAX as usize {
  //     return Err(EncodeError::TooLarge);
  //   }

  //   let buf_len = buf.len();
  //   let mut offset = 0;
  //   offset += identifier
  //     .encode_to(&mut buf[offset..])
  //     .map_err(|e| insufficient_buffer!(@update self(e, identifier, buf_len)))?;

  //   check_bound!(self(identifier, offset, buf_len));
  //   offset += varing::encode_u32_varint_to(len as u32, &mut buf[offset..]).map_err(|_| insufficient_buffer!(self(identifier, buf_len)))?;

  //   check_bound!(self(identifier, offset, buf_len));
  //   offset += self
  //     .partial_encode(&mut buf[offset..], selection)
  //     .map_err(|e| insufficient_buffer!(@update self(e, identifier, buf_len)))?;

  //   #[cfg(debug_assertions)]
  //   debug_assert_write_eq::<Self>(offset, self.partial_encoded_len_with_identifier(identifier, selection));

  //   Ok(offset)
  // }
}

impl<T> Wirable for &T
where
  T: Wirable + ?Sized,
{
  const WIRE_TYPE: WireType = T::WIRE_TYPE;
}

impl<T: Wirable> Wirable for Option<T> {
  const WIRE_TYPE: WireType = T::WIRE_TYPE;
}

impl<T> Encode for &T
where
  T: Encode + ?Sized,
{
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    (*self).encode(context, buf)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    (*self).encoded_len(context)
  }
}

impl<T: Encode> Encode for Option<T> {
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
    if let Some(value) = self {
      value.encode(context, buf)
    } else {
      Ok(0)
    }
  }

  fn encoded_len(&self, context: &Context) -> usize {
    if let Some(value) = self {
      value.encoded_len(context)
    } else {
      0
    }
  }
}

impl<T> PartialEncode for &T
where
  T: PartialEncode + ?Sized,
{
  type Selection = T::Selection;

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selection: &Self::Selection,
  ) -> Result<usize, EncodeError> {
    (*self).partial_encode(context, buf, selection)
  }

  fn partial_encoded_len(&self, context: &Context, selection: &Self::Selection) -> usize {
    (*self).partial_encoded_len(context, selection)
  }
}

impl<T> PartialEncode for Option<T>
where
  T: PartialEncode,
{
  type Selection = T::Selection;

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selection: &Self::Selection,
  ) -> Result<usize, EncodeError> {
    if let Some(value) = self {
      value.partial_encode(context, buf, selection)
    } else {
      Ok(0)
    }
  }

  fn partial_encoded_len(&self, context: &Context, selection: &Self::Selection) -> usize {
    if let Some(value) = self {
      value.partial_encoded_len(context, selection)
    } else {
      0
    }
  }
}
