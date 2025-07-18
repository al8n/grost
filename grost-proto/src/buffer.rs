use core::ops::{Bound, RangeBounds};

pub use stack_buffer::StackBuffer;

use crate::{flavors::Flavor, unknown::Unknown};
mod stack_buffer;

/// The default buffer type for storing unknown data or repeated values in decoding operations.
#[cfg(all(
  any(feature = "std", feature = "alloc"),
  not(any(feature = "smallvec_1"))
))]
pub type DefaultBuffer<T> = std::vec::Vec<T>;

/// The default buffer type for storing unknown data or repeated values in decoding operations.
#[cfg(all(
  not(any(feature = "std", feature = "alloc")),
  not(any(feature = "arrayvec_0_7"))
))]
pub type DefaultBuffer<T> = stack_buffer::StackBuffer<T>;

/// The default buffer type for storing unknown data or repeated values in decoding operations.
#[cfg(all(not(any(feature = "std", feature = "alloc")), feature = "arrayvec_0_7"))]
pub type DefaultBuffer<T> = arrayvec_0_7::ArrayVec<T, 16>;

/// The default buffer type for storing unknown data or repeated values in decoding operations.
#[cfg(all(any(feature = "std", feature = "alloc"), feature = "smallvec_1"))]
pub type DefaultBuffer<T> = smallvec_1::SmallVec<[T; 2]>;

/// A trait for implementing custom buffers that can store values of type `T`.
///
/// This trait provides a unified interface for different buffer implementations used
/// in decoding operations where you may encounter unknown, repeated, or unexpected data
/// that needs to be collected and processed. The trait is designed to work across
/// different environments, from `no-std`/`no-alloc` embedded systems to standard
/// heap-allocated environments.
///
/// # Built-in Implementations
///
/// The following buffer types implement this trait:
///
/// ## Heap-Allocated Buffers
///
/// - **[`Vec<T>`]**
/// - **[`SmallVec<[T; 4]>`](smallvec_1::SmallVec)** (requires `smallvec` and any of `std` and `alloc` features are enabled)
/// - **[`TinyVec<A>`](tinyvec_1::TinyVec)** (requires `tinyvec` feature and any of `std` and `alloc` features are enabled)
///
/// ## Stack-Allocated Buffers
///
/// All stack buffers are `no-alloc` and have fixed capacity. When full, `push()` will return
/// the value back instead of storing it, and in the derived [`Decode`](crate::decode::Decode) implementations, it will
/// raise an error.
///
/// - **[`StackBuffer<T>`]**
/// - **[`arrayvec::ArrayVec<T, 16>`](arrayvec_0_7::ArrayVec)** (requires `arrayvec` feature)
/// - **[`tinyvec::ArrayVec<A>`](tinyvec_1::ArrayVec)** (requires `tinyvec` feature)
pub trait Buffer {
  /// The type of the items stored in the buffer.
  type Item;

  /// Creates a new buffer.
  fn new() -> Self;

  /// Creates a new buffer with the specified capacity.
  ///
  /// Returns `None` if the capacity is too large for this buffer type.
  fn with_capacity(capacity: usize) -> Option<Self>
  where
    Self: Sized;

  /// Pushes a value to the buffer.
  ///
  /// If the buffer is full, the given value will be returned back.
  fn push(&mut self, value: Self::Item) -> Option<Self::Item>;

  /// Returns the capacity of the buffer.
  fn capacity(&self) -> usize;

  /// Returns the length of the buffer.
  fn len(&self) -> usize;

  /// Returns a slice of the stored data.
  fn as_slice(&self) -> &[Self::Item];

  /// Returns a mutable slice of the stored data.
  fn as_mut_slice(&mut self) -> &mut [Self::Item];

  /// Returns `true` if the buffer is empty.
  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Returns an iterator over the items in the buffer.
  fn iter(&self) -> impl Iterator<Item = &Self::Item> {
    self.as_slice().iter()
  }

  /// Returns a mutable iterator over the items in the buffer.
  fn iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Item> {
    self.as_mut_slice().iter_mut()
  }

  /// Consumes the buffer and returns an iterator over the items.
  fn into_iter(self) -> impl Iterator<Item = Self::Item>;
}

/// A trait for implementing custom buffers that can store unknown data.
pub trait UnknownBuffer<RB, F: Flavor + ?Sized>: Buffer<Item = Unknown<RB, F>> {}

impl<T, RB, F> UnknownBuffer<RB, F> for T
where
  T: Buffer<Item = Unknown<RB, F>>,
  RB: ReadBuf,
  F: Flavor + ?Sized,
{
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::vec::Vec;

  impl<T> Buffer for Vec<T> {
    type Item = T;

    fn new() -> Self {
      Vec::new()
    }

    fn with_capacity(capacity: usize) -> Option<Self>
    where
      Self: Sized,
    {
      Some(Vec::with_capacity(capacity))
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

    fn as_mut_slice(&mut self) -> &mut [T] {
      self.as_mut_slice()
    }

    fn into_iter(self) -> impl Iterator<Item = Self::Item> {
      IntoIterator::into_iter(self)
    }
  }
};

#[cfg(feature = "smallvec_1")]
const _: () = {
  use smallvec_1::SmallVec;

  impl<T, const N: usize> Buffer for SmallVec<[T; N]> {
    type Item = T;

    fn new() -> Self {
      SmallVec::new()
    }

    fn with_capacity(capacity: usize) -> Option<Self>
    where
      Self: Sized,
    {
      Some(SmallVec::with_capacity(capacity))
    }

    fn push(&mut self, value: T) -> Option<T> {
      self.push(value);
      None
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

    fn as_mut_slice(&mut self) -> &mut [T] {
      self.as_mut_slice()
    }

    fn into_iter(self) -> impl Iterator<Item = Self::Item> {
      IntoIterator::into_iter(self)
    }
  }
};

#[cfg(feature = "arrayvec_0_7")]
const _: () = {
  use arrayvec_0_7::ArrayVec;

  impl<T, const N: usize> Buffer for ArrayVec<T, N> {
    type Item = T;

    fn new() -> Self {
      ArrayVec::new()
    }

    fn with_capacity(capacity: usize) -> Option<Self>
    where
      Self: Sized,
    {
      if capacity > N {
        None
      } else {
        Some(ArrayVec::new())
      }
    }

    fn push(&mut self, value: T) -> Option<T> {
      if self.len() < N {
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

    fn as_mut_slice(&mut self) -> &mut [T] {
      self.as_mut_slice()
    }

    fn into_iter(self) -> impl Iterator<Item = Self::Item> {
      IntoIterator::into_iter(self)
    }
  }
};

#[cfg(feature = "tinyvec_1")]
const _: () = {
  use tinyvec_1::{Array, ArrayVec};

  impl<T, A> Buffer for ArrayVec<A>
  where
    A: Array<Item = T>,
  {
    type Item = T;

    fn new() -> Self {
      ArrayVec::new()
    }

    fn with_capacity(capacity: usize) -> Option<Self>
    where
      Self: Sized,
    {
      if capacity > A::CAPACITY {
        None
      } else {
        Some(ArrayVec::new())
      }
    }

    fn push(&mut self, value: T) -> Option<T> {
      if self.len() < A::CAPACITY {
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

    fn as_mut_slice(&mut self) -> &mut [T] {
      self.as_mut_slice()
    }

    fn into_iter(self) -> impl Iterator<Item = Self::Item> {
      IntoIterator::into_iter(self)
    }
  }

  #[cfg(any(feature = "std", feature = "alloc"))]
  const _: () = {
    use tinyvec_1::TinyVec;

    impl<T, A> Buffer for TinyVec<A>
    where
      A: Array<Item = T>,
    {
      type Item = T;

      fn new() -> Self {
        TinyVec::new()
      }

      fn with_capacity(capacity: usize) -> Option<Self>
      where
        Self: Sized,
      {
        Some(TinyVec::with_capacity(capacity))
      }

      fn push(&mut self, value: T) -> Option<T> {
        self.push(value);
        None
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

      fn as_mut_slice(&mut self) -> &mut [T] {
        self.as_mut_slice()
      }

      fn into_iter(self) -> impl Iterator<Item = Self::Item> {
        IntoIterator::into_iter(self)
      }
    }
  };
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
pub trait WriteBuf {
  /// Advance the internal cursor of the write buffer
  fn advance_mut(&mut self, cnt: usize);

  /// Returns a mutable slice of the buffer.
  fn remaining_mut(&mut self) -> usize;
}

impl WriteBuf for &mut [u8] {
  fn advance_mut(&mut self, _: usize) {}

  fn remaining_mut(&mut self) -> usize {
    self.len()
  }
}

/// A trait for implementing custom buffers that can store and manipulate byte sequences.
pub trait ReadBuf: Clone {
  /// Returns an empty read buffer.
  fn empty() -> Self;

  /// Returns the number of bytes remaining in the buffer.
  fn len(&self) -> usize;

  /// Returns `true` if there are remaining bytes in the buffer.
  fn is_empty(&self) -> bool;

  /// Returns a slice of self for the provided range.
  fn slice(&self, range: impl RangeBounds<usize>) -> Self;

  /// Returns the bytes of the buffer.
  fn as_bytes(&self) -> &[u8];

  #[cfg(any(feature = "std", feature = "alloc"))]
  #[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
  /// Converts the read buffer to a `Vec<u8>` instance.
  fn to_vec(&self) -> Vec<u8> {
    self.as_bytes().to_vec()
  }

  #[cfg(all(feature = "bytes_1", any(feature = "std", feature = "alloc")))]
  #[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "bytes_1", any(feature = "std", feature = "alloc"))))
  )]
  /// Converts the read buffer to a `Bytes` instance.
  fn to_bytes(&self) -> crate::bytes::Bytes {
    crate::bytes::Bytes::copy_from_slice(self.as_bytes())
  }
}

impl ReadBuf for &[u8] {
  #[inline]
  fn empty() -> Self {
    &[]
  }

  #[inline]
  fn len(&self) -> usize {
    <[u8]>::len(self)
  }

  #[inline]
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

  #[inline]
  fn as_bytes(&self) -> &[u8] {
    self
  }
}

#[cfg(feature = "bytes_1")]
const _: () = {
  use bytes_1::Bytes;

  impl ReadBuf for Bytes {
    #[inline]
    fn empty() -> Self {
      Bytes::new()
    }

    #[inline]
    fn len(&self) -> usize {
      self.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
      self.is_empty()
    }

    fn slice(&self, range: impl RangeBounds<usize>) -> Self {
      Bytes::slice(self, range)
    }

    fn as_bytes(&self) -> &[u8] {
      self.as_ref()
    }

    #[cfg(all(feature = "bytes_1", any(feature = "std", feature = "alloc")))]
    fn to_bytes(&self) -> crate::bytes::Bytes {
      self.clone()
    }
  }
};

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
