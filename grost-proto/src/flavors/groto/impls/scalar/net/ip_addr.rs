use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::{
  buffer::{ReadBuf, WriteBuf},
  decode::Decode,
  default_scalar_wire_format,
  encode::Encode,
  flatten_state,
  flavors::{
    Groto,
    groto::{Context, Error, Fixed32, Fixed128, LengthDelimited, Varint},
  },
  partial_encode_scalar, partial_ref_state, partial_state, ref_state, selectable,
};

macro_rules! ip_addr {
  ($addr:ident::$variant:ident($convert:ident)) => {
    default_scalar_wire_format!(Groto: $addr as $variant);

    impl Encode<$variant, Groto> for $addr {
      fn encode_raw<B>(
        &self,
        context: &Context,
        buf: &mut B,
      ) -> Result<usize, Error>
      where
        B: WriteBuf + ?Sized,
      {
        <$convert as Encode<$variant, Groto>>::encode_raw(
          &self.to_bits(),
          context,
          buf,
        )
      }

      fn encoded_raw_len(&self, _: &Context) -> usize {
        <$convert as Encode<$variant, Groto>>::encoded_raw_len(
          &self.to_bits(),
          &Context::default(),
        )
      }

      fn encode<B>(
        &self,
        context: &Context,
        buf: &mut B,
      ) -> Result<usize, Error>
      where
        B: WriteBuf + ?Sized,
      {
        <$convert as Encode<$variant, Groto>>::encode(
          &self.to_bits(),
          context,
          buf,
        )
      }

      fn encoded_len(&self, _: &Context) -> usize {
        <$convert as Encode<$variant, Groto>>::encoded_len(
          &self.to_bits(),
          &Context::default(),
        )
      }
    }

    impl Encode<Varint, Groto> for $addr {
      fn encode_raw<B>(
        &self,
        context: &Context,
        buf: &mut B,
      ) -> Result<usize, Error>
      where
        B: WriteBuf + ?Sized,
      {
        <$convert as Encode<Varint, Groto>>::encode_raw(
          &self.to_bits(),
          context,
          buf,
        )
      }

      fn encoded_raw_len(&self, ctx: &Context) -> usize {
        <$convert as Encode<Varint, Groto>>::encoded_raw_len(
          &self.to_bits(),
          ctx,
        )
      }

      fn encode<B>(
        &self,
        context: &Context,
        buf: &mut B,
      ) -> Result<usize, Error>
      where
        B: WriteBuf + ?Sized,
      {
        <$convert as Encode<Varint, Groto>>::encode(
          &self.to_bits(),
          context,
          buf,
        )
      }

      fn encoded_len(&self, ctx: &Context) -> usize {
        <$convert as Encode<Varint, Groto>>::encoded_len(
          &self.to_bits(),
          ctx,
        )
      }
    }

    impl<'de, RB, B> Decode<'de, $variant, RB, B, Groto> for $addr {
      fn decode(
        ctx: &'de Context,
        src: RB,
      ) -> Result<(usize, Self), Error>
      where
        Self: Sized + 'de,
        RB: ReadBuf + 'de,
        B: crate::buffer::UnknownBuffer<RB, Groto> + 'de,
      {
        <$convert as Decode<'de, $variant, RB, B, Groto>>::decode(ctx, src)
          .map(|(len, val)| (len, $addr::from_bits($convert::from_le(val))))
      }
    }

    impl<'de, RB, B> Decode<'de, Varint, RB, B, Groto> for $addr {
      fn decode(
        ctx: &'de Context,
        src: RB,
      ) -> Result<(usize, Self), Error>
      where
        Self: Sized + 'de,
        RB: ReadBuf + 'de,
        B: crate::buffer::UnknownBuffer<RB, Groto> + 'de,
      {
        <$convert as Decode<'de, Varint, RB, B, Groto>>::decode(ctx, src)
          .map(|(len, val)| (len, $addr::from_bits($convert::from_le(val))))
      }
    }
  };
}

ip_addr!(Ipv4Addr::Fixed32(u32));
ip_addr!(Ipv6Addr::Fixed128(u128));
selectable!(@scalar Groto: Ipv4Addr, Ipv6Addr, IpAddr);
partial_encode_scalar!(Groto: Ipv4Addr as Fixed32, Ipv4Addr as Varint, Ipv6Addr as Fixed128, Ipv6Addr as Varint, IpAddr as LengthDelimited);
ref_state!(@scalar &'a Groto: Ipv4Addr as Fixed32, Ipv4Addr as Varint, Ipv6Addr as Fixed128, Ipv6Addr as Varint, IpAddr as LengthDelimited);
partial_ref_state!(@scalar &'a Groto: Ipv4Addr as Fixed32, Ipv4Addr as Varint, Ipv6Addr as Fixed128, Ipv6Addr as Varint, IpAddr as LengthDelimited);
partial_state!(@scalar Groto: Ipv4Addr, Ipv6Addr, IpAddr);
flatten_state!(Ipv4Addr, Ipv6Addr, IpAddr);

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

default_scalar_wire_format!(Groto: IpAddr as LengthDelimited);

impl Encode<LengthDelimited, Groto> for IpAddr {
  fn encode_raw<B>(
    &self,
    context: &<Groto as crate::flavors::Flavor>::Context,
    buf: &mut B,
  ) -> Result<usize, <Groto as crate::flavors::Flavor>::Error>
  where
    B: WriteBuf + ?Sized,
  {
    match self {
      Self::V4(ipv4_addr) => {
        <Ipv4Addr as Encode<Fixed32, Groto>>::encode_raw(ipv4_addr, context, buf)
      }
      Self::V6(ipv6_addr) => {
        <Ipv6Addr as Encode<Fixed128, Groto>>::encode_raw(ipv6_addr, context, buf)
      }
    }
  }

  fn encoded_raw_len(&self, context: &<Groto as crate::flavors::Flavor>::Context) -> usize {
    match self {
      Self::V4(ipv4_addr) => {
        <Ipv4Addr as Encode<Fixed32, Groto>>::encoded_raw_len(ipv4_addr, context)
      }
      Self::V6(ipv6_addr) => {
        <Ipv6Addr as Encode<Fixed128, Groto>>::encoded_raw_len(ipv6_addr, context)
      }
    }
  }

  fn encode<B>(&self, context: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: WriteBuf + ?Sized,
  {
    macro_rules! encode_ip_variant {
      ($variant:ident::$wt:ident($buf:ident, $ip:ident)) => {{
        paste::paste! {
          if buf.len() < [< $variant:upper _LEN >] + 1 {
            return Err(Error::insufficient_buffer(
              [< $variant:upper _LEN >] + 1,
              buf.len(),
            ));
          }

          <[< $variant:camel Addr>] as Encode<$wt, Groto>>::encode_length_delimited(
            $ip,
            context,
            buf,
          )
        }
      }};
    }

    match self {
      Self::V4(ip) => encode_ip_variant!(IPV4::Fixed32(buf, ip)),
      Self::V6(ip) => encode_ip_variant!(IPV6::Fixed128(buf, ip)),
    }
  }

  fn encoded_len(&self, _: &Context) -> usize {
    let len = match self {
      Self::V4(_) => IPV4_LEN,
      Self::V6(_) => IPV6_LEN,
    };

    // encoded_u32_varint_len for `4` and `6` will always be 1 byte,
    1 + len
  }
}

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for IpAddr {
  fn decode(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: crate::buffer::UnknownBuffer<RB, Groto> + 'de,
  {
    let src = src.remaining_slice();
    macro_rules! decode_ip_variant {
      ($repr:ident($variant:literal)) => {{
        paste::paste! {
          if src.len() < [< IPV $variant _LEN >] + 1 {
            return Err(Error::buffer_underflow());
          }

          let ip = [< Ipv $variant Addr >]::from_bits($repr::from_le_bytes(
            src[1..[< IPV $variant _LEN >] + 1].try_into().unwrap(),
          ));

          ([< IPV $variant _LEN >] + 1, IpAddr::from(ip))
        }
      }};
    }

    if !src.has_remaining() {
      return Err(Error::buffer_underflow());
    }

    let tag = src[0];
    Ok(match tag {
      IPV4_TAG => decode_ip_variant!(u32(4)),
      IPV6_TAG => decode_ip_variant!(u128(6)),
      _ => return Err(Error::custom("unknown ip tag")),
    })
  }

  fn decode_length_delimited(_: &Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: ReadBuf,
    B: crate::buffer::UnknownBuffer<RB, Groto> + 'de,
  {
    let src = src.remaining_slice();

    macro_rules! decode_ip_variant {
      ($variant:ident($repr:ident, $read:ident)) => {{
        paste::paste! {
          if src.len() < [< $variant _ENCODED_LENGTH_DELIMITED_LEN >] {
            return Err(Error::buffer_underflow());
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
      _ => Err(Error::custom("unknown ip tag")),
    }
  }
}

#[cfg(all(test, feature = "std"))]
mod tests {
  use super::*;

  quickcheck::quickcheck! {
    fn fuzzy_ipv4_round_trip(ip: Ipv4Addr) -> bool {
      let mut buf = [0u8; 128];
      let len = <Ipv4Addr as Encode<Fixed32, Groto>>::encode_length_delimited(&ip, &Context::default(), &mut buf).unwrap();
      let encoded_len = <Ipv4Addr as Encode<Fixed32, Groto>>::encoded_length_delimited_len(&ip, &Context::default());
      assert_eq!(len, encoded_len);

      let (len, decoded) = <Ipv4Addr as Decode<Fixed32, &[u8], Vec<_>, Groto>>::decode_length_delimited(&Context::default(), &buf[..]).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      let len = <Ipv4Addr as Encode<Varint, Groto>>::encode_length_delimited(&ip, &Context::default(), &mut buf).unwrap();
      let encoded_len = <Ipv4Addr as Encode<Varint, Groto>>::encoded_length_delimited_len(&ip, &Context::default());
      assert_eq!(len, encoded_len);

      let (len, decoded) = <Ipv4Addr as Decode<'_, Varint, &[u8], Vec<_>, Groto>>::decode_length_delimited(&Context::default(), &buf[..]).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      true
    }

    fn fuzzy_ipv6_round_trip(ip: Ipv6Addr) -> bool {
      let mut buf = [0u8; 128];
      let len = <Ipv6Addr as Encode<Fixed128, Groto>>::encode_length_delimited(&ip, &Context::default(), &mut buf).unwrap();
      let encoded_len = <Ipv6Addr as Encode<Fixed128, Groto>>::encoded_length_delimited_len(&ip, &Context::default());
      assert_eq!(len, encoded_len);

      let (len, decoded) = <Ipv6Addr as Decode<Fixed128, &[u8], Vec<_>, Groto,>>::decode_length_delimited(&Context::default(), &buf[..]).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      let len = <Ipv6Addr as Encode<Varint, Groto>>::encode_length_delimited(&ip, &Context::default(), &mut buf).unwrap();
      let encoded_len = <Ipv6Addr as Encode<Varint, Groto>>::encoded_length_delimited_len(&ip, &Context::default());
      assert_eq!(len, encoded_len);

      let (len, decoded) = <Ipv6Addr as Decode<'_, Varint, &[u8], Vec<_>, Groto>>::decode_length_delimited(&Context::default(), &buf[..]).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      true
    }

    fn fuzzy_ip_round_trip(ip: IpAddr) -> bool {
      let mut buf = [0u8; 128];
      let len = ip.encode_length_delimited(&Context::default(), &mut buf).unwrap();
      let encoded_len = ip.encoded_length_delimited_len(&Context::default(), );
      assert_eq!(len, encoded_len);

      let (len, decoded) = <IpAddr as Decode<LengthDelimited, &[u8], Vec<_>, Groto>>::decode_length_delimited(&Context::default(), &buf[..]).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      let len = ip.encode(&Context::default(),  &mut buf).unwrap();
      let encoded_len = ip.encoded_len(&Context::default(), );
      assert_eq!(len, encoded_len);

      let (len, decoded) = <IpAddr as Decode<LengthDelimited, &[u8], Vec<_>, Groto>>::decode(&Context::default(), &buf[..]).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, ip);

      true
    }
  }
}
