pub use packed_set_decoder::PackedSetDecoder;

/// The default partial set buffer type.
pub type DefaultPartialSetBuffer<K> = crate::buffer::DefaultBuffer<K>;

#[cfg(any(feature = "std", feature = "hashbrown_0_15"))]
mod hashset;

mod packed_set_decoder;
