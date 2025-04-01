use crate::bytes::Bytes;
use smallvec_1::SmallVec;

bytes_bridge!(
  SmallVec<[u8; N]> [const N: usize] {
    from_bytes: |bytes: &[u8]| Ok(SmallVec::from_slice(bytes));
    to_bytes: SmallVec::as_slice;

    type SerializedOwned = Bytes {
      from_ref: |s: &Bytes| Ok(SmallVec::from_slice(s));
      from: |s: Bytes| Ok(SmallVec::from_slice(&s));
    };
  }
);
