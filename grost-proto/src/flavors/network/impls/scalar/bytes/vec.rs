#[cfg(feature = "bytes_1")]
const _: () = {
  use crate::{
    default_wire_format,
    flavors::network::{LengthDelimited, Network},
    referenceable,
  };
  use bytes_1::Bytes;
  use std::vec::Vec;

  use crate::{into_target, type_owned, type_ref};

  default_wire_format!(Network: Vec<u8> as LengthDelimited);

  bytes_bridge!(Network: Vec<u8> {
    from_slice: |val: &[u8]| val.to_vec();
    as_slice: AsRef::as_ref;

    type EncodedOwned = Bytes;
  },);

  into_target!(Network: Bytes => Vec<u8> {
    |val: Bytes| Ok(val.into())
  });
  into_target!(Network: &[u8] => Vec<u8> {
    |val: &[u8]| Ok(val.to_vec())
  });
  referenceable!(
    Network: Vec<u8> as LengthDelimited => &'a [u8]
  );
  type_ref!(Network: &[u8] => Vec<u8> {
    |val: &[u8]| Ok(val.to_vec())
  });
  type_owned!(Network: Bytes => Vec<u8> {
    |val: &Bytes| Ok(val.to_vec())
  });
};
