mod arrayvec;

#[cfg(any(feature = "std", feature = "alloc"))]
mod tinyvec;

use crate::{
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, WireFormat,
    groto::Error,
  },
};

#[cfg(not(any(feature = "std", feature = "alloc")))]
pub fn larger_than_array_capacity<A>() -> Error
where
  A: tinyvec_1::Array,
{
  Error::custom("cannot allocate array with length greater than the capacity")
}

#[cfg(any(feature = "std", feature = "alloc"))]
pub fn larger_than_array_capacity<A>() -> Error
where
  A: tinyvec_1::Array,
{
  Error::custom(std::format!(
    "cannot allocate array with length greater than the capacity {}",
    A::CAPACITY
  ))
}

trait DefaultEncode<W: WireFormat<Groto>>: Encode<W, Groto> + Default {}

impl<T, W> DefaultEncode<W> for T
where
  T: Encode<W, Groto> + Default,
  W: WireFormat<Groto>,
{
}

trait DefaultPartialEncode<W: WireFormat<Groto>>: PartialEncode<W, Groto> + Default {}

impl<T, W> DefaultPartialEncode<W> for T
where
  T: PartialEncode<W, Groto> + Default,
  W: WireFormat<Groto>,
{
}

#[test]
fn t() {}