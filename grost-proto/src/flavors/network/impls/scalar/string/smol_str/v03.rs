// use crate::flavors::network::{Network, WireType};
// use ::smol_str_0_3::SmolStr;

// str_bridge!(Network:(WireType::LengthDelimited):SmolStr {
//   from_str: |val| Ok(SmolStr::new(val));
//   to_str: SmolStr::as_str;

//   type EncodedOwned = SmolStr {
//     from_ref: |val: &SmolStr| Ok(val.clone());
//     from: |val: SmolStr| Ok(val);
//   }
// },);

use crate::{
  Message, PartialMessage,
  decode::{Decode, DecodeOwned},
  decode_bridge, encode_bridge,
  flavors::network::{Context, DecodeError, Network, Unknown, WireType},
  into_target, type_owned, type_ref,
};
use smol_str_0_3::SmolStr;

encode_bridge!(
  Network: str {
    SmolStr {
      convert: SmolStr::as_str;
    },
  },
);

decode_bridge!(
  Network: &'de str {
    SmolStr {
      convert: |src: &str| SmolStr::new(src);
    },
  },
);

into_target!(@self Network: SmolStr);
into_target!(Network: &str => SmolStr {
  |val: &str| Ok(SmolStr::new(val))
});
type_ref!(@mapping Network: &str => SmolStr {
  |val: &str| Ok(SmolStr::new(val))
});
type_owned!(@clone Network: SmolStr);

impl DecodeOwned<Network, Self> for SmolStr {
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
    <str as Decode<'_, Network, &str>>::decode::<()>(context, wire_type, src.as_bytes())
      .map(|(len, bytes)| (len, SmolStr::new(bytes)))
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
    <str as Decode<'_, Network, &str>>::decode_length_delimited::<()>(
      context,
      wire_type,
      src.as_bytes(),
    )
    .map(|(len, bytes)| (len, SmolStr::new(bytes)))
  }
}

impl PartialMessage<Network> for SmolStr {
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

impl Message<Network> for SmolStr {
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
