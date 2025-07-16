use indexmap_2::IndexSet;
use varing::decode_u32_varint;

use core::hash::{BuildHasher, Hash};

use crate::{
  buffer::{Buffer, ReadBuf, UnknownBuffer},
  convert::{
    Flattened, Inner, Partial, PartialIdentity, PartialRef, PartialTryFromRef, Ref, TryFromPartial,
    TryFromPartialRef, TryFromRef,
  },
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    groto::{Context, Error, PackedSetDecoder}, DefaultRepeatedWireFormat, DefaultSetWireFormat, Groto, Packed, Repeated, WireFormat
  },
  selection::{Selectable, Selector},
  state::State,
};

use super::DefaultPartialSetBuffer;

impl<K, S> State<Flattened<Inner>> for IndexSet<K, S> {
  type Output = K;
}

impl<K, S> DefaultSetWireFormat<Groto> for IndexSet<K, S> {
  type Format<KM>
    = Packed<KM>
  where
    KM: WireFormat<Groto> + 'static;
}

impl<K, S> DefaultRepeatedWireFormat<Groto> for IndexSet<K, S> {
  type Format<KM, const TAG: u32>
    = Repeated<KM, TAG>
  where
    KM: WireFormat<Groto> + 'static;
}

impl<K, S> State<Partial<Groto>> for IndexSet<K, S>
where
  K: State<Partial<Groto>>,
  K::Output: Sized,
{
  type Output = super::DefaultPartialSetBuffer<K::Output>;
}

impl<'a, K, KW, S, RB, B> State<PartialRef<'a, RB, B, Packed<KW>, Groto>> for IndexSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: State<PartialRef<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
{
  type Output = PackedSetDecoder<'a, K::Output, RB, B, KW>;
}

impl<'a, K, KW, S, RB, B> State<Ref<'a, RB, B, Packed<KW>, Groto>> for IndexSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  Packed<KW>: WireFormat<Groto> + 'a,
  K: State<Ref<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
{
  type Output = PackedSetDecoder<'a, K::Output, RB, B, KW>;
}

impl<K, S> Selectable<Groto> for IndexSet<K, S>
where
  K: Selectable<Groto>,
{
  type Selector = K::Selector;
}

impl<'a, K, KW, S, RB, B> Decode1<'a, Packed<KW>, RB, B, Groto> for IndexSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  S: BuildHasher + Default,
  K: Eq + Hash + Decode1<'a, KW, RB, B, Groto>,
{
  fn decode(context: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let bytes = src.as_bytes();
    let bytes_len = bytes.len();
    if bytes_len == 0 {
      return Err(Error::buffer_underflow());
    }

    // decode total bytes
    let (mut offset, total_bytes) = decode_u32_varint(bytes)?;

    if bytes_len < offset + total_bytes as usize {
      return Err(Error::buffer_underflow());
    }

    // decode the number of elements
    let (num_elements_size, num_elements) = decode_u32_varint(&bytes[offset..])?;
    offset += num_elements_size;
    if num_elements == 0 {
      return Ok((offset, IndexSet::with_capacity_and_hasher(0, S::default())));
    }

    let mut set = IndexSet::with_capacity_and_hasher(num_elements as usize, S::default());
    while set.len() < num_elements as usize && offset < bytes_len {
      let (read, item) = K::decode(context, src.slice(offset..))?;
      offset += read;

      if !set.insert(item) && context.err_on_duplicated_set_keys() {
        return Err(Error::custom("duplicated keys in set"));
      }
    }

    if set.len() != num_elements as usize && context.err_on_length_mismatch() {
      return Err(Error::custom(format!(
        "expected {num_elements} elements in set, but got {} elements",
        set.len()
      )));
    }

    Ok((offset, set))
  }
}

impl<K, KW, S> Encode<Packed<KW>, Groto> for IndexSet<K, S>
where
  KW: WireFormat<Groto>,
  K: Encode<KW, Groto>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let encoded_len = self.encoded_raw_len(context);
    let buf_len = buf.len();
    if buf_len < encoded_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let mut offset = 0;
    // encode num of elements
    let num_elems = self.len();
    let num_elems_size = varing::encode_u32_varint_to(num_elems as u32, buf)?;
    offset += num_elems_size;

    // encode the elements
    for item in self {
      let item_encoded_len = item.encode(context, &mut buf[offset..])?;
      offset += item_encoded_len;
    }

    Ok(offset)
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    let num_elems = self.len();
    let mut len = varing::encoded_u32_varint_len(num_elems as u32);

    if let Some(fixed_length) = KW::FIXED_LENGTH {
      len += fixed_length * num_elems;
    } else {
      for item in self {
        len += item.encoded_len(context);
      }
    }
    len
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    let encoded_raw_len = self.encoded_raw_len(context);
    let encoded_len = varing::encoded_u32_varint_len(encoded_raw_len as u32) + encoded_raw_len;

    let buf_len = buf.len();
    if buf_len < encoded_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let mut offset = 0;

    // encode total bytes
    if encoded_len > u32::MAX as usize {
      return Err(Error::too_large(encoded_len, u32::MAX as usize));
    }

    let total_bytes = encoded_raw_len as u32;
    let total_bytes_size = varing::encode_u32_varint_to(total_bytes, buf)?;
    offset += total_bytes_size;

    // encode num of elements
    let num_elems = self.len();
    let num_elems_size = varing::encode_u32_varint_to(num_elems as u32, buf)?;
    offset += num_elems_size;

    // encode the elements
    for item in self {
      let item_encoded_len = item.encode(context, &mut buf[offset..])?;
      offset += item_encoded_len;
    }

    Ok(offset)
  }

  fn encoded_len(&self, context: &Context) -> usize {
    let total_bytes = self.encoded_raw_len(context);
    varing::encoded_u32_varint_len(total_bytes as u32) + total_bytes
  }
}

impl<K, KW, S> PartialEncode<Packed<KW>, Groto> for IndexSet<K, S>
where
  KW: WireFormat<Groto>,
  K: PartialEncode<KW, Groto>,
{
  fn partial_encode_raw(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if selector.is_empty() {
      return Ok(0);
    }

    let encoded_len = self.partial_encoded_raw_len(context, selector);
    let buf_len = buf.len();
    if buf_len < encoded_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let mut offset = 0;
    // encode num of elements
    let num_elems = self.len();
    let num_elems_size = varing::encode_u32_varint_to(num_elems as u32, buf)?;
    offset += num_elems_size;

    // encode the elements
    for item in self {
      let item_encoded_len = item.partial_encode(context, &mut buf[offset..], selector)?;
      offset += item_encoded_len;
    }

    Ok(offset)
  }

  fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    let num_elems = self.len();
    let mut len = varing::encoded_u32_varint_len(num_elems as u32);

    for item in self {
      len += item.partial_encoded_len(context, selector);
    }
    len
  }

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if selector.is_empty() {
      return Ok(0);
    }

    let encoded_raw_len = self.partial_encoded_raw_len(context, selector);
    let encoded_len = varing::encoded_u32_varint_len(encoded_raw_len as u32) + encoded_raw_len;

    let buf_len = buf.len();
    if buf_len < encoded_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len));
    }

    let mut offset = 0;

    // encode total bytes
    if encoded_len > u32::MAX as usize {
      return Err(Error::too_large(encoded_len, u32::MAX as usize));
    }

    let total_bytes = encoded_raw_len as u32;
    let total_bytes_size = varing::encode_u32_varint_to(total_bytes, buf)?;
    offset += total_bytes_size;

    // encode num of elements
    let num_elems = self.len();
    let num_elems_size = varing::encode_u32_varint_to(num_elems as u32, buf)?;
    offset += num_elems_size;

    // encode the elements
    for item in self {
      let item_encoded_len = item.partial_encode(context, &mut buf[offset..], selector)?;
      offset += item_encoded_len;
    }

    Ok(offset)
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if selector.is_empty() {
      return 0;
    }

    let total_bytes = self.partial_encoded_raw_len(context, selector);
    varing::encoded_u32_varint_len(total_bytes as u32) + total_bytes
  }
}

impl<K, S> TryFromPartial<Groto> for IndexSet<K, S>
where
  K: TryFromPartial<Groto> + Eq + Hash,
  K::Output: Sized,
  S: BuildHasher + Default,
{
  fn try_from_partial(ctx: &Context, input: Self::Output) -> Result<Self, Error> {
    let mut set = IndexSet::with_capacity_and_hasher(input.len(), S::default());

    for item in input {
      let item = K::try_from_partial(ctx, item)?;
      if !set.insert(item) && ctx.err_on_duplicated_set_keys() {
        return Err(Error::custom("duplicated keys in set"));
      }
    }

    Ok(set)
  }
}

impl<'a, K, KW, S, RB, B> TryFromRef<'a, RB, B, Packed<KW>, Groto> for IndexSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  K: TryFromRef<'a, RB, B, KW, Groto> + Eq + Hash + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, B, Groto>,
  S: BuildHasher + Default,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_ref(
    ctx: &'a Context,
    input: <Self as State<Ref<'a, RB, B, Packed<KW>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<Ref<'a, RB, B, Packed<KW>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut set = IndexSet::with_capacity_and_hasher(capacity_hint, S::default());

    for res in iter {
      match res {
        Ok((_, item)) => {
          let item = K::try_from_ref(ctx, item)?;
          if !set.insert(item) && ctx.err_on_duplicated_set_keys() {
            return Err(Error::custom("duplicated keys in set"));
          }
        }
        Err(e) => return Err(e),
      }
    }

    if set.len() != capacity_hint && ctx.err_on_length_mismatch() {
      return Err(Error::custom(format!(
        "expected {capacity_hint} elements in set, but got {} elements",
        set.len()
      )));
    }

    Ok(set)
  }
}

impl<'a, K, KW, S, RB, B> TryFromPartialRef<'a, RB, B, Packed<KW>, Groto> for IndexSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  K: TryFromPartialRef<'a, RB, B, KW, Groto> + Eq + Hash + 'a,
  K::Output: Sized + Decode1<'a, KW, RB, B, Groto>,
  S: BuildHasher + Default,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn try_from_partial_ref(
    ctx: &'a Context,
    input: <Self as State<PartialRef<'a, RB, B, Packed<KW>, Groto>>>::Output,
  ) -> Result<Self, Error>
  where
    Self: Sized,
    <Self as State<PartialRef<'a, RB, B, Packed<KW>, Groto>>>::Output: Sized,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto>,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let mut set = IndexSet::with_capacity_and_hasher(capacity_hint, S::default());

    for res in iter {
      match res {
        Ok((_, item)) => {
          let item = K::try_from_partial_ref(ctx, item)?;
          if !set.insert(item) && ctx.err_on_duplicated_set_keys() {
            return Err(Error::custom("duplicated keys in set"));
          }
        }
        Err(e) => return Err(e),
      }
    }

    if set.len() != capacity_hint && ctx.err_on_length_mismatch() {
      return Err(Error::custom(format!(
        "expected {capacity_hint} elements in set, but got {} elements",
        set.len()
      )));
    }

    Ok(set)
  }
}

impl<'a, K, KW, S, RB, B> PartialTryFromRef<'a, RB, B, Packed<KW>, Groto> for IndexSet<K, S>
where
  KW: WireFormat<Groto> + 'a,
  K: PartialTryFromRef<'a, RB, B, KW, Groto> + Eq + Hash + 'a,
  <K as State<PartialRef<'a, RB, B, KW, Groto>>>::Output:
    Sized + Decode1<'a, KW, RB, B, Groto> + Selectable<Groto, Selector = K::Selector>,
  <K as State<Partial<Groto>>>::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  S: BuildHasher + Default,
  RB: ReadBuf + 'a,
  B: UnknownBuffer<RB, Groto> + 'a,
{
  fn partial_try_from_ref(
    input: <Self as State<PartialRef<'a, RB, B, Packed<KW>, Groto>>>::Output,
    selector: &Self::Selector,
  ) -> Result<<Self as State<Partial<Groto>>>::Output, Error>
  where
    <Self as State<Partial<Groto>>>::Output: Sized,
    <Self as State<PartialRef<'a, RB, B, Packed<KW>, Groto>>>::Output: Sized,
  {
    let iter = input.iter();
    let capacity_hint = iter.capacity_hint();
    let Some(mut partial_set) =
      <DefaultPartialSetBuffer<_> as Buffer>::with_capacity(capacity_hint)
    else {
      return Err(Error::custom("failed to allocate partial set buffer"));
    };

    for res in iter {
      match res {
        Ok((_, item)) => {
          let item = K::partial_try_from_ref(item, selector)?;
          if <DefaultPartialSetBuffer<_> as Buffer>::push(&mut partial_set, item).is_some() {
            return Err(Error::custom("capacity exceeded for partial set buffer"));
          }
        }
        Err(e) => return Err(e),
      }
    }

    Ok(partial_set)
  }
}

impl<K, S> PartialIdentity<Groto> for IndexSet<K, S>
where
  K: PartialIdentity<Groto> + Ord,
  K::Output: Sized + Selectable<Groto, Selector = K::Selector>,
  S: BuildHasher + Default,
{
  fn partial_identity<'a>(
    input: &'a mut Self::Output,
    selector: &'a Self::Selector,
  ) -> &'a mut Self::Output {
    input.as_mut_slice().iter_mut().for_each(|item| {
      K::partial_identity(item, selector);
    });

    input
  }
}
