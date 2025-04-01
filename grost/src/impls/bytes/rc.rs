use crate::bytes::Bytes;
use std::rc::Rc;

bytes_bridge!(Rc<[u8]> {
  from_bytes: |val: &[u8]| Ok(Rc::<[u8]>::from(val));
  to_bytes: AsRef::as_ref;

  type SerializedOwned = Bytes {
    from_ref: |s: &Bytes| Ok(Rc::<[u8]>::from(s.as_ref()));
    from: |s: Bytes| Ok(Rc::from(s.as_ref()));
  }
},);
