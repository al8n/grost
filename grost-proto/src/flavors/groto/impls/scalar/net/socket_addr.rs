use core::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use crate::{
  buffer::WriteBuf,
  decode::Decode,
  default_scalar_wire_format,
  encode::Encode,
  flatten_state,
  flavors::{
    Groto,
    groto::{Context, Error, LengthDelimited},
  },
  partial_encode_scalar, partial_ref_state, partial_state, ref_state, selectable,
};

const PORT_SIZE: usize = 2;
const IPV4_LEN: usize = 4;
const IPV6_LEN: usize = 16;
const SOCKET_ADDR_V4_LEN: usize = IPV4_LEN + PORT_SIZE;
const SOCKET_ADDR_V6_LEN: usize = IPV6_LEN + PORT_SIZE;
const SOCKET_ADDR_V4_LENGTH_DELIMITED_LEN: usize = ENCODED_SOCKET_ADDR_V4_LENGTH_DELIMITED.len();
const SOCKET_ADDR_V6_LENGTH_DELIMITED_LEN: usize = ENCODED_SOCKET_ADDR_V6_LENGTH_DELIMITED.len();
const SOCKET_ADDR_V4_ENCODED_LENGTH_DELIMITED_LEN: usize =
  varing::encoded_u32_varint_len(SOCKET_ADDR_V4_LEN as u32) + SOCKET_ADDR_V4_LEN;
const SOCKET_ADDR_V6_ENCODED_LENGTH_DELIMITED_LEN: usize =
  varing::encoded_u32_varint_len(SOCKET_ADDR_V6_LEN as u32) + SOCKET_ADDR_V6_LEN;
const ENCODED_SOCKET_ADDR_V4_LENGTH_DELIMITED: &[u8] =
  varing::encode_u32_varint(SOCKET_ADDR_V4_LEN as u32).as_slice();
const ENCODED_SOCKET_ADDR_V6_LENGTH_DELIMITED: &[u8] =
  varing::encode_u32_varint(SOCKET_ADDR_V6_LEN as u32).as_slice();

macro_rules! socket_addr_impl {
  ($($bridge:ident($variant:literal)), +$(,)?) => {
    $(
      paste::paste! {
        impl Encode<LengthDelimited, Groto> for [< SocketAddrV $variant >] {
          fn encode_raw<B>(
            &self,
            _: &Context,
            buf: &mut B,
          ) -> Result<usize, Error>
          where
            B: WriteBuf + ?Sized,
          {
            if buf.len() < [< SOCKET_ADDR_V $variant _LEN >] {
              return Err(Error::insufficient_buffer([< SOCKET_ADDR_V $variant _LEN >], buf.len()));
            }

            let buf = buf.as_mut_slice();

            buf[0..[< IPV $variant _LEN >]].copy_from_slice(&self.ip().to_bits().to_le_bytes());
            buf[[< IPV $variant _LEN >]..[< SOCKET_ADDR_V $variant _LEN >]].copy_from_slice(&self.port().to_le_bytes());

            Ok([< SOCKET_ADDR_V $variant _LEN >])
          }

          fn encoded_raw_len(&self, _: &Context) -> usize {
            [< SOCKET_ADDR_V $variant _LEN >]
          }

          fn encode<B>(&self, ctx: &Context, buf: &mut B) -> Result<usize, Error>
          where
            B: WriteBuf + ?Sized,
          {
            if buf.len() < [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >] {
              return Err(Error::insufficient_buffer(
                [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >],
                buf.len(),
              ));
            }

            let mut offset = 0;
            let buf = buf.as_mut_slice();
            buf[offset..[< SOCKET_ADDR_V $variant _LENGTH_DELIMITED_LEN >]]
              .copy_from_slice([< ENCODED_SOCKET_ADDR_V $variant _LENGTH_DELIMITED >]);
            offset += [< SOCKET_ADDR_V $variant _LENGTH_DELIMITED_LEN >];
            offset += <Self as Encode<LengthDelimited, Groto>>::encode(self, ctx, &mut buf[offset..])?;

            #[cfg(debug_assertions)]
            crate::debug_assert_write_eq::<Self>(offset, [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >]);

            Ok(offset)
          }

          fn encoded_len(&self, _: &Context) -> usize {
            [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >]
          }
        }

        impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for [< SocketAddrV $variant >] {
          fn decode(_: &'de Context, src: RB) -> Result<(usize, Self), Error>
          where
            Self: Sized + 'de,
            RB: crate::buffer::ReadBuf,
            B: crate::buffer::UnknownBuffer<RB, Groto> + 'de,
          {
            if src.len() < [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >] {
              return Err(Error::buffer_underflow());
            }

            let buf = src.as_bytes();
            let (offset, remaining) = varing::decode_u32_varint(buf).map_err(|_| Error::buffer_underflow())?;
            if remaining != [< SOCKET_ADDR_V $variant _LEN >] as u32 {
              return Err(Error::custom(concat!("invalid socket v", $variant, " address length")));
            }

            let ip = <$bridge>::from_le_bytes(buf[offset..offset + [< IPV $variant _LEN >]].try_into().unwrap());
            let port = u16::from_le_bytes(buf[offset + [< IPV $variant _LEN >]..offset + [< SOCKET_ADDR_V $variant _LEN >]].try_into().unwrap());
            let socket_addr = [< new_socket_v $variant >](ip.into(), port);
            Ok(([< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >], socket_addr))
          }
        }
      }
    )*
  };
}

#[inline]
const fn new_socket_v4(ip: Ipv4Addr, port: u16) -> SocketAddrV4 {
  SocketAddrV4::new(ip, port)
}

#[inline]
const fn new_socket_v6(ip: Ipv6Addr, port: u16) -> SocketAddrV6 {
  SocketAddrV6::new(ip, port, 0, 0)
}

socket_addr_impl!(u32(4), u128(6));
partial_encode_scalar!(Groto: SocketAddrV4 as LengthDelimited, SocketAddrV6 as LengthDelimited, SocketAddr as LengthDelimited);
default_scalar_wire_format!(
  Groto:
    SocketAddrV4 as LengthDelimited;
    SocketAddrV6 as LengthDelimited;
    SocketAddr as LengthDelimited;
);
selectable!(@scalar Groto: SocketAddrV4, SocketAddrV6, SocketAddr);
ref_state!(@scalar &'a Groto: SocketAddrV4 as LengthDelimited, SocketAddrV6 as LengthDelimited, SocketAddr as LengthDelimited);
partial_ref_state!(@scalar &'a Groto: SocketAddrV4 as LengthDelimited, SocketAddrV6 as LengthDelimited, SocketAddr as LengthDelimited);
partial_state!(@scalar Groto: SocketAddrV4, SocketAddrV6, SocketAddr);
flatten_state!(SocketAddrV4, SocketAddrV6, SocketAddr);

impl Encode<LengthDelimited, Groto> for SocketAddr {
  fn encode_raw<B>(&self, context: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: WriteBuf + ?Sized,
  {
    match self {
      Self::V4(addr) => {
        <SocketAddrV4 as Encode<LengthDelimited, Groto>>::encode_raw(addr, context, buf)
      }
      Self::V6(addr) => {
        <SocketAddrV6 as Encode<LengthDelimited, Groto>>::encode_raw(addr, context, buf)
      }
    }
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    match self {
      Self::V4(addr) => {
        <SocketAddrV4 as Encode<LengthDelimited, Groto>>::encoded_raw_len(addr, context)
      }
      Self::V6(addr) => {
        <SocketAddrV6 as Encode<LengthDelimited, Groto>>::encoded_raw_len(addr, context)
      }
    }
  }

  fn encode<B>(&self, context: &Context, buf: &mut B) -> Result<usize, Error>
  where
    B: WriteBuf + ?Sized,
  {
    macro_rules! encode_addr {
      ($addr:ident, $variant:literal) => {{
        paste::paste! {
          if buf.len() < 1 + [< SOCKET_ADDR_V $variant _LEN>] {
            return Err(Error::insufficient_buffer(
              1 + [< SOCKET_ADDR_V $variant _LEN>],
              buf.len(),
            ));
          }
          let buf = buf.as_mut_slice();
          buf[0] = [< SOCKET_ADDR_V $variant _LEN>] as u8;
          <[< SocketAddrV $variant >] as Encode<LengthDelimited, Groto>>::encode_raw($addr, context, &mut buf[1..])
            .map(|val| 1 + val)
        }
      }};
    }
    match self {
      Self::V4(addr) => encode_addr!(addr, 4),
      Self::V6(addr) => encode_addr!(addr, 6),
    }
  }

  fn encoded_len(&self, _: &Context) -> usize {
    match self {
      Self::V4(_) => SOCKET_ADDR_V4_ENCODED_LENGTH_DELIMITED_LEN,
      Self::V6(_) => SOCKET_ADDR_V6_ENCODED_LENGTH_DELIMITED_LEN,
    }
  }
}

impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for SocketAddr {
  fn decode(_: &'de Context, src: RB) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    RB: crate::buffer::ReadBuf + 'de,
    B: crate::buffer::UnknownBuffer<RB, Groto> + 'de,
  {
    let len = src.len();
    if len == 0 {
      return Err(Error::buffer_underflow());
    }

    let buf = src.as_bytes();
    let remaining = buf[0];

    macro_rules! decode_addr {
      ($bridge:ident($variant:literal, $src:ident)) => {{
        paste::paste! {
          if len < 1 + [< SOCKET_ADDR_V $variant _LEN >] {
            return Err(Error::buffer_underflow());
          }

          let ip_bytes: [u8; [< IPV $variant _LEN>]] = buf[1..1 + [< IPV $variant _LEN >]].try_into().unwrap();
          let ip = [<Ipv $variant Addr>]::from_bits(<$bridge>::from_le_bytes(ip_bytes));
          let port = u16::from_le_bytes(buf[1 + [< IPV $variant _LEN >]..1 + [< SOCKET_ADDR_V $variant _LEN >]].try_into().unwrap());

          Ok(([< SOCKET_ADDR_V $variant _LEN >] + 1, (ip, port).into()))
        }
      }};
    }

    match remaining as usize {
      SOCKET_ADDR_V4_LEN => decode_addr!(u32(4, src)),
      SOCKET_ADDR_V6_LEN => decode_addr!(u128(6, src)),
      _ => Err(Error::custom("invalid socket address variant")),
    }
  }
}

#[cfg(all(test, feature = "std"))]
mod tests {
  use super::*;

  quickcheck::quickcheck! {
    fn fuzzy_socket_v4_round_trip(addr: SocketAddrV4) -> bool {
      let mut buf = [0u8; 128];
      let len = <SocketAddrV4 as Encode<LengthDelimited, Groto>>::encode_length_delimited(&addr, &Context::default(), &mut buf).unwrap();
      let encoded_len = <SocketAddrV4 as Encode<LengthDelimited, Groto>>::encoded_length_delimited_len(&addr, &Context::default());
      assert_eq!(len, encoded_len);

      let (len, decoded) = <SocketAddrV4 as Decode<LengthDelimited, &[u8], Vec<_>, Groto>>::decode_length_delimited(&Context::default(), &buf[..]).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, addr);

      true
    }

    fn fuzzy_socket_v6_round_trip(addr: SocketAddrV6) -> bool {
      let mut buf = [0u8; 128];
      let len = <SocketAddrV6 as Encode<LengthDelimited, Groto>>::encode_length_delimited(&addr, &Context::default(), &mut buf).unwrap();
      let encoded_len = <SocketAddrV6 as Encode<LengthDelimited, Groto>>::encoded_length_delimited_len(&addr, &Context::default());
      assert_eq!(len, encoded_len);

      let (len, decoded) = <SocketAddrV6 as Decode<LengthDelimited, &[u8], Vec<_>, Groto>>::decode_length_delimited(&Context::default(), &buf[..]).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded.ip(), addr.ip());
      assert_eq!(decoded.port(), addr.port());

      true
    }

    fn fuzzy_socket_addr_round_trip(addr: SocketAddr) -> bool {
      let mut buf = [0u8; 128];
      let len = addr.encode_length_delimited(&Context::default(), &mut buf).unwrap();
      let encoded_len = addr.encoded_length_delimited_len(&Context::default(), );
      assert_eq!(len, encoded_len);

      let (len, decoded) = <SocketAddr as Decode<LengthDelimited, &[u8], Vec<_>, Groto>>::decode_length_delimited(&Context::default(), &buf[..]).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, addr);

      let len = addr.encode(&Context::default(),  &mut buf).unwrap();
      let encoded_len = addr.encoded_len(&Context::default(), );
      assert_eq!(len, encoded_len);

      let (len, decoded) = <SocketAddr as Decode<LengthDelimited, &[u8], Vec<_>, Groto>>::decode(&Context::default(), &buf[..]).unwrap();
      assert_eq!(len, encoded_len);
      match (decoded, addr) {
        (SocketAddr::V4(decoded), SocketAddr::V4(original)) => assert_eq!(decoded, original),
        (SocketAddr::V6(decoded), SocketAddr::V6(original)) => {
          assert_eq!(decoded.ip(), original.ip());
          assert_eq!(decoded.port(), original.port());
        },
        _ => return false,
      }

      true
    }
  }
}
