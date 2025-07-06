use crate::{
  buffer::{Buffer, ReadBuf},
  convert::{PartialRef, PartialTransform, State, Transform},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultFlattenWireFormat, Flatten, Flavor, Groto, Nullable, WireFormat,
    groto::{Context, Error},
  },
  selection::Selectable,
};

impl<T> DefaultFlattenWireFormat<Groto> for Option<T> {
  type Format<V>
    = Flatten<Nullable<V>, V>
  where
    V: WireFormat<Groto>;
}

impl<'a, RB, UB, W, T> State<PartialRef<'a, RB, UB, Flatten<Nullable<W>, W>, Groto>> for Option<T>
where
  T: State<PartialRef<'a, RB, UB, W, Groto>>,
  T::Output: Sized,
  W: ?Sized,
  RB: ?Sized,
  UB: ?Sized,
{
  type Output = Option<T::Output>;
}

impl<W, I, O, T> Transform<Option<I>, Option<O>, Flatten<Nullable<W>, W>, Groto> for Option<T>
where
  W: WireFormat<Groto>,
  T: Transform<I, O, W, Groto>,
{
  fn transform(input: Option<I>) -> Result<Option<O>, <Groto as Flavor>::Error> {
    match input {
      Some(value) => T::transform(value).map(Some),
      None => Ok(None),
    }
  }
}

impl<I, O, W, T> PartialTransform<Option<I>, Option<O>, Flatten<Nullable<W>, W>, Groto>
  for Option<T>
where
  W: WireFormat<Groto>,
  T: PartialTransform<I, Option<O>, W, Groto> + Sized + Selectable<Groto>,
  I: Selectable<Groto, Selector = Self::Selector>,
  O: Selectable<Groto, Selector = Self::Selector>,
{
  fn partial_transform(
    input: Option<I>,
    selector: &Self::Selector,
  ) -> Result<Option<O>, <Groto as Flavor>::Error> {
    match input {
      Some(value) => {
        <T as PartialTransform<I, Option<O>, W, Groto>>::partial_transform(value, selector)
      }
      None => Ok(None),
    }
  }
}

impl<W, T> Encode<Flatten<Nullable<W>, W>, Groto> for Option<T>
where
  T: Encode<W, Groto>,
  W: WireFormat<Groto>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if let Some(value) = self {
      value.encode_raw(context, buf)
    } else {
      Ok(0)
    }
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    if let Some(value) = self {
      value.encoded_raw_len(context)
    } else {
      0
    }
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if let Some(value) = self {
      value.encode(context, buf)
    } else {
      Ok(0)
    }
  }

  fn encoded_len(&self, context: &Context) -> usize {
    if let Some(value) = self {
      value.encoded_len(context)
    } else {
      0
    }
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if let Some(value) = self {
      value.encode_length_delimited(context, buf)
    } else {
      Ok(0)
    }
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    if let Some(value) = self {
      value.encoded_length_delimited_len(context)
    } else {
      0
    }
  }
}

impl<W, T> PartialEncode<Flatten<Nullable<W>, W>, Groto> for Option<T>
where
  T: PartialEncode<W, Groto>,
  W: WireFormat<Groto>,
{
  fn partial_encode_raw(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, <Groto as Flavor>::Error> {
    if let Some(value) = self {
      value.partial_encode_raw(context, buf, selector)
    } else {
      Ok(0)
    }
  }

  fn partial_encoded_raw_len(
    &self,
    context: &<Groto as Flavor>::Context,
    selector: &Self::Selector,
  ) -> usize {
    if let Some(value) = self {
      value.partial_encoded_raw_len(context, selector)
    } else {
      0
    }
  }

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if let Some(value) = self {
      value.partial_encode(context, buf, selector)
    } else {
      Ok(0)
    }
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if let Some(value) = self {
      value.partial_encoded_len(context, selector)
    } else {
      0
    }
  }

  fn partial_encoded_length_delimited_len(
    &self,
    context: &Context,
    selector: &Self::Selector,
  ) -> usize {
    if let Some(value) = self {
      value.partial_encoded_length_delimited_len(context, selector)
    } else {
      0
    }
  }

  fn partial_encode_length_delimited(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if let Some(value) = self {
      value.partial_encode_length_delimited(context, buf, selector)
    } else {
      Ok(0)
    }
  }
}

impl<'de, W, O, RB, B, T> Decode<'de, Option<O>, Flatten<Nullable<W>, W>, RB, B, Groto>
  for Option<T>
where
  T: Decode<'de, O, W, RB, B, Groto>,
  W: WireFormat<Groto>,
{
  fn decode(
    context: &'de <Groto as Flavor>::Context,
    src: RB,
  ) -> Result<(usize, Option<O>), <Groto as Flavor>::Error>
  where
    O: Sized + 'de,
    RB: ReadBuf + 'de,
    B: Buffer<<Groto as Flavor>::Unknown<RB>> + 'de,
  {
    T::decode(context, src).map(|(read, val)| (read, Some(val)))
  }
}
