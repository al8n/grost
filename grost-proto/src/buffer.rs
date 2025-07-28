use core::ops::{Bound, RangeBounds};

pub use stack_buffer::StackBuffer;

use crate::{
  error::{DecodeVarintError, EncodeVarintError},
  flavors::Flavor,
  unknown::Unknown,
};
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

  /// Reserves capacity for at least `additional` more elements.
  ///
  /// Returns `true` if the reservation was successful, `false` if the buffer is full.
  fn try_reserve(&mut self, additional: usize) -> bool;

  /// Try reserving capacity for exact `additional` more elements.
  ///
  /// Returns `true` if the reservation was successful, `false` if the buffer is full.
  fn try_reserve_exact(&mut self, additional: usize) -> bool;

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

    fn try_reserve(&mut self, additional: usize) -> bool {
      self.try_reserve(additional).is_ok()
    }

    fn try_reserve_exact(&mut self, additional: usize) -> bool {
      self.try_reserve_exact(additional).is_ok()
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

    fn try_reserve(&mut self, additional: usize) -> bool {
      self.try_reserve(additional).is_ok()
    }

    fn try_reserve_exact(&mut self, additional: usize) -> bool {
      self.try_reserve_exact(additional).is_ok()
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

    fn try_reserve(&mut self, additional: usize) -> bool {
      if self.len() + additional <= N {
        true
      } else {
        false
      }
    }

    fn try_reserve_exact(&mut self, additional: usize) -> bool {
      self.try_reserve(additional)
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

    fn try_reserve(&mut self, additional: usize) -> bool {
      if self.len() + additional <= A::CAPACITY {
        true
      } else {
        false
      }
    }

    fn try_reserve_exact(&mut self, additional: usize) -> bool {
      self.try_reserve(additional)
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

      fn try_reserve(&mut self, additional: usize) -> bool {
        self.try_reserve(additional).is_ok()
      }

      fn try_reserve_exact(&mut self, additional: usize) -> bool {
        self.try_reserve_exact(additional).is_ok()
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

/// The error type returned when resizing a write buffer fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResizeError {
  new_len: usize,
  current_len: usize,
  fill_value: u8,
}

impl core::fmt::Display for ResizeError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    if self.new_len > self.current_len {
      write!(
        f,
        "cannot resize buffer to {} bytes with value {}, current length is {} bytes",
        self.new_len, self.fill_value, self.current_len
      )
    } else {
      write!(
        f,
        "cannot shrink buffer to {} bytes, current length is {} bytes",
        self.new_len, self.current_len
      )
    }
  }
}

impl core::error::Error for ResizeError {}

impl ResizeError {
  /// Creates a new `ResizeError` instance.
  #[inline]
  pub const fn new(new_len: usize, current_len: usize, fill_value: u8) -> Self {
    Self {
      new_len,
      current_len,
      fill_value,
    }
  }

  /// Returns the new length of the resize error.
  #[inline]
  pub const fn new_len(&self) -> usize {
    self.new_len
  }

  /// Returns the current length of the resize error.
  #[inline]
  pub const fn current_len(&self) -> usize {
    self.current_len
  }

  /// Returns the fill value of the resize error.
  #[inline]
  pub const fn fill_value(&self) -> u8 {
    self.fill_value
  }
}

macro_rules! write_varint {
  ($($ty:ty),+$(,)?) => {
    paste::paste! {
      $(
        #[doc = "Writes an `" $ty "` value as a variable-length integer to the buffer."]
        fn [< write_ $ty _varint >](&mut self, value: $ty) -> Result<usize, EncodeVarintError> {
          varing::[< encode_ $ty _varint_to >](value, self.as_mut_slice())
        }
      )*
    }
  };
}

macro_rules! write_fixed {
  ($($ty:ty),+$(,)?) => {
    paste::paste! {
      $(
        #[doc = "Writes a `" $ty "` value in little-endian byte order to the beginning of the buffer."]
        ///
        #[doc = "Returns the number of bytes written (always `size_of::<" $ty ">()` for this type)"]
        ///
        /// # Panics
        ///
        /// Panics if the buffer is too small to hold the value.
        fn [< write_ $ty _le>](&mut self, value: $ty) -> usize {
          let len = core::mem::size_of::<$ty>();
          self.prefix_mut(len)
            .copy_from_slice(&value.to_le_bytes());
          len
        }

        #[doc = "Tries to write a `" $ty "` value in little-endian byte order to the beginning of the buffer."]
        ///
        #[doc = "This is the non-panicking version of [`write_" $ty "_le`](WriteBuf::write_" $ty "_le)."]
        ///
        /// Returns `None` if the buffer is too small to hold the value. Otherwise, it returns the number of bytes written.
        fn [< write_ $ty _le_checked>](&mut self, value: $ty) -> Option<usize> {
          let len = core::mem::size_of::<$ty>();
          self.prefix_mut_checked(len)
            .map(|slice| {
              slice.copy_from_slice(&value.to_le_bytes());
              len
            })
        }

        #[doc = "Writes a `" $ty "` value in big-endian byte order to the beginning of the buffer."]
        ///
        #[doc = "Returns the number of bytes written (always `size_of::<" $ty ">()` for this type)"]
        ///
        /// # Panics
        ///
        /// Panics if the buffer is too small to hold the value.
        fn [< write_ $ty _be>](&mut self, value: $ty) -> usize {
          let len = core::mem::size_of::<$ty>();
          self.prefix_mut(len)
            .copy_from_slice(&value.to_be_bytes());
          len
        }

        #[doc = "Tries to write a `" $ty "` value in big-endian byte order to the beginning of the buffer."]
        ///
        #[doc = "This is the non-panicking version of [`write_" $ty "_be`](WriteBuf::write_" $ty "_be)."]
        ///
        /// Returns `None` if the buffer is too small to hold the value. Otherwise, it returns the number of bytes written.
        fn [< write_ $ty _be_checked>](&mut self, value: $ty) -> Option<usize> {
          let len = core::mem::size_of::<$ty>();
          self.prefix_mut_checked(len)
            .map(|slice| {
              slice.copy_from_slice(&value.to_be_bytes());
              len
            })
        }

        #[doc = "Writes a `" $ty "` value in native-endian byte order to the beginning of the buffer."]
        ///
        #[doc = "Returns the number of bytes written (always `size_of::<" $ty ">()` for this type)"]
        ///
        /// # Panics
        ///
        /// Panics if the buffer is too small to hold the value.
        fn [< write_ $ty _ne>](&mut self, value: $ty) -> usize {
          let len = core::mem::size_of::<$ty>();
          self.prefix_mut(len)
            .copy_from_slice(&value.to_ne_bytes());
          len
        }

        #[doc = "Tries to write a `" $ty "` value in native-endian byte order to the beginning of the buffer."]
        ///
        /// The byte order depends on the target platform's endianness.
        #[doc = "This is the non-panicking version of [`write_" $ty "_ne`](WriteBuf::write_" $ty "_ne)."]
        ///
        /// Returns `None` if the buffer is too small to hold the value. Otherwise, it returns the number of bytes written.
        fn [< write_ $ty _ne_checked>](&mut self, value: $ty) -> Option<usize> {
          let len = core::mem::size_of::<$ty>();
          self.prefix_mut_checked(len)
            .map(|slice| {
              slice.copy_from_slice(&value.to_ne_bytes());
              len
            })
        }
      )*
    }
  };
}

/// A trait for implementing custom buffers that can store and manipulate byte sequences.
pub trait WriteBuf {
  /// Returns `true` if the buffer is empty.
  fn is_empty(&self) -> bool {
    self.len() == 0
  }

  /// Returns the length of the buffer.
  fn len(&self) -> usize;

  /// Returns the mutable slice of the buffer.
  fn as_mut_slice(&mut self) -> &mut [u8];

  /// Copies data from a slice into the buffer at the specified offset.
  /// Returns the number of bytes copied.
  ///
  /// # Panics
  ///
  /// Panics if `offset + src.len()` exceeds the buffer length.
  fn copy_from_slice_at(&mut self, src: &[u8], offset: usize) -> usize {
    let len = self.len();
    let end = offset + len;
    self.as_mut_slice()[offset..end].copy_from_slice(src);
    len
  }

  /// Tries to copy data from a slice into the buffer at the specified offset.
  ///
  /// Returns `None` if `offset + src.len()` would exceed the buffer length.
  fn try_copy_from_slice_at(&mut self, src: &[u8], offset: usize) -> Option<usize> {
    let len = self.len();
    let end = offset + len;
    if end <= len {
      self.as_mut_slice()[offset..end].copy_from_slice(src);
      Some(len)
    } else {
      None
    }
  }

  /// Fills the entire buffer with the specified byte value.
  fn fill(&mut self, value: u8) {
    self.as_mut_slice().fill(value);
  }

  /// Returns a mutable slice containing the first `len` bytes of the buffer.
  ///
  /// This method provides access to a prefix of the buffer's contents,
  /// allowing for efficient manipulation of a specific portion without
  /// affecting the rest of the buffer.
  ///
  /// # Panics
  ///
  /// Panics if `len` is greater than the buffer's length.
  fn prefix_mut(&mut self, len: usize) -> &mut [u8] {
    &mut self.as_mut_slice()[..len]
  }

  /// Returns a mutable slice containing the first `len` bytes of the buffer,
  /// or `None` if the buffer is too short.
  ///
  /// This is the non-panicking version of [`prefix_mut`](WriteBuf::prefix_mut).
  /// If `len ≤ buffer.len()`, returns a mutable slice containing the first
  /// `len` bytes. Otherwise, returns `None`.
  fn prefix_mut_checked(&mut self, len: usize) -> Option<&mut [u8]> {
    if len <= self.len() {
      Some(&mut self.as_mut_slice()[..len])
    } else {
      None
    }
  }

  /// Returns a mutable slice containing the last `len` bytes of the buffer.
  ///
  /// This method provides access to a suffix of the buffer's contents,
  /// allowing for efficient manipulation of the trailing portion without
  /// affecting the rest of the buffer.
  ///
  /// # Panics
  ///
  /// Panics if `len` is greater than the buffer's length.
  fn suffix_mut(&mut self, len: usize) -> &mut [u8] {
    let total_len = self.len();
    &mut self.as_mut_slice()[total_len - len..]
  }

  /// Returns a mutable slice containing the last `len` bytes of the buffer,
  /// or `None` if the buffer is too short.
  ///
  /// This is the non-panicking version of [`suffix_mut`](WriteBuf::suffix_mut).
  /// If `len ≤ buffer.len()`, returns a mutable slice containing the last
  /// `len` bytes. Otherwise, returns `None`.
  fn suffix_mut_checked(&mut self, len: usize) -> Option<&mut [u8]> {
    let total_len = self.len();
    if len <= total_len {
      Some(&mut self.as_mut_slice()[total_len - len..])
    } else {
      None
    }
  }

  /// Divides one mutable slice into two at an index.
  ///
  /// The first will contain all indices from `[0, mid)` (excluding
  /// the index `mid` itself) and the second will contain all
  /// indices from `[mid, len)` (excluding the index `len` itself).
  ///
  /// # Panics
  ///
  /// Panics if `mid > len`.  For a non-panicking alternative see
  /// [`split_at_mut_checked`](WriteBuf::split_at_mut_checked).
  fn split_at_mut(&mut self, mid: usize) -> (&mut [u8], &mut [u8]) {
    self.as_mut_slice().split_at_mut(mid)
  }

  /// Divides one mutable slice into two at an index, returning `None` if the
  /// slice is too short.
  ///
  /// If `mid ≤ len` returns a pair of slices where the first will contain all
  /// indices from `[0, mid)` (excluding the index `mid` itself) and the
  /// second will contain all indices from `[mid, len)` (excluding the index
  /// `len` itself).
  ///
  /// Otherwise, if `mid > len`, returns `None`.
  fn split_at_mut_checked(&mut self, mid: usize) -> Option<(&mut [u8], &mut [u8])> {
    self.as_mut_slice().split_at_mut_checked(mid)
  }

  /// Writes a slice of bytes to the beginning of the buffer.
  ///
  /// # Panics
  /// Panics if the buffer is too small to hold the slice.
  fn write_slice(&mut self, slice: &[u8]) -> usize {
    let len = slice.len();
    self.as_mut_slice()[..len].copy_from_slice(slice);
    len
  }

  /// Tries to write a slice of bytes to the beginning of the buffer.
  ///
  /// This is the non-panicking version of [`write_slice`](WriteBuf::write_slice).
  /// Returns `None` if the buffer is too small to hold the slice.
  /// Otherwise, it returns the number of bytes written.
  fn write_slice_checked(&mut self, slice: &[u8]) -> Option<usize> {
    let len = slice.len();
    if len <= self.len() {
      self.as_mut_slice()[..len].copy_from_slice(slice);
      Some(len)
    } else {
      None
    }
  }

  write_varint!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

  write_fixed!(u16, u32, u64, u128, i16, i32, i64, i128);

  /// Writes a `u8` value to the beginning of the buffer.
  ///
  /// Returns the number of bytes written (always `1` for this type).
  ///
  /// # Panics
  /// Panics if the buffer is too small to hold the byte.
  #[inline]
  fn write_u8(&mut self, value: u8) -> usize {
    let len = core::mem::size_of::<u8>();
    self.as_mut_slice()[0] = value;
    len
  }

  /// Tries to write a `u8` value to the beginning of the buffer.
  ///
  /// This is the non-panicking version of [`write_u8`](WriteBuf::write_u8)
  /// .
  /// Returns `None` if the buffer is too small to hold the byte.
  /// Otherwise, it returns the number of bytes written (always `1` for this type).
  #[inline]
  fn write_u8_checked(&mut self, value: u8) -> Option<usize> {
    if self.is_empty() {
      None
    } else {
      self.as_mut_slice()[0] = value;
      Some(1)
    }
  }

  /// Writes a `i8` value to the beginning of the buffer.
  ///
  /// Returns the number of bytes written (always `1` for this type).
  ///
  /// # Panics
  /// Panics if the buffer is too small to hold the byte.
  #[inline]
  fn write_i8(&mut self, value: i8) -> usize {
    let len = core::mem::size_of::<i8>();
    self.as_mut_slice()[0] = value as u8;
    len
  }

  /// Tries to write a `i8` value to the beginning of the buffer.
  ///
  /// This is the non-panicking version of [`write_u8`](WriteBuf::write_u8)
  /// .
  /// Returns `None` if the buffer is too small to hold the byte.
  /// Otherwise, it returns the number of bytes written (always `1` for this type).
  #[inline]
  fn write_i8_checked(&mut self, value: i8) -> Option<usize> {
    if self.is_empty() {
      None
    } else {
      self.as_mut_slice()[0] = value as u8;
      Some(1)
    }
  }
}

impl<T: WriteBuf> WriteBuf for &mut T {
  fn len(&self) -> usize {
    <T as WriteBuf>::len(self)
  }

  fn as_mut_slice(&mut self) -> &mut [u8] {
    <T as WriteBuf>::as_mut_slice(self)
  }

  fn split_at_mut(&mut self, mid: usize) -> (&mut [u8], &mut [u8]) {
    <T as WriteBuf>::split_at_mut(self, mid)
  }
}

impl<const N: usize> WriteBuf for [u8; N] {
  fn is_empty(&self) -> bool {
    <[u8]>::is_empty(self)
  }

  fn len(&self) -> usize {
    <[u8]>::len(self)
  }

  fn as_mut_slice(&mut self) -> &mut [u8] {
    self
  }
}

impl WriteBuf for [u8] {
  fn is_empty(&self) -> bool {
    <[u8]>::is_empty(self)
  }

  fn len(&self) -> usize {
    <[u8]>::len(self)
  }

  fn as_mut_slice(&mut self) -> &mut [u8] {
    self
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

  #[cfg(all(feature = "bytes_1", any(feature = "std", feature = "alloc")))]
  #[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "bytes_1", any(feature = "std", feature = "alloc"))))
  )]
  /// Converts the read buffer to a `BytesMut` instance.
  fn to_bytes_mut(&self) -> crate::bytes::BytesMut {
    crate::bytes::BytesMut::from(self.to_bytes())
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
  use bytes_1::{Bytes, BytesMut};

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

  impl WriteBuf for BytesMut {
    #[inline]
    fn is_empty(&self) -> bool {
      self.is_empty()
    }

    #[inline]
    fn len(&self) -> usize {
      self.len()
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [u8] {
      self.as_mut()
    }
  }
};

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::vec::Vec;

  impl WriteBuf for Vec<u8> {
    #[inline]
    fn is_empty(&self) -> bool {
      self.is_empty()
    }

    #[inline]
    fn len(&self) -> usize {
      self.len()
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [u8] {
      self.as_mut_slice()
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
