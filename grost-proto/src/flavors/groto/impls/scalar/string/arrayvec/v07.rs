use arrayvec_0_7::ArrayString;

array_str!(impl<const N: usize> ArrayString<N> {
  fn new = Self::new;
  fn from_str = |src: &str| {
    ArrayString::<N>::try_from(src)
      .map_err(|_| super::super::larger_than_str_capacity::<N>())
  };
  fn as_bytes = Self::as_bytes;
});
