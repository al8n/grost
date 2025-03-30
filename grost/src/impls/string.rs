#[cfg(any(feature = "std", feature = "alloc"))]
#[allow(clippy::module_inception)]
mod string;

#[cfg(any(feature = "std", feature = "alloc"))]
mod smolstr;

mod str;

#[cfg(feature = "heapless")]
mod heapless;
