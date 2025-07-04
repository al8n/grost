use crate::{
  convert::{Flattened, Inner, State},
  marker::{ListMarker, Marker},
};

use super::{DefaultWireFormat, Flavor, WireFormat};

/// The default wire format for a list type on flavor `F`.
pub trait DefaultListWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format<V>: WireFormat<F>
  where
    V: WireFormat<F>;
}

impl<T, VM, F> DefaultWireFormat<F> for ListMarker<T, VM>
where
  F: ?Sized + Flavor,
  VM: DefaultWireFormat<F> + Marker,
  T: State<Flattened<Inner>, Output = VM::Marked> + ?Sized + DefaultListWireFormat<F>,
{
  type Format = T::Format<VM::Format>;
}
