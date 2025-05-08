#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::{
    default_wire_format,
    flavors::network::{LengthDelimited, Network},
    referenceable, selectable_bridge,
  };
  use ::tinyvec_1::TinyVec;
  use bytes_1::Bytes;

  use crate::{into_target, type_owned, type_ref};

  default_wire_format!(Network: TinyVec<[u8; N]> [const N: usize] as LengthDelimited);

  selectable_bridge!(Network:
    [u8] [TinyVec<[u8; N]> [const N: usize]]
  );

  bytes_bridge!(Network: TinyVec<[u8; N]> [const N: usize] {
    from_slice: |val: &[u8]| TinyVec::<[u8; N]>::from(val);
    as_slice: AsRef::as_ref;
  
    type EncodedOwned = Bytes;
  },);

  into_target!(Network: Bytes => TinyVec<[u8; N]> {
    |val: Bytes| Ok(TinyVec::Heap(Vec::from(val)))
  } [const N: usize]);
  into_target!(Network: &[u8] => TinyVec<[u8; N]> {
    |val: &[u8]| Ok(TinyVec::from(val))
  } [const N: usize]);
  type_ref!( Network: &[u8] => TinyVec<[u8; N]> {
    |val: &[u8]| Ok(TinyVec::from(val))
  } [const N: usize]);
  referenceable!(
    Network: TinyVec<[u8; N]> [const N: usize] as LengthDelimited => &'a [u8]
  );
  type_owned!( Network: Bytes => TinyVec<[u8; N]> {
    |val: &Bytes| Ok(TinyVec::from(val.as_ref()))
  } [const N: usize]);
};
