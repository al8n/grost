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

impl<T: ?Sized, M: ?Sized> Marker for NullableMarker<T, M> {
  type Marked = T;
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

/// A marker for a nullable type.
///
/// The first type parameter `T` is the type be marked,
/// and the second type parameter `V` is the value type of the nullable.
#[phantom]
pub struct NullableMarker<T: ?Sized, V: ?Sized>;

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
