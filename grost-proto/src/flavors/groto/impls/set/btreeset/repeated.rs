use std::collections::BTreeSet;
use varing::decode_u32_varint;

use crate::{
  buffer::{Buffer, DefaultBuffer, ReadBuf, UnknownBuffer},
  convert::{
    Flattened, Inner, Partial, PartialIdentity, PartialRef, PartialTryFromRef, Ref, TryFromPartial,
    TryFromPartialRef, TryFromRef,
  },
  decode::Decode1,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultRepeatedWireFormat, DefaultSetWireFormat, Groto, Packed, Repeated, WireFormat,
    groto::{
      Context, Error, Identifier, PackedSetDecoder, RepeatedDecoder, RepeatedDecoderBuffer, Tag,
    },
  },
  selection::Selectable,
  state::State,
};

impl<K> DefaultRepeatedWireFormat<Groto> for BTreeSet<K> {
  type Format<KM, const TAG: u32>
    = Repeated<KM, TAG>
  where
    KM: WireFormat<Groto> + 'static;
}

impl<'a, K, KW, RB, B, const TAG: u32> State<PartialRef<'a, RB, B, Repeated<KW, TAG>, Groto>>
  for BTreeSet<K>
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: State<PartialRef<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
{
  type Output = RepeatedDecoderBuffer<'a, K::Output, KW, RB, B, TAG>;
}

impl<'a, K, KW, RB, B, const TAG: u32> State<Ref<'a, RB, B, Repeated<KW, TAG>, Groto>>
  for BTreeSet<K>
where
  KW: WireFormat<Groto> + 'a,
  Repeated<KW, TAG>: WireFormat<Groto> + 'a,
  K: State<Ref<'a, RB, B, KW, Groto>>,
  K::Output: Sized,
{
  type Output = RepeatedDecoderBuffer<'a, K::Output, KW, RB, B, TAG>;
}

impl<'a, K, KW, RB, B, const TAG: u32> Decode1<'a, Repeated<KW, TAG>, RB, B, Groto> for BTreeSet<K>
where
  KW: WireFormat<Groto> + 'a,
  K: Ord + Decode1<'a, KW, RB, B, Groto>,
{
  fn decode(ctx: &'a Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let mut this = BTreeSet::new();
    <Self as Decode1<'a, Repeated<KW, TAG>, RB, B, Groto>>::merge_decode(&mut this, ctx, src)
      .map(|size| (size, this))
  }

  fn merge_decode(&mut self, ctx: &'a Context, src: RB) -> Result<usize, Error>
  where
    Self: Sized + 'a,
    RB: ReadBuf + 'a,
    B: UnknownBuffer<RB, Groto> + 'a,
  {
    let identifier = Identifier::new(KW::WIRE_TYPE, Tag::new(TAG));
    let mut offset = 0;
    let buf = src.as_bytes();
    let buf_len = buf.len();
    if buf_len == 0 {
      return Err(Error::buffer_underflow());
    }

    // The following elements should be prefixed with the identifier.
    // | identifier | element | identifier | element | ...
    loop {
      if offset >= buf_len {
        break;
      }

      let (read, next_id) = Identifier::decode(&buf[offset..])?;

      // If the next identifier does not match the expected identifier, which means we have reached the end of the repeated elements.
      // We should stop decoding. We do not need to increment the offset here because we are not consuming the next identifier.
      if next_id != identifier {
        break;
      }

      // increment the offset by the size of the identifier
      offset += read;
      // consum the next element
      let (read, k) = K::decode(ctx, src.slice(offset..))?;
      offset += read;
      if !self.insert(k) && ctx.err_on_duplicated_set_keys() {
        return Err(Error::custom("duplicated keys in set"));
      }
    }

    Ok(offset)
  }
}
