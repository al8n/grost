use crate::flavors::network::{Network, WireType};
use std::sync::Arc;

#[cfg(feature = "bytes_1")]
const _: () = {
  use bytes_1::Bytes;

  bytes_bridge!(Network: (WireType::LengthDelimited): Arc<[u8]> {
    from_bytes: |val: &[u8]| Ok(Arc::<[u8]>::from(val));
    to_bytes: AsRef::as_ref;
  
    type EncodedOwned = Bytes {
      from_ref: |s: &Bytes| Ok(Arc::<[u8]>::from(s.as_ref()));
      from: |s: Bytes| Ok(Arc::from(s.as_ref()));
    }
  },);
};
