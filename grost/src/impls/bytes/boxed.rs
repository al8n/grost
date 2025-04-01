use crate::bytes::Bytes;
use std::boxed::Box;

bytes_bridge!(Box<[u8]> {
  from_bytes: |val: &[u8]| Ok(Box::<[u8]>::from(val));
  to_bytes: AsRef::as_ref;

  type SerializedOwned = Bytes {
    from_ref: |s: &Bytes| Ok(Box::<[u8]>::from(s.as_ref()));
    from: |s: Bytes| Ok(Box::from(s.as_ref()));
  }
},);