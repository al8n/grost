use ghost::phantom;

mod sealed {
  use super::*;

  pub trait Sealed {}

  impl<T: ?Sized> Sealed for ScalarMarker<T> {}
  impl<T: ?Sized> Sealed for StringMarker<T> {}
  impl<T: ?Sized> Sealed for BytesMarker<T> {}
  impl<T: ?Sized> Sealed for InterfaceMarker<T> {}
  impl<T: ?Sized, M: ?Sized> Sealed for NullableMarker<T, M> {}
  impl<T: ?Sized, M: ?Sized> Sealed for ListMarker<T, M> {}
  impl<T: ?Sized, M: ?Sized> Sealed for SetMarker<T, M> {}
  impl<T: ?Sized, KM: ?Sized, VM: ?Sized> Sealed for MapMarker<T, KM, VM> {}
  impl<T: ?Sized> Sealed for EnumMarker<T> {}
  impl<T: ?Sized> Sealed for UnionMarker<T> {}
  impl<T: ?Sized> Sealed for ObjectMarker<T> {}
  impl<T: ?Sized, M: ?Sized> Sealed for FlattenMarker<T, M> {}
  impl<T: ?Sized, M: ?Sized, const TAG: u32> Sealed for RepeatedMarker<T, M, TAG> {}
  impl<T: ?Sized, KM: ?Sized, VM: ?Sized, const TAG: u32> Sealed
    for RepeatedEntryMarker<T, KM, VM, TAG>
  {
  }

  impl<M: ?Sized> Sealed for GenericMarker<M> {}
}

/// A marker trait that associates to the type being marked.
pub trait Marker: sealed::Sealed {
  /// The type that is marked by this marker.
  type Marked: ?Sized;
}

impl<T: ?Sized> Marker for ScalarMarker<T> {
  type Marked = T;
}

impl<T: ?Sized> Marker for StringMarker<T> {
  type Marked = T;
}

impl<T: ?Sized> Marker for BytesMarker<T> {
  type Marked = T;
}

impl<T: ?Sized> Marker for InterfaceMarker<T> {
  type Marked = T;
}

impl<T: ?Sized, M: ?Sized> Marker for FlattenMarker<T, M> {
  type Marked = T;
}

impl<T: ?Sized, M: ?Sized> Marker for NullableMarker<T, M> {
  type Marked = T;
}

impl<T: ?Sized, M: ?Sized, const TAG: u32> Marker for RepeatedMarker<T, M, TAG> {
  type Marked = T;
}

impl<T: ?Sized, KM: ?Sized, VM: ?Sized, const TAG: u32> Marker
  for RepeatedEntryMarker<T, KM, VM, TAG>
{
  type Marked = T;
}

impl<M: ?Sized + Marker> Marker for GenericMarker<M> {
  type Marked = M::Marked;
}

impl<T: ?Sized, M: ?Sized> Marker for ListMarker<T, M> {
  type Marked = T;
}

impl<T: ?Sized, M: ?Sized> Marker for SetMarker<T, M> {
  type Marked = T;
}

impl<T: ?Sized, KM: ?Sized, VM: ?Sized> Marker for MapMarker<T, KM, VM> {
  type Marked = T;
}

impl<T: ?Sized> Marker for EnumMarker<T> {
  type Marked = T;
}

impl<T: ?Sized> Marker for UnionMarker<T> {
  type Marked = T;
}

impl<T: ?Sized> Marker for ObjectMarker<T> {
  type Marked = T;
}

/// A marker for a string type.
#[phantom]
pub struct ScalarMarker<T: ?Sized>;

/// A marker for a bytes type.
#[phantom]
pub struct BytesMarker<T: ?Sized>;

/// A marker for a string type.
#[phantom]
pub struct StringMarker<T: ?Sized>;

/// A marker for an enum type.
#[phantom]
pub struct EnumMarker<T: ?Sized>;

/// A marker for a union type.
#[phantom]
pub struct UnionMarker<T: ?Sized>;

/// A marker for an object type.
#[phantom]
pub struct ObjectMarker<T: ?Sized>;

/// A marker for an interface type.
#[phantom]
pub struct InterfaceMarker<T: ?Sized>;

/// A marker for repeating.
///
/// - `T` is the type being marked.
/// - `VM` is the marked type of the value of the repeated value.
/// - `O` is the default output type of the repeated value.
/// - `TAG` is the tag of the repeated value.
#[phantom]
pub struct RepeatedMarker<T: ?Sized, VM: ?Sized, const TAG: u32>;

/// A marker for repeating entry.
///
/// - `T` is the type being marked.
/// - `KM` is the marked type of the key of the repeated entry.
/// - `VM` is the marked type of the value of the repeated entry.
/// - `O` is the default output type of the repeated entry.
/// - `TAG` is the tag of the repeated entry.
#[phantom]
pub struct RepeatedEntryMarker<T: ?Sized, KM: ?Sized, VM: ?Sized, const TAG: u32>;

/// A marker for a nullable type.
///
/// The first type parameter `T` is the type be marked,
/// and the second type parameter `V` is the value type of the nullable.
#[phantom]
pub struct NullableMarker<T: ?Sized, V: ?Sized>;

/// A marker for flatten a nested type.
///
/// This marker is mostly used in the below situation:
///
/// ```rust,ignore
/// struct User {
///   email: Option<String>,
/// }
/// ```
///
/// For email field, encoding it as a nullable makes no sense,
/// as `Grost` already natively support nullable field in encoding/decoding layer.
#[phantom]
pub struct FlattenMarker<T: ?Sized, V: ?Sized>;

/// A marker for a list type.
///
/// The first type parameter `T` is the type be marked,
/// and the second type parameter `V` is the value type of the list.
#[phantom]
pub struct ListMarker<T: ?Sized, V: ?Sized>;

/// A marker for a set type.
///
/// The first type parameter `T` is the type be marked,
/// and the second type parameter `V` is the value type of the set.
#[phantom]
pub struct SetMarker<T: ?Sized, V: ?Sized>;

/// A marker for a map type.
///
/// The first type parameter `T` is the type be marked,
/// the second type parameter `K` is the key type of the map,
/// and the third type parameter `V` is the value type of the map.
#[phantom]
pub struct MapMarker<T: ?Sized, K: ?Sized, V: ?Sized>;

/// A marker for a generic type
#[phantom]
pub struct GenericMarker<M: ?Sized>;
