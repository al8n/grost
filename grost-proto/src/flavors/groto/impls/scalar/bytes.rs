#[allow(unused)]
#[cfg(not(any(feature = "std", feature = "alloc")))]
pub fn larger_than_array_capacity<const N: usize>() -> crate::flavors::groto::Error {
  crate::flavors::groto::Error::custom("cannot decode array with length greater than the capacity")
}

#[allow(unused)]
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn larger_than_array_capacity<const N: usize>() -> crate::flavors::groto::Error {
  crate::flavors::groto::Error::custom(std::format!(
    "cannot decode array with length greater than the capacity {N}"
  ))
}

mod array;
#[cfg(any(feature = "std", feature = "alloc"))]
#[allow(clippy::module_inception)]
mod bytes;
mod heapless;
