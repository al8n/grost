use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::{
  decode::{Decode, DecodeOwned}, encode::Encode, flavors::{
    network::{Context, DecodeError, EncodeError, Unknown, WireType}, Network
  }, partial_encode_primitives,
};

macro_rules! ip_addr {
  ($addr:ident::$variant:ident($convert:ident)) => {
    impl Encode<Network> for $addr {
      fn encode(
        &self,
        context: &Context,
        wire_type: WireType,
        buf: &mut [u8],
      ) -> Result<usize, EncodeError> {
        match wire_type {
          WireType::Varint => self.to_bits().to_le().encode(context, wire_type, buf),
          WireType::$variant => self
            .to_bits()
            .to_le()
            .encode(context, WireType::$variant, buf),
          wt => Err(EncodeError::unsupported_wire_type(
            core::any::type_name::<Self>(),
            wt,
          )),
        }
      }

      fn encoded_len(&self, context: &Context, wire_type: WireType) -> Result<usize, EncodeError> {
        match wire_type {
          WireType::Varint => self.to_bits().to_le().encoded_len(context, wire_type),
          WireType::$variant => Ok(::core::mem::size_of::<Self>()),
          wt => Err(EncodeError::unsupported_wire_type(
            core::any::type_name::<Self>(),
            wt,
          )),
        }
      }

      fn encoded_length_delimited_len(
        &self,
        context: &Context,
        wire_type: WireType,
      ) -> Result<usize, EncodeError> {
        match wire_type {
          WireType::Varint => self
            .to_bits()
            .to_le()
            .encoded_length_delimited_len(context, wire_type),
          WireType::$variant => Ok(::core::mem::size_of::<Self>()),
          wt => Err(EncodeError::unsupported_wire_type(
            core::any::type_name::<Self>(),
            wt,
          )),
        }
      }

      fn encode_length_delimited(
        &self,
        context: &Context,
        wire_type: WireType,
        buf: &mut [u8],
      ) -> Result<usize, EncodeError> {
        match wire_type {
          WireType::Varint => self
            .to_bits()
            .to_le()
            .encode_length_delimited(context, wire_type, buf),
          WireType::$variant => {
            self
              .to_bits()
              .to_le()
              .encode_length_delimited(context, WireType::$variant, buf)
          }
          wt => Err(EncodeError::unsupported_wire_type(
            core::any::type_name::<Self>(),
            wt,
          )),
        }
      }
    }

    impl<'de> Decode<'de, Network, Self> for $addr {
      fn decode<UB>(
        ctx: &Context,
        wire_type: WireType,
        src: &'de [u8],
      ) -> Result<(usize, Self), DecodeError>
      where
        Self: Sized + 'de,
        UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
      {
        match wire_type {
          WireType::Varint => {
            <$convert as Decode<'de, Network, $convert>>::decode::<UB>(ctx, wire_type, src)
              .map(|(len, val)| (len, $addr::from_bits($convert::from_le(val))))
          }
          WireType::$variant => {
            <$convert as Decode<'de, Network, $convert>>::decode::<UB>(ctx, wire_type, src)
              .map(|(len, val)| (len, $addr::from_bits($convert::from_le(val))))
          }
          wt => Err(DecodeError::unsupported_wire_type(
            core::any::type_name::<Self>(),
            wt,
          )),
        }
      }

      fn decode_length_delimited<UB>(
        context: &Context,
        wire_type: WireType,
        src: &'de [u8],
      ) -> Result<(usize, Self), DecodeError>
      where
        Self: Sized + 'de,
        UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
      {
        Self::decode::<()>(context, wire_type, src)
      }
    }

    impl DecodeOwned<Network, Self> for $addr {
      fn decode_owned<B, UB>(
        context: &Context,
        wire_type: WireType,
        src: B,
      ) -> Result<(usize, Self), DecodeError>
      where
        Self: Sized + 'static,
        B: crate::buffer::BytesBuffer + 'static,
        UB: crate::buffer::Buffer<Unknown<B>> + 'static,
      {
        Self::decode::<()>(context, wire_type, src.as_bytes())
      }

      fn decode_length_delimited_owned<B, UB>(
        context: &Context,
        wire_type: WireType,
        src: B,
      ) -> Result<(usize, Self), DecodeError>
      where
        Self: Sized + 'static,
        B: crate::buffer::BytesBuffer + 'static,
        UB: crate::buffer::Buffer<Unknown<B>> + 'static,
      {
        Self::decode_length_delimited::<()>(context, wire_type, src.as_bytes())
      }
    }

    $crate::message!(Network: $addr);
  };
}

ip_addr!(Ipv4Addr::Fixed32(u32));
ip_addr!(Ipv6Addr::Fixed128(u128));
partial_encode_primitives!(Network: Ipv4Addr, Ipv6Addr, IpAddr);

const IPV4_LEN: usize = 4;
const IPV6_LEN: usize = 16;
const IPV4_TAG: u8 = 4;
const IPV6_TAG: u8 = 6;
const IPV4_ENCODED_LENGTH_DELIMITED_LEN_BYTES: &[u8] = varing::encode_u32_varint(IPV4_LEN as u32).as_slice();
const IPV6_ENCODED_LENGTH_DELIMITED_LEN_BYTES: &[u8] = varing::encode_u32_varint(IPV6_LEN as u32).as_slice();
const IPV4_ENCODED_LENGTH_DELIMITED_LEN: usize = IPV4_ENCODED_LENGTH_DELIMITED_LEN_BYTES.len() + IPV4_LEN;
const IPV6_ENCODED_LENGTH_DELIMITED_LEN: usize = IPV6_ENCODED_LENGTH_DELIMITED_LEN_BYTES.len() + IPV6_LEN;


impl Encode<Network> for IpAddr {
  fn encode(
    &self,
    context: &Context,
    wire_type: WireType,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    if let WireType::LengthDelimited = wire_type {
      macro_rules! encode_ip_variant {
        ($variant:ident::$wt:ident($buf:ident, $ip:ident)) => {{
          paste::paste! {
            if buf.len() < [< $variant:upper _LEN >] + 1 {
              return Err(EncodeError::insufficient_buffer(
                [< $variant:upper _LEN >] + 1,
                buf.len(),
              ));
            }

            buf[0] = [< $variant:upper _TAG >];
            $ip.to_bits().to_le().encode(context, WireType::$wt, &mut buf[1..])
              .map(|_| 1 + [< $variant:upper _LEN >])
          }
        }};
      }

      match self {
        Self::V4(ip) => encode_ip_variant!(IPV4::Fixed32(buf, ip)),
        Self::V6(ip) => encode_ip_variant!(IPV6::Fixed128(buf, ip)),
      }
    } else {
      Err(EncodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wire_type,
      ))
    }
  }

  fn encoded_len(&self, _: &Context, wire_type: WireType) -> Result<usize, EncodeError> {
    if let WireType::LengthDelimited = wire_type {
      Ok(1 + match self {
        Self::V4(_) => IPV4_LEN,
        Self::V6(_) => IPV6_LEN,
      })
    } else {
      Err(EncodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wire_type,
      ))
    }
  }

  fn encoded_length_delimited_len(
    &self,
    _: &Context,
    wire_type: WireType,
  ) -> Result<usize, EncodeError> {
    if let WireType::LengthDelimited = wire_type {
      Ok(match self {
        Self::V4(_) => IPV4_ENCODED_LENGTH_DELIMITED_LEN,
        Self::V6(_) => IPV6_ENCODED_LENGTH_DELIMITED_LEN,
      })
    } else {
      Err(EncodeError::unsupported_wire_type(core::any::type_name::<Self>(), wire_type))
    }
  }

  fn encode_length_delimited(
    &self,
    _: &Context,
    wire_type: WireType,
    buf: &mut [u8],
  ) -> Result<usize, EncodeError> {
    if let WireType::LengthDelimited = wire_type {
      macro_rules! encode_ip_variant {
        ($variant:ident($buf:ident, $ip:ident)) => {{
          paste::paste! {
            if buf.len() < [< $variant:upper _ENCODED_LENGTH_DELIMITED_LEN >] {
              return Err(EncodeError::insufficient_buffer(
                [< $variant:upper _ENCODED_LENGTH_DELIMITED_LEN >],
                buf.len(),
              ));
            }
            const LEN_END: usize = [< $variant:upper _ENCODED_LENGTH_DELIMITED_LEN_BYTES >].len();
            buf[..LEN_END]
              .copy_from_slice([< $variant:upper _ENCODED_LENGTH_DELIMITED_LEN_BYTES >]);
            buf[LEN_END..LEN_END + [< $variant _LEN >]].copy_from_slice(&$ip.to_bits().to_le_bytes());
            Ok([< $variant:upper _ENCODED_LENGTH_DELIMITED_LEN >])
          }
        }};
      }

      match self {
        Self::V4(ip) => encode_ip_variant!(IPV4(buf, ip)),
        Self::V6(ip) => encode_ip_variant!(IPV6(buf, ip)),
      }
    } else {
      Err(EncodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wire_type,
      ))
    }
  }
}

impl<'de> Decode<'de, Network, Self> for IpAddr {
  fn decode<UB>(
    _: &Context,
    wire_type: WireType,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
    if let WireType::LengthDelimited = wire_type {
      macro_rules! decode_ip_variant {
        ($repr:ident($variant:literal)) => {{
          paste::paste! {
            if src.len() < [< IPV $variant _LEN >] + 1 {
              return Err(DecodeError::buffer_underflow());
            }
  
            let ip = [< Ipv $variant Addr >]::from_bits($repr::from_le_bytes(
              src[1..[< IPV $variant _LEN >] + 1].try_into().unwrap(),
            ));
  
            ([< IPV $variant _LEN >] + 1, IpAddr::from(ip))
          }
        }};
      }

      if src.is_empty() {
        return Err(DecodeError::buffer_underflow());
      }

      let tag = src[0];
      Ok(match tag {
        IPV4_TAG => decode_ip_variant!(u32(4)),
        IPV6_TAG => decode_ip_variant!(u128(6)),
        _ => return Err(DecodeError::custom("unknown ip tag")),
      })
    } else {
      Err(DecodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wire_type,
      ))
    }
  }

  fn decode_length_delimited<UB>(
    _: &Context,
    wire_type: WireType,
    src: &'de [u8],
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
    if let WireType::LengthDelimited = wire_type {
      macro_rules! decode_ip_variant {
        ($variant:ident($repr:ident, $read:ident)) => {{
          paste::paste! {
            if src.len() < [< $variant _ENCODED_LENGTH_DELIMITED_LEN >] {
              return Err(DecodeError::buffer_underflow());
            }
            let ip = [< $variant:camel Addr >]::from_bits($repr::from_le_bytes(
              src[$read..$read + [< $variant _LEN >]].try_into().unwrap(),
            ));
            Ok(([< $variant _ENCODED_LENGTH_DELIMITED_LEN >], IpAddr::from(ip)))
          }
        }};
      }

      let (read, len) = varing::decode_u32_varint(src)?;
      match len as usize {
        IPV4_LEN => decode_ip_variant!(IPV4(u32, read)), 
        IPV6_LEN => decode_ip_variant!(IPV6(u128, read)),
        _ => Err(DecodeError::custom("unknown ip tag")),
      }
    } else {
      Err(DecodeError::unsupported_wire_type(
        core::any::type_name::<Self>(),
        wire_type,
      ))
    }
  }
}

impl DecodeOwned<Network, Self> for IpAddr {
  fn decode_owned<B, UB>(
    context: &Context,
    wire_type: WireType,
    src: B,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    Self::decode::<()>(context, wire_type, src.as_bytes())
  }

  fn decode_length_delimited_owned<B, UB>(
    context: &Context,
    wire_type: WireType,
    src: B,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    Self::decode_length_delimited::<()>(context, wire_type, src.as_bytes())
  }
}

#[cfg(all(test, feature = "std"))]
mod tests {
  use super::*;

  quickcheck::quickcheck! {
    fn fuzzy_ipv4_round_trip(ip: Ipv4Addr) -> bool {
      let mut buf = [0u8; 128];
      let len = ip.encode_length_delimited(&Context::default(), WireType::Fixed32, &mut buf).unwrap();
      let encoded_len = ip.encoded_length_delimited_len(&Context::default(), WireType::Fixed32).unwrap();
      assert_eq!(len, encoded_len);

      let (len, decoded) = Ipv4Addr::decode_length_delimited::<()>(&Context::default(), WireType::Fixed32, &buf).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      let len = ip.encode_length_delimited(&Context::default(), WireType::Varint, &mut buf).unwrap();
      let encoded_len = ip.encoded_length_delimited_len(&Context::default(), WireType::Varint).unwrap();
      assert_eq!(len, encoded_len);

      let (len, decoded) = Ipv4Addr::decode_length_delimited::<()>(&Context::default(), WireType::Varint, &buf).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      true
    }

    fn fuzzy_ipv6_round_trip(ip: Ipv6Addr) -> bool {
      let mut buf = [0u8; 128];
      let len = ip.encode_length_delimited(&Context::default(), WireType::Fixed128, &mut buf).unwrap();
      let encoded_len = ip.encoded_length_delimited_len(&Context::default(), WireType::Fixed128).unwrap();
      assert_eq!(len, encoded_len);

      let (len, decoded) = Ipv6Addr::decode_length_delimited::<()>(&Context::default(), WireType::Fixed128, &buf).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      let len = ip.encode_length_delimited(&Context::default(), WireType::Varint, &mut buf).unwrap();
      let encoded_len = ip.encoded_length_delimited_len(&Context::default(), WireType::Varint).unwrap();
      assert_eq!(len, encoded_len);

      let (len, decoded) = Ipv6Addr::decode_length_delimited::<()>(&Context::default(), WireType::Varint, &buf).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      true
    }

    fn fuzzy_ip_round_trip(ip: IpAddr) -> bool {
      let mut buf = [0u8; 128];
      let len = ip.encode_length_delimited(&Context::default(), WireType::LengthDelimited, &mut buf).unwrap();
      let encoded_len = ip.encoded_length_delimited_len(&Context::default(), WireType::LengthDelimited).unwrap();
      assert_eq!(len, encoded_len);

      let (len, decoded) = IpAddr::decode_length_delimited::<()>(&Context::default(), WireType::LengthDelimited, &buf).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      let len = ip.encode(&Context::default(), WireType::LengthDelimited, &mut buf).unwrap();
      let encoded_len = ip.encoded_len(&Context::default(), WireType::LengthDelimited).unwrap();
      assert_eq!(len, encoded_len);

      let (len, decoded) = IpAddr::decode::<()>(&Context::default(), WireType::LengthDelimited, &buf).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      true
    }
  }

  
}