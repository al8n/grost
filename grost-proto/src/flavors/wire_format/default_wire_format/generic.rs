use crate::marker::{GenericMarker, Marker};

use super::{DefaultWireFormat, Flavor};

impl<T, M, F> DefaultWireFormat<F> for GenericMarker<T, M>
where
  T: ?Sized,
  M: DefaultWireFormat<F> + Marker<Marked = T> + ?Sized,
  F: ?Sized + Flavor,
{
  type Format = M::Format;
}
