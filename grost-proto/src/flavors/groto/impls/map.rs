use crate::{
  buffer::Buffer,
  convert::Partial,
  flavors::{Groto, groto::Context},
  selection::{Selectable, Selector},
  state::State,
};

pub use entry::PartialMapEntry;
pub use packed_map_decoder::PackedMapDecoder;
pub use repeated_map_decoder::RepeatedMapDecoder;
pub use selector::MapSelector;

/// The default partial set buffer type.
pub type DefaultPartialMapBuffer<K, V> = crate::buffer::DefaultBuffer<PartialMapEntry<K, V>>;

use entry::MapEntry;

#[cfg(any(feature = "std", feature = "alloc"))]
mod btreemap;
#[cfg(any(feature = "std", feature = "hashbrown_0_15"))]
mod hashmap;
#[cfg(feature = "indexmap_2")]
mod indexmap;

mod entry;
mod packed_map_decoder;
mod repeated_map_decoder;
mod selector;
