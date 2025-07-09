use core::marker::PhantomData;

use grost::{Object, flavors::groto::LengthDelimited};

// #[derive(Object)]
// struct GenericWithMarker<I, M> {
//   #[grost(tag = 1, generic(marker = "M"))]
//   id: I,
//   #[grost(skip)]
//   _m: PhantomData<M>,
// }

#[cfg(feature = "std")]
#[derive(Object)]
struct GenericWithMarkerVec<I, M> {
  #[grost(tag = 1, list(generic(marker = "M")))]
  id: Vec<I>,
  #[grost(skip)]
  _m: PhantomData<M>,
}

// #[derive(Object)]
// struct GenericWithWireFormat<I> {
//   #[grost(tag = 1, generic(as = "LengthDelimited"))]
//   id: I,
// }

// #[cfg(feature = "std")]
// #[derive(Object)]
// struct GenericWithWireFormatVec<I> {
//   #[grost(tag = 1, list(generic(as = "LengthDelimited")))]
//   id: Vec<I>,
// }

#[test]
fn compile() {}
