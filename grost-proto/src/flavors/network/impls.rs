// mod map_selector;
pub use packed_decoder::PackedDecoder;

use crate::{
  buffer::{Buffer, ReadBuf},
  decode::Decode,
  flavors::{Flavor, Network, WireFormat},
};

mod list;
mod packed_decoder;
mod repeated;
mod scalar;
mod tuple;

impl<'de, W, O, UB, T> Decode<'de, Network, W, O, UB> for Option<T>
where
  T: Decode<'de, Network, W, O, UB>,
  W: WireFormat<Network>,
{
  fn decode<B>(
    context: &'de <Network as Flavor>::Context,
    src: B,
  ) -> Result<(usize, O), <Network as Flavor>::Error>
  where
    O: Sized + 'de,
    B: ReadBuf<'de>,
    UB: Buffer<<Network as Flavor>::Unknown<B>> + 'de,
  {
    T::decode(context, src)
  }
}
