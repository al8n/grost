use crate::{
  Message, PartialMessage,
  decode::{Decode, DecodeOwned},
  decode_bridge, default_wire_format, encode_bridge,
  flavors::network::{Context, DecodeError, LengthDelimited, Network, Unknown},
  into_target, selectable, state, type_ref,
};
use bytes_1::Bytes;

default_wire_format!(Network: Bytes as LengthDelimited);

selectable!(@scalar Network:Bytes);

encode_bridge!(
  Network: [u8] {
    Bytes as LengthDelimited {
      convert: Bytes::as_ref;
    },
  },
);

decode_bridge!(
  Network: &'de [u8] {
    Bytes as LengthDelimited {
      convert: |src: &[u8]| Bytes::copy_from_slice(src);
    },
  },
);

into_target!(Network: &[u8] => Bytes {
  |val: &[u8]| Ok(Bytes::copy_from_slice(val))
});
type_ref!( Network: &[u8] => Bytes {
  |val: &[u8]| Ok(Bytes::copy_from_slice(val))
});
state!(
  &'a Network: Bytes as LengthDelimited => &'a [u8]
);

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

impl PartialMessage<Network, LengthDelimited> for Bytes {
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

impl Message<Network, LengthDelimited> for Bytes {
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
