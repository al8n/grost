pub use buffer::{DecomposablePartialMapBuffer, DefaultDecomposablePartialMapBuffer};
pub use entry::{DecomposableMapEntry, PartialDecomposableMapEntry};
pub use packed_map_decoder::DecomposablePackedMapDecoder;
pub use repeated_map_decoder::{
  DecomposableRepeatedMapDecoder, DecomposableRepeatedMapDecoderBuffer,
};
pub use selector::DecomposableMapSelector;

#[cfg(any(feature = "std", feature = "alloc"))]
mod btreemap;
#[cfg(any(feature = "std", feature = "hashbrown_0_15"))]
mod hashmap;
#[cfg(feature = "indexmap_2")]
mod indexmap;

mod buffer;
mod entry;
mod packed_map_decoder;
mod repeated_map_decoder;
mod selector;
