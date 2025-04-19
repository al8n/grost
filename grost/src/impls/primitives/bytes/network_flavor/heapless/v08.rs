use heapless_0_8::Vec;

array_bytes!(impl<const N: usize> Vec<u8, N> {
  fn new = Self::new;
  fn from_bytes = |src: &[u8]| {
    Vec::from_slice(src).map_err(|_| crate::__private::larger_than_array_capacity::<N>())
  };
  fn decode = |src: &[u8]| {
    Vec::from_slice(src).map_err(|_| crate::__private::larger_than_array_capacity::<N>()).map(|val| (src.len(), val))
  };
  fn as_bytes = Self::as_slice;
});
