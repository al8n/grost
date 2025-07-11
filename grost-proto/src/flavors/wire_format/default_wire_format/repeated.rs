use crate::{
  buffer::{Buffer, DefaultBuffer, ReadBuf},
  convert::{Flattened, Inner, State},
  flavors::MergedWireFormat,
  marker::{Marker, RepeatedEntryMarker, RepeatedMarker},
};

use super::{DefaultWireFormat, Flavor, WireFormat};

/// The default wire format for a repeated encoded list-like type on flavor `F`.
pub trait DefaultRepeatedWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  ///
  /// - `V` is the wire format of the value type.
  /// - `O` is the default output type of the repeated value.
  /// - `TAG` is the tag of the repeated value.
  type Format<V, const TAG: u32>: WireFormat<F> + 'static
  where
    V: WireFormat<F> + 'static;
}

impl<T, VM, F, const TAG: u32> DefaultWireFormat<F> for RepeatedMarker<T, VM, TAG>
where
  F: ?Sized + Flavor,
  VM: DefaultWireFormat<F> + Marker,
  T: State<Flattened<Inner>, Output = VM::Marked> + ?Sized + DefaultRepeatedWireFormat<F>,
{
  type Format = T::Format<VM::Format, TAG>;
}

/// The default wire format for a repeated encoded map type on flavor `F`.
pub trait DefaultRepeatedEntryWireFormat<F: Flavor + ?Sized> {
  /// The default wire format of the type for this flavor.
  ///
  /// - `K` is the wire format of the key type.
  /// - `V` is the wire format of the value type.
  /// - `O` is the default output type of the repeated entry.
  /// - `TAG` is the tag of the repeated entry.
  type Format<K, V, const TAG: u32>: WireFormat<F> + 'static
  where
    K: WireFormat<F> + 'static,
    V: WireFormat<F> + 'static,
    MergedWireFormat<K, V>: WireFormat<F> + 'static;
}

impl<T, KM, VM, F, const TAG: u32> DefaultWireFormat<F> for RepeatedEntryMarker<T, KM, VM, TAG>
where
  KM: DefaultWireFormat<F> + Marker + ?Sized,
  KM::Marked: Sized,
  VM: DefaultWireFormat<F> + Marker + ?Sized,
  VM::Marked: Sized,
  MergedWireFormat<KM::Format, VM::Format>: WireFormat<F> + 'static,
  T: ?Sized
    + State<Flattened<Inner>, Output = (KM::Marked, VM::Marked)>
    + DefaultRepeatedEntryWireFormat<F>,
  F: ?Sized + Flavor,
{
  type Format = T::Format<KM::Format, VM::Format, TAG>;
}
