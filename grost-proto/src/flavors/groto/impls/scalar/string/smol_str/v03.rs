use crate::{
  buffer::ReadBuf,
  convert::{PartialTransform, Transform},
  decode::{Decode, Str},
  decode_bridge, default_string_wire_format, encode_bridge, flatten_state,
  flavors::groto::{Groto, LengthDelimited},
  groto_identity_transform, partial_ref_state, partial_state, selectable,
};
use smol_str_0_3::SmolStr;

default_string_wire_format!(Groto: SmolStr as LengthDelimited);

selectable!(@scalar Groto:SmolStr);

encode_bridge!(
  Groto: str {
    SmolStr as LengthDelimited {
      convert: SmolStr::as_str;
    },
  },
);

decode_bridge!(
  Groto: &'de str => Str<RB> {
    SmolStr as LengthDelimited {
      convert: |src: Str<RB>| SmolStr::new(src.as_ref());
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
groto_identity_transform!(SmolStr as LengthDelimited,);
identity_partial_transform!(
  Groto {
    SmolStr as LengthDelimited,
  }
);
bidi_equivalent!(impl <str, LengthDelimited> for <SmolStr, LengthDelimited>);
bidi_equivalent!(:<RB: ReadBuf>: impl<SmolStr, LengthDelimited> for <Str<RB>, LengthDelimited>);

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

impl<RB> Transform<Str<RB>, Self, LengthDelimited, Groto> for SmolStr
where
  RB: ReadBuf,
{
  fn transform(input: Str<RB>) -> Result<Self, <Groto as crate::flavors::Flavor>::Error> {
    Ok(SmolStr::new(input))
  }
}

impl<RB> PartialTransform<Str<RB>, Option<Self>, LengthDelimited, Groto> for SmolStr
where
  RB: ReadBuf,
{
  fn partial_transform(
    input: Str<RB>,
    selector: &bool,
  ) -> Result<Option<Self>, <Groto as crate::flavors::Flavor>::Error> {
    if *selector {
      <Self as Transform<Str<RB>, Self, LengthDelimited, Groto>>::transform(input).map(Some)
    } else {
      Ok(None)
    }
  }
}

impl<'a, RB, B> Decode<'a, Str<RB>, LengthDelimited, RB, B, Groto> for SmolStr {
  fn decode(
    context: &'a <Groto as crate::flavors::Flavor>::Context,
    src: RB,
  ) -> Result<(usize, Str<RB>), <Groto as crate::flavors::Flavor>::Error>
  where
    Str<RB>: Sized + 'a,
    RB: crate::buffer::ReadBuf,
    B: crate::buffer::Buffer<<Groto as crate::flavors::Flavor>::Unknown<RB>> + 'a,
  {
    <&str as Decode<'a, Str<RB>, LengthDelimited, RB, B, Groto>>::decode(context, src)
  }
}
