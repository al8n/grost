use heapless_0_8::String;

array_str!(impl<const N: usize> String<N> {
  fn new = Self::new;
  fn from_str = |src: &str| from_str!(String::new.push_str(src));
  fn decode = |src: &[u8]| decode_str!(String::new.push_str(src));
  fn as_bytes = |s: &Self| s.as_bytes();
});
