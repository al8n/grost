use super::{DecodeError, EncodeError, Tag, WireType, split};

/// A buffer that stores the [`Unknown`] data type.
///
/// This trait is used for forward and backward compatibility for structs.
pub trait UnknownBuffer<B>: AsRef<[Unknown<B>]> {
  /// Pushes the unknown data type to the buffer, if the buffer is full,
  /// the given value will be returned back.
  fn push(&mut self, value: Unknown<B>) -> Option<Unknown<B>>;

  /// Returns the capacity of the buffer.
  fn capacity(&self) -> usize;

  /// Returns the length of the buffer.
  fn len(&self) -> usize;

  /// Returns `true` if the buffer is empty.
  fn is_empty(&self) -> bool {
    self.len() == 0
  }
}

/// A buffer that stores the [`UnknownRef`] data type.
///
/// This trait is used for forward and backward compatibility for structs.
pub trait UnknownRefBuffer<'a>: AsRef<[UnknownRef<'a>]> {
  /// Pushes the unknown data type to the buffer, if the buffer is full,
  /// the given value will be returned back.
  fn push(&mut self, value: UnknownRef<'a>) -> Option<UnknownRef<'a>>;

  /// Returns the capacity of the buffer.
  fn capacity(&self) -> usize;

  /// Returns the length of the buffer.
  fn len(&self) -> usize;

  /// Returns `true` if the buffer is empty.
  fn is_empty(&self) -> bool {
    self.len() == 0
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::vec::Vec;

  impl<B> UnknownBuffer<B> for Vec<Unknown<B>> {
    fn push(&mut self, value: Unknown<B>) -> Option<Unknown<B>> {
      self.push(value);
      None
    }

    fn capacity(&self) -> usize {
      self.capacity()
    }

    fn len(&self) -> usize {
      self.len()
    }

    fn is_empty(&self) -> bool {
      self.is_empty()
    }
  }

  impl<'a> UnknownRefBuffer<'a> for Vec<UnknownRef<'a>> {
    fn push(&mut self, value: UnknownRef<'a>) -> Option<UnknownRef<'a>> {
      self.push(value);
      None
    }

    fn capacity(&self) -> usize {
      self.capacity()
    }

    fn len(&self) -> usize {
      self.len()
    }

    fn is_empty(&self) -> bool {
      self.is_empty()
    }
  }
};

#[cfg(feature = "heapless_0_8")]
const _: () = {
  use heapless_0_8::Vec;

  impl<B, const N: usize> UnknownBuffer<B> for Vec<Unknown<B>, N> {
    fn push(&mut self, value: Unknown<B>) -> Option<Unknown<B>> {
      self.push(value).err()
    }

    fn capacity(&self) -> usize {
      self.capacity()
    }

    fn len(&self) -> usize {
      self.as_slice().len()
    }

    fn is_empty(&self) -> bool {
      self.is_empty()
    }
  }

  impl<'a, const N: usize> UnknownRefBuffer<'a> for Vec<UnknownRef<'a>, N> {
    fn push(&mut self, value: UnknownRef<'a>) -> Option<UnknownRef<'a>> {
      self.push(value).err()
    }

    fn capacity(&self) -> usize {
      self.capacity()
    }

    fn len(&self) -> usize {
      self.as_slice().len()
    }

    fn is_empty(&self) -> bool {
      self.is_empty()
    }
  }
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
pub struct Unknown<B> {
  wire_type: WireType,
  tag: Tag,
  data_offset: usize,
  data: B,
}

impl<B> Unknown<B> {
  /// Returns the [`WireType`] of the unknown data type.
  #[inline]
  pub const fn wire_type(&self) -> WireType {
    self.wire_type
  }

  /// Returns the tag of the unknown data type.
  #[inline]
  pub const fn tag(&self) -> Tag {
    self.tag
  }

  /// Returns the actual data of the unknown data type.
  ///
  /// Note: The data does not include the wire type and the tag.
  /// If you want to access the original data, use [`raw`] instead.
  #[inline]
  pub fn data(&self) -> &[u8]
  where
    B: AsRef<[u8]>,
  {
    &self.data.as_ref()[self.data_offset..]
  }

  /// Returns the raw data of the unknown data type.
  ///
  /// Note: The data includes the wire type and the tag.
  /// If you want to access the actual data, use [`data`] instead.
  #[inline]
  pub fn raw(&self) -> &[u8]
  where
    B: AsRef<[u8]>,
  {
    self.data.as_ref()
  }

  /// Encodes the unknown data type.
  pub fn encode(&self, dst: &mut [u8]) -> Result<usize, EncodeError>
  where
    B: AsRef<[u8]>,
  {
    let data = self.data.as_ref();
    let data_len = data.len();
    let dst_len = dst.len();
    if data_len > dst_len {
      return Err(EncodeError::insufficient_buffer(data_len, dst_len));
    }

    dst[..data_len].copy_from_slice(data);
    Ok(data_len)
  }

  /// Decodes the unknown data type.
  pub fn decode<'a>(buf: &'a [u8]) -> Result<(usize, Self), DecodeError>
  where
    B: From<&'a [u8]>,
  {
    let (mut data_offset, merged) = varing::decode_u32_varint(buf)?;
    let (wire_type, tag) = split(merged);

    macro_rules! consume_fixed {
      ($size:literal) => {{
        let end = data_offset + $size;
        if end > buf.len() {
          return Err(DecodeError::buffer_underflow());
        }

        Ok((
          end,
          Self {
            wire_type,
            tag,
            data_offset,
            data: B::from(&buf[..end]),
          },
        ))
      }};
    }

    match wire_type {
      WireType::LengthDelimited => {
        let (size_len, size) = varing::decode_u32_varint(&buf[data_offset..])?;
        data_offset += size_len;
        let end = data_offset + size as usize;

        if end > buf.len() {
          return Err(DecodeError::buffer_underflow());
        }

        let data = B::from(&buf[..end]);
        Ok((
          end,
          Self {
            wire_type,
            tag,
            data_offset,
            data,
          },
        ))
      }
      WireType::Varint => {
        let size_len = varing::consume_varint(&buf[data_offset..])?;
        let end = data_offset + size_len;
        Ok((
          end,
          Self {
            wire_type,
            tag,
            data_offset,
            data: B::from(&buf[..end]),
          },
        ))
      }
      WireType::Byte => consume_fixed!(1),
      WireType::Fixed16 => consume_fixed!(2),
      WireType::Fixed32 => consume_fixed!(4),
      WireType::Fixed64 => consume_fixed!(8),
      WireType::Fixed128 => consume_fixed!(16),
      WireType::Merged => consume_fixed!(0),
    }
  }

  /// Returns the length of the encoded data.
  #[inline]
  pub fn encoded_len(&self) -> usize
  where
    B: AsRef<[u8]>,
  {
    self.data.as_ref().len()
  }

  /// Converts the unknown data type to a reference type.
  pub fn as_ref(&self) -> UnknownRef<'_>
  where
    B: AsRef<[u8]>,
  {
    UnknownRef {
      wire_type: self.wire_type,
      tag: self.tag,
      data_offset: self.data_offset,
      data: self.data.as_ref(),
    }
  }

  /// Converts the unknown data type to an borrowed type.
  pub fn borrow(&self) -> Unknown<&B> {
    Unknown {
      wire_type: self.wire_type,
      tag: self.tag,
      data_offset: self.data_offset,
      data: &self.data,
    }
  }
}

impl<B> Unknown<B>
where
  B: super::Buffer,
{
  /// Decodes the unknown data type.
  pub fn decode_owned(buf: &B) -> Result<(usize, Self), DecodeError> {
    let buf_ref = buf.as_ref();
    let (mut data_offset, merged) = varing::decode_u32_varint(buf_ref)?;
    let (wire_type, tag) = split(merged);

    macro_rules! consume_fixed {
      ($size:literal) => {{
        let end = data_offset + $size;
        if end > buf_ref.len() {
          return Err(DecodeError::buffer_underflow());
        }

        Ok((
          end,
          Self {
            wire_type,
            tag,
            data_offset,
            data: buf.slice(..end),
          },
        ))
      }};
    }

    match wire_type {
      WireType::LengthDelimited => {
        let (size_len, size) = varing::decode_u32_varint(&buf_ref[data_offset..])?;
        data_offset += size_len;
        let end = data_offset + size as usize;

        if end > buf_ref.len() {
          return Err(DecodeError::buffer_underflow());
        }

        let data = buf.slice(..end);
        Ok((
          end,
          Self {
            wire_type,
            tag,
            data_offset,
            data,
          },
        ))
      }
      WireType::Varint => {
        let size_len = varing::consume_varint(&buf_ref[data_offset..])?;
        let end = data_offset + size_len;
        let data = buf.slice(..end);
        Ok((
          end,
          Self {
            wire_type,
            tag,
            data_offset,
            data,
          },
        ))
      }
      WireType::Byte => consume_fixed!(1),
      WireType::Fixed16 => consume_fixed!(2),
      WireType::Fixed32 => consume_fixed!(4),
      WireType::Fixed64 => consume_fixed!(8),
      WireType::Fixed128 => consume_fixed!(16),
      WireType::Merged => consume_fixed!(0),
    }
  }

  /// Converts the `Unknown<B>` to `Unknown<N>`.
  pub fn map<'a, N>(&'a self) -> Unknown<N>
  where
    N: From<&'a [u8]>,
  {
    Unknown {
      wire_type: self.wire_type,
      tag: self.tag,
      data_offset: self.data_offset,
      data: N::from(self.raw()),
    }
  }

  /// Converts the `Unknown<B>` to `Unknown<N>`.
  pub fn consume_map<N>(self) -> Unknown<N>
  where
    N: From<B>,
  {
    Unknown {
      wire_type: self.wire_type,
      tag: self.tag,
      data_offset: self.data_offset,
      data: N::from(self.data),
    }
  }
}

impl<'a, B> From<&'a Unknown<B>> for UnknownRef<'a>
where
  B: AsRef<[u8]>,
{
  fn from(value: &'a Unknown<B>) -> Self {
    value.as_ref()
  }
}

impl<'a, B> From<UnknownRef<'a>> for Unknown<B>
where
  B: From<&'a [u8]>,
{
  fn from(value: UnknownRef<'a>) -> Self {
    value.to_owned()
  }
}

impl<'a, B> From<&UnknownRef<'a>> for Unknown<B>
where
  B: From<&'a [u8]>,
{
  fn from(value: &UnknownRef<'a>) -> Self {
    value.to_owned()
  }
}

/// The reference type of the unknown data type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnknownRef<'a> {
  wire_type: WireType,
  tag: Tag,
  data_offset: usize,
  data: &'a [u8],
}

impl<'a> UnknownRef<'a> {
  /// Returns the [`WireType`] of the unknown data type.
  #[inline]
  pub const fn wire_type(&self) -> WireType {
    self.wire_type
  }

  /// Returns the tag of the unknown data type.
  #[inline]
  pub const fn tag(&self) -> Tag {
    self.tag
  }

  /// Returns the actual data of the unknown data type.
  ///
  /// Note: The data does not include the wire type and the tag.
  /// If you want to access the original data, use [`raw`] instead.
  #[inline]
  pub fn data(&self) -> &'a [u8] {
    &self.data[self.data_offset..]
  }

  /// Returns the raw data of the unknown data type.
  ///
  /// Note: The data includes the wire type and the tag.
  /// If you want to access the actual data, use [`data`] instead.
  #[inline]
  pub fn raw(&self) -> &'a [u8] {
    self.data
  }

  /// Converts the unknown data type to an owned type.
  pub fn to_owned<B>(&self) -> Unknown<B>
  where
    B: From<&'a [u8]>,
  {
    Unknown {
      wire_type: self.wire_type(),
      tag: self.tag(),
      data_offset: self.data_offset,
      data: B::from(self.raw()),
    }
  }

  /// Encodes the unknown data type.
  pub fn encode(&self, dst: &mut [u8]) -> Result<usize, EncodeError> {
    let data_len = self.data.len();
    let dst_len = dst.len();
    if data_len > dst_len {
      return Err(EncodeError::insufficient_buffer(data_len, dst_len));
    }

    dst[..data_len].copy_from_slice(self.data);
    Ok(data_len)
  }

  /// Returns the length of the encoded data.
  #[inline]
  pub fn encoded_len(&self) -> usize {
    self.data.len()
  }

  /// Decodes the unknown data type.
  pub fn decode(buf: &'a [u8]) -> Result<(usize, Self), DecodeError> {
    let (mut data_offset, merged) = varing::decode_u32_varint(buf)?;
    let (wire_type, tag) = split(merged);

    macro_rules! consume_fixed {
      ($size:literal) => {{
        let end = data_offset + $size;
        if end > buf.len() {
          return Err(DecodeError::buffer_underflow());
        }

        Ok((
          end,
          Self {
            wire_type,
            tag,
            data_offset,
            data: &buf[..end],
          },
        ))
      }};
    }

    match wire_type {
      WireType::LengthDelimited => {
        let (size_len, size) = varing::decode_u32_varint(&buf[data_offset..])?;
        data_offset += size_len;
        let end = data_offset + size as usize;

        if end > buf.len() {
          return Err(DecodeError::buffer_underflow());
        }

        let data = &buf[..end];
        Ok((
          end,
          Self {
            wire_type,
            tag,
            data_offset,
            data,
          },
        ))
      }
      WireType::Varint => {
        let size_len = varing::consume_varint(&buf[data_offset..])?;
        let end = data_offset + size_len;
        Ok((
          end,
          Self {
            wire_type,
            tag,
            data_offset,
            data: &buf[..end],
          },
        ))
      }
      WireType::Byte => consume_fixed!(1),
      WireType::Fixed16 => consume_fixed!(2),
      WireType::Fixed32 => consume_fixed!(4),
      WireType::Fixed64 => consume_fixed!(8),
      WireType::Fixed128 => consume_fixed!(16),
      WireType::Merged => consume_fixed!(0),
    }
  }
}
