#[cfg(any(feature = "std", feature = "alloc"))]
mod bytes;
#[cfg(any(feature = "std", feature = "alloc"))]
mod vec;

#[cfg(any(feature = "std", feature = "alloc"))]
mod tinyvec;
#[cfg(any(feature = "std", feature = "alloc"))]
pub use self::tinyvec::TinyBytes as Bytes;

#[cfg(not(any(feature = "std", feature = "alloc")))]
mod array_vec;
#[cfg(not(any(feature = "std", feature = "alloc")))]
pub use array_vec::ArrayBytes as Bytes;

#[cfg(feature = "heapless")]
mod heapless;

mod array;
mod slice;
