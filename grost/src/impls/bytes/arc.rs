use crate::bytes::Bytes;
use std::sync::Arc;

bytes_bridge!(Arc<[u8]> {
  from_bytes: |val: &[u8]| Ok(Arc::<[u8]>::from(val));
  to_bytes: AsRef::as_ref;

  type EncodedOwned = Bytes {
    from_ref: |s: &Bytes| Ok(Arc::<[u8]>::from(s.as_ref()));
    from: |s: Bytes| Ok(Arc::from(s.as_ref()));
  }
},);
