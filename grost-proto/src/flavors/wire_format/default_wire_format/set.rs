use crate::{convert::{Flattened, Inner, State}, marker::{Marker, SetMarker}};

use super::{Flavor, WireFormat, DefaultWireFormat};

/// The default wire format for a set type on flavor `F`.
pub trait DefaultSetWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format<V>: WireFormat<F> where V: WireFormat<F>;
}

impl<T, VM, F> DefaultWireFormat<F> for SetMarker<T, VM>
where
  F: ?Sized + Flavor,
  VM: DefaultWireFormat<F> + Marker,
  T: State<Flattened<Inner>, Output = VM::Marked> + ?Sized + DefaultSetWireFormat<F>,
{
  type Format = T::Format<VM::Format>;
}
