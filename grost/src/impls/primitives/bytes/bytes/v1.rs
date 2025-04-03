use bytes_1::Bytes;

bytes_bridge!(
  Bytes {
    from_bytes: |bytes: &[u8]| Ok(Bytes::copy_from_slice(bytes));
    to_bytes: Bytes::as_ref;

    type EncodedOwned = Bytes {
      from_ref: |s: &Bytes| Ok(s.clone());
      from: |s: Bytes| Ok(s);
    };
  }
);
