use super::Buffer;
use core::mem::MaybeUninit;

/// The default buffer for `no-alloc` and `no-std` environments.
///
/// This buffer provides a stack-allocated, fixed-capacity container that can store
/// up to `CAP` items of type `T` (default: 16). It uses `MaybeUninit<T>` internally
/// to avoid initialization costs and support types that don't implement `Default`.
///
/// If any of `arrayvec` or `heapless` feature flags are enabled, this buffer
/// will not be used as the default implementation.
///
/// # Examples
///
/// ```rust
/// use grost::buffer::StackBuffer;
///
/// let mut buffer = StackBuffer::<i32>::new();
/// buffer.push(1);
/// buffer.push(2);
/// assert_eq!(buffer.len(), 2);
/// assert_eq!(buffer.as_slice()[0], 1);
/// ```
#[derive(Debug)]
pub struct StackBuffer<T, const CAP: usize = 16> {
  /// Internal storage using `MaybeUninit` to avoid unnecessary initialization.
  /// Only the first `len` elements are guaranteed to be initialized.
  items: [MaybeUninit<T>; CAP],
  /// Number of initialized elements. Invariant: `len <= CAP`
  len: usize,
}

impl<T, const CAP: usize> core::borrow::Borrow<[T]> for StackBuffer<T, CAP> {
  fn borrow(&self) -> &[T] {
    self.as_slice()
  }
}

impl<T: Clone, const CAP: usize> Clone for StackBuffer<T, CAP> {
  fn clone(&self) -> Self {
    let mut new_buffer = Self::new();
    for i in 0..self.len {
      // SAFETY: The loop bound ensures `i < self.len`, and our type invariant
      // guarantees that `self.items[0..self.len]` contains initialized values.
      // Therefore, `self.items[i]` is safe to read as an initialized `T`.
      new_buffer.items[i] = unsafe { MaybeUninit::new(self.items[i].assume_init_ref().clone()) };
    }
    new_buffer.len = self.len;
    new_buffer
  }
}

impl<T, const CAP: usize> core::ops::Deref for StackBuffer<T, CAP> {
  type Target = [T];

  fn deref(&self) -> &Self::Target {
    self.as_slice()
  }
}

impl<T, const CAP: usize> core::ops::DerefMut for StackBuffer<T, CAP> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_slice_mut()
  }
}

impl<T, const CAP: usize> AsRef<[T]> for StackBuffer<T, CAP> {
  fn as_ref(&self) -> &[T] {
    self.as_slice()
  }
}

impl<T, const CAP: usize> AsMut<[T]> for StackBuffer<T, CAP> {
  fn as_mut(&mut self) -> &mut [T] {
    self.as_slice_mut()
  }
}

impl<T, const CAP: usize> core::ops::Index<usize> for StackBuffer<T, CAP> {
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
    &self.as_slice()[index]
  }
}

impl<T, const CAP: usize> core::ops::IndexMut<usize> for StackBuffer<T, CAP> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.as_slice_mut()[index]
  }
}

impl<T, const CAP: usize> Buffer for StackBuffer<T, CAP> {
  type Item = T;

  fn new() -> Self {
    Self {
      items: core::array::from_fn(|_| MaybeUninit::uninit()),
      len: 0,
    }
  }

  fn with_capacity(capacity: usize) -> Option<Self>
  where
    Self: Sized,
  {
    if capacity > CAP {
      None
    } else {
      Some(Self::new())
    }
  }

  fn push(&mut self, value: T) -> Option<T> {
    if self.len < CAP {
      self.items[self.len] = MaybeUninit::new(value);
      self.len += 1;
      None
    } else {
      Some(value)
    }
  }

  fn capacity(&self) -> usize {
    CAP
  }

  fn len(&self) -> usize {
    self.len
  }

  fn as_slice(&self) -> &[T] {
    Self::as_slice(self)
  }

  fn as_mut_slice(&mut self) -> &mut [T] {
    Self::as_slice_mut(self)
  }
}

// Implement Drop to properly handle cleanup of initialized elements
impl<T, const CAP: usize> Drop for StackBuffer<T, CAP> {
  fn drop(&mut self) {
    // SAFETY: Our invariant guarantees that elements [0..len) are initialized.
    // We iterate through each initialized element and drop it properly.
    for i in 0..self.len {
      unsafe {
        self.items[i].assume_init_drop();
      }
    }
  }
}

// Additional convenience methods
impl<T, const CAP: usize> StackBuffer<T, CAP> {
  /// Returns `true` if the buffer is empty.
  #[inline]
  pub const fn is_empty(&self) -> bool {
    self.len == 0
  }

  /// Returns the number of initialized elements in the buffer.
  #[inline]
  pub const fn len(&self) -> usize {
    self.len
  }

  /// Returns `true` if the buffer is at full capacity.
  #[inline]
  pub const fn is_full(&self) -> bool {
    self.len == CAP
  }

  /// Returns the maximum capacity of this buffer.
  #[inline]
  pub const fn capacity(&self) -> usize {
    CAP
  }

  /// Returns a slice of the stored data.
  #[inline]
  pub const fn as_slice(&self) -> &[T] {
    if self.len == 0 {
      return &[];
    }

    // SAFETY:
    // 1. `self.items.as_ptr()` is always valid since `items` is a valid array
    // 2. The cast from `*const MaybeUninit<T>` to `*const T` is valid because
    //    `MaybeUninit<T>` has the same layout as `T`
    // 3. We only create a slice of length `self.len`, and our invariant guarantees
    //    that the first `self.len` elements are initialized
    // 4. The resulting slice lifetime is tied to `&self`, ensuring memory safety
    unsafe { core::slice::from_raw_parts(self.items.as_ptr() as *const T, self.len) }
  }

  /// Returns a mutable slice of the stored data.
  #[inline]
  pub const fn as_slice_mut(&mut self) -> &mut [T] {
    if self.len == 0 {
      return &mut [];
    }

    // SAFETY: Similar to `as_slice`, but for mutable access:
    // 1. `self.items.as_mut_ptr()` is always valid since `items` is a valid array
    // 2. The cast from `*mut MaybeUninit<T>` to `*mut T` is valid due to layout compatibility
    // 3. We only create a slice of length `self.len` with initialized elements
    // 4. The resulting slice lifetime is tied to `&mut self`, ensuring exclusive access
    unsafe { core::slice::from_raw_parts_mut(self.items.as_mut_ptr() as *mut T, self.len) }
  }

  /// Removes and returns the last element, or `None` if empty.
  pub fn pop(&mut self) -> Option<T> {
    if self.len == 0 {
      None
    } else {
      self.len -= 1;
      // SAFETY: We just decremented len, so len is now the index of the last
      // initialized element. Our invariant guarantees this element is initialized.
      Some(unsafe { self.items[self.len].assume_init_read() })
    }
  }

  /// Clears the buffer, dropping all elements.
  pub fn clear(&mut self) {
    while self.len > 0 {
      self.len -= 1;
      // SAFETY: len is now the index of an initialized element
      unsafe {
        self.items[self.len].assume_init_drop();
      }
    }
  }

  /// Returns an iterator over the elements.
  pub fn iter(&self) -> core::slice::Iter<'_, T> {
    self.as_slice().iter()
  }

  /// Returns a mutable iterator over the elements.
  pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
    self.as_slice_mut().iter_mut()
  }

  /// Tries to insert an element at the specified index, shifting elements to the right.
  ///
  /// Returns `Err(value)` if the buffer is full or the index is out of bounds.
  pub fn try_insert(&mut self, index: usize, value: T) -> Result<(), T> {
    if index > self.len || self.len >= CAP {
      return Err(value);
    }

    // Shift elements to the right
    for i in (index..self.len).rev() {
      // SAFETY: Both indices are within bounds and elements are initialized
      unsafe {
        let temp = self.items[i].assume_init_read();
        self.items[i + 1] = MaybeUninit::new(temp);
      }
    }

    self.items[index] = MaybeUninit::new(value);
    self.len += 1;
    Ok(())
  }

  /// Removes and returns the element at the specified index, shifting elements to the left.
  ///
  /// # Panics
  ///
  /// Panics if the index is out of bounds.
  pub fn remove(&mut self, index: usize) -> T {
    assert!(index < self.len, "index out of bounds");

    // SAFETY: Index is bounds-checked above
    let removed = unsafe { self.items[index].assume_init_read() };

    // Shift elements to the left
    for i in index..self.len - 1 {
      // SAFETY: Both indices are within bounds and elements are initialized
      unsafe {
        let temp = self.items[i + 1].assume_init_read();
        self.items[i] = MaybeUninit::new(temp);
      }
    }

    self.len -= 1;
    removed
  }
}
