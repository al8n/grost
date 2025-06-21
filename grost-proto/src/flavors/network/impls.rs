// mod map_selector;
pub use packed_decoder::PackedDecoder;

use crate::{
  buffer::{Buffer, ReadBuf},
  decode::{Decode, PartialTransform, Transform},
  flavors::{Flavor, Network, WireFormat},
  selection::Selectable,
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
    B: ReadBuf + 'de,
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

impl<W, I, T> PartialTransform<Network, W, I> for Option<T>
where
  W: WireFormat<Network>,
  T: PartialTransform<Network, W, I> + Sized,
  I: Selectable<Network>,
  T: Selectable<Network, Selector = <I as Selectable<Network>>::Selector>,
{
  fn partial_transform(
    input: I,
    selector: &<I as Selectable<Network>>::Selector,
  ) -> Result<Option<Self>, <Network as Flavor>::Error>
  where
    Self: Sized,
  {
    T::partial_transform(input, selector).map(|opt| opt.map(Some))
  }
}
