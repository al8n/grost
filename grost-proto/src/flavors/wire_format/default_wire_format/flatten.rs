use crate::{
  convert::{Extracted, Inner},
  marker::{FlattenMarker, Marker},
  state::State,
};

use super::{DefaultWireFormat, Flavor, WireFormat};

/// The default wire format for a nullable type on flavor `F`.
pub trait DefaultFlattenWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format<V>: WireFormat<F> + 'static
  where
    V: WireFormat<F> + 'static;
}

impl<T, VM, F> DefaultWireFormat<F> for FlattenMarker<T, VM>
where
  F: ?Sized + Flavor,
  VM: DefaultWireFormat<F> + Marker,
  T: State<Extracted<Inner>, Output = VM::Marked> + ?Sized + DefaultFlattenWireFormat<F>,
{
  type Format = T::Format<VM::Format>;
}
