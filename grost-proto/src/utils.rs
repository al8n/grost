#[cfg(not(feature = "simdutf8"))]
pub use ::core::str::from_utf8;
#[cfg(feature = "simdutf8")]
pub use simdutf8::basic::from_utf8;

/// An immutable inlined bytes
#[derive(Debug, Clone, Copy, Eq)]
pub struct InlinedBytes<const N: usize> {
  data: [u8; N],
  len: usize,
}

impl<const N: usize> core::borrow::Borrow<[u8]> for InlinedBytes<N> {
  #[inline]
  fn borrow(&self) -> &[u8] {
    self
  }
}

impl<const N: usize> Default for InlinedBytes<N> {
  #[inline]
  fn default() -> Self {
    Self {
      data: [0; N],
      len: 0,
    }
  }
}

impl<const N: usize> PartialEq for InlinedBytes<N> {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.as_bytes().eq(other.as_bytes())
  }
}

impl<const N: usize> core::hash::Hash for InlinedBytes<N> {
  #[inline]
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.as_bytes().hash(state);
  }
}

impl<const N: usize> PartialOrd for InlinedBytes<N> {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    self.as_bytes().partial_cmp(other.as_bytes())
  }
}

impl<const N: usize> Ord for InlinedBytes<N> {
  #[inline]
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.as_bytes().cmp(other.as_bytes())
  }
}

impl<const N: usize> InlinedBytes<N> {
  /// Create a new `InlinedBytes` from a byte slice.
  ///
  /// If the length of the byte slice exceeds `N`, it returns `None`.
  #[inline]
  pub const fn from_slice(data: &[u8]) -> Option<Self> {
    let len = data.len();
    if len > N {
      return None;
    }

    if len == 0 {
      return Some(Self {
        data: [0; N],
        len: 0,
      });
    }

    let mut arr = [0; N];

    // Safety: `data` is guaranteed to be less than or equal to `N` in length.
    unsafe {
      core::ptr::copy_nonoverlapping::<u8>(data.as_ptr(), arr.as_mut_ptr(), len);
    }
    Some(Self { data: arr, len })
  }

  /// Get the inner byte slice.
  #[inline]
  pub const fn as_bytes(&self) -> &[u8] {
    if self.len == 0 {
      &[]
    } else {
      self.data.split_at(self.len).0
    }
  }
}

impl<const N: usize> AsRef<[u8]> for InlinedBytes<N> {
  #[inline]
  fn as_ref(&self) -> &[u8] {
    self
  }
}

impl<const N: usize> core::ops::Deref for InlinedBytes<N> {
  type Target = [u8];

  #[inline]
  fn deref(&self) -> &[u8] {
    self.as_bytes()
  }
}
