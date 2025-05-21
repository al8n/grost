use core::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use crate::{
  decode::Decode,
  decode_owned_scalar, decoded_state, default_wire_format,
  encode::Encode,
  flatten_state,
  flavors::{
    Network,
    network::{Context, Error, LengthDelimited, Unknown},
  },
  partial_encode_scalar, selectable,
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
        impl Encode<Network, LengthDelimited> for [< SocketAddrV $variant >] {
          fn encode(&self, _: &Context, buf: &mut [u8]) -> Result<usize, Error> {
            if buf.len() < [< SOCKET_ADDR_V $variant _LEN >] {
              return Err(Error::insufficient_buffer([< SOCKET_ADDR_V $variant _LEN >], buf.len()));
            }

            buf[0..[< IPV $variant _LEN >]].copy_from_slice(&self.ip().to_bits().to_le_bytes());
            buf[[< IPV $variant _LEN >]..[< SOCKET_ADDR_V $variant _LEN >]].copy_from_slice(&self.port().to_le_bytes());

            Ok([< SOCKET_ADDR_V $variant _LEN >])
          }

          fn encoded_len(&self, _: &Context) -> usize {
            [< SOCKET_ADDR_V $variant _LEN >]
          }

          fn encoded_length_delimited_len(&self, _: &Context) -> usize {
            [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >]
          }

          fn encode_length_delimited(
            &self,
            ctx: &Context,
            buf: &mut [u8],
          ) -> Result<usize, Error> {
            if buf.len() < [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >] {
              return Err(Error::insufficient_buffer(
                [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >],
                buf.len(),
              ));
            }

            let mut offset = 0;
            buf[offset..[< SOCKET_ADDR_V $variant _LENGTH_DELIMITED_LEN >]]
              .copy_from_slice([< ENCODED_SOCKET_ADDR_V $variant _LENGTH_DELIMITED >]);
            offset += [< SOCKET_ADDR_V $variant _LENGTH_DELIMITED_LEN >];
            offset += <Self as Encode<Network, LengthDelimited>>::encode(self, ctx, &mut buf[offset..])?;

            #[cfg(debug_assertions)]
            crate::debug_assert_write_eq::<Self>(offset, [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >]);

            Ok(offset)
          }
        }

        impl<'de> Decode<'de, Network, LengthDelimited, Self> for [< SocketAddrV $variant >] {
          fn decode<UB>(_: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
          where
            Self: Sized + 'de,
            UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
          {
            if src.len() < [< SOCKET_ADDR_V $variant _LEN >] {
              return Err(Error::buffer_underflow());
            }

            let ip = <$bridge>::from_le_bytes(src[0.. [< IPV $variant _LEN >]].try_into().unwrap());
            let port = u16::from_le_bytes(src[[< IPV $variant _LEN >]..[< SOCKET_ADDR_V $variant _LEN >]].try_into().unwrap());
            let socket_addr = [< new_socket_v $variant >](ip.into(), port);
            Ok(([< SOCKET_ADDR_V $variant _LEN >], socket_addr))
          }

          fn decode_length_delimited<UB>(
            context: &Context,
            src: &'de [u8],
          ) -> Result<(usize, Self), Error>
          where
            Self: Sized + 'de,
            UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
          {
            if src.len() < [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >] {
              return Err(Error::buffer_underflow());
            }

            let (read, remaining) = varing::decode_u32_varint(src).map_err(|_| Error::buffer_underflow())?;
            if remaining != [< SOCKET_ADDR_V $variant _LEN >] as u32 {
              return Err(Error::custom(concat!("invalid socket v", $variant, " address length")));
            }

            let (len, socket_addr) = <Self as Decode<'_, Network, LengthDelimited, Self>>::decode::<UB>(context, &src[read..])?;

            #[cfg(debug_assertions)]
            crate::debug_assert_read_eq::<Self>(len + read, [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >]);

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
partial_encode_scalar!(Network: SocketAddrV4 as LengthDelimited, SocketAddrV6 as LengthDelimited, SocketAddr as LengthDelimited);
decode_owned_scalar!(
  Network:
    SocketAddrV4 as LengthDelimited,
    SocketAddrV6 as LengthDelimited,
    SocketAddr as LengthDelimited,
);
default_wire_format!(
  Network: SocketAddrV4 as LengthDelimited;
  SocketAddrV6 as LengthDelimited;
  SocketAddr as LengthDelimited;
);
selectable!(@scalar Network: SocketAddrV4, SocketAddrV6, SocketAddr);
decoded_state!(@scalar &'a Network: SocketAddrV4 as LengthDelimited, SocketAddrV6 as LengthDelimited, SocketAddr as LengthDelimited);
flatten_state!(SocketAddrV4, SocketAddrV6, SocketAddr);

impl Encode<Network, LengthDelimited> for SocketAddr {
  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    macro_rules! encode_addr {
      ($addr:ident, $variant:literal) => {{
        paste::paste! {
          if buf.len() < 1 + [< SOCKET_ADDR_V $variant _LEN>] {
            return Err(Error::insufficient_buffer(
              1 + [< SOCKET_ADDR_V $variant _LEN>],
              buf.len(),
            ));
          }
          buf[0] = $variant;
          <[< SocketAddrV $variant >] as Encode<Network, LengthDelimited>>::encode($addr, context, &mut buf[1..])
            .map(|val| 1 + val)
        }
      }};
    }
    match self {
      Self::V4(addr) => encode_addr!(addr, 4),
      Self::V6(addr) => encode_addr!(addr, 6),
    }
  }

  fn encoded_len(&self, context: &Context) -> usize {
    1 + match self {
      Self::V4(addr) => {
        <SocketAddrV4 as Encode<Network, LengthDelimited>>::encoded_len(addr, context)
      }
      Self::V6(addr) => {
        <SocketAddrV6 as Encode<Network, LengthDelimited>>::encoded_len(addr, context)
      }
    }
  }

  fn encoded_length_delimited_len(&self, _: &Context) -> usize {
    match self {
      Self::V4(_) => SOCKET_ADDR_V4_ENCODED_LENGTH_DELIMITED_LEN,
      Self::V6(_) => SOCKET_ADDR_V6_ENCODED_LENGTH_DELIMITED_LEN,
    }
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    match self {
      Self::V4(addr) => {
        <SocketAddrV4 as Encode<Network, LengthDelimited>>::encode_length_delimited(
          addr, context, buf,
        )
      }
      Self::V6(addr) => {
        <SocketAddrV6 as Encode<Network, LengthDelimited>>::encode_length_delimited(
          addr, context, buf,
        )
      }
    }
  }
}

impl<'de> Decode<'de, Network, LengthDelimited, Self> for SocketAddr {
  fn decode<UB>(context: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
    let len = src.len();
    if len == 0 {
      return Err(Error::buffer_underflow());
    }

    let tag = src[0];

    macro_rules! decode_addr {
      ($variant:literal, $src:ident) => {{
        paste::paste! {
          let (read, addr) = <[<SocketAddrV $variant >] as Decode<'de, Network, LengthDelimited, [<SocketAddrV $variant >]>>::decode::<UB>(context, &$src[1..])?;
          #[cfg(debug_assertions)]
          crate::debug_assert_read_eq::<Self>(read + 1, 1 + [< SOCKET_ADDR_V $variant _LEN >]);

          Ok((read + 1, addr.into()))
        }
      }};
    }

    match tag {
      4 => decode_addr!(4, src),
      6 => decode_addr!(6, src),
      _ => Err(Error::custom("invalid socket address variant")),
    }
  }

  fn decode_length_delimited<UB>(context: &Context, src: &'de [u8]) -> Result<(usize, Self), Error>
  where
    Self: Sized + 'de,
    UB: crate::buffer::Buffer<Unknown<&'de [u8]>> + 'de,
  {
    macro_rules! decode_addr {
      ($read:ident, $variant:literal) => {{
        paste::paste! {
          <[< SocketAddrV $variant >] as Decode<'de, Network, LengthDelimited, [< SocketAddrV $variant >]>>::decode::<UB>(context, &src[$read..])
          .map(|(len, addr)| (len + $read, addr.into()))
        }
      }};
    }

    let (read, len) = varing::decode_u32_varint(src).map_err(|_| Error::buffer_underflow())?;
    match len as usize {
      SOCKET_ADDR_V4_LEN => decode_addr!(read, 4),
      SOCKET_ADDR_V6_LEN => decode_addr!(read, 6),
      _ => Err(Error::custom("invalid socket address length")),
    }
  }
}

#[cfg(all(test, feature = "std"))]
mod tests {
  use super::*;

  quickcheck::quickcheck! {
    fn fuzzy_socket_v4_round_trip(addr: SocketAddrV4) -> bool {
      let mut buf = [0u8; 128];
      let len = <SocketAddrV4 as Encode<Network, LengthDelimited>>::encode_length_delimited(&addr, &Context::default(), &mut buf).unwrap();
      let encoded_len = <SocketAddrV4 as Encode<Network, LengthDelimited>>::encoded_length_delimited_len(&addr, &Context::default());
      assert_eq!(len, encoded_len);

      let (len, decoded) = <SocketAddrV4 as Decode<Network, LengthDelimited, SocketAddrV4>>::decode_length_delimited::<()>(&Context::default(), &buf).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, addr);

      true
    }

    fn fuzzy_socket_v6_round_trip(addr: SocketAddrV6) -> bool {
      let mut buf = [0u8; 128];
      let len = <SocketAddrV6 as Encode<Network, LengthDelimited>>::encode_length_delimited(&addr, &Context::default(), &mut buf).unwrap();
      let encoded_len = <SocketAddrV6 as Encode<Network, LengthDelimited>>::encoded_length_delimited_len(&addr, &Context::default());
      assert_eq!(len, encoded_len);

      let (len, decoded) = <SocketAddrV6 as Decode<Network, LengthDelimited, SocketAddrV6>>::decode_length_delimited::<()>(&Context::default(), &buf).unwrap();
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

      let (len, decoded) = SocketAddr::decode_length_delimited::<()>(&Context::default(), &buf).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, addr);

      let len = addr.encode(&Context::default(),  &mut buf).unwrap();
      let encoded_len = addr.encoded_len(&Context::default(), );
      assert_eq!(len, encoded_len);

      let (len, decoded) = SocketAddr::decode::<()>(&Context::default(), &buf).unwrap();
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
