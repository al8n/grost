use crate::{
  decode::{Decode, DecodeOwned}, decode_bridge, default_wire_format, encode_bridge, flavors::network::{Context, DecodeError, LengthDelimited, Network, Unknown}, into_target, type_ref, Message, PartialMessage
};
use smol_str_0_3::SmolStr;

default_wire_format!(Network: SmolStr as LengthDelimited);

encode_bridge!(
  Network: str {
    SmolStr as LengthDelimited {
      convert: SmolStr::as_str;
    },
  },
);

decode_bridge!(
  Network: &'de str {
    SmolStr as LengthDelimited {
      convert: |src: &str| SmolStr::new(src);
    },
  },
);

into_target!(Network: &str => SmolStr {
  |val: &str| Ok(SmolStr::new(val))
});
type_ref!( Network: &str => SmolStr {
  |val: &str| Ok(SmolStr::new(val))
});

impl DecodeOwned<Network, LengthDelimited, Self> for SmolStr {
  fn decode_owned<B, UB>(context: &Context, src: B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    <str as Decode<'_, Network, LengthDelimited, &str>>::decode::<()>(context, src.as_bytes())
      .map(|(len, bytes)| (len, SmolStr::new(bytes)))
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
    <str as Decode<'_, Network, LengthDelimited, &str>>::decode_length_delimited::<()>(
      context,
      src.as_bytes(),
    )
    .map(|(len, bytes)| (len, SmolStr::new(bytes)))
  }
}

impl PartialMessage<Network, LengthDelimited> for SmolStr {
  type UnknownBuffer<B> = ();

  type Encoded<'a>
    = &'a str
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

impl Message<Network, LengthDelimited> for SmolStr {
  type Partial = Self;

  type Encoded<'a>
    = &'a str
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
