use arrayvec_0_7::ArrayString;

array_str!(impl<const N: usize> ArrayString<N> {
  fn new = Self::new;
  fn from_str = |src: &str| from_str!(ArrayString::new.try_push_str(src));
  fn decode = |src: &[u8]| decode_str!(ArrayString::new.try_push_str(src));
  fn as_bytes = |s: &Self| s.as_bytes();
});
