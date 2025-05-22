#[cfg(feature = "bytes_1")]
const _: () = {
  use bytes_1::{Bytes, BytesMut};

  impl_reflectable_with_variant!(
    Bytes:Bytes,
    BytesMut:Bytes,
  );
};
