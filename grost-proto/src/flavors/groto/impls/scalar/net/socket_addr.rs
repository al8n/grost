use core::{
  net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
  num::NonZeroUsize,
};

use crate::{
  buffer::{Chunk, ChunkExt, ChunkMut, ChunkWriter, UnknownBuffer},
  decode::Decode,
  default_scalar_wire_format,
  encode::Encode,
  flatten_state,
  flavors::{
    Groto,
    groto::{Context, DecodeError, EncodeError, LengthDelimited},
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
            buf: impl Into<ChunkWriter<B>>,
          ) -> Result<usize, EncodeError>
          where
            B: ChunkMut,
          {
            let mut buf: ChunkWriter<B> = buf.into();
            let remaining = buf.remaining_mut();

            if remaining < [< SOCKET_ADDR_V $variant _LEN >] {
              return Err(EncodeError::buffer_too_small([< SOCKET_ADDR_V $variant _LEN >], remaining));
            }

            buf.write_slice(&self.ip().to_bits().to_le_bytes());
            buf.write_u16_le(self.port());

            Ok([< SOCKET_ADDR_V $variant _LEN >])
          }

          fn encoded_raw_len(&self, _: &Context) -> usize {
            [< SOCKET_ADDR_V $variant _LEN >]
          }

          fn encode<B>(&self, ctx: &Context, buf: impl Into<ChunkWriter<B>>) -> Result<usize, EncodeError>
          where
            B: ChunkMut,
          {
            let mut buf: ChunkWriter<B> = buf.into();
            let remaining = buf.remaining_mut();
            if remaining < [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >] {
              return Err(EncodeError::buffer_too_small(
                [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >],
                remaining,
              ));
            }

            let mut offset = buf.write_slice([< ENCODED_SOCKET_ADDR_V $variant _LENGTH_DELIMITED >]);
            offset += <Self as Encode<LengthDelimited, Groto>>::encode::<&mut B>(self, ctx, buf.as_mut())?;

            #[cfg(debug_assertions)]
            crate::debug_assert_write_eq::<Self>(offset, [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >]);

            Ok(offset)
          }

          fn encoded_len(&self, _: &Context) -> usize {
            [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >]
          }
        }

        impl<'de, RB, B> Decode<'de, LengthDelimited, RB, B, Groto> for [< SocketAddrV $variant >] {
          fn decode(_: &'de Context, mut src: RB) -> Result<(usize, Self), DecodeError>
          where
            Self: Sized + 'de,
            RB: Chunk,
            B: UnknownBuffer<RB, Groto> + 'de,
          {
            let remaining = src.remaining();
            if remaining < [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >] {
              return Err(DecodeError::insufficient_data_with_requested(remaining, [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >]));
            }

            let (_, data_len) = src.read_varint::<u32>().map_err(|e| propagate_buffer_info(
              e,
              [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >],
              remaining,
            ))?;
            if data_len != [< SOCKET_ADDR_V $variant _LEN >] as u32 {
              return Err(DecodeError::other(concat!("invalid socket v", $variant, " address length")));
            }

            let ip = src.try_read_array::<[< IPV $variant _LEN >]>()
              .map_err(|e| propagate_buffer_info(
                e,
                [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >],
                remaining,
              ))?;
            let port = src.try_read_u16_le()
              .map_err(|e| propagate_buffer_info(
                e,
                [< SOCKET_ADDR_V $variant _ENCODED_LENGTH_DELIMITED_LEN >],
                remaining,
              ))?;
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
  fn encode_raw<B>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<B>>,
  ) -> Result<usize, EncodeError>
  where
    B: ChunkMut,
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

  fn encode<B>(
    &self,
    context: &Context,
    buf: impl Into<ChunkWriter<B>>,
  ) -> Result<usize, EncodeError>
  where
    B: ChunkMut,
  {
    let mut buf: ChunkWriter<B> = buf.into();
    let remaining = buf.remaining_mut();
    macro_rules! encode_addr {
      ($addr:ident, $variant:literal) => {{
        paste::paste! {
          if remaining < 1 + [< SOCKET_ADDR_V $variant _LEN>] {
            return Err(EncodeError::buffer_too_small(
              1 + [< SOCKET_ADDR_V $variant _LEN>],
              remaining,
            ));
          }

          buf.write_u8([< SOCKET_ADDR_V $variant _LEN>] as u8);
          <[< SocketAddrV $variant >] as Encode<LengthDelimited, Groto>>::encode_raw::<&mut B>($addr, context, buf.as_mut())
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
  fn decode(_: &'de Context, mut src: RB) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'de,
    RB: Chunk + 'de,
    B: UnknownBuffer<RB, Groto> + 'de,
  {
    let remaining = src.remaining();
    let data_len = src
      .try_read_u8()
      .map_err(|_| DecodeError::insufficient_data(remaining))?;

    macro_rules! decode_addr {
      ($bridge:ident($variant:literal, $src:ident)) => {{
        paste::paste! {
          let ip_bytes = $src.try_read_array::<[< IPV $variant _LEN >]>()
            .map_err(|e| propagate_buffer_info(e, 1 + [< SOCKET_ADDR_V $variant _LEN >], remaining))?;
          let ip = [<Ipv $variant Addr>]::from_bits(<$bridge>::from_le_bytes(ip_bytes));
          let port = $src.try_read_u16_le()
            .map_err(|e| propagate_buffer_info(e, 1 + [< SOCKET_ADDR_V $variant _LEN >], remaining))?;

          Ok(([< SOCKET_ADDR_V $variant _LEN >] + 1, (ip, port).into()))
        }
      }};
    }

    match data_len as usize {
      SOCKET_ADDR_V4_LEN => decode_addr!(u32(4, src)),
      SOCKET_ADDR_V6_LEN => decode_addr!(u128(6, src)),
      _ => Err(DecodeError::other("invalid socket address variant")),
    }
  }
}

#[inline(always)]
fn propagate_buffer_info(
  e: impl Into<DecodeError>,
  requested: usize,
  remaining: usize,
) -> DecodeError {
  let e: DecodeError = e.into();
  e.propagate_buffer_info(
    || Some(NonZeroUsize::new(requested).expect("+ 1 must be non-zero")),
    || remaining,
  )
}

#[cfg(all(test, feature = "std"))]
mod tests {
  use super::*;

  quickcheck::quickcheck! {
    fn fuzzy_socket_v4_round_trip(addr: SocketAddrV4) -> bool {
      let mut buf = [0u8; 128];
      let len = <SocketAddrV4 as Encode<LengthDelimited, Groto>>::encode_length_delimited(&addr, &Context::default(), buf.as_mut()).unwrap();
      let encoded_len = <SocketAddrV4 as Encode<LengthDelimited, Groto>>::encoded_length_delimited_len(&addr, &Context::default());
      assert_eq!(len, encoded_len);

      let (len, decoded) = <SocketAddrV4 as Decode<LengthDelimited, &[u8], Vec<_>, Groto>>::decode_length_delimited(&Context::default(), &buf[..]).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, addr);

      true
    }

    fn fuzzy_socket_v6_round_trip(addr: SocketAddrV6) -> bool {
      let mut buf = [0u8; 128];
      let len = <SocketAddrV6 as Encode<LengthDelimited, Groto>>::encode_length_delimited(&addr, &Context::default(), buf.as_mut()).unwrap();
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
      let len = addr.encode_length_delimited(&Context::default(), buf.as_mut()).unwrap();
      let encoded_len = addr.encoded_length_delimited_len(&Context::default(), );
      assert_eq!(len, encoded_len);

      let (len, decoded) = <SocketAddr as Decode<LengthDelimited, &[u8], Vec<_>, Groto>>::decode_length_delimited(&Context::default(), &buf[..]).unwrap();
      assert_eq!(len, encoded_len);
      assert_eq!(decoded, addr);

      let len = addr.encode(&Context::default(), buf.as_mut()).unwrap();
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
