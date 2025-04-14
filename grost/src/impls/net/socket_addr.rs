use core::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use varing::utils::Buffer;

use crate::{
  Decode, DecodeError, DecodeOwned, Encode, EncodeError, Tag, Wirable, WireType, Identifier,
};

type U32VarintBuffer = Buffer<{ <u32 as varing::Varint>::MAX_ENCODED_LEN + 1 }>;

const PORT_LEN: usize = 2;
const V4_LEN: usize = 4;
const V6_LEN: usize = 16;
const V4_TAG: Tag = Tag::new(4);
const V6_TAG: Tag = Tag::new(6);
const V4_MERGED: Identifier = Identifier::new(WireType::Fixed32, V4_TAG);
const V6_MERGED: Identifier = Identifier::new(WireType::Fixed128, V6_TAG);
const V4_MERGED_ENCODED_LEN: usize = V4_MERGED.encoded_len();
const V6_MERGED_ENCODED_LEN: usize = V6_MERGED.encoded_len();
const V4_MERGED_ENCODED: U32VarintBuffer = V4_MERGED.encode();
const V6_MERGED_ENCODED: U32VarintBuffer = V6_MERGED.encode();

message!(SocketAddr, SocketAddrV4, SocketAddrV6,);

macro_rules! impl_codec {
  ($variant:ident($bits:ident $(, $($others:literal),+$(,)?)?)) => {
    paste::paste! {
      impl Wirable for [< SocketAddr $variant >] {}

      impl Encode for [< SocketAddr $variant >] {
        fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
          Helper::[< from_ip $variant:snake >](*self.ip(), self.port()).encode(buf)
        }

        fn encoded_len(&self,) -> usize {
          Helper::[< from_ip $variant:snake >](*self.ip(), self.port()).encoded_len()
        }
      }

      impl<'de> Decode<'de> for [< SocketAddr $variant >] {
        fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
        where
          Self: Sized + 'de,
          B: crate::UnknownRefBuffer<'de>,
        {
          let (len, helper) = Helper::<[< $variant _LEN >]>::decode(src)?;
          Ok((
            len,
            Self::new(
              [< Ip $variant:snake Addr >]::from_bits($bits::from_be_bytes(helper.ip)),
              helper.port,
              $($($others),+)?
            ),
          ))
        }
      }

      impl DecodeOwned for [< SocketAddr $variant >] {
        #[cfg(any(feature = "std", feature = "alloc"))]
        fn decode_from_bytes<U>(src: bytes_1::Bytes, _: &mut U) -> Result<(usize, Self), DecodeError>
        where
          Self: Sized + 'static,
          U: crate::UnknownBuffer<bytes_1::Bytes>,
        {
          let (len, helper) = Helper::<[< $variant _LEN >]>::decode(src.as_ref())?;
          Ok((
            len,
            Self::new(
              [< Ip $variant:snake Addr >]::from_bits($bits::from_be_bytes(helper.ip)),
              helper.port,
              $($($others),+)?
            ),
          ))
        }
      }
    }
  };
}

impl_codec!(V4(u32));
impl_codec!(V6(u128, 0, 0));

partial_encode_primitives!(SocketAddr, SocketAddrV4, SocketAddrV6);

struct Helper<const N: usize> {
  ip: [u8; N],
  port: u16,
}

impl Helper<4> {
  const fn from_ipv4(ip: Ipv4Addr, port: u16) -> Self {
    Self {
      ip: ip.to_bits().to_be_bytes(),
      port: port.to_be(),
    }
  }
}

impl Helper<16> {
  const fn from_ipv6(ip: Ipv6Addr, port: u16) -> Self {
    Self {
      ip: ip.to_bits().to_be_bytes(),
      port: port.to_be(),
    }
  }
}

impl<const N: usize> Helper<N> {
  const V4_ENCODED_LEN: usize = V4_LEN + PORT_LEN;
  const V6_ENCODED_LEN: usize = V6_LEN + PORT_LEN;
}

impl<const N: usize> Wirable for Helper<N> {}

impl<const N: usize> Helper<N> {
  fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
    macro_rules! encode_variant {
      ($variant:ident($buf:ident)) => {{
        paste::paste! {
          let buf_len = $buf.len();
          if buf_len < Self::[< $variant _ENCODED_LEN >] {
            return Err(EncodeError::insufficient_buffer(Self::[< $variant _ENCODED_LEN >], buf_len));
          }

          let mut offset = 0;
          buf[offset..offset + [< $variant _LEN >]].copy_from_slice(&self.ip);
          offset += [< $variant _LEN >];
          buf[offset..offset + PORT_LEN].copy_from_slice(&self.port.to_be_bytes());
          Ok(Self::[< $variant _ENCODED_LEN >])
        }
      }};
    }

    match N {
      V4_LEN => {
        encode_variant!(V4(buf))
      }
      V6_LEN => {
        encode_variant!(V6(buf))
      }
      _ => unreachable!("Ip address family has new member! Tell me the year!"),
    }
  }

  fn encoded_len(&self) -> usize {
    match N {
      V4_LEN => Self::V4_ENCODED_LEN,
      V6_LEN => Self::V6_ENCODED_LEN,
      _ => unreachable!("Ip address family has new member! Tell me the year!"),
    }
  }
}

impl<'de, const N: usize> Helper<N> {
  fn decode(src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
  {
    macro_rules! decode_variant {
      ($variant:ident($buf:ident)) => {{
        paste::paste! {
          if src.len() < Self::[< $variant _ENCODED_LEN >] {
            return Err(DecodeError::buffer_underflow());
          }

          let port = u16::from_be_bytes(src[[< $variant _LEN>]..[< $variant _LEN>] + PORT_LEN].try_into().unwrap());
          Ok((Self::[< $variant _ENCODED_LEN >], Helper {
            ip: src[..[< $variant _LEN>]].try_into().unwrap(),
            port,
          }))
        }
      }};
    }
    match N {
      V4_LEN => decode_variant!(V4(src)),
      V6_LEN => decode_variant!(V6(src)),
      _ => unreachable!("Ip address family has new member! Tell me the year!"),
    }
  }
}

impl Wirable for SocketAddr {}

impl Encode for SocketAddr {
  fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
    let buf_len = buf.len();
    match self {
      Self::V4(addr) => {
        let required = V4_MERGED_ENCODED_LEN + Helper::<V4_LEN>::V4_ENCODED_LEN;
        if required > buf_len {
          return Err(EncodeError::insufficient_buffer(required, buf_len));
        }
        buf[..V4_MERGED_ENCODED_LEN].copy_from_slice(&V4_MERGED_ENCODED);
        addr.encode(buf)
      }
      Self::V6(addr) => {
        let required = V6_MERGED_ENCODED_LEN + Helper::<V6_LEN>::V6_ENCODED_LEN;
        if required > buf_len {
          return Err(EncodeError::insufficient_buffer(required, buf_len));
        }
        buf[..V6_MERGED_ENCODED_LEN].copy_from_slice(&V6_MERGED_ENCODED);
        addr.encode(buf)
      }
    }
  }

  fn encoded_len(&self) -> usize {
    match self {
      Self::V4(_) => V4_MERGED_ENCODED_LEN + Helper::<V4_LEN>::V4_ENCODED_LEN,
      Self::V6(_) => V6_MERGED_ENCODED_LEN + Helper::<V6_LEN>::V6_ENCODED_LEN,
    }
  }
}

impl<'de> Decode<'de> for SocketAddr {
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: crate::UnknownRefBuffer<'de>,
  {
    decode(src)
  }
}

impl DecodeOwned for SocketAddr {
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn decode_from_bytes<U>(src: bytes_1::Bytes, _: &mut U) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    U: crate::UnknownBuffer<bytes_1::Bytes>,
  {
    decode(src.as_ref())
  }
}

fn decode(src: &[u8]) -> Result<(usize, SocketAddr), DecodeError> {
  let (len, merged) = Identifier::decode(src)?;
  match merged {
    V4_MERGED => {
      let (offset, addr) = Helper::<V4_LEN>::decode(&src[len..])?;
      Ok((
        len + offset,
        SocketAddr::V4(SocketAddrV4::new(
          u32::from_be_bytes(addr.ip).into(),
          addr.port,
        )),
      ))
    }
    V6_MERGED => {
      let (offset, addr) = Helper::<V6_LEN>::decode(&src[len..])?;
      Ok((
        len + offset,
        SocketAddr::V6(SocketAddrV6::new(
          u128::from_be_bytes(addr.ip).into(),
          addr.port,
          0,
          0,
        )),
      ))
    }
    val => {
      Err(DecodeError::unknown_identifier("SocketAddr", val))
    }
  }
}
