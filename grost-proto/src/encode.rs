use super::{
  error::Error,
  flavors::{Flavor, FlavorError, WireFormat},
  selection::Selectable,
};

/// A trait for serializing data into a binary format using a specified [`Flavor`] and [`WireFormat`].
///
/// This trait provides methods for encoding a value into a byte buffer or into heap-allocated
/// containers like [`Vec<u8>`] or [`Bytes`](bytes_1::Bytes). It also supports length-delimited encoding,
/// commonly used in framed transport protocols (e.g., Protobuf messages).
///
/// Types implementing this trait should ensure consistency between `encoded_len` and `encode`.
pub trait Encode<W: WireFormat<F>, F: Flavor + ?Sized> {
  /// Encodes the message into the provided buffer.
  ///
  /// Returns the number of bytes written to the buffer or an error if the operation fails.
  ///
  /// [`Encode::encoded_len`] can be used to determine the required buffer size.
  fn encode(&self, context: &F::Context, buf: &mut [u8]) -> Result<usize, F::Error>;

  /// Returns the number of bytes needed to encode the message.
  ///
  /// This is used to determine the buffer size required for encoding.
  fn encoded_len(&self, context: &F::Context) -> usize;

  /// Returns the number of bytes needed to encode the message with length-delimited.
  ///
  /// This is used to determine the buffer size required for encoding.
  fn encoded_length_delimited_len(&self, context: &F::Context) -> usize {
    let encoded_len = self.encoded_len(context);
    let len_size = varing::encoded_u32_varint_len(encoded_len as u32);
    len_size + encoded_len
  }

  /// Encodes the message into the provided buffer with length-delimited.
  ///
  /// Returns the number of bytes written to the buffer or an error if the operation fails.
  fn encode_length_delimited(
    &self,
    context: &F::Context,
    buf: &mut [u8],
  ) -> Result<usize, F::Error> {
    let encoded_len = self.encoded_len(context);
    let buf_len = buf.len();
    let offset = varing::encode_u32_varint_to(encoded_len as u32, buf).map_err(|e| {
      Error::from_varint_encode_error(e).update(self.encoded_length_delimited_len(context), buf_len)
    })?;

    let required = encoded_len + offset;
    if offset + encoded_len > buf_len {
      return Err(Error::insufficient_buffer(required, buf_len).into());
    }

    if offset >= buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    self
      .encode(context, &mut buf[offset..])
      .map(|v| {
        #[cfg(debug_assertions)]
        {
          crate::debug_assert_write_eq::<Self>(v, encoded_len);
        }

        required
      })
      .map_err(|mut e| {
        e.update_insufficient_buffer(required, buf_len);
        e
      })
  }

  /// Encodes the message into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_vec(&self, context: &F::Context) -> Result<std::vec::Vec<u8>, F::Error> {
    let mut buf = std::vec![0; self.encoded_len(context)];
    self.encode(context, &mut buf)?;
    Ok(buf)
  }

  /// Encodes the message into a [`Bytes`](::bytes::Bytes).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_bytes(&self, context: &F::Context) -> Result<super::bytes::Bytes, F::Error> {
    self.encode_to_vec(context).map(Into::into)
  }

  /// Encodes the message into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_length_delimited_to_vec(
    &self,
    context: &F::Context,
  ) -> Result<std::vec::Vec<u8>, F::Error> {
    let mut buf = std::vec![0; self.encoded_length_delimited_len(context)];
    self.encode_length_delimited(context, &mut buf)?;
    Ok(buf)
  }

  /// Encodes the message into a [`Bytes`](::bytes::Bytes).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_length_delimited_to_bytes(
    &self,
    context: &F::Context,
  ) -> Result<super::bytes::Bytes, F::Error> {
    self.encode_length_delimited_to_vec(context).map(Into::into)
  }
}

/// A trait for encoding only selected parts of a message based on a [`Selector`](Selectable::Selector).
///
/// `PartialEncode` is useful when you need to serialize only a subset of a message's fields,
/// as determined by a runtime selector. This is commonly used for filtering, patch updates,
/// or field projections in protocols with optional fields.
///
/// This trait complements [`Encode`] by offering more control over which fields are included.
pub trait PartialEncode<W: WireFormat<F>, F: Flavor + ?Sized>: Selectable<F> {
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
  ) -> Result<usize, F::Error>;

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
  fn partial_encoded_length_delimited_len(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> usize {
    let encoded_len = self.partial_encoded_len(context, selector);
    let len_size = varing::encoded_u32_varint_len(encoded_len as u32);
    len_size + encoded_len
  }

  /// Encodes the message into the provided buffer with length-delimited.
  ///
  /// Returns the number of bytes written to the buffer or an error if the operation fails.
  fn partial_encode_length_delimited(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::Error> {
    let encoded_len = self.partial_encoded_len(context, selector);
    let buf_len = buf.len();
    let offset = varing::encode_u32_varint_to(encoded_len as u32, buf).map_err(|e| {
      Error::from_varint_encode_error(e).update(
        self.partial_encoded_length_delimited_len(context, selector),
        buf_len,
      )
    })?;

    let required = encoded_len + offset;
    if offset + encoded_len > buf_len {
      return Err(Error::insufficient_buffer(required, buf_len).into());
    }

    if offset >= buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    self
      .partial_encode(context, &mut buf[offset..], selector)
      .map(|v| {
        #[cfg(debug_assertions)]
        {
          crate::debug_assert_write_eq::<Self>(v, encoded_len);
        }

        required
      })
      .map_err(|mut e| {
        e.update_insufficient_buffer(required, buf_len);
        e
      })
  }

  /// Encodes the message into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_to_vec(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> Result<std::vec::Vec<u8>, F::Error> {
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
  ) -> Result<super::bytes::Bytes, F::Error> {
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
  ) -> Result<std::vec::Vec<u8>, F::Error> {
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
  ) -> Result<super::bytes::Bytes, F::Error> {
    self
      .partial_encode_length_delimited_to_vec(context, selector)
      .map(Into::into)
  }
}

impl<F, W, T> Encode<W, F> for &T
where
  T: Encode<W, F> + ?Sized,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  fn encode(&self, context: &F::Context, buf: &mut [u8]) -> Result<usize, F::Error> {
    (*self).encode(context, buf)
  }

  fn encoded_len(&self, context: &F::Context) -> usize {
    (*self).encoded_len(context)
  }

  fn encode_length_delimited(
    &self,
    context: &F::Context,
    buf: &mut [u8],
  ) -> Result<usize, F::Error> {
    (*self).encode_length_delimited(context, buf)
  }

  fn encoded_length_delimited_len(&self, context: &F::Context) -> usize {
    (*self).encoded_length_delimited_len(context)
  }
}

impl<F, W, T> Encode<W, F> for Option<T>
where
  T: Encode<W, F>,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  fn encode(&self, context: &F::Context, buf: &mut [u8]) -> Result<usize, F::Error> {
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
  ) -> Result<usize, F::Error> {
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

impl<F, W, T> PartialEncode<W, F> for &T
where
  T: PartialEncode<W, F> + ?Sized,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  fn partial_encode(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::Error> {
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
  ) -> Result<usize, F::Error> {
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

impl<F, W, T> PartialEncode<W, F> for Option<T>
where
  T: PartialEncode<W, F>,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  fn partial_encode(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::Error> {
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
  ) -> Result<usize, F::Error> {
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
      impl<F, W, T> Encode<W, F> for $ty
      where
        T: Encode<W, F> + ?Sized,
        F: Flavor + ?Sized,
        W: WireFormat<F>,
      {
        fn encode(&self, context: &F::Context, buf: &mut [u8]) -> Result<usize, F::Error> {
          (**self).encode(context, buf)
        }

        fn encoded_len(&self, context: &F::Context) -> usize {
          (**self).encoded_len(context)
        }

        fn encode_length_delimited(
          &self,
          context: &F::Context,
          buf: &mut [u8],
        ) -> Result<usize, F::Error> {
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
      impl<T, F> Selectable<F> for $ty
      where
        T: ?Sized + Selectable<F>,
        F: Flavor + ?Sized,
      {
        type Selector = T::Selector;

        fn is_empty(&self) -> bool {
          (**self).is_empty()
        }
      }

      impl<F, W, T> PartialEncode<W, F> for $ty
      where
        T: PartialEncode<W, F> + Selectable<F> + ?Sized,
        F: Flavor + ?Sized,
        W: WireFormat<F>,
      {
        fn partial_encode(
          &self,
          context: &F::Context,
          buf: &mut [u8],
          selector: &Self::Selector,
        ) -> Result<usize, F::Error> {
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
        ) -> Result<usize, F::Error> {
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
