pub use buffer::{DefaultPartialSetBuffer, PartialSetBuffer};
pub use packed_set_decoder::PackedSetDecoder;

#[cfg(any(feature = "std", feature = "alloc"))]
mod btreeset;
#[cfg(any(feature = "std", feature = "hashbrown_0_15"))]
mod hashset;
#[cfg(feature = "indexmap_2")]
mod indexset;

mod buffer;
mod decomposable;
mod packed_set_decoder;
