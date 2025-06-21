use crate::{
  decode::{BytesSlice, Transform},
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
  Network: &'de [u8] => BytesSlice<B> {
    Bytes as LengthDelimited {
      convert: |src: BytesSlice<B>| Bytes::copy_from_slice(src.as_ref());
    },
    BytesMut as LengthDelimited {
      convert: |src: BytesSlice<B>| {
        BytesMut::from(src.as_ref())
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

impl Transform<Network, LengthDelimited, &[u8]> for Bytes {
  fn transform(input: &[u8]) -> Result<Self, <Network as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    Ok(Bytes::copy_from_slice(input))
  }
}

impl Transform<Network, LengthDelimited, &[u8]> for BytesMut {
  fn transform(input: &[u8]) -> Result<Self, <Network as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    Ok(BytesMut::from(input))
  }
}
