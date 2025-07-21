pub use buffer::{DefaultPartialSetBuffer, PartialSetBuffer};
pub use packed_set_decoder::PackedSetDecoder;

use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  flavors::{
    Groto, WireFormat,
    groto::Error,
  },
};

#[cfg(any(feature = "std", feature = "alloc"))]
mod btreeset;
#[cfg(any(feature = "std", feature = "hashbrown_0_15"))]
mod hashset;
#[cfg(feature = "indexmap_2")]
mod indexset;

mod buffer;
mod packed_set_decoder;

fn try_from<'a, K, KO, KW, RB, B, I, T>(
  set: &mut T,
  iter: I,
  check: impl FnOnce(&T) -> Result<(), Error>,
  mut insert: impl FnMut(&mut T, K) -> Result<(), Error>,
  mut from_key: impl FnMut(KO) -> Result<K, Error>,
) -> Result<(), Error>
where
  KW: WireFormat<Groto> + 'a,
  K: 'a,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
  I: Iterator<Item = Result<(usize, KO), Error>>,
{
  for res in iter {
    let (_, k) = res?;
    insert(set, from_key(k)?)?;
  }

  check(set)
}
