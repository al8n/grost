use crate::marker::BytesMarker;

use super::{DefaultWireFormat, Flavor, WireFormat};

/// The default wire format for a bytes type on flavor `F`.
pub trait DefaultBytesWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format: WireFormat<F> + 'static;
}

impl<T, F> DefaultWireFormat<F> for BytesMarker<T>
where
  T: DefaultBytesWireFormat<F> + ?Sized,
  F: ?Sized + Flavor,
{
  type Format = <T as DefaultBytesWireFormat<F>>::Format;
}
