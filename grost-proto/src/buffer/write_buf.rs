use super::EncodeVarintError;

/// A trait for implementing custom buffers that can be resized.
pub trait Resizable {
  /// Resizes the buffer to the specified length, filling new bytes with the given value.
  fn resize(&mut self, new_len: usize, fill_value: u8);
}

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

  write_varint!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

  write_fixed!(u16, u32, u64, u128, i16, i32, i64, i128, f32, f64);

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
