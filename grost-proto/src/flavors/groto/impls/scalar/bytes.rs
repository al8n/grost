use crate::flavors::Groto;

#[allow(unused)]
#[cfg(not(any(feature = "std", feature = "alloc")))]
pub fn larger_than_array_capacity<const N: usize>() -> crate::error::DecodeError<Groto> {
  crate::error::DecodeError::other("cannot decode array with length greater than the capacity")
}

#[allow(unused)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn larger_than_array_capacity<const N: usize>() -> crate::error::DecodeError<Groto> {
  crate::error::DecodeError::other(std::format!(
    "cannot decode array with length greater than the capacity {N}"
  ))
}

mod array;
#[cfg(any(feature = "std", feature = "alloc"))]
#[allow(clippy::module_inception)]
mod bytes;
mod heapless;
