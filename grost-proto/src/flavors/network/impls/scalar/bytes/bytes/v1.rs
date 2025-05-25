use crate::{
  decode_bridge, decoded_state, default_wire_format, encode_bridge, flatten_state,
  flavors::network::{LengthDelimited, Network},
  selectable,
};
use bytes_1::{Bytes, BytesMut};

default_wire_format!(Network: Bytes as LengthDelimited; BytesMut as LengthDelimited);

selectable!(@scalar Network: Bytes, BytesMut);

encode_bridge!(
  Network: [u8] {
    Bytes as LengthDelimited {
      convert: Bytes::as_ref;
    },
    BytesMut as LengthDelimited {
      convert: BytesMut::as_ref;
    },
  },
);

decode_bridge!(
  Network: &'de [u8] {
    Bytes as LengthDelimited {
      convert: |src: &[u8]| Bytes::copy_from_slice(src);
    },
    BytesMut as LengthDelimited {
      convert: |src: &[u8]| {
        BytesMut::from(src)
      };
    },
  },
);

decoded_state!(
  &'a Network:
    Bytes as LengthDelimited => &'a [u8],
    BytesMut as LengthDelimited => &'a [u8],
);
flatten_state!(Bytes, BytesMut);
bytes_bridge!(
  Network: Bytes, BytesMut,
);
