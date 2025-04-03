use smallvec_1::SmallVec;

#[cfg(feature = "bytes_1")]
const _: () = {
  use bytes_1::Bytes;

  bytes_bridge!(
    SmallVec<[u8; N]> [const N: usize] {
      from_bytes: |bytes: &[u8]| Ok(SmallVec::from_slice(bytes));
      to_bytes: SmallVec::as_slice;

      type EncodedOwned = Bytes {
        from_ref: |s: &Bytes| Ok(SmallVec::from_slice(s));
        from: |s: Bytes| Ok(SmallVec::from_slice(&s));
      };
    }
  );
};
