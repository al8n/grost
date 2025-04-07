use ref_cast::RefCast;
use varing::encoded_u32_varint_len;

use crate::{merge, Encode, EncodeError, Tag, Wirable, WireType};

/// A wrapper type for repeated fields.
/// 
/// This type is used to implement `Encode` and `Decode` traits for collections
/// of items. It is a transparent wrapper around the inner type collection `T`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, RefCast)]
#[repr(transparent)]
pub struct Repeated<T>([T]);

impl<T> AsRef<[T]> for Repeated<T> {
  fn as_ref(&self) -> &[T] {
    &self.0
  }
}

impl<T> AsMut<[T]> for Repeated<T> {
  fn as_mut(&mut self) -> &mut [T] {
    &mut self.0
  }
}

impl<T> Repeated<T> {
  /// Creates a new `Repeated` instance.
  #[inline]
  pub fn new(inner: &[T]) -> &Self {
    RefCast::ref_cast(inner)
  }

  /// Returns a reference to the inner collection.
  #[inline]
  pub const fn inner(&self) -> &[T] {
    &self.0
  }
}

impl<T> Wirable for Repeated<T> {}

impl<T> Repeated<T>
where
  T: Encode,
{
  fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
    todo!()
  }

  /// Returns the number of bytes required to encode the collection.
  /// This method is used to calculate the size of the encoded data.
  pub fn encoded_len(&self, tag: Tag) -> usize {
    match T::WIRE_TYPE {
      WireType::Zst => encoded_u32_varint_len(self.0.len() as u32),
      WireType::Varint => self.0.iter().map(Encode::encoded_len).sum(),
      WireType::LengthDelimited => {
        let merged = merge(WireType::LengthDelimited, tag);
        let merged_len = encoded_u32_varint_len(merged);
        self.0.iter().map(|item| {
          merged_len + item.encoded_len_with_prefix()
        }).sum()
      },
      WireType::Byte => self.0.len(),
      WireType::Fixed16 => self.0.len() * 2,
      WireType::Fixed32 => self.0.len() * 4,
      WireType::Fixed64 => self.0.len() * 8,
      WireType::Fixed128 => self.0.len() * 16,
    }
  }
}
