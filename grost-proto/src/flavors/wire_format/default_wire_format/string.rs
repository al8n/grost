use crate::marker::StringMarker;

use super::{DefaultWireFormat, Flavor, WireFormat};

/// The default wire format for a string type on flavor `F`.
pub trait DefaultStringWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format: WireFormat<F>;
}

impl<T, F> DefaultWireFormat<F> for StringMarker<T>
where
  T: DefaultStringWireFormat<F> + ?Sized,
  F: ?Sized + Flavor,
{
  type Format = <T as DefaultStringWireFormat<F>>::Format;
}
