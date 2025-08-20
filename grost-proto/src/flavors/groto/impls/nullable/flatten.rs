use crate::{
  buffer::{Chunk, ChunkMut, ChunkWriter, UnknownBuffer},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultFlattenWireFormat, Flatten, Flavor, Groto, Nullable, WireFormat,
    groto::{Context, DecodeError, EncodeError},
  },
  selection::Selector,
  state::{PartialRef, Ref, State},
};

impl<T> DefaultFlattenWireFormat<Groto> for Option<T> {
  type Format<V>
    = Flatten<Nullable<V>, V>
  where
    V: WireFormat<Groto> + 'static;
}

impl<'a, RB, UB, W, T> State<PartialRef<'a, Flatten<Nullable<W>, W>, RB, UB, Groto>> for Option<T>
where
  T: State<PartialRef<'a, W, RB, UB, Groto>>,
  T::Output: Sized,
  W: ?Sized,
  RB: ?Sized,
  UB: ?Sized,
{
  type Output = Option<T::Output>;
}

impl<'a, RB, UB, W, T> State<Ref<'a, Flatten<Nullable<W>, W>, RB, UB, Groto>> for Option<T>
where
  T: State<Ref<'a, W, RB, UB, Groto>>,
  T::Output: Sized,
  W: ?Sized,
  RB: ?Sized,
  UB: ?Sized,
{
  type Output = Option<T::Output>;
}

impl<W, T> Encode<Flatten<Nullable<W>, W>, Groto> for Option<T>
where
  T: Encode<W, Groto>,
  W: WireFormat<Groto>,
{
  fn encode_raw<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
  ) -> Result<usize, EncodeError>
  where
    WB: ChunkMut,
  {
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

  fn encode<B>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<B>>,
  ) -> Result<usize, EncodeError>
  where
    B: ChunkMut,
  {
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

  fn encode_length_delimited<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
  ) -> Result<usize, EncodeError>
  where
    WB: ChunkMut,
  {
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
  fn partial_encode_raw<WB>(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: impl Into<ChunkWriter<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError>
  where
    WB: ChunkMut,
  {
    if selector.is_empty() {
      return Ok(0); // If the selector is empty, no encoding is needed
    }

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
    if selector.is_empty() {
      return 0; // If the selector is empty, no encoding is needed
    }

    if let Some(value) = self {
      value.partial_encoded_raw_len(context, selector)
    } else {
      0
    }
  }

  fn partial_encode<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError>
  where
    WB: ChunkMut,
  {
    if selector.is_empty() {
      return Ok(0); // If the selector is empty, no encoding is needed
    }

    if let Some(value) = self {
      value.partial_encode(context, buf, selector)
    } else {
      Ok(0)
    }
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0; // If the selector is empty, no encoding is needed
    }

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
    if selector.is_empty() {
      return 0; // If the selector is empty, no encoding is needed
    }

    if let Some(value) = self {
      value.partial_encoded_length_delimited_len(context, selector)
    } else {
      0
    }
  }

  fn partial_encode_length_delimited<WB>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError>
  where
    WB: ChunkMut,
  {
    if selector.is_empty() {
      return Ok(0); // If the selector is empty, no encoding is needed
    }

    if let Some(value) = self {
      value.partial_encode_length_delimited(context, buf, selector)
    } else {
      Ok(0)
    }
  }
}

impl<'de, W, RB, B, T> Decode<'de, Flatten<Nullable<W>, W>, RB, B, Groto> for Option<T>
where
  T: Decode<'de, W, RB, B, Groto>,
  W: WireFormat<Groto>,
{
  fn decode(context: &'de <Groto as Flavor>::Context, src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Chunk + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    if !src.has_remaining() {
      return Ok((0, None)); // If the source is empty, return None
    }

    T::decode(context, src).map(|(read, val)| (read, Some(val)))
  }
}
