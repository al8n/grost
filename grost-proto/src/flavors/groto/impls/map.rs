use core::{marker::PhantomData, mem};

use crate::{
  buffer::Buffer,
  convert::{Partial, State},
  flavors::{Groto, groto::Context},
  selection::{Selectable, Selector},
};

pub use packed_entries_decoder::PackedEntriesDecoder;

#[cfg(any(feature = "std", feature = "hashbrown_0_15"))]
mod hashmap;

mod packed_entries_decoder;
