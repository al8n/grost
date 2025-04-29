use arrayvec_0_7::ArrayVec;

array_bytes!(impl<const N: usize> ArrayVec<u8, N> {
  fn new = Self::new;
  fn from_bytes = |src: &[u8]| {
    ArrayVec::<u8, N>::try_from(src)
      .map_err(|_| super::super::larger_than_array_capacity::<N>())
  };
  fn as_bytes = Self::as_bytes;
});
