use crate::marker::InterfaceMarker;

use super::{DefaultWireFormat, Flavor, WireFormat};

/// The default wire format for a bytes type on flavor `F`.
pub trait DefaultInterfaceWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format: WireFormat<F>;
}

impl<T, F> DefaultWireFormat<F> for InterfaceMarker<T>
where
  T: DefaultInterfaceWireFormat<F> + ?Sized,
  F: ?Sized + Flavor,
{
  type Format = <T as DefaultInterfaceWireFormat<F>>::Format;
}
