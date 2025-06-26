use super::{ReadBuf, Tag, WireType};

/// The unknown type, used for forward and backward compatibility.
/// The data is stored as a byte array, including the wire type and the tag,
/// and the data.
///
/// When the older version trying to decode the newer version, for the tag that
/// is not recognized, it will be stored in this variant. And when the older version
/// trying to forward the data, the data stored in this variant will be forwarded
/// as is.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Unknown<B: ?Sized> {
  tag: Tag,
  wire_type: WireType,
  data_offset: usize,
  data: B,
}

impl<'a, B> Unknown<&'a B>
where
  B: ?Sized,
{
  /// Creates a new unknown by ref
  #[inline]
  pub const fn from_data_ref(
    tag: Tag,
    wire_type: WireType,
    data_offset: usize,
    data: &'a B,
  ) -> Self {
    Self {
      tag,
      wire_type,
      data_offset,
      data,
    }
  }
}

impl<B> Unknown<B>
where
  B: ?Sized,
{
  /// Creates a new unknown
  ///
  /// In some of [`Flavor`] implementation, the data contains the identifier at the beginning
  /// of the data. The identifier is used to identify the unknown data type. So the `data_offset`
  /// is used to skip the identifier. The reason why the identifier is better to keep in the
  /// data is that when encoding the unknown data, there is no more need to encode the identifier
  /// again, and the data itself is the encoded format of the `Unknown` type.
  #[inline]
  pub(super) const fn new(tag: Tag, wire_type: WireType, data_offset: usize, data: B) -> Self
  where
    B: Sized,
  {
    Self {
      tag,
      wire_type,
      data_offset,
      data,
    }
  }

  /// Returns the tag of the unknown data type.
  #[inline]
  pub const fn tag(&self) -> Tag {
    self.tag
  }

  /// Returns the wire type of the unknown data type.
  #[inline]
  pub const fn wire_type(&self) -> WireType {
    self.wire_type
  }

  /// Returns the actual data of the unknown data type.
  ///
  /// Note: The data does not include the wire type and the tag.
  /// If you want to access the original data, use [`raw`] instead.
  #[inline]
  pub fn data(&self) -> &[u8]
  where
    B: ReadBuf,
  {
    let bytes = self.data.as_bytes();
    let len = bytes.len();
    if len < self.data_offset {
      return &[];
    }

    if self.data_offset == 0 {
      return bytes;
    }

    &self.data.as_bytes()[self.data_offset..]
  }

  /// Returns the raw data of the unknown data type.
  ///
  /// Note: The data includes the wire type and the tag.
  /// If you want to access the actual data, use [`data`] instead.
  #[inline]
  pub fn raw(&self) -> &[u8]
  where
    B: ReadBuf,
  {
    self.data.as_bytes()
  }

  /// Converts the unknown data type to an borrowed type.
  #[inline]
  pub const fn as_ref(&self) -> Unknown<&B> {
    Unknown {
      tag: self.tag,
      wire_type: self.wire_type,
      data_offset: self.data_offset,
      data: &self.data,
    }
  }
}

impl<B: ?Sized> Unknown<B> {
  /// Converts the `Unknown<B>` to `Unknown<N>`.
  pub fn map<'a, N>(&'a self) -> Unknown<N>
  where
    N: From<&'a [u8]>,
    B: ReadBuf,
  {
    Unknown {
      tag: self.tag,
      wire_type: self.wire_type,
      data_offset: self.data_offset,
      data: N::from(self.raw()),
    }
  }

  /// Converts the `Unknown<B>` to `Unknown<N>`.
  pub fn consume_map<N>(self) -> Unknown<N>
  where
    N: From<B>,
    B: Sized,
  {
    Unknown {
      tag: self.tag,
      wire_type: self.wire_type,
      data_offset: self.data_offset,
      data: N::from(self.data),
    }
  }
}
