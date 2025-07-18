pub use buffer::{DefaultPartialMapBuffer, PartialMapBuffer};
pub use entry::PartialMapEntry;
pub use packed_map_decoder::*;
pub use repeated_map_decoder::*;
pub use selector::MapSelector;

use entry::MapEntry;

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
