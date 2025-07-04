use core::marker::PhantomData;

/// A marker trait that associates to the type being marked.
pub trait Marker {
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
pub struct ScalarMarker<T: ?Sized>(PhantomData<T>);

/// A marker for a bytes type.
pub struct BytesMarker<T: ?Sized>(PhantomData<T>);

/// A marker for a string type.
pub struct StringMarker<T: ?Sized>(PhantomData<T>);

/// A marker for an enum type.
pub struct EnumMarker<T: ?Sized>(PhantomData<T>);

/// A marker for a union type.
pub struct UnionMarker<T: ?Sized>(PhantomData<T>);

/// A marker for an object type.
pub struct ObjectMarker<T: ?Sized>(PhantomData<T>);

/// A marker for a nullable type.
/// 
/// The first type parameter `T` is the type be marked,
/// and the second type parameter `V` is the value type of the nullable.
pub struct NullableMarker<T: ?Sized, V: ?Sized> {
  _t: PhantomData<T>,
  _v: PhantomData<V>,
}

/// A marker for a list type.
/// 
/// The first type parameter `T` is the type be marked,
/// and the second type parameter `V` is the value type of the list.
pub struct ListMarker<T: ?Sized, V: ?Sized> {
  _t: PhantomData<T>,
  _v: PhantomData<V>,
}

/// A marker for a set type.
/// 
/// The first type parameter `T` is the type be marked,
/// and the second type parameter `V` is the value type of the set.
pub struct SetMarker<T: ?Sized, V: ?Sized> {
  _t: PhantomData<T>,
  _v: PhantomData<V>,
}

/// A marker for a map type.
/// 
/// The first type parameter `T` is the type be marked,
/// the second type parameter `K` is the key type of the map,
/// and the third type parameter `V` is the value type of the map.
pub struct MapMarker<T: ?Sized, K: ?Sized, V: ?Sized> {
  _k: PhantomData<K>,
  _v: PhantomData<V>,
  _t: PhantomData<T>,
}

