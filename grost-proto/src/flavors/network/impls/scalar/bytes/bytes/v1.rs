use crate::{
  Message, PartialMessage,
  decode::{Decode, DecodeOwned},
  decode_bridge, encode_bridge,
  flavors::network::{Context, DecodeError, Network, Unknown, WireType},
  into_target, type_owned, type_ref,
};
use bytes_1::Bytes;

encode_bridge!(
  Network: [u8] {
    Bytes {
      convert: Bytes::as_ref;
    },
  },
);

decode_bridge!(
  Network: &'de [u8] {
    Bytes {
      convert: |src: &[u8]| Bytes::copy_from_slice(src);
    },
  },
);

into_target!(@self Network: Bytes);
into_target!(Network: &[u8] => Bytes {
  |val: &[u8]| Ok(Bytes::copy_from_slice(val))
});
type_ref!(@mapping Network: &[u8] => Bytes {
  |val: &[u8]| Ok(Bytes::copy_from_slice(val))
});
type_owned!(@clone Network: Bytes);

impl DecodeOwned<Network, Self> for Bytes {
  fn decode_owned<B, UB>(
    context: &Context,
    wire_type: WireType,
    src: B,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    <&[u8] as Decode<'_, Network, &[u8]>>::decode::<()>(context, wire_type, src.as_bytes())
      .map(|(len, bytes)| (len, Bytes::copy_from_slice(bytes)))
  }

  fn decode_length_delimited_owned<B, UB>(
    context: &Context,
    wire_type: WireType,
    src: B,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    <&[u8] as Decode<'_, Network, &[u8]>>::decode_length_delimited::<()>(
      context,
      wire_type,
      src.as_bytes(),
    )
    .map(|(len, bytes)| (len, Bytes::copy_from_slice(bytes)))
  }
}

impl PartialMessage<Network> for Bytes {
  type UnknownBuffer<B> = ();

  type Encoded<'a>
    = &'a [u8]
  where
    Self: Sized + 'a;

  type Borrowed<'a>
    = &'a Self
  where
    Self: 'a;

  type EncodedOwned
    = Self
  where
    Self: Sized + 'static;
}

impl Message<Network> for Bytes {
  type Partial = Self;

  type Encoded<'a>
    = &'a [u8]
  where
    Self: Sized + 'a;

  type Borrowed<'a>
    = &'a Self
  where
    Self: 'a;

  type EncodedOwned
    = Self
  where
    Self: Sized + 'static;
}
