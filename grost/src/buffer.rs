use core::ops::{Bound, RangeBounds};

pub trait Buffer {
  /// Returns the length of the buffer.
  fn len(&self) -> usize;

  /// Returns `true` if the buffer is empty.
  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Extracts a slice containing the entire vector.
  fn as_bytes(&self) -> &[u8];

  /// Returns a slice of self for the provided range.
  ///
  /// This will increment the reference count for the underlying memory and
  /// return a new `Bytes` handle set to the slice.
  fn slice(&self, range: impl RangeBounds<usize>) -> Self;
}

impl Buffer for &[u8] {
  fn is_empty(&self) -> bool {
    <[u8]>::is_empty(self)
  }

  fn len(&self) -> usize {
    <[u8]>::len(self)
  }

  fn slice(&self, range: impl RangeBounds<usize>) -> Self {
    let len = self.len();

    let begin = match range.start_bound() {
      Bound::Included(&n) => n,
      Bound::Excluded(&n) => n.checked_add(1).expect("out of range"),
      Bound::Unbounded => 0,
    };

    let end = match range.end_bound() {
      Bound::Included(&n) => n.checked_add(1).expect("out of range"),
      Bound::Excluded(&n) => n,
      Bound::Unbounded => len,
    };

    &self[begin..end]
  }

  fn as_bytes(&self) -> &[u8] {
    self
  }
}

#[cfg(feature = "bytes_1")]
impl Buffer for bytes_1::Bytes {
  fn len(&self) -> usize {
    self.len()
  }

  fn is_empty(&self) -> bool {
    self.is_empty()
  }

  fn slice(&self, range: impl RangeBounds<usize>) -> Self {
    self.slice(range)
  }

  fn as_bytes(&self) -> &[u8] {
    self.as_ref()
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Buffer for Vec<u8> {
  fn len(&self) -> usize {
    self.len()
  }

  fn is_empty(&self) -> bool {
    self.is_empty()
  }

  fn slice(&self, range: impl RangeBounds<usize>) -> Self {
    let len = self.len();

    let begin = match range.start_bound() {
      Bound::Included(&n) => n,
      Bound::Excluded(&n) => n.checked_add(1).expect("out of range"),
      Bound::Unbounded => 0,
    };

    let end = match range.end_bound() {
      Bound::Included(&n) => n.checked_add(1).expect("out of range"),
      Bound::Excluded(&n) => n,
      Bound::Unbounded => len,
    };

    self[begin..end].to_vec()
  }

  fn as_bytes(&self) -> &[u8] {
    self.as_slice()
  }
}

#[cfg(feature = "heapless_0_8")]
impl<const N: usize> Buffer for heapless_0_8::Vec<u8, N> {
  fn is_empty(&self) -> bool {
    self.is_empty()
  }

  fn len(&self) -> usize {
    self.len()
  }

  fn slice(&self, range: impl RangeBounds<usize>) -> Self {
    let len = self.len();

    let begin = match range.start_bound() {
      Bound::Included(&n) => n,
      Bound::Excluded(&n) => n.checked_add(1).expect("out of range"),
      Bound::Unbounded => 0,
    };

    let end = match range.end_bound() {
      Bound::Included(&n) => n.checked_add(1).expect("out of range"),
      Bound::Excluded(&n) => n,
      Bound::Unbounded => len,
    };

    // Safety: the slice is guaranteed to be within the bounds of the original buffer
    heapless_0_8::Vec::from_slice(&self[begin..end]).unwrap()
  }

  fn as_bytes(&self) -> &[u8] {
    self.as_slice()
  }
}
