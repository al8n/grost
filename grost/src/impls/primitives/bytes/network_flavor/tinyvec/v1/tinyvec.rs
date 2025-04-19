use crate::flavors::network::{Network, WireType};
use tinyvec_1::TinyVec;

#[cfg(feature = "bytes_1")]
const _: () = {
  use bytes_1::Bytes;

  bytes_bridge!(
    Network: (WireType::LengthDelimited): TinyVec<[u8; N]> [const N: usize] {
      from_bytes: |bytes: &[u8]| Ok(TinyVec::from(bytes));
      to_bytes: TinyVec::as_slice;

      type EncodedOwned = Bytes {
        from_ref: |s: &Bytes| Ok(TinyVec::from(s.as_ref()));
        from: |s: Bytes| Ok(TinyVec::from(s.as_ref()));
      };
    }
  );
};
