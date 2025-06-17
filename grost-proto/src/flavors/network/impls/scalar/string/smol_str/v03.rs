use crate::{
  decode::{Decode, Transform},
  decode_bridge, decoded_state, default_wire_format, encode_bridge, flatten_state,
  flavors::network::{LengthDelimited, Network},
  identity_transform, selectable,
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
identity_transform!(
  Network {
    SmolStr as LengthDelimited,
  }
);

impl Transform<Network, LengthDelimited, &str> for SmolStr {
  fn transform(input: &str) -> Result<Self, <Network as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    Ok(SmolStr::new(input))
  }
}

impl<'a, UB> Decode<'a, Network, LengthDelimited, &'a str, UB> for SmolStr {
  fn decode<B>(
    context: &'a <Network as crate::flavors::Flavor>::Context,
    src: B,
  ) -> Result<(usize, &'a str), <Network as crate::flavors::Flavor>::Error>
  where
    &'a str: Sized + 'a,
    B: crate::buffer::ReadBuf<'a>,
    UB: crate::buffer::Buffer<<Network as crate::flavors::Flavor>::Unknown<B>> + 'a,
  {
    <&str as Decode<'a, Network, LengthDelimited, &'a str, UB>>::decode(context, src)
  }
}
