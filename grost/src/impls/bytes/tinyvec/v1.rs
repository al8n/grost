use tinyvec_1::Array;

mod arrayvec;

#[cfg(any(feature = "std", feature = "alloc"))]
mod tinyvec;

#[cfg(not(any(feature = "std", feature = "alloc")))]
fn larger_than_array_capacity<A: Array>() -> crate::DecodeError {
  crate::DecodeError::custom("cannot decode array with length greater than the capacity")
}

#[cfg(any(feature = "std", feature = "alloc"))]
fn larger_than_array_capacity<A: Array>() -> crate::DecodeError {
  crate::DecodeError::custom(std::format!(
    "cannot decode array with length greater than the capacity {}",
    A::CAPACITY,
  ))
}
