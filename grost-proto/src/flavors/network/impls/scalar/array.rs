use crate::flavors::network::DecodeError;

mod u8;

#[cfg(not(any(feature = "std", feature = "alloc")))]
pub fn larger_than_array_capacity<const N: usize>()
-> DecodeError {
  DecodeError::custom(
    "cannot decode array with length greater than the capacity",
  )
}

#[cfg(any(feature = "std", feature = "alloc"))]
pub fn larger_than_array_capacity<const N: usize>()
-> DecodeError {
  DecodeError::custom(std::format!(
    "cannot decode array with length greater than the capacity {N}"
  ))
}