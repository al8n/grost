use crate::{
  buffer::{ReadBuf, UnknownBuffer},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultNullableWireFormat, Flavor, Groto, Nullable, WireFormat,
    groto::{Context, Error, WireType},
  },
  selection::{Selectable, Selector},
  state::PartialRef,
  state::State,
};

impl<T> DefaultNullableWireFormat<Groto> for Option<T> {
  type Format<V>
    = Nullable<V>
  where
    V: WireFormat<Groto> + 'static;
}

impl<'a, RB, UB, W, T> State<PartialRef<'a, RB, UB, Nullable<W>, Groto>> for Option<T>
where
  T: State<PartialRef<'a, RB, UB, W, Groto>>,
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
  fn encode_nullable<F, R>(
    &self,
    context: &Context,
    buf: &mut [u8],
    get_len: F,
    encode_fn: R,
  ) -> Result<usize, Error>
  where
    F: Fn(&T, &Context) -> usize,
    R: Fn(&T, &Context, &mut [u8]) -> Result<usize, Error>,
  {
    if buf.is_empty() {
      let required_len = if let Some(value) = self.value {
        2 + get_len(value, context)
      } else {
        1
      };
      return Err(Error::insufficient_buffer(required_len, 0));
    }

    if let Some(value) = self.value {
      let encoded_len = 2 + get_len(value, context);
      let buf_len = buf.len();
      if encoded_len > buf_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }
      buf[0] = 1; // Write a one byte to indicate presence
      buf[1] = W::WIRE_TYPE.as_u8(); // Write the wire type
      encode_fn(value, context, &mut buf[2..]).map(|written| written + 2)
    } else {
      buf[0] = 0; // Write a zero byte to indicate absence
      Ok(1)
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
  fn partial_encode_nullable<F, R, S>(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &S,
    get_len: F,
    encode_fn: R,
  ) -> Result<usize, Error>
  where
    F: Fn(&T, &Context, &S) -> usize,
    R: Fn(&T, &Context, &mut [u8], &S) -> Result<usize, Error>,
    S: Selector<Groto>,
  {
    if selector.is_empty() {
      return Ok(0); // If the selector is empty, no encoding is needed
    }

    if buf.is_empty() {
      let required_len = if let Some(value) = self.value {
        2 + get_len(value, context, selector)
      } else {
        1
      };
      return Err(Error::insufficient_buffer(required_len, 0));
    }

    if let Some(value) = self.value {
      let encoded_len = 2 + get_len(value, context, selector);
      let buf_len = buf.len();
      if encoded_len > buf_len {
        return Err(Error::insufficient_buffer(encoded_len, buf_len));
      }
      buf[0] = 1; // Write a one byte to indicate presence
      buf[1] = W::WIRE_TYPE.as_u8(); // Write the wire type
      encode_fn(value, context, &mut buf[2..], selector).map(|written| written + 2)
    } else {
      buf[0] = 0; // Write a zero byte to indicate absence
      Ok(1)
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
    src: RB,
    decode_fn: F,
  ) -> Result<(usize, Option<T>), <Groto as Flavor>::Error>
  where
    RB: ReadBuf + 'de,
    F: Fn(&'de <Groto as Flavor>::Context, RB) -> Result<(usize, T), <Groto as Flavor>::Error>,
  {
    let buf = src.as_bytes();
    if buf.is_empty() {
      return Err(Error::buffer_underflow());
    }

    let marker = buf[0];
    if marker == 0 {
      // This is a zero byte indicating absence, so we return None
      return Ok((1, None));
    }

    if buf.len() < 2 {
      return Err(Error::buffer_underflow());
    }
    let wire_type = WireType::try_from_u8(buf[1])?;
    if wire_type != W::WIRE_TYPE {
      return Err(Error::unexpected_wire_type(W::WIRE_TYPE, wire_type));
    }

    decode_fn(context, src.slice(2..)).map(|(read, val)| (read + 2, Some(val)))
  }
}

impl<W, T> Encode<Nullable<W>, Groto> for Option<T>
where
  T: Encode<W, Groto>,
  W: WireFormat<Groto>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    OptionImpl::<W, _>::from(self).encode_nullable(
      context,
      buf,
      |value, ctx| value.encoded_raw_len(ctx),
      |value, ctx, buf| value.encode_raw(ctx, buf),
    )
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    OptionImpl::<W, _>::from(self)
      .encoded_nullable_len(context, |value, ctx| value.encoded_raw_len(ctx))
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    OptionImpl::<W, _>::from(self).encode_nullable(
      context,
      buf,
      |value, ctx| value.encoded_len(ctx),
      |value, ctx, buf| value.encode(ctx, buf),
    )
  }

  fn encoded_len(&self, context: &Context) -> usize {
    OptionImpl::<W, _>::from(self)
      .encoded_nullable_len(context, |value, ctx| value.encoded_len(ctx))
  }

  fn encode_length_delimited(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
  ) -> Result<usize, <Groto as Flavor>::Error> {
    OptionImpl::<W, _>::from(self).encode_nullable(
      context,
      buf,
      |value, ctx| value.encoded_length_delimited_len(ctx),
      |value, ctx, buf| value.encode_length_delimited(ctx, buf),
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
  fn partial_encode_raw(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, <Groto as Flavor>::Error> {
    OptionImpl::<W, _>::from(self).partial_encode_nullable(
      context,
      buf,
      selector,
      |value, ctx, sel| value.partial_encoded_raw_len(ctx, sel),
      |value, ctx, buf, sel| value.partial_encode_raw(ctx, buf, sel),
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

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    OptionImpl::<W, _>::from(self).partial_encode_nullable(
      context,
      buf,
      selector,
      |value, ctx, sel| value.partial_encoded_len(ctx, sel),
      |value, ctx, buf, sel| value.partial_encode(ctx, buf, sel),
    )
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    OptionImpl::<W, _>::from(self).partial_encoded_nullable_len(
      context,
      selector,
      |value, ctx, sel| value.partial_encoded_len(ctx, sel),
    )
  }

  fn partial_encode_length_delimited(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, <Groto as Flavor>::Error> {
    OptionImpl::<W, _>::from(self).partial_encode_nullable(
      context,
      buf,
      selector,
      |value, ctx, sel| value.partial_encoded_length_delimited_len(ctx, sel),
      |value, ctx, buf, sel| value.partial_encode_length_delimited(ctx, buf, sel),
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
  fn decode(
    context: &'de <Groto as Flavor>::Context,
    src: RB,
  ) -> Result<(usize, Self), <Groto as Flavor>::Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    OptionImpl::<W, T>::decode_nullable::<RB, _>(context, src, |ctx, src| T::decode(ctx, src))
  }

  fn decode_length_delimited(
    context: &'de <Groto as Flavor>::Context,
    src: RB,
  ) -> Result<(usize, Self), <Groto as Flavor>::Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    OptionImpl::<W, T>::decode_nullable::<RB, _>(context, src, |ctx, src| {
      T::decode_length_delimited(ctx, src)
    })
  }
}
