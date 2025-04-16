use core::net::IpAddr;

use hostaddr_0_1::{Domain, Host};

use crate::{
  Decode, DecodeError, DecodeOwned, Encode, EncodeError, Identifier, IntoTarget, Message,
  PartialEncode, Tag, TypeRef, Wirable, WireType,
};

impl<S> Wirable for Host<S> {}

const IPV4_LEN: usize = 4;
const IPV6_LEN: usize = 16;

const IPV4_TAG: Tag = Tag::new(4);
const IPV6_TAG: Tag = Tag::new(6);
const DOMAIN_TAG: Tag = Tag::new(1);

const IPV4_MERGED: Identifier = Identifier::new(WireType::Fixed32, IPV4_TAG);
const IPV6_MERGED: Identifier = Identifier::new(WireType::Fixed128, IPV6_TAG);
const DOMAIN_MERGED: Identifier = Identifier::new(WireType::LengthDelimited, DOMAIN_TAG);

const IPV4_MERGED_ENCODED_LEN: usize = IPV4_MERGED.encoded_len();
const IPV6_MERGED_ENCODED_LEN: usize = IPV6_MERGED.encoded_len();
const DOMAIN_MERGED_ENCODED_LEN: usize = DOMAIN_MERGED.encoded_len();

const IPV4_MERGED_BUFFER: &[u8] = IPV4_MERGED.encode().as_slice();
const IPV6_MERGED_BUFFER: &[u8] = IPV6_MERGED.encode().as_slice();
const DOMAIN_MERGED_BUFFER: &[u8] = DOMAIN_MERGED.encode().as_slice();

const IPV4_ENCODED_LEN: usize = IPV4_MERGED_ENCODED_LEN + 4;
const IPV6_ENCODED_LEN: usize = IPV6_MERGED_ENCODED_LEN + 16;

impl<S> Encode for Host<S>
where
  S: Encode,
{
  fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
    macro_rules! encode {
      ($ip:ident($kind:literal)) => {{
        paste::paste! {
          let buf_len = buf.len();
          if buf_len < [< IPV $kind _ENCODED_LEN >] {
            return Err(EncodeError::insufficient_buffer([< IPV $kind _ENCODED_LEN >], buf_len));
          }

          buf[..[<IPV $kind _MERGED_ENCODED_LEN >]].copy_from_slice([< IPV $kind _MERGED_BUFFER >]);
          buf[[<IPV $kind _MERGED_ENCODED_LEN >]..][.. [< IPV $kind _LEN >] ].copy_from_slice(&$ip.to_bits().to_le_bytes());
          Ok([< IPV $kind _ENCODED_LEN >])
        }
      }};
    }

    match self {
      Self::Ip(IpAddr::V4(ip)) => encode!(ip(4)),
      Self::Ip(IpAddr::V6(ip)) => encode!(ip(6)),
      Self::Domain(d) => {
        let buf_len = buf.len();
        if buf_len < DOMAIN_MERGED_ENCODED_LEN {
          return Err(EncodeError::insufficient_buffer(
            DOMAIN_MERGED_ENCODED_LEN + d.encoded_len(),
            buf_len,
          ));
        }

        buf[..DOMAIN_MERGED_ENCODED_LEN].copy_from_slice(DOMAIN_MERGED_BUFFER);
        let mut offset = DOMAIN_MERGED_ENCODED_LEN;
        offset += d
          .encode(&mut buf[offset..])
          .map_err(|e| e.update(DOMAIN_MERGED_ENCODED_LEN + d.encoded_len(), buf_len))?;
        Ok(offset)
      }
    }
  }

  fn encoded_len(&self) -> usize {
    match self {
      Self::Ip(IpAddr::V4(_)) => IPV4_ENCODED_LEN,
      Self::Ip(IpAddr::V6(_)) => IPV6_ENCODED_LEN,
      Self::Domain(d) => DOMAIN_MERGED_ENCODED_LEN + d.encoded_len(),
    }
  }
}

impl<S> PartialEncode for Host<S>
where
  S: Encode,
{
  type Selection = ();

  #[inline]
  fn partial_encode(
    &self,
    buf: &mut [u8],
    _: &Self::Selection,
  ) -> Result<usize, crate::EncodeError> {
    self.encode(buf)
  }

  #[inline]
  fn partial_encoded_len(&self, _: &Self::Selection) -> usize {
    self.encoded_len()
  }
}

macro_rules! decode {
  ($ty:ident($src:ident, $kind:literal)) => {{
    paste::paste! {
      if $src.len() < [< IPV $kind _MERGED_ENCODED_LEN >] {
        return Err(DecodeError::buffer_underflow());
      }
      let ip = <$ty>::from_le_bytes($src[ [< IPV $kind _MERGED_ENCODED_LEN >]..][.. [< IPV $kind _LEN >]].try_into().unwrap());

      Ok(([< IPV $kind _ENCODED_LEN >], Host::Ip(IpAddr::[< V $kind >](ip.into()))))
    }
  }};
}

impl<'de, S> Decode<'de> for Host<S>
where
  Domain<S>: Decode<'de>,
{
  fn decode<B>(src: &'de [u8], ub: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: crate::UnknownRefBuffer<'de>,
  {
    let buf_len = src.len();
    if buf_len == 0 {
      return Err(DecodeError::buffer_underflow());
    }

    let (read, id) = Identifier::decode(src)?;
    let src = &src[read..];
    match id {
      IPV4_MERGED => decode!(u32(src, 4)),
      IPV6_MERGED => decode!(u128(src, 6)),
      DOMAIN_MERGED => {
        let (offset, domain) = Domain::<S>::decode(&src[DOMAIN_MERGED_ENCODED_LEN..], ub)?;
        Ok((
          DOMAIN_MERGED_ENCODED_LEN + offset,
          Host::Domain(domain.into_inner()),
        ))
      }
      _ => Err(DecodeError::custom("unknown identifier of Host")),
    }
  }
}

impl<S> DecodeOwned for Host<S>
where
  Domain<S>: DecodeOwned,
{
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn decode_from_bytes<U>(
    mut src: crate::bytes::Bytes,
    ub: &mut U,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    U: crate::UnknownBuffer<crate::bytes::Bytes>,
  {
    use bytes_1::Buf;

    let buf_len = src.len();
    if buf_len == 0 {
      return Err(DecodeError::buffer_underflow());
    }

    let (read, id) = Identifier::decode(&src)?;
    let buf = &src[read..];
    match id {
      IPV4_MERGED => decode!(u32(buf, 4)),
      IPV6_MERGED => decode!(u128(buf, 6)),
      DOMAIN_MERGED => {
        src.advance(read);
        let (offset, domain) =
          Domain::<S>::decode_from_bytes(src.split_to(DOMAIN_MERGED_ENCODED_LEN), ub)?;
        Ok((
          DOMAIN_MERGED_ENCODED_LEN + offset,
          Host::Domain(domain.into_inner()),
        ))
      }
      _ => Err(DecodeError::custom("unknown identifier of Host")),
    }
  }
}

#[cfg(feature = "bytes_1")]
const _: () = {
  use bytes_1::Bytes;

  conversion!(@clone Host<Bytes>);

  impl IntoTarget<Host<Bytes>> for Host<&[u8]> {
    fn into_target(self) -> Result<Host<Bytes>, DecodeError> {
      match self {
        Host::Ip(ip) => Ok(Host::Ip(ip)),
        Host::Domain(d) => Ok(Host::Domain(Bytes::copy_from_slice(d))),
      }
    }
  }

  impl TypeRef<Host<Bytes>> for Host<&[u8]> {
    fn to(&self) -> Result<Host<Bytes>, DecodeError> {
      match self {
        Host::Ip(ip) => Ok(Host::Ip(*ip)),
        Host::Domain(d) => Ok(Host::Domain(Bytes::copy_from_slice(d))),
      }
    }
  }

  impl Message for Host<Bytes> {
    type Encoded<'a>
      = Host<&'a [u8]>
    where
      Self: Sized + 'a;

    type Borrowed<'a>
      = &'a Self
    where
      Self: 'a;

    type EncodedOwned
      = Host<Bytes>
    where
      Self: Sized + 'static;
  }
};
