use core::ops::RangeBounds;

pub trait Buffer: AsRef<[u8]> {
  fn slice(&self, range: impl RangeBounds<usize>) -> Self;
}

#[cfg(feature = "bytes_1")]
impl Buffer for bytes_1::Bytes {
  fn slice(&self, range: impl RangeBounds<usize>) -> Self {
    self.slice(range)
  }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl Buffer for Vec<u8> {
  fn slice(&self, range: impl RangeBounds<usize>) -> Self {
    use core::ops::Bound;

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
}

#[cfg(feature = "heapless_0_8")]
impl<const N: usize> Buffer for heapless_0_8::Vec<u8, N> {
  fn slice(&self, range: impl RangeBounds<usize>) -> Self {
    use core::ops::Bound;

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
}
