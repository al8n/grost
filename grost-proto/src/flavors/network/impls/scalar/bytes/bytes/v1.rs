use crate::{
  decode::{Decode, DecodeOwned},
  decode_bridge, decoded_state, default_wire_format, encode_bridge, flatten_state,
  flavors::network::{Context, DecodeError, LengthDelimited, Network, Unknown},
  selectable,
};
use bytes_1::{Bytes, BytesMut};

default_wire_format!(Network: Bytes as LengthDelimited; BytesMut as LengthDelimited);

selectable!(@scalar Network:Bytes, BytesMut);

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

impl DecodeOwned<Network, LengthDelimited, Self> for Bytes {
  fn decode_owned<B, UB>(context: &Context, src: B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    <&[u8] as Decode<'_, Network, LengthDelimited, &[u8]>>::decode::<()>(context, src.as_bytes())
      .map(|(len, bytes)| (len, Bytes::copy_from_slice(bytes)))
  }

  fn decode_length_delimited_owned<B, UB>(
    context: &Context,
    src: B,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    <&[u8] as Decode<'_, Network, LengthDelimited, &[u8]>>::decode_length_delimited::<()>(
      context,
      src.as_bytes(),
    )
    .map(|(len, bytes)| (len, Bytes::copy_from_slice(bytes)))
  }
}
