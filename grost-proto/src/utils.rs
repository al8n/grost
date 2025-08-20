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

/// A zero-cost wrapper that enables full decomposition of wrapped types in partial selection contexts.
///
/// `Decomposable<T>` is a transparent wrapper that changes how a type behaves under partial selection.
/// While regular types may have atomic semantics (like `HashSet<T>` maintaining set membership integrity),
/// wrapping them with `Decomposable` allows all parts of the type to be decomposed and partially selected.
///
/// # Type System Behavior
///
/// In a GraphQL-like type system with `State<Partial<_>>` modes:
///
/// ## Regular Collections (Atomic Semantics)
/// - `HashSet<K>` → `HashSet<K>` (set membership is atomic)
/// - `HashMap<K, V>` → `HashMap<K, <V as State<Partial<_>>>::Output>` (keys atomic, values decomposable)
///
/// ## Decomposable Collections (Full Decomposition)
/// - `Decomposable<HashSet<K>>` → `*SetBuffer<<K as State<Partial<_>>>::Output>>` (set becomes decomposable list)
/// - `Decomposable<HashMap<K, V>>` → `*MapBuffer<<K as State<Partial<_>>>::Output>, <V as State<Partial<_>>>::Output>>` (both keys and values decomposable)
///
/// ## When to Use Decomposable
///
/// **Use `Decomposable<T>` when:**
/// - You need GraphQL-like field selection within collections
/// - You're building APIs where bandwidth optimization is important
/// - You want to expose partial views of data structures
/// - The structural integrity of the collection is less important than field selection
///
/// **Don't use `Decomposable<T>` when:**
/// - You need to maintain collection semantics (set membership, map lookups)
/// - Performance of collection operations is critical
/// - The data structure is used for internal logic rather than external APIs
///
/// # Performance Characteristics
///
/// `Decomposable<T>` is a zero-cost abstraction at runtime:
/// - `#[repr(transparent)]` ensures identical memory layout to `T`
/// - All operations delegate directly to the wrapped type
/// - No performance overhead compared to using `T` directly
/// - Type-level behavior changes only affect compile-time transformations
///
/// # Design Philosophy
///
/// `Decomposable<T>` embodies the principle of **explicit intent** in type design:
/// - Regular collections maintain their semantic contracts by default
/// - `Decomposable` wrapper explicitly opts into decomposition behavior
/// - Users can choose the exact semantics they need through type selection
/// - The type system makes trade-offs visible and intentional
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Decomposable<T: ?Sized>(pub T);

impl<T: ?Sized> core::borrow::Borrow<T> for Decomposable<T> {
  #[inline]
  fn borrow(&self) -> &T {
    &self.0
  }
}

impl<T: ?Sized> core::borrow::BorrowMut<T> for Decomposable<T> {
  #[inline]
  fn borrow_mut(&mut self) -> &mut T {
    &mut self.0
  }
}

impl<T: ?Sized> AsRef<T> for Decomposable<T> {
  #[inline]
  fn as_ref(&self) -> &T {
    &self.0
  }
}

impl<T: ?Sized> AsMut<T> for Decomposable<T> {
  #[inline]
  fn as_mut(&mut self) -> &mut T {
    &mut self.0
  }
}

impl<T: ?Sized> core::ops::Deref for Decomposable<T> {
  type Target = T;

  #[inline]
  fn deref(&self) -> &T {
    &self.0
  }
}

impl<T: ?Sized> core::ops::DerefMut for Decomposable<T> {
  #[inline]
  fn deref_mut(&mut self) -> &mut T {
    &mut self.0
  }
}

impl<T> From<T> for Decomposable<T> {
  #[inline]
  fn from(value: T) -> Self {
    Self(value)
  }
}

impl<T> Decomposable<T> {
  /// Creates a new `Decomposable` wrapper around the given value.
  ///
  /// This is a zero-cost operation that simply wraps the value to change its
  /// behavior in partial selection contexts.
  #[inline]
  pub const fn new(value: T) -> Self {
    Self(value)
  }

  /// Unwraps the `Decomposable` wrapper, returning the inner value.
  #[inline]
  pub fn into_inner(self) -> T {
    self.0
  }
}

#[test]
fn t() {}
