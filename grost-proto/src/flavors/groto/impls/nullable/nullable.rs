use crate::{
  buffer::{Chunk, ChunkMut, ChunkWriter, UnknownBuffer},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultNullableWireFormat, Flavor, Groto, Nullable, WireFormat,
    groto::{Context, DecodeError, EncodeError, WireType},
  },
  selection::Selector,
  state::{PartialRef, Ref, State},
};

impl<T> DefaultNullableWireFormat<Groto> for Option<T> {
  type Format<V>
    = Nullable<V>
  where
    V: WireFormat<Groto> + 'static;
}

impl<'a, RB, UB, W, T> State<PartialRef<'a, Nullable<W>, RB, UB, Groto>> for Option<T>
where
  T: State<PartialRef<'a, W, RB, UB, Groto>>,
  T::Output: Sized,
  W: ?Sized,
  RB: ?Sized,
  UB: ?Sized,
{
  type Output = Option<T::Output>;
}

impl<'a, RB, UB, W, T> State<Ref<'a, Nullable<W>, RB, UB, Groto>> for Option<T>
where
  T: State<Ref<'a, W, RB, UB, Groto>>,
  T::Output: Sized,
  W: ?Sized,
  RB: ?Sized,
  UB: ?Sized,
{
  type Output = Option<T::Output>;
}

struct OptionImpl<'a, W: ?Sized, T> {
  value: &'a Option<T>,
  _m: core::marker::PhantomData<W>,
}

impl<'a, W: ?Sized, T> From<&'a Option<T>> for OptionImpl<'a, W, T> {
  fn from(value: &'a Option<T>) -> Self {
    Self {
      value,
      _m: core::marker::PhantomData,
    }
  }
}

impl<W, T> OptionImpl<'_, W, T>
where
  W: WireFormat<Groto>,
{
  /// Helper function to handle the common encoding pattern
  fn encode_nullable<B, F, R>(
    &self,
    context: &Context,
    mut buf: ChunkWriter<B>,
    get_len: F,
    encode_fn: R,
  ) -> Result<usize, EncodeError>
  where
    F: Fn(&T, &Context) -> usize,
    R: Fn(&T, &Context, ChunkWriter<&mut B>) -> Result<usize, EncodeError>,
    B: ChunkMut,
  {
    let remaining = buf.remaining_mut();

    if remaining == 0 {
      let required_len = if let Some(value) = self.value {
        2 + get_len(value, context)
      } else {
        1
      };
      return Err(EncodeError::buffer_too_small(required_len, 0));
    }

    if let Some(value) = self.value {
      let encoded_len = 2 + get_len(value, context);
      if encoded_len > remaining {
        return Err(EncodeError::buffer_too_small(encoded_len, remaining));
      }

      buf
        .try_write_slice(&[1, W::WIRE_TYPE.as_u8()])
        .map_err(|e| EncodeError::from(e).propagate_buffer_info(|| encoded_len, || remaining))?;
      encode_fn(value, context, buf.as_mut()).map(|written| written + 2)
    } else {
      Ok(buf.write_u8(0))
    }
  }

  /// Helper function to calculate encoded length
  fn encoded_nullable_len<F>(&self, context: &Context, get_len: F) -> usize
  where
    F: Fn(&T, &Context) -> usize,
  {
    if let Some(value) = self.value {
      1 + 1 + get_len(value, context) // presence marker + wire type + value length
    } else {
      1 // Length of the zero byte indicating absence
    }
  }

  /// Helper function for partial encoding with selector
  fn partial_encode_nullable<B, F, R, S>(
    &self,
    context: &Context,
    mut buf: ChunkWriter<B>,
    selector: &S,
    get_len: F,
    encode_fn: R,
  ) -> Result<usize, EncodeError>
  where
    F: Fn(&T, &Context, &S) -> usize,
    R: Fn(&T, &Context, ChunkWriter<&mut B>, &S) -> Result<usize, EncodeError>,
    S: Selector<Groto>,
    B: ChunkMut,
  {
    if selector.is_empty() {
      return Ok(0); // If the selector is empty, no encoding is needed
    }

    let remaining = buf.remaining_mut();
    if remaining == 0 {
      let required_len = if let Some(value) = self.value {
        2 + get_len(value, context, selector)
      } else {
        1
      };
      return Err(EncodeError::buffer_too_small(required_len, 0));
    }

    if let Some(value) = self.value {
      let encoded_len = 2 + get_len(value, context, selector);
      if encoded_len > remaining {
        return Err(EncodeError::buffer_too_small(encoded_len, remaining));
      }
      buf
        .try_write_slice(&[1, W::WIRE_TYPE.as_u8()])
        .map_err(|e| EncodeError::from(e).propagate_buffer_info(|| encoded_len, || remaining))?;
      encode_fn(value, context, buf.as_mut(), selector).map(|written| written + 2)
    } else {
      Ok(buf.write_u8(0))
    }
  }

  /// Helper function for partial encoded length with selector
  fn partial_encoded_nullable_len<F, S>(&self, context: &Context, selector: &S, get_len: F) -> usize
  where
    F: Fn(&T, &Context, &S) -> usize,
    S: Selector<Groto>,
  {
    if selector.is_empty() {
      return 0; // If the selector is empty, no encoding is needed
    }

    if let Some(value) = self.value {
      1 + 1 + get_len(value, context, selector) // presence marker + wire type + value length
    } else {
      1 // Length of the zero byte indicating absence
    }
  }

  /// Helper function for decoding pattern
  fn decode_nullable<'de, RB, F>(
    context: &'de <Groto as Flavor>::Context,
    mut src: RB,
    decode_fn: F,
  ) -> Result<(usize, Option<T>), DecodeError>
  where
    RB: Chunk + 'de,
    F: Fn(&'de <Groto as Flavor>::Context, RB) -> Result<(usize, T), DecodeError>,
  {
    let remaining = src.remaining();
    let marker = src
      .try_read_u8()
      .map_err(|e| DecodeError::from(e).propagate_buffer_info(|| None, || remaining))?;

    if marker == 0 {
      // This is a zero byte indicating absence, so we return None
      return Ok((1, None));
    }

    let wt = src
      .try_read_u8()
      .map_err(|e| DecodeError::from(e).propagate_buffer_info(|| None, || remaining))?;
    let wire_type = WireType::try_from_u8(wt)?;
    if wire_type != W::WIRE_TYPE {
      return Err(DecodeError::unexpected_wire_type(W::WIRE_TYPE, wire_type));
    }

    decode_fn(context, src).map(|(read, val)| (read + 2, Some(val)))
  }
}

impl<W, T> Encode<Nullable<W>, Groto> for Option<T>
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
    OptionImpl::<W, _>::from(self).encode_nullable(
      context,
      buf.into(),
      |value, ctx| value.encoded_raw_len(ctx),
      |value, ctx, buf| value.encode_raw::<&mut WB>(ctx, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    OptionImpl::<W, _>::from(self)
      .encoded_nullable_len(context, |value, ctx| value.encoded_raw_len(ctx))
  }

  fn encode<B>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<B>>,
  ) -> Result<usize, EncodeError>
  where
    B: ChunkMut,
  {
    OptionImpl::<W, _>::from(self).encode_nullable(
      context,
      buf.into(),
      |value, ctx| value.encoded_len(ctx),
      |value, ctx, buf| value.encode::<&mut B>(ctx, buf),
    )
  }

  fn encoded_len(&self, context: &Context) -> usize {
    OptionImpl::<W, _>::from(self)
      .encoded_nullable_len(context, |value, ctx| value.encoded_len(ctx))
  }

  fn encode_length_delimited<WB>(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: impl Into<ChunkWriter<WB>>,
  ) -> Result<usize, EncodeError>
  where
    WB: ChunkMut,
  {
    OptionImpl::<W, _>::from(self).encode_nullable(
      context,
      buf.into(),
      |value, ctx| value.encoded_length_delimited_len(ctx),
      |value, ctx, buf| value.encode_length_delimited::<&mut WB>(ctx, buf),
    )
  }

  fn encoded_length_delimited_len(&self, context: &<Groto as Flavor>::Context) -> usize {
    OptionImpl::<W, _>::from(self).encoded_nullable_len(context, |value, ctx| {
      value.encoded_length_delimited_len(ctx)
    })
  }
}

impl<W, T> PartialEncode<Nullable<W>, Groto> for Option<T>
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
    OptionImpl::<W, _>::from(self).partial_encode_nullable(
      context,
      buf.into(),
      selector,
      |value, ctx, sel| value.partial_encoded_raw_len(ctx, sel),
      |value, ctx, buf, sel| value.partial_encode_raw::<&mut WB>(ctx, buf, sel),
    )
  }

  fn partial_encoded_raw_len(
    &self,
    context: &<Groto as Flavor>::Context,
    selector: &Self::Selector,
  ) -> usize {
    OptionImpl::<W, _>::from(self).partial_encoded_nullable_len(
      context,
      selector,
      |value, ctx, sel| value.partial_encoded_raw_len(ctx, sel),
    )
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
    OptionImpl::<W, _>::from(self).partial_encode_nullable(
      context,
      buf.into(),
      selector,
      |value, ctx, sel| value.partial_encoded_len(ctx, sel),
      |value, ctx, buf, sel| value.partial_encode::<&mut WB>(ctx, buf, sel),
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    OptionImpl::<W, _>::from(self).partial_encoded_nullable_len(
      context,
      selector,
      |value, ctx, sel| value.partial_encoded_len(ctx, sel),
    )
  }

  fn partial_encode_length_delimited<WB>(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: impl Into<ChunkWriter<WB>>,
    selector: &Self::Selector,
  ) -> Result<usize, EncodeError>
  where
    WB: ChunkMut,
  {
    OptionImpl::<W, _>::from(self).partial_encode_nullable(
      context,
      buf.into(),
      selector,
      |value, ctx, sel| value.partial_encoded_length_delimited_len(ctx, sel),
      |value, ctx, buf, sel| value.partial_encode_length_delimited::<&mut WB>(ctx, buf, sel),
    )
  }

  fn partial_encoded_length_delimited_len(
    &self,
    context: &<Groto as Flavor>::Context,
    selector: &Self::Selector,
  ) -> usize {
    OptionImpl::<W, _>::from(self).partial_encoded_nullable_len(
      context,
      selector,
      |value, ctx, sel| value.partial_encoded_length_delimited_len(ctx, sel),
    )
  }
}

impl<'de, W, RB, B, T> Decode<'de, Nullable<W>, RB, B, Groto> for Option<T>
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
    OptionImpl::<W, T>::decode_nullable::<RB, _>(context, src, |ctx, src| T::decode(ctx, src))
  }

  fn decode_length_delimited(
    context: &'de <Groto as Flavor>::Context,
    src: RB,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Chunk + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    OptionImpl::<W, T>::decode_nullable::<RB, _>(context, src, |ctx, src| {
      T::decode_length_delimited(ctx, src)
    })
  }
}
