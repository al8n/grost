use crate::marker::ObjectMarker;

use super::{DefaultWireFormat, Flavor, WireFormat};

/// The default wire format for an object type on flavor `F`.
pub trait DefaultObjectWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format: WireFormat<F> + 'static;
}

impl<T, F> DefaultWireFormat<F> for ObjectMarker<T>
where
  T: DefaultObjectWireFormat<F> + ?Sized,
  F: ?Sized + Flavor,
{
  type Format = <T as DefaultObjectWireFormat<F>>::Format;
}
