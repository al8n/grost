use crate::marker::{GenericMarker, Marker};

use super::{DefaultWireFormat, Flavor};

impl<M, F> DefaultWireFormat<F> for GenericMarker<M>
where
  M: DefaultWireFormat<F> + Marker + ?Sized,
  F: ?Sized + Flavor,
{
  type Format = M::Format;
}
