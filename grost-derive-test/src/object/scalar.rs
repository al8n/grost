#![allow(warnings)]

use core::num::*;
use grost::{Object, flavors::groto::*};

#[derive(Object)]
struct Uints {
  #[grost(tag = 1, scalar)]
  field1: u8,
  #[grost(tag = 2, scalar)]
  field2: u16,
  #[grost(tag = 3, scalar)]
  field3: u32,
  #[grost(tag = 4, scalar)]
  field4: u64,
  #[grost(tag = 5, scalar)]
  field5: u128,

  #[grost(tag = 6, scalar, wire_format = "Varint")]
  field6: u8,
  #[grost(tag = 7, scalar, wire_format = "Fixed16")]
  field7: u16,
  #[grost(tag = 8, scalar, wire_format = "Fixed32")]
  field8: u32,
  #[grost(tag = 9, scalar, wire_format = "Fixed64")]
  field9: u64,
  #[grost(tag = 10, scalar, wire_format = "Fixed128")]
  field10: u128,
}

#[derive(Object)]
struct NonZeroUints {
  #[grost(tag = 1, scalar)]
  field1: NonZeroU8,
  #[grost(tag = 2, scalar)]
  field2: NonZeroU16,
  #[grost(tag = 3, scalar)]
  field3: NonZeroU32,
  #[grost(tag = 4, scalar)]
  field4: NonZeroU64,
  #[grost(tag = 5, scalar)]
  field5: NonZeroU128,

  #[grost(tag = 6, scalar, wire_format = "Varint")]
  field6: NonZeroU8,
  #[grost(tag = 7, scalar, wire_format = "Fixed16")]
  field7: NonZeroU16,
  #[grost(tag = 8, scalar, wire_format = "Fixed32")]
  field8: NonZeroU32,
  #[grost(tag = 9, scalar, wire_format = "Fixed64")]
  field9: NonZeroU64,
  #[grost(tag = 10, scalar, wire_format = "Fixed128")]
  field10: NonZeroU128,
}

#[derive(Object)]
struct Ints {
  #[grost(tag = 1, scalar)]
  field1: i8,
  #[grost(tag = 2, scalar)]
  field2: i16,
  #[grost(tag = 3, scalar)]
  field3: i32,
  #[grost(tag = 4, scalar)]
  field4: i64,
  #[grost(tag = 5, scalar)]
  field5: i128,

  #[grost(tag = 6, scalar, wire_format = "Varint")]
  field6: i8,
  #[grost(tag = 7, scalar, wire_format = "Fixed16")]
  field7: i16,
  #[grost(tag = 8, scalar, wire_format = "Fixed32")]
  field8: i32,
  #[grost(tag = 9, scalar, wire_format = "Fixed64")]
  field9: i64,
  #[grost(tag = 10, scalar, wire_format = "Fixed128")]
  field10: i128,
}

#[derive(Object)]
struct NonZeroInts {
  #[grost(tag = 1, scalar)]
  field1: NonZeroI8,
  #[grost(tag = 2, scalar)]
  field2: NonZeroI16,
  #[grost(tag = 3, scalar)]
  field3: NonZeroI32,
  #[grost(tag = 4, scalar)]
  field4: NonZeroI64,
  #[grost(tag = 5, scalar)]
  field5: NonZeroI128,

  #[grost(tag = 6, scalar, wire_format = "Varint")]
  field6: NonZeroI8,
  #[grost(tag = 7, scalar, wire_format = "Fixed16")]
  field7: NonZeroI16,
  #[grost(tag = 8, scalar, wire_format = "Fixed32")]
  field8: NonZeroI32,
  #[grost(tag = 9, scalar, wire_format = "Fixed64")]
  field9: NonZeroI64,
  #[grost(tag = 10, scalar, wire_format = "Fixed128")]
  field10: NonZeroI128,
}

#[derive(Object)]
struct Bool {
  #[grost(tag = 1, scalar)]
  field1: bool,

  #[grost(tag = 2, scalar, wire_format = "Varint")]
  field2: bool,
}

#[derive(Object)]
struct Char {
  #[grost(tag = 1, scalar)]
  field1: char,

  #[grost(tag = 2, scalar, wire_format = "Varint")]
  field2: char,
}

#[derive(Object)]
struct Floats {
  #[grost(tag = 1, scalar)]
  field1: f32,
  #[grost(tag = 2, scalar)]
  field2: f64,
  #[cfg(feature = "half_2")]
  #[grost(tag = 3, scalar)]
  field3: half_2::f16,
  #[cfg(feature = "decimal_1")]
  #[grost(tag = 4, scalar)]
  field4: rust_decimal_1::Decimal,

  #[grost(tag = 5, scalar, wire_format = "Varint")]
  field5: f32,
  #[grost(tag = 6, scalar, wire_format = "Varint")]
  field6: f64,
  #[cfg(feature = "half_2")]
  #[grost(tag = 7, scalar, wire_format = "Varint")]
  field7: half_2::f16,
  #[cfg(feature = "decimal_1")]
  #[grost(tag = 8, scalar, wire_format = "Varint")]
  field8: rust_decimal_1::Decimal,
}

#[derive(Object)]
struct Time {
  #[grost(tag = 1, scalar)]
  #[cfg(feature = "time_0_3")]
  field1: time_0_3::Date,
  #[grost(tag = 2, scalar)]
  #[cfg(feature = "time_0_3")]
  field2: time_0_3::Time,
  #[grost(tag = 3, scalar)]
  #[cfg(feature = "time_0_3")]
  field3: time_0_3::PrimitiveDateTime,
  #[grost(tag = 4, scalar)]
  #[cfg(feature = "time_0_3")]
  field4: time_0_3::UtcDateTime,
  #[cfg(feature = "time_0_3")]
  #[grost(tag = 5, scalar)]
  field5: time_0_3::Duration,
  #[grost(tag = 6, scalar)]
  field6: core::time::Duration,
  #[cfg(feature = "chrono_0_4")]
  #[grost(tag = 7, scalar)]
  field7: chrono_0_4::NaiveDate,
  #[cfg(feature = "chrono_0_4")]
  #[grost(tag = 8, scalar)]
  field8: chrono_0_4::NaiveTime,
  #[cfg(feature = "chrono_0_4")]
  #[grost(tag = 9, scalar)]
  field9: chrono_0_4::NaiveDateTime,
  #[cfg(feature = "chrono_0_4")]
  #[grost(tag = 10, scalar)]
  field10: chrono_0_4::Duration,
  #[cfg(feature = "chrono_0_4")]
  #[grost(tag = 11, scalar)]
  field11: chrono_0_4::DateTime<chrono_0_4::Utc>,
  #[cfg(feature = "chrono-tz_0_10")]
  #[grost(tag = 12, scalar)]
  field12: chrono_tz_0_10::Tz,
}

#[cfg(feature = "bnum_0_13")]
#[derive(Object)]
struct BUints {
  #[grost(tag = 1, scalar)]
  field1: bnum_0_13::BUint<1>,
  #[grost(tag = 2, scalar)]
  field2: bnum_0_13::BUintD8<2>,
  #[grost(tag = 3, scalar)]
  field3: bnum_0_13::BUintD16<3>,
  #[grost(tag = 4, scalar)]
  field4: bnum_0_13::BUintD32<4>,
}

#[cfg(feature = "bnum_0_13")]
#[derive(Object)]
struct BInts {
  #[grost(tag = 1, scalar)]
  field1: bnum_0_13::BInt<1>,
  #[grost(tag = 2, scalar)]
  field2: bnum_0_13::BIntD8<2>,
  #[grost(tag = 3, scalar)]
  field3: bnum_0_13::BIntD16<3>,
  #[grost(tag = 4, scalar)]
  field4: bnum_0_13::BIntD32<4>,
}

#[cfg(feature = "ruint_1")]
#[derive(Object)]
struct RUints {
  #[grost(tag = 1, scalar)]
  field0: ruint_1::aliases::U0,
  #[grost(tag = 2, scalar)]
  field1: ruint_1::aliases::U1,
  #[grost(tag = 3, scalar)]
  field2: ruint_1::aliases::U16,
  #[grost(tag = 4, scalar)]
  field3: ruint_1::aliases::U32,
  #[grost(tag = 5, scalar)]
  field4: ruint_1::aliases::U64,
  #[grost(tag = 6, scalar)]
  field5: ruint_1::aliases::U128,
  #[grost(tag = 7, scalar)]
  field6: ruint_1::aliases::U256,
  #[grost(tag = 8, scalar)]
  field7: ruint_1::aliases::U512,
}

#[cfg(feature = "arbitrary-int_1")]
#[derive(Object)]
struct ArbitraryInts {
  #[grost(tag = 1, scalar)]
  field0: arbitrary_int_1::u1,
  #[grost(tag = 2, scalar)]
  field1: arbitrary_int_1::u2,
  #[grost(tag = 3, scalar)]
  field2: arbitrary_int_1::u3,
  #[grost(tag = 4, scalar)]
  field3: arbitrary_int_1::u4,
  #[grost(tag = 5, scalar)]
  field4: arbitrary_int_1::u5,
  #[grost(tag = 6, scalar)]
  field5: arbitrary_int_1::u6,
}

#[derive(Object)]
struct Net {
  #[grost(tag = 1, scalar)]
  ip: core::net::IpAddr,
  #[grost(tag = 2, scalar)]
  ipv4: core::net::Ipv4Addr,
  #[grost(tag = 3, scalar)]
  ipv6: core::net::Ipv6Addr,
  #[grost(tag = 4, scalar)]
  socket: core::net::SocketAddr,
  #[grost(tag = 5, scalar)]
  socket_v6: core::net::SocketAddrV6,
  #[grost(tag = 6, scalar)]
  socket_v4: core::net::SocketAddrV4,

  #[grost(tag = 7, scalar, wire_format = "Varint")]
  ipv4_varint: core::net::Ipv4Addr,
  #[grost(tag = 8, scalar, wire_format = "Varint")]
  ipv6_varint: core::net::Ipv6Addr,
}

#[test]
fn compile() {}
