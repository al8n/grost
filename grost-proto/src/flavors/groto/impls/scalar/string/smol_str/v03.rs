use crate::{
  buffer::ReadBuf,
  convert::{PartialTransform, Transform},
  decode::{Decode, Str},
  decode_bridge, default_wire_format, encode_bridge, flatten_state,
  flavors::groto::{Groto, LengthDelimited},
  groto_identity_transform, partial_ref_state, partial_state, selectable,
};
use smol_str_0_3::SmolStr;

default_wire_format!(Groto: SmolStr as LengthDelimited);

selectable!(@scalar Groto:SmolStr);

encode_bridge!(
  Groto: str {
    SmolStr as LengthDelimited {
      convert: SmolStr::as_str;
    },
  },
);

decode_bridge!(
  Groto: &'de str => Str<B> {
    SmolStr as LengthDelimited {
      convert: |src: Str<B>| SmolStr::new(src.as_ref());
    },
  },
);

flatten_state!(SmolStr);
partial_ref_state!(
  &'a Groto:
    SmolStr as LengthDelimited => Str<__GROST_READ_BUF__>,
);
partial_state!(
  Groto: SmolStr => SmolStr
);
groto_identity_transform!(
  SmolStr as LengthDelimited,
);
identity_partial_transform!(
  Groto {
    SmolStr as LengthDelimited,
  }
);

impl Transform<&str, Self, LengthDelimited, Groto> for SmolStr {
  fn transform(input: &str) -> Result<Self, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    Ok(SmolStr::new(input))
  }
}

impl PartialTransform<&str, Option<Self>, LengthDelimited, Groto> for SmolStr {
  fn partial_transform(
    input: &str,
    selector: &bool,
  ) -> Result<Option<Self>, <Groto as crate::flavors::Flavor>::Error>
  where
    Self: Sized,
  {
    if *selector {
      <Self as Transform<&str, Self, LengthDelimited, Groto>>::transform(input).map(Some)
    } else {
      Ok(None)
    }
  }
}

impl<B> Transform<Str<B>, Self, LengthDelimited, Groto> for SmolStr
where
  B: ReadBuf,
{
  fn transform(input: Str<B>) -> Result<Self, <Groto as crate::flavors::Flavor>::Error> {
    Ok(SmolStr::new(input))
  }
}

impl<B> PartialTransform<Str<B>, Option<Self>, LengthDelimited, Groto> for SmolStr
where
  B: ReadBuf,
{
  fn partial_transform(
    input: Str<B>,
    selector: &bool,
  ) -> Result<Option<Self>, <Groto as crate::flavors::Flavor>::Error> {
    if *selector {
      <Self as Transform<Str<B>, Self, LengthDelimited, Groto>>::transform(input).map(Some)
    } else {
      Ok(None)
    }
  }
}

impl<'a, B, UB> Decode<'a, Groto, LengthDelimited, Str<B>, B, UB> for SmolStr {
  fn decode(
    context: &'a <Groto as crate::flavors::Flavor>::Context,
    src: B,
  ) -> Result<(usize, Str<B>), <Groto as crate::flavors::Flavor>::Error>
  where
    Str<B>: Sized + 'a,
    B: crate::buffer::ReadBuf,
    UB: crate::buffer::Buffer<<Groto as crate::flavors::Flavor>::Unknown<B>> + 'a,
  {
    <&str as Decode<'a, Groto, LengthDelimited, Str<B>, B, UB>>::decode(context, src)
  }
}
