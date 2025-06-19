// mod map_selector;
pub use packed_decoder::PackedDecoder;

use crate::{
  buffer::{Buffer, ReadBuf},
  decode::{Decode, Transform},
  flavors::{Flavor, Network, WireFormat},
};

mod list;
mod packed_decoder;
mod repeated;
mod scalar;
mod tuple;

impl<'de, W, O, B, UB, T> Decode<'de, Network, W, O, B, UB> for Option<T>
where
  T: Decode<'de, Network, W, O, B, UB>,
  W: WireFormat<Network>,
{
  fn decode(
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

impl<W, I, T> Transform<Network, W, I> for Option<T>
where
  W: WireFormat<Network>,
  T: Transform<Network, W, I> + Sized,
{
  fn transform(input: I) -> Result<Self, <Network as Flavor>::Error> {
    T::transform(input).map(Some)
  }
}
