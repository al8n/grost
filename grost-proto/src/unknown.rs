use crate::{
  buffer::Buf,
  error::{DecodeError, EncodeError},
  flavors::Flavor,
  identifier::Identifier,
};

/// The unknown type, used for forward and backward compatibility.
/// The data is stored as a byte array, including the wire type and the tag,
/// and the data.
///
/// When the older version trying to decode the newer version, for the tag that
/// is not recognized, it will be stored in this variant. And when the older version
/// trying to forward the data, the data stored in this variant will be forwarded
/// as is.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Unknown<B: ?Sized, F: Flavor + ?Sized> {
  _flavor: core::marker::PhantomData<F>,
  tag: F::Tag,
  wire_type: F::WireType,
  encoded_identifier_len: usize,
  data: B,
}

impl<B, F> Unknown<B, F>
where
  F: Flavor + ?Sized,
  B: ?Sized,
{
  /// Decodes the unknown data from the given context and data.
  ///
  /// Returns the number of bytes consumed and the decoded unknown data.
  pub fn decode(ctx: &F::Context, data: &B) -> Result<(usize, Self), DecodeError<F>>
  where
    B: Buf + Sized,
  {
    let (identifier_len, identifier) = <F::Identifier as Identifier<F>>::decode(data.buffer())?;

    let data_len = F::peek_raw(ctx, identifier.wire_type(), data)?;
    let total_len = identifier_len + data_len;
    if total_len > data.remaining() {
      return Err(DecodeError::insufficient_data_with_requested(
        data.remaining(),
        total_len,
      ));
    }
    Ok((
      identifier_len + data_len,
      Self {
        _flavor: core::marker::PhantomData,
        tag: identifier.tag(),
        wire_type: identifier.wire_type(),
        encoded_identifier_len: identifier_len,
        data: data.segment(..total_len),
      },
    ))
  }

  /// Encodes the unknown data into the given buffer.
  pub fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError<F>>
  where
    B: Buf,
  {
    let value_bytes = self.raw();
    let value_len = value_bytes.len();
    if value_len > buf.len() {
      return Err(EncodeError::buffer_too_small(value_len, buf.len()));
    }

    buf[..value_len].copy_from_slice(value_bytes);
    Ok(value_len)
  }

  /// Returns the encoded length of the unknown data.
  pub fn encoded_len(&self) -> usize
  where
    B: Buf,
  {
    self.raw().len()
  }

  /// Returns the tag of the unknown data type.
  #[inline]
  pub const fn tag(&self) -> F::Tag {
    self.tag
  }

  /// Returns the wire type of the unknown data type.
  #[inline]
  pub const fn wire_type(&self) -> F::WireType {
    self.wire_type
  }

  /// Returns the actual data of the unknown data type.
  ///
  /// Note: The data does not include the wire type and the tag.
  /// If you want to access the original data, use [`raw`] instead.
  #[inline]
  pub fn data(&self) -> &[u8]
  where
    B: Buf,
  {
    if self.data.remaining() <= self.encoded_identifier_len {
      return &[];
    }

    if self.encoded_identifier_len == 0 {
      return self.data.buffer();
    }

    &self.data.buffer()[self.encoded_identifier_len..]
  }

  /// Returns the owned data of the unknown data type.
  ///
  /// Note: The data does not include the wire type and the tag.
  #[inline]
  pub fn data_owned(&self) -> B
  where
    B: Buf + Sized,
  {
    if self.data.remaining() <= self.encoded_identifier_len {
      return self.data.segment(..0);
    }

    if self.encoded_identifier_len == 0 {
      return self.data.segment(..);
    }

    self.data.segment(self.encoded_identifier_len..)
  }

  /// Returns the raw data of the unknown data type.
  ///
  /// Note: The data includes the wire type and the tag.
  /// If you want to access the actual data, use [`data`] instead.
  #[inline]
  pub fn raw(&self) -> &[u8]
  where
    B: Buf,
  {
    self.data.buffer()
  }

  /// Returns the owned raw data of the unknown data type.
  ///
  /// Note: The data includes the wire type and the tag.
  /// If you want to access the actual data, use [`data_owned`] instead.
  #[inline]
  pub fn raw_owned(&self) -> B
  where
    B: Buf + Sized,
  {
    self.data.segment(..)
  }
}

impl<B, F> Unknown<B, F>
where
  F: Flavor + ?Sized,
{
  /// Converts the `Unknown<B>` to `Unknown<N>`.
  pub fn map<'a, N>(&'a self) -> Unknown<N, F>
  where
    N: From<&'a [u8]>,
    B: Buf,
  {
    Unknown {
      tag: self.tag,
      wire_type: self.wire_type,
      encoded_identifier_len: self.encoded_identifier_len,
      data: N::from(self.raw()),
      _flavor: core::marker::PhantomData,
    }
  }

  /// Converts the `Unknown<B>` to `Unknown<N>`.
  pub fn consume_map<N>(self) -> Unknown<N, F>
  where
    N: From<B>,
    B: Sized,
  {
    Unknown {
      tag: self.tag,
      wire_type: self.wire_type,
      encoded_identifier_len: self.encoded_identifier_len,
      data: N::from(self.data),
      _flavor: core::marker::PhantomData,
    }
  }
}

#[test]
fn t() {}
