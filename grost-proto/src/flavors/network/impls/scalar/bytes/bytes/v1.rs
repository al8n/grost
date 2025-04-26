use crate::{
  decode::{Decode, DecodeOwned}, decode_bridge, encode_bridge, flavors::network::{Context, DecodeError, Network, Unknown, WireType}, IntoTarget, Message, PartialMessage, TypeOwned, TypeRef
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

impl IntoTarget<Network, Self> for Bytes {
  fn into_target(self) -> Result<Self, <Network as crate::flavors::Flavor>::DecodeError> {
    Ok(self)
  }
}

impl IntoTarget<Network, Bytes> for &[u8] {
  fn into_target(self) -> Result<Bytes, <Network as crate::flavors::Flavor>::DecodeError> {
    Ok(Bytes::copy_from_slice(self))
  }
}

impl TypeRef<Network, Bytes> for &[u8] {
  fn to(&self) -> Result<Bytes, <Network as crate::flavors::Flavor>::DecodeError> {
    Ok(Bytes::copy_from_slice(self))
  }
}

impl TypeOwned<Network, Self> for Bytes {
  fn to(&self) -> Result<Self, <Network as crate::flavors::Flavor>::DecodeError> {
    Ok(self.clone())
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
