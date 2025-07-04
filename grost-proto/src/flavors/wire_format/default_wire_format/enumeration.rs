use crate::marker::EnumMarker;

use super::{Flavor, WireFormat, DefaultWireFormat};


/// The default wire format for an enum type on flavor `F`.
pub trait DefaultEnumWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format: WireFormat<F>;
}

impl<T, F> DefaultWireFormat<F> for EnumMarker<T>
where
  T: DefaultEnumWireFormat<F> + ?Sized,
  F: ?Sized + Flavor,
{
  type Format = <T as DefaultEnumWireFormat<F>>::Format;
}
