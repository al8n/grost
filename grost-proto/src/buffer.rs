use core::ops::{Bound, RangeBounds};

/// A buffer that stores the `T`.
pub trait Buffer<T> {
  /// Creates a new buffer.
  fn new() -> Self;

  /// Pushes the unknown data type to the buffer, if the buffer is full,
  /// the given value will be returned back.
  fn push(&mut self, value: T) -> Option<T>;

  /// Returns the capacity of the buffer.
  fn capacity(&self) -> usize;

  /// Returns the length of the buffer.
  fn len(&self) -> usize;

  /// Returns a slice of the unknown data type.
  fn as_slice(&self) -> &[T];

  /// Returns `true` if the buffer is empty.
  fn is_empty(&self) -> bool {
    self.len() == 0
  }
}

impl<T> Buffer<T> for () {
  fn new() -> Self {}

  fn push(&mut self, value: T) -> Option<T> {
    Some(value)
  }

  fn capacity(&self) -> usize {
    0
  }

  fn len(&self) -> usize {
    0
  }

  fn as_slice(&self) -> &[T] {
    &[]
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::vec::Vec;

  impl<T> Buffer<T> for Vec<T> {
    fn new() -> Self {
      Vec::new()
    }

    fn push(&mut self, value: T) -> Option<T> {
      if self.len() < self.capacity() {
        self.push(value);
        None
      } else {
        Some(value)
      }
    }

    fn capacity(&self) -> usize {
      self.capacity()
    }

    fn len(&self) -> usize {
      self.len()
    }

    fn as_slice(&self) -> &[T] {
      self.as_slice()
    }
  }
};

#[cfg(feature = "heapless_0_8")]
const _: () = {
  use heapless_0_8::Vec;

  impl<T, const N: usize> Buffer<T> for Vec<T, N> {
    fn new() -> Self {
      Vec::new()
    }

    fn push(&mut self, value: T) -> Option<T> {
      self.push(value).err()
    }

    fn capacity(&self) -> usize {
      self.capacity()
    }

    fn len(&self) -> usize {
      <[T]>::len(self)
    }

    fn is_empty(&self) -> bool {
      <[T]>::is_empty(self)
    }

    fn as_slice(&self) -> &[T] {
      self.as_slice()
    }
  }
};

pub trait BytesBuffer {
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

impl BytesBuffer for &[u8] {
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
impl BytesBuffer for bytes_1::Bytes {
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
impl BytesBuffer for Vec<u8> {
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
impl<const N: usize> BytesBuffer for heapless_0_8::Vec<u8, N> {
  fn is_empty(&self) -> bool {
    self.is_empty()
  }

  fn len(&self) -> usize {
    <[u8]>::len(self)
  }

  fn slice(&self, range: impl RangeBounds<usize>) -> Self {
    let len = BytesBuffer::len(self);

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
