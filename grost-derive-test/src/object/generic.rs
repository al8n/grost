use core::marker::PhantomData;

use grost::{Object, flavors::groto::LengthDelimited, marker::BytesMarker};

// #[derive(Object)]
// struct GenericWithMarker<I, M> {
//   #[grost(tag = 1, generic(marker = "M"))]
//   id: I,
//   #[grost(skip)]
//   _m: PhantomData<M>,
// }

// #[derive(Object)]
// struct GenericWithKnownMarker<I> {
//   #[grost(tag = 1, generic(marker = "BytesMarker<I>"))]
//   id: I,
// }

// #[derive(Object)]
// struct GenericWithWireFormat<I> {
//   #[grost(tag = 1, generic(as = "LengthDelimited"))]
//   id: I,
// }

// #[cfg(feature = "std")]
// #[derive(Object)]
// struct GenericWithKnownMarkerVec<I> {
//   #[grost(tag = 1, list(generic(marker = "BytesMarker<I>")))]
//   id: Vec<I>,
// }

// #[derive(Object)]
// struct GenericWithKnownMarkerNullable<I> {
//   #[grost(tag = 1, nullable(generic(marker = "BytesMarker<I>")))]
//   id: Option<I>,
// }


// #[cfg(feature = "std")]
// #[derive(Object)]
// struct GenericWithMarkerVec<I, M> {
//   #[grost(tag = 1, list(generic(marker = "M")))]
//   id: Vec<I>,
//   #[grost(skip)]
//   _m: PhantomData<M>,
// }


// #[cfg(feature = "std")]
// #[derive(Object)]
// struct GenericWithWireFormatVec<I> {
//   #[grost(tag = 1, list(generic(as = "LengthDelimited")))]
//   id: Vec<I>,
// }

#[test]
fn compile() {}
