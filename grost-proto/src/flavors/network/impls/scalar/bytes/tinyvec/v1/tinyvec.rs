#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::flavors::network::Network;
  use bytes_1::Bytes;
  use tinyvec_1::TinyVec;

  use crate::{bytes_bridge, into_target, type_owned, type_ref};

  bytes_bridge!(Network: TinyVec<[u8; N]> [const N: usize] {
    from_slice: |val: &[u8]| TinyVec::<[u8; N]>::from(val);
    as_slice: AsRef::as_ref;
  
    type EncodedOwned = Bytes {
      from_ref: |s: &Bytes| Ok(TinyVec::<[u8; N]>::from(s.as_ref()));
      from: |s: Bytes| Ok(Vec::from(s).into());
    }
  },);

  into_target!(Network: Bytes => TinyVec<[u8; N]> {
    |val: Bytes| Ok(TinyVec::Heap(Vec::from(val)))
  } [const N: usize]);
  into_target!(Network: &[u8] => TinyVec<[u8; N]> {
    |val: &[u8]| Ok(TinyVec::from(val))
  } [const N: usize]);
  into_target!(@self Network: TinyVec<[u8; N]> [const N: usize]);
  type_ref!(@mapping Network: &[u8] => TinyVec<[u8; N]> {
    |val: &[u8]| Ok(TinyVec::from(val))
  } [const N: usize]);
  type_owned!(@mapping Network: Bytes => TinyVec<[u8; N]> {
    |val: &Bytes| Ok(TinyVec::from(val.as_ref()))
  } [const N: usize]);
  type_owned!(@clone Network: TinyVec<[u8; N]> [const N: usize]);
  bytes_message!(TinyVec<[u8; N]> => Bytes [const N: usize]);
};
