use std::rc::Rc;

#[cfg(feature = "bytes_1")]
const _: () = {
  use bytes_1::Bytes;

  bytes_bridge!(Rc<[u8]> {
    from_bytes: |val: &[u8]| Ok(Rc::<[u8]>::from(val));
    to_bytes: AsRef::as_ref;
  
    type EncodedOwned = Bytes {
      from_ref: |s: &Bytes| Ok(Rc::<[u8]>::from(s.as_ref()));
      from: |s: Bytes| Ok(Rc::from(s.as_ref()));
    }
  },);
};
