use crate::{
  decode::{Decode, DecodeOwned},
  decode_bridge, decoded_state, default_wire_format, encode_bridge, flatten_state,
  flavors::network::{Context, Error, LengthDelimited, Network, Unknown},
  selectable,
};
use smol_str_0_3::SmolStr;

default_wire_format!(Network: SmolStr as LengthDelimited);

selectable!(@scalar Network:SmolStr);

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

flatten_state!(SmolStr);
decoded_state!(
  &'a Network: SmolStr as LengthDelimited => &'a str
);

impl DecodeOwned<Network, LengthDelimited, Self> for SmolStr {
  fn decode_owned<B, UB>(context: &Context, src: B) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    <str as Decode<'_, Network, LengthDelimited, &str>>::decode::<()>(context, src.as_bytes())
      .map(|(len, bytes)| (len, SmolStr::new(bytes)))
  }

  fn decode_length_delimited_owned<B, UB>(context: &Context, src: B) -> Result<(usize, Self), Error>
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
