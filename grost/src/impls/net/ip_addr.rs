use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use varing::utils::Buffer;

use crate::{
  Decode, DecodeError, DecodeOwned, Encode, EncodeError, Identifier, Tag, Wirable, WireType,
};

type U32VarintBuffer = Buffer<{ <u32 as varing::Varint>::MAX_ENCODED_LEN + 1 }>;

const IPV4_LEN: usize = 4;
const IPV6_LEN: usize = 16;
const IPV4_TAG: Tag = Tag::new(4);
const IPV6_TAG: Tag = Tag::new(6);
const IPV4_MERGED: Identifier = Identifier::new(WireType::Fixed32, IPV4_TAG);
const IPV6_MERGED: Identifier = Identifier::new(WireType::Fixed128, IPV6_TAG);
const IPV4_MERGED_ENCODED_LEN: usize = IPV4_MERGED.encoded_len();
const IPV6_MERGED_ENCODED_LEN: usize = IPV6_MERGED.encoded_len();
const IPV4_MERGED_ENCODED: U32VarintBuffer = IPV4_MERGED.encode();
const IPV6_MERGED_ENCODED: U32VarintBuffer = IPV6_MERGED.encode();

message!(IpAddr, Ipv4Addr, Ipv6Addr,);

macro_rules! impl_codec {
  ($ty:ident::$wire_ty:ident::$size:ident::$int:ident) => {
    impl Wirable for $ty {
      const WIRE_TYPE: WireType = WireType::$wire_ty;
    }

    impl Encode for $ty {
      fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
        let buf_len = buf.len();
        if buf_len < $size {
          return Err(EncodeError::insufficient_buffer($size, buf_len));
        }

        buf[..$size].copy_from_slice(&self.to_bits().to_be_bytes());
        Ok($size)
      }

      fn encoded_len(&self) -> usize {
        $size
      }
    }

    impl<'de> Decode<'de> for $ty {
      fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
      where
        Self: Sized + 'de,
        B: crate::UnknownRefBuffer<'de>,
      {
        if src.len() < $size {
          return Err(DecodeError::buffer_underflow());
        }

        let val = $int::from_be_bytes(src[..$size].try_into().unwrap());
        Ok(($size, $ty::from(val)))
      }
    }

    impl DecodeOwned for $ty {
      #[cfg(any(feature = "std", feature = "alloc"))]
      fn decode_from_bytes<U>(src: bytes_1::Bytes, _: &mut U) -> Result<(usize, Self), DecodeError>
      where
        Self: Sized + 'static,
        U: crate::UnknownBuffer<bytes_1::Bytes>,
      {
        if src.len() < $size {
          return Err(DecodeError::buffer_underflow());
        }

        let val = $int::from_be_bytes(src[..$size].try_into().unwrap());
        Ok(($size, $ty::from(val)))
      }
    }
  };
}

impl_codec!(Ipv6Addr::Fixed128::IPV6_LEN::u128);
impl_codec!(Ipv4Addr::Fixed32::IPV4_LEN::u32);

impl Wirable for IpAddr {}

impl Encode for IpAddr {
  fn encode(&self, buf: &mut [u8]) -> Result<usize, EncodeError> {
    macro_rules! encode_ip_variant {
      ($variant:ident($buf:ident, $ip:ident)) => {{
        paste::paste! {
          let buf_len = $buf.len();
          let total = [< $variant _MERGED_ENCODED_LEN >] + [< $variant _LEN >];
          if buf_len < total {
            return Err(EncodeError::insufficient_buffer(total, buf_len));
          }

          $buf[.. [< $variant _MERGED_ENCODED_LEN >]].copy_from_slice(&[< $variant _MERGED_ENCODED >]);
          $buf[[< $variant _MERGED_ENCODED_LEN >]..total].copy_from_slice(&$ip.to_bits().to_be_bytes());
          Ok(total)
        }
      }};
    }

    match self {
      Self::V4(ip) => encode_ip_variant!(IPV4(buf, ip)),
      Self::V6(ip) => encode_ip_variant!(IPV6(buf, ip)),
    }
  }

  fn encoded_len(&self) -> usize {
    match self {
      Self::V4(_) => IPV4_MERGED_ENCODED_LEN + IPV4_LEN,
      Self::V6(_) => IPV6_MERGED_ENCODED_LEN + IPV6_LEN,
    }
  }
}

macro_rules! decode_ip {
  ($buf:expr) => {{
    let (offset, merged) = Identifier::decode($buf)?;

    match merged {
      IPV4_MERGED => {
        if $buf.len() < offset + IPV4_LEN {
          return Err(DecodeError::buffer_underflow());
        }
        let ip = Ipv4Addr::from(u32::from_be_bytes(
          $buf[offset..offset + IPV4_LEN].try_into().unwrap(),
        ));
        Ok((offset + IPV4_LEN, IpAddr::V4(ip)))
      }
      IPV6_MERGED => {
        if $buf.len() < offset + IPV6_LEN {
          return Err(DecodeError::buffer_underflow());
        }
        let ip = Ipv6Addr::from(u128::from_be_bytes(
          $buf[offset..offset + IPV6_LEN].try_into().unwrap(),
        ));
        Ok((offset + IPV6_LEN, IpAddr::V6(ip)))
      }
      val => Err(DecodeError::unknown_identifier("IpAddr", val)),
    }
  }};
}

impl<'de> Decode<'de> for IpAddr {
  fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    B: crate::UnknownRefBuffer<'de>,
  {
    decode_ip!(src)
  }
}

impl DecodeOwned for IpAddr {
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn decode_from_bytes<U>(src: bytes_1::Bytes, _: &mut U) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    U: crate::UnknownBuffer<bytes_1::Bytes>,
  {
    decode_ip!(&src)
  }
}

partial_encode_primitives!(Ipv4Addr, Ipv6Addr, IpAddr);
