use crate::marker::UnionMarker;

use super::{DefaultWireFormat, Flavor, WireFormat};

/// The default wire format for a union type on flavor `F`.
pub trait DefaultUnionWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format: WireFormat<F>;
}

impl<T, F> DefaultWireFormat<F> for UnionMarker<T>
where
  T: DefaultUnionWireFormat<F> + ?Sized,
  F: ?Sized + Flavor,
{
  type Format = <T as DefaultUnionWireFormat<F>>::Format;
}
