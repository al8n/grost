use std::vec::Vec;

#[cfg(feature = "bytes_1")]
const _: () = {
  use bytes_1::Bytes;

  bytes_bridge!(
    Vec<u8> {
      from_bytes: |bytes: &[u8]| Ok(bytes.to_vec());
      to_bytes: Vec::as_slice;
  
      type EncodedOwned = Bytes {
        from_ref: |s: &Bytes| Ok(s.to_vec());
        from: |s: Bytes| Ok(s.into());
      };
    }
  );
};
