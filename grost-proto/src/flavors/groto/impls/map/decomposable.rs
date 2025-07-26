pub use selector::DecomposableMapSelector;
pub use entry::{DecomposableMapEntry, PartialDecomposableMapEntry};

#[cfg(any(feature = "std", feature = "alloc"))]
mod btreemap;
#[cfg(any(feature = "std", feature = "hashbrown_0_15"))]
mod hashmap;
#[cfg(feature = "indexmap_2")]
mod indexmap;

mod selector;
mod entry;
