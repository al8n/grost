use crate::marker::ObjectMarker;

use super::{Flavor, WireFormat, DefaultWireFormat};

/// The default wire format for an object type on flavor `F`.
pub trait DefaultObjectWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format: WireFormat<F>;
}

impl<T, F> DefaultWireFormat<F> for ObjectMarker<T>
where
  T: DefaultObjectWireFormat<F> + ?Sized,
  F: ?Sized + Flavor,
{
  type Format = <T as DefaultObjectWireFormat<F>>::Format;
}
