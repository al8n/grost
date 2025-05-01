#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::{
    default_wire_format,
    flavors::network::{LengthDelimited, Network},
  };
  use bytes_1::Bytes;
  use smallvec_1::SmallVec;

  use crate::{into_target, type_owned, type_ref};

  default_wire_format!(Network: SmallVec<[u8; N]> [const N: usize] as LengthDelimited:LengthDelimited);

  bytes_bridge!(Network: SmallVec<[u8; N]> [const N: usize] {
    from_slice: |val: &[u8]| SmallVec::<[u8; N]>::from(val);
    as_slice: AsRef::as_ref;
  
    type EncodedOwned = Bytes;
  },);

  into_target!(Network: Bytes => SmallVec<[u8; N]> {
    |val: Bytes| Ok(Vec::from(val).into())
  } [const N: usize]);
  into_target!(Network: &[u8] => SmallVec<[u8; N]> {
    |val: &[u8]| Ok(SmallVec::from(val))
  } [const N: usize]);
  type_ref!( Network: &[u8] => SmallVec<[u8; N]> {
    |val: &[u8]| Ok(SmallVec::from(val))
  } [const N: usize]);
  type_owned!( Network: Bytes => SmallVec<[u8; N]> {
    |val: &Bytes| Ok(SmallVec::from(val.as_ref()))
  } [const N: usize]);
};
