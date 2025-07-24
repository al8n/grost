use crate::{
  convert::{Extracted, MapKey, MapValue},
  marker::{MapMarker, Marker},
  state::State,
};

use super::{DefaultWireFormat, Flavor, WireFormat};

/// The default wire format for a map type on flavor `F`.
pub trait DefaultMapWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  type Format<K, V>: WireFormat<F> + 'static
  where
    K: WireFormat<F> + 'static,
    V: WireFormat<F> + 'static;
}

impl<T, KM, VM, F> DefaultWireFormat<F> for MapMarker<T, KM, VM>
where
  KM: DefaultWireFormat<F> + Marker + ?Sized,
  KM::Marked: Sized,
  VM: DefaultWireFormat<F> + Marker + ?Sized,
  VM::Marked: Sized,
  T: ?Sized
    + State<Extracted<MapKey>, Output = KM::Marked>
    + State<Extracted<MapValue>, Output = VM::Marked>
    + DefaultMapWireFormat<F>,
  F: ?Sized + Flavor,
{
  type Format = T::Format<KM::Format, VM::Format>;
}
