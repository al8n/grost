use core::ops::{Bound, RangeBounds};

/// A trait for implementing custom buffers that can store values of type `T`.
///
/// This trait is designed for scenarios where you need flexible storage behavior,
/// particularly useful in decoding operations where you may encounter unknown or
/// unexpected data that needs to be handled differently based on your requirements.
///
/// ## Built-in Implementations
///
/// There are two built-in buffer implementations:
///
/// - `()` (unit type): A "black hole" buffer that silently discards all values.
///   Use this when you want to ignore unknown data during decoding operations.
/// - [`Vec<T>`]: A growable buffer that stores all values and automatically expands
///   its capacity as needed. Use this when you want to collect and preserve
///   unknown data for later inspection or processing.
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

  fn push(&mut self, _: T) -> Option<T> {
    None
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

#[cfg(feature = "heapless_0_9")]
const _: () = {
  use heapless_0_9::{LenType, Vec};

  impl<T, const N: usize, L: LenType> Buffer<T> for Vec<T, N, L> {
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

/// A trait for implementing custom buffers that can store and manipulate byte sequences.
pub trait Buf<'a>: 'a {
  // /// Advances the buffer by `n` bytes.
  // fn advance(&mut self, n: usize);

  /// Returns the number of bytes remaining in the buffer.
  fn len(&self) -> usize;

  /// Returns `true` if there are remaining bytes in the buffer.
  fn is_empty(&self) -> bool;

  /// Returns a slice of self for the provided range.
  fn slice(&self, range: impl RangeBounds<usize>) -> Self;

  // /// Splits the buf into two at the given index.
  // ///
  // /// Afterwards `self` contains elements `[at, len)`, and the returned
  // /// `Buf` contains elements `[0, at)`.
  // fn split_to(&mut self, at: usize) -> Self;

  // /// Splits the buf into two at the given index.
  // ///
  // /// Afterwards `self` contains elements `[0, at)`, and the returned `Buf`
  // /// contains elements `[at, len)`.
  // fn split_off(&mut self, at: usize) -> Self;

  /// Returns a slice of the buffer.
  fn chunk(&self) -> &'a [u8];
}

impl<'a> Buf<'a> for &'a [u8] {
  // fn advance(&mut self, n: usize) {
  //   *self = &self[n..];
  // }

  fn len(&self) -> usize {
    <[u8]>::len(self)
  }

  fn is_empty(&self) -> bool {
    <[u8]>::is_empty(self)
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

  // fn split_to(&mut self, at: usize) -> Self {
  //   let len = self.len();

  //   if len == at {
  //     let old = *self;
  //     *self = &[];
  //     return old;
  //   }

  //   if at == 0 {
  //     return &[];
  //   }

  //   assert!(
  //     at <= len,
  //     "split_to out of bounds: {:?} <= {:?}",
  //     at,
  //     len,
  //   );

  //   let output = &self[..at];
  //   *self = &self[at..];

  //   output
  // }

  // fn split_off(&mut self, at: usize) -> Self {
  //   let len = self.len();

  //   if len == at {
  //     return &[];
  //   }

  //   if at == 0 {
  //     return *self;
  //   }

  //   assert!(
  //     at <= len,
  //     "split_off out of bounds: {:?} <= {:?}",
  //     at,
  //     len,
  //   );

  //   &self[at..]
  // }

  fn chunk(&self) -> &'a [u8] {
    self
  }
}

// #[cfg(feature = "bytes_1")]
// const _: () = {
//   impl<'a> Buf<'a> for bytes_1::Bytes {
//     // fn advance(&mut self, n: usize) {
//     //   bytes_1::Buf::advance(self, n);
//     // }

//     fn len(&self) -> usize {
//       self.len()
//     }

//     fn is_empty(&self) -> bool {
//       self.is_empty()
//     }

//     fn slice(&self, range: impl RangeBounds<usize>) -> Self {
//       self.slice(range)
//     }

//     // fn split_to(&mut self, at: usize) -> Self {
//     //   self.split_to(at)
//     // }

//     // fn split_off(&mut self, at: usize) -> Self {
//     //   self.split_off(at)
//     // }

//     fn chunk(&self) -> &[u8] {
//       self.as_ref()
//     }
//   }
// };

#[cfg(feature = "heapless_0_9")]
impl<const N: usize, L: heapless_0_9::LenType> BytesBuffer for heapless_0_9::Vec<u8, N, L> {
  // fn new() -> Self {
  //   heapless_0_9::Vec::new()
  // }

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
    heapless_0_9::Vec::from_slice(&self[begin..end]).unwrap()
  }

  fn as_bytes(&self) -> &[u8] {
    self.as_slice()
  }
}
