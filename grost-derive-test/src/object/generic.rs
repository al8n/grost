#![allow(warnings)]

use core::marker::PhantomData;

use grost::{Object, flavors::groto::LengthDelimited, marker::{BytesMarker, NullableMarker, FlattenMarker}};

#[derive(Object)]
struct GenericWithMarker<I, M> {
  #[grost(tag = 1, generic(marker = "M"))]
  id: I,
  #[grost(skip)]
  _m: PhantomData<M>,
}

#[derive(Object)]
struct GenericWithKnownMarker<I> {
  #[grost(tag = 1, generic(marker = "BytesMarker<I>"))]
  id: I,
}

#[derive(Object)]
struct GenericWithWireFormat<I> {
  #[grost(tag = 1, generic(as = "LengthDelimited"))]
  id: I,
}

#[cfg(feature = "std")]
#[derive(Object)]
struct GenericWithMarkerNullable<I, M> {
  #[grost(tag = 1, generic(marker = "NullableMarker<Option<I>, M>", type = "I"))]
  id: I,
  #[grost(skip)]
  _m: PhantomData<M>,
}

#[cfg(feature = "std")]
#[derive(Object)]
struct GenericWithMarkerFlattenNullable<I, M> {
  #[grost(tag = 1, generic(marker = "FlattenMarker<Option<I>, M>", type = "I"))]
  id: I,
  #[grost(skip)]
  _m: PhantomData<M>,
}

#[test]
fn compile() {}
