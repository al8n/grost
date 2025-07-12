use crate::{
  convert::{Flattened, Inner},
  marker::{Marker, NullableMarker},
  state::State,
};

use super::{DefaultWireFormat, Flavor, WireFormat};

/// The default wire format for a nullable type on flavor `F`.
pub trait DefaultNullableWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format<V>: WireFormat<F> + 'static
  where
    V: WireFormat<F> + 'static;
}

impl<T, VM, F> DefaultWireFormat<F> for NullableMarker<T, VM>
where
  F: ?Sized + Flavor,
  VM: DefaultWireFormat<F> + Marker,
  T: State<Flattened<Inner>, Output = VM::Marked> + ?Sized + DefaultNullableWireFormat<F>,
{
  type Format = T::Format<VM::Format>;
}
