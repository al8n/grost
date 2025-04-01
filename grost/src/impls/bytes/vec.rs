use crate::bytes::Bytes;
use std::vec::Vec;

bytes_bridge!(
  Vec<u8> {
    from_bytes: |bytes: &[u8]| Ok(bytes.to_vec());
    to_bytes: Vec::as_slice;

    type SerializedOwned = Bytes {
      from_ref: |s: &Bytes| Ok(s.to_vec());
      from: |s: Bytes| Ok(s.into());
    };
  }
);
