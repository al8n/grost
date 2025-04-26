use tinyvec_1::Array;

mod arrayvec;

#[cfg(any(feature = "std", feature = "alloc"))]
mod tinyvec;

#[cfg(not(any(feature = "std", feature = "alloc")))]
fn larger_than_array_capacity<A: Array>() -> crate::flavors::network::DecodeError {
  crate::DecodeError::custom("cannot decode array with length greater than the capacity")
}

#[cfg(any(feature = "std", feature = "alloc"))]
fn larger_than_array_capacity<A: Array>() -> crate::flavors::network::DecodeError {
  crate::flavors::network::DecodeError::custom(std::format!(
    "cannot decode array with length greater than the capacity {}",
    A::CAPACITY,
  ))
}
