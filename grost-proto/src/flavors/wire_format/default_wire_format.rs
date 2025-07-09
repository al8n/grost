use super::{Flavor, WireFormat};

pub use bytes::*;
pub use enumeration::*;
pub use flatten::*;
pub use interface::*;
pub use list::*;
pub use map::*;
pub use nullable::*;
pub use object::*;
pub use repeated::*;
pub use scalar::*;
pub use set::*;
pub use string::*;
pub use union::*;

mod bytes;
mod enumeration;
mod flatten;
mod interface;
mod list;
mod map;
mod nullable;
mod object;
mod repeated;
mod scalar;
mod set;
mod string;
mod union;
mod generic;

#[inline(always)]
const fn assert_zst<T>() {
  assert!(core::mem::size_of::<T>() == 0, "DefaultWireFormat requires the Format be zero-sized type (ZST)");
}

/// The default wire format for a type on flavor `F`.
pub trait DefaultWireFormat<F: Flavor + ?Sized>: sealed::Sealed<F> {
  /// The default wire format of the type for this flavor.
  type Format: WireFormat<F> + 'static;

  /// A singleton instance of the default wire format.
  /// 
  /// For ZSTs, this represents the canonical instance.
  const INSTANCE: Self::Format = {
    assert_zst::<Self::Format>();

    <Self::Format as WireFormat<F>>::INSTANCE
  };

  /// The static reference to the wire format.
  const STATIC_REF: &'static Self::Format = {
    assert_zst::<Self::Format>();

    // SAFETY: For Zero-Sized Types, we can safely create a reference from any properly
    // aligned non-null pointer since ZSTs have no actual memory representation.
    // NonNull::dangling() provides a well-aligned pointer that's safe to use for ZSTs.
    unsafe { 
      &*core::ptr::NonNull::<Self::Format>::dangling().as_ptr()
    }
  };
}

mod sealed {
  use crate::marker::*;

  pub trait Sealed<F: ?Sized> {}

  impl<F: ?Sized, T: ?Sized> Sealed<F> for T where T: Marker {}
}

#[allow(clippy::type_complexity)]
const _: () = {
  const fn static_checks() {
    use crate::{
      flavors::{groto::*, *},
      marker::*,
    };

    #[cfg(feature = "std")]
    {
      use std::collections::HashMap;

      let _: <FlattenMarker<Option<u16>, ScalarMarker<u16>> as DefaultWireFormat<Groto>>::Format =
        Flatten::<Nullable<Varint>, Varint>;
      let _: <BytesMarker<Vec<u8>> as DefaultWireFormat<Groto>>::Format = LengthDelimited;
      let _: <NullableMarker<Option<Vec<u8>>, BytesMarker<Vec<u8>>> as DefaultWireFormat<Groto>>::Format = Nullable::<LengthDelimited>;
      let _: <StringMarker<str> as DefaultWireFormat<Groto>>::Format = LengthDelimited;
      let _: <ListMarker<Vec<Vec<u8>>, BytesMarker<Vec<u8>>> as DefaultWireFormat<Groto>>::Format =
        Packed::<LengthDelimited>;
      let _: <ListMarker<Vec<&str>, StringMarker<&str>> as DefaultWireFormat<Groto>>::Format =
        Packed::<LengthDelimited>;
      let _: <ListMarker<Vec<Vec<Vec<u8>>>, ListMarker<Vec<Vec<u8>>, BytesMarker<Vec<u8>>>> as DefaultWireFormat<Groto>>::Format =
      Packed::<Packed<LengthDelimited>>;
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

      let _: <MapMarker<
        HashMap<u16, Vec<Vec<u32>>>,
        ScalarMarker<u16>,
        ListMarker<Vec<Vec<u32>>, ListMarker<Vec<u32>, ScalarMarker<u32>>>,
      > as DefaultWireFormat<Groto>>::Format = PackedEntry::<Varint, Packed<Packed<Varint>>>;

      let _: <MapMarker<
        HashMap<u16, Vec<HashMap<u32, Vec<u32>>>>,
        ScalarMarker<u16>,
        ListMarker<
          Vec<HashMap<u32, Vec<u32>>>,
          MapMarker<
            HashMap<u32, Vec<u32>>,
            ScalarMarker<u32>,
            ListMarker<Vec<u32>, ScalarMarker<u32>>,
          >,
        >,
      > as DefaultWireFormat<Groto>>::Format =
        PackedEntry::<Varint, Packed<PackedEntry<Varint, Packed<Varint>>>>;
    }
  }

  static_checks();
};

#[cfg(feature = "std")]
#[test]
fn test_static_ref() {
  use crate::{
    flavors::{groto::*, *},
    marker::*,
  };
  
  assert_eq!(<BytesMarker<Vec<u8>> as DefaultWireFormat<Groto>>::STATIC_REF, &LengthDelimited);
}