use crate::{convert::{Flattened, Inner, State}, marker::{MapMarker, Marker}};

use super::{Flavor, WireFormat, DefaultWireFormat};


/// The default wire format for a map type on flavor `F`.
pub trait DefaultMapWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format<K, V>: WireFormat<F> where K: WireFormat<F>, V: WireFormat<F>;
}

impl<T, KM, VM, F> DefaultWireFormat<F> for MapMarker<T, KM, VM>
where
  KM: DefaultWireFormat<F> + Marker + ?Sized,
  KM::Marked: Sized,
  VM: DefaultWireFormat<F> + Marker + ?Sized,
  VM::Marked: Sized,
  T: ?Sized + State<Flattened<Inner>, Output = (KM::Marked, VM::Marked)> + DefaultMapWireFormat<F>,
  F: ?Sized + Flavor,
{
  type Format = T::Format<KM::Format, VM::Format>;
}
