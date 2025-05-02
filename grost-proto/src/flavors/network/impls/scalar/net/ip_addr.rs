use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::{
  decode::{Decode, DecodeOwned},
  default_wire_format,
  encode::Encode,
  flavors::{
    Network,
    network::{
      Context, DecodeError, EncodeError, Fixed32, Fixed128, LengthDelimited, Unknown, Varint,
    },
  },
  partial_encode_scalar, selectable_scalar,
};

macro_rules! ip_addr {
  ($addr:ident::$variant:ident($convert:ident)) => {
    default_wire_format!(Network: $addr as $variant);

    impl Encode<Network, $variant> for $addr {
      fn encode(
        &self,
        context: &Context,
        buf: &mut [u8],
      ) -> Result<usize, EncodeError> {
        <$convert as Encode<Network, $variant>>::encode(
          &self.to_bits(),
          context,
          buf,
        )
      }

      fn encoded_len(&self, _: &Context) -> usize {
        <$convert as Encode<Network, $variant>>::encoded_len(
          &self.to_bits(),
          &Context::default(),
        )
      }

      fn encoded_length_delimited_len(
        &self,
        context: &Context,
      ) -> usize {
        <Self as Encode<Network, $variant>>::encoded_len(self, context)
      }

      fn encode_length_delimited(
        &self,
        context: &Context,
        buf: &mut [u8],
      ) -> Result<usize, EncodeError> {
        <Self as Encode<Network, $variant>>::encode(self, context, buf)
      }
    }

    impl Encode<Network, Varint> for $addr {
      fn encode(
        &self,
        context: &Context,
        buf: &mut [u8],
      ) -> Result<usize, EncodeError> {
        <$convert as Encode<Network, Varint>>::encode(
          &self.to_bits(),
          context,
          buf,
        )
      }

      fn encoded_len(&self, ctx: &Context) -> usize {
        <$convert as Encode<Network, Varint>>::encoded_len(
          &self.to_bits(),
          ctx,
        )
      }

      fn encoded_length_delimited_len(
        &self,
        context: &Context,
      ) -> usize {
        <Self as Encode<Network, Varint>>::encoded_len(self, context)
      }

      fn encode_length_delimited(
        &self,
        context: &Context,
        buf: &mut [u8],
      ) -> Result<usize, EncodeError> {
        <Self as Encode<Network, Varint>>::encode(self, context, buf)
      }
    }

    impl<'de> Decode<'de, Network, $variant, Self> for $addr {
      fn decode<UB>(
        ctx: &Context,
        src: &'de [u8],
      ) -> Result<(usize, Self), DecodeError>
      where
        Self: Sized + 'de,
        UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
      {
        <$convert as Decode<'de, Network, $variant, $convert>>::decode::<UB>(ctx, src)
          .map(|(len, val)| (len, $addr::from_bits($convert::from_le(val))))
      }

      fn decode_length_delimited<UB>(
        context: &Context,
        src: &'de [u8],
      ) -> Result<(usize, Self), DecodeError>
      where
        Self: Sized + 'de,
        UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
      {
        <Self as Decode<'de, Network, $variant, Self>>::decode::<()>(context, src)
      }
    }

    impl<'de> Decode<'de, Network, Varint, Self> for $addr {
      fn decode<UB>(
        ctx: &Context,
        src: &'de [u8],
      ) -> Result<(usize, Self), DecodeError>
      where
        Self: Sized + 'de,
        UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
      {
        <$convert as Decode<'de, Network, Varint, $convert>>::decode::<UB>(ctx, src)
          .map(|(len, val)| (len, $addr::from_bits($convert::from_le(val))))
      }

      fn decode_length_delimited<UB>(
        context: &Context,
        src: &'de [u8],
      ) -> Result<(usize, Self), DecodeError>
      where
        Self: Sized + 'de,
        UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
      {
        <Self as Decode<'de, Network, Varint, Self>>::decode::<()>(context, src)
      }
    }

    $crate::decode_owned_scalar!(
      Network: $addr as $variant,
      $addr as Varint,
    );

    $crate::message!(Network: $addr as $variant);
  };
}

ip_addr!(Ipv4Addr::Fixed32(u32));
ip_addr!(Ipv6Addr::Fixed128(u128));
selectable_scalar!(Ipv4Addr, Ipv6Addr, IpAddr);
partial_encode_scalar!(Network: Ipv4Addr as Fixed32, Ipv4Addr as Varint, Ipv6Addr as Fixed128, Ipv6Addr as Varint, IpAddr as LengthDelimited);

const IPV4_LEN: usize = 4;
const IPV6_LEN: usize = 16;
const IPV4_TAG: u8 = 4;
const IPV6_TAG: u8 = 6;
const IPV4_ENCODED_LENGTH_DELIMITED_LEN_BYTES: &[u8] =
  varing::encode_u32_varint(IPV4_LEN as u32).as_slice();
const IPV6_ENCODED_LENGTH_DELIMITED_LEN_BYTES: &[u8] =
  varing::encode_u32_varint(IPV6_LEN as u32).as_slice();
const IPV4_ENCODED_LENGTH_DELIMITED_LEN: usize =
  IPV4_ENCODED_LENGTH_DELIMITED_LEN_BYTES.len() + IPV4_LEN;
const IPV6_ENCODED_LENGTH_DELIMITED_LEN: usize =
  IPV6_ENCODED_LENGTH_DELIMITED_LEN_BYTES.len() + IPV6_LEN;

default_wire_format!(Network: IpAddr as LengthDelimited);

impl Encode<Network, LengthDelimited> for IpAddr {
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
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
          <[< $variant:camel Addr>] as Encode<Network, $wt>>::encode(
            $ip,
            context,
            &mut buf[1..],
          ).map(|_| 1 + [< $variant:upper _LEN >])
        }
      }};
    }

    match self {
      Self::V4(ip) => encode_ip_variant!(IPV4::Fixed32(buf, ip)),
      Self::V6(ip) => encode_ip_variant!(IPV6::Fixed128(buf, ip)),
    }
  }

  fn encoded_len(&self, _: &Context) -> usize {
    1 + match self {
      Self::V4(_) => IPV4_LEN,
      Self::V6(_) => IPV6_LEN,
    }
  }

  fn encoded_length_delimited_len(&self, _: &Context) -> usize {
    match self {
      Self::V4(_) => IPV4_ENCODED_LENGTH_DELIMITED_LEN,
      Self::V6(_) => IPV6_ENCODED_LENGTH_DELIMITED_LEN,
    }
  }

  fn encode_length_delimited(&self, _: &Context, buf: &mut [u8]) -> Result<usize, EncodeError> {
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
  }
}

impl<'de> Decode<'de, Network, LengthDelimited, Self> for IpAddr {
  fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
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
  }

  fn decode_length_delimited<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
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
  }
}

impl DecodeOwned<Network, LengthDelimited, Self> for IpAddr {
  fn decode_owned<B, UB>(context: &Context, src: B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    Self::decode::<()>(context, src.as_bytes())
  }

  fn decode_length_delimited_owned<B, UB>(
    context: &Context,
    src: B,
  ) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: crate::buffer::BytesBuffer + 'static,
    UB: crate::buffer::Buffer<Unknown<B>> + 'static,
  {
    Self::decode_length_delimited::<()>(context, src.as_bytes())
  }
}

#[cfg(all(test, feature = "std"))]
mod tests {
  use super::*;

  quickcheck::quickcheck! {
    fn fuzzy_ipv4_round_trip(ip: Ipv4Addr) -> bool {
      let mut buf = [0u8; 128];
      let len = <Ipv4Addr as Encode<Network, Fixed32>>::encode_length_delimited(&ip, &Context::default(), &mut buf).unwrap();
      let encoded_len = <Ipv4Addr as Encode<Network, Fixed32>>::encoded_length_delimited_len(&ip, &Context::default());
      assert_eq!(len, encoded_len);

      let (len, decoded) = <Ipv4Addr as Decode<Network, Fixed32, Ipv4Addr>>::decode_length_delimited::<()>(&Context::default(), &buf).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      let len = <Ipv4Addr as Encode<Network, Varint>>::encode_length_delimited(&ip, &Context::default(), &mut buf).unwrap();
      let encoded_len = <Ipv4Addr as Encode<Network, Varint>>::encoded_length_delimited_len(&ip, &Context::default());
      assert_eq!(len, encoded_len);

      let (len, decoded) = <Ipv4Addr as Decode<'_, Network, Varint, Ipv4Addr>>::decode_length_delimited::<()>(&Context::default(), &buf).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      true
    }

    fn fuzzy_ipv6_round_trip(ip: Ipv6Addr) -> bool {
      let mut buf = [0u8; 128];
      let len = <Ipv6Addr as Encode<Network, Fixed128>>::encode_length_delimited(&ip, &Context::default(), &mut buf).unwrap();
      let encoded_len = <Ipv6Addr as Encode<Network, Fixed128>>::encoded_length_delimited_len(&ip, &Context::default());
      assert_eq!(len, encoded_len);

      let (len, decoded) = <Ipv6Addr as Decode<Network, Fixed128, Ipv6Addr>>::decode_length_delimited::<()>(&Context::default(), &buf).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      let len = <Ipv6Addr as Encode<Network, Varint>>::encode_length_delimited(&ip, &Context::default(), &mut buf).unwrap();
      let encoded_len = <Ipv6Addr as Encode<Network, Varint>>::encoded_length_delimited_len(&ip, &Context::default());
      assert_eq!(len, encoded_len);

      let (len, decoded) = <Ipv6Addr as Decode<'_, Network, Varint, Ipv6Addr>>::decode_length_delimited::<()>(&Context::default(), &buf).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      true
    }

    fn fuzzy_ip_round_trip(ip: IpAddr) -> bool {
      let mut buf = [0u8; 128];
      let len = ip.encode_length_delimited(&Context::default(), &mut buf).unwrap();
      let encoded_len = ip.encoded_length_delimited_len(&Context::default(), );
      assert_eq!(len, encoded_len);

      let (len, decoded) = IpAddr::decode_length_delimited::<()>(&Context::default(), &buf).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      let len = ip.encode(&Context::default(),  &mut buf).unwrap();
      let encoded_len = ip.encoded_len(&Context::default(), );
      assert_eq!(len, encoded_len);

      let (len, decoded) = IpAddr::decode::<()>(&Context::default(), &buf).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      true
    }
  }
}
