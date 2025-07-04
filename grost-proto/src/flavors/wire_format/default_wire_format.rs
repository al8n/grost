use super::{Flavor, WireFormat};

pub use bytes::*;
pub use enumeration::*;
pub use list::*;
pub use map::*;
pub use nullable::*;
pub use object::*;
pub use scalar::*;
pub use set::*;
pub use string::*;
pub use union::*;

mod bytes;
mod enumeration;
mod list;
mod map;
mod nullable;
mod object;
mod scalar;
mod set;
mod string;
mod union;

/// The default wire format for a type on flavor `F`.
pub trait DefaultWireFormat<F: Flavor + ?Sized>: sealed::Sealed<F> {
  /// The default wire format of the type for this flavor.
  type Format: WireFormat<F>;
}

mod sealed {
  use crate::marker::*;

  pub trait Sealed<F: ?Sized> {}

  impl<F: ?Sized, T: ?Sized> Sealed<F> for T where T: Marker {}
}

fn checks() {
  use crate::{flavors::groto::*, marker::*};
  use core::marker::PhantomData;

  let _: <BytesMarker<Vec<u8>> as DefaultWireFormat<Groto>>::Format = LengthDelimited;
  let _: <ListMarker<Vec<Vec<u8>>, BytesMarker<Vec<u8>>> as DefaultWireFormat<Groto>>::Format =
    Packed::<LengthDelimited>;
  let _: <ListMarker<Vec<Vec<Vec<u8>>>, ListMarker<Vec<Vec<u8>>, BytesMarker<Vec<u8>>>> as DefaultWireFormat<Groto>>::Format = Packed::<Packed<LengthDelimited>>;
  let _: <ListMarker<Vec<u16>, ScalarMarker<u16>> as DefaultWireFormat<Groto>>::Format =
    Packed::<Varint>;
  let _: <ListMarker<Vec<Vec<u16>>, ListMarker<Vec<u16>, ScalarMarker<u16>>> as DefaultWireFormat<Groto>>::Format = Packed::<Packed<Varint>>;
  let _: <ListMarker<
    Vec<Vec<Vec<u16>>>,
    ListMarker<Vec<Vec<u16>>, ListMarker<Vec<u16>, ScalarMarker<u16>>>,
  > as DefaultWireFormat<Groto>>::Format = Packed::<Packed<Packed<Varint>>>;

  let _: <ScalarMarker<u16> as DefaultWireFormat<Groto>>::Format = Varint;
  let _: <ScalarMarker<u32> as DefaultWireFormat<Groto>>::Format = Varint;

  let _: <ListMarker<Vec<Vec<u32>>, ListMarker<Vec<u32>, ScalarMarker<u32>>> as DefaultWireFormat<Groto>>::Format = Packed::<Packed<Varint>>;

  // let _: <MapMarker<
  //     HashMap<u16, Vec<Vec<u32>>>,
  //     ScalarMarker<u16>,
  //     ListMarker<
  //         Vec<Vec<u32>>,
  //         ListMarker<Vec<u32>, ScalarMarker<u32>>>
  //     > as DefaultWireFormat<Groto>
  // >::Format = PackedEntry::<Fixed16, Packed<Packed<Fixed32>>> {
  //     _k: PhantomData,
  //     _v: PhantomData,
  // };

  // let _: <MapMarker<
  //     HashMap<u16, Vec<HashMap<u32, Vec<u32>>>>,
  //     ScalarMarker<u16>,
  //     ListMarker<
  //         Vec<HashMap<u32, Vec<u32>>>,
  //         MapMarker<HashMap<u32, Vec<u32>>, ScalarMarker<u32>, ListMarker<Vec<u32>, ScalarMarker<u32>>>>
  //     > as DefaultWireFormat<Groto>
  // >::Format = PackedEntry::<Fixed16, Packed<PackedEntry<Fixed32, Packed<Fixed32>>>> {
  //     _k: PhantomData,
  //     _v: PhantomData,
  // };
}
