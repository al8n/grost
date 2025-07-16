use crate::{
  buffer::Buffer,
  convert::Partial,
  flavors::{Groto, groto::Context},
  selection::{Selectable, Selector},
  state::State,
};

pub use packed_map_decoder::PackedMapDecoder;
pub use repeated_map_decoder::RepeatedMapDecoder;

#[cfg(any(feature = "std", feature = "hashbrown_0_15"))]
mod hashmap;

mod packed_map_decoder;
mod repeated_map_decoder;

pub enum MapEntry<K, V> {
  Key(K),
  Value(V),
  Pair {
    key: K,
    value: V,
  }
}
