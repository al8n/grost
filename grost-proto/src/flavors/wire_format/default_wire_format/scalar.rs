use crate::marker::ScalarMarker;

use super::{Flavor, WireFormat, DefaultWireFormat};


/// The default wire format for a scalar type on flavor `F`.
pub trait DefaultScalarWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format: WireFormat<F>;
}

impl<T, F> DefaultWireFormat<F> for ScalarMarker<T>
where
  T: DefaultScalarWireFormat<F> + ?Sized,
  F: ?Sized + Flavor,
{
  type Format = <T as DefaultScalarWireFormat<F>>::Format;
}
