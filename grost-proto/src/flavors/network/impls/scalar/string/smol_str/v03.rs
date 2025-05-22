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

impl<B> DecodeOwned<Network, LengthDelimited, Self, B> for SmolStr {
  fn decode_owned<D>(context: &Context, src: D) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'static,
    D: crate::buffer::BytesBuffer + 'static,
    B: crate::buffer::Buffer<Unknown<D>> + 'static,
  {
    <str as Decode<'_, Network, LengthDelimited, &str, ()>>::decode(context, src.as_bytes())
      .map(|(len, bytes)| (len, SmolStr::new(bytes)))
  }
}
