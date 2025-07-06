#![allow(warnings)]

use core::num::*;
use grost::{
  Object,
  flavors::{Nullable, groto::*},
};

#[derive(Object)]
struct NullableUints {
  #[grost(tag = 1, nullable(scalar))]
  field1: Option<u8>,
  #[grost(tag = 2, nullable(scalar))]
  field2: Option<u16>,
  #[grost(tag = 3, nullable(scalar))]
  field3: Option<u32>,
  #[grost(tag = 4, nullable(scalar))]
  field4: Option<u64>,
  #[grost(tag = 5, nullable(scalar))]
  field5: Option<u128>,

  #[grost(tag = 6, nullable(scalar), wire_format = "Nullable<Varint>")]
  field6: Option<u8>,
  #[grost(tag = 7, nullable(scalar), wire_format = "Nullable<Fixed16>")]
  field7: Option<u16>,
  #[grost(tag = 8, nullable(scalar), wire_format = "Nullable<Fixed32>")]
  field8: Option<u32>,
  #[grost(tag = 9, nullable(scalar), wire_format = "Nullable<Fixed64>")]
  field9: Option<u64>,
  #[grost(tag = 10, nullable(scalar), wire_format = "Nullable<Fixed128>")]
  field10: Option<u128>,
}

#[derive(Object)]
struct NullableNonZeroUints {
  #[grost(tag = 1, nullable(scalar))]
  field1: Option<NonZeroU8>,
  #[grost(tag = 2, nullable(scalar))]
  field2: Option<NonZeroU16>,
  #[grost(tag = 3, nullable(scalar))]
  field3: Option<NonZeroU32>,
  #[grost(tag = 4, nullable(scalar))]
  field4: Option<NonZeroU64>,
  #[grost(tag = 5, nullable(scalar))]
  field5: Option<NonZeroU128>,

  #[grost(tag = 6, nullable(scalar), wire_format = "Nullable<Varint>")]
  field6: Option<NonZeroU8>,
  #[grost(tag = 7, nullable(scalar), wire_format = "Nullable<Fixed16>")]
  field7: Option<NonZeroU16>,
  #[grost(tag = 8, nullable(scalar), wire_format = "Nullable<Fixed32>")]
  field8: Option<NonZeroU32>,
  #[grost(tag = 9, nullable(scalar), wire_format = "Nullable<Fixed64>")]
  field9: Option<NonZeroU64>,
  #[grost(tag = 10, nullable(scalar), wire_format = "Nullable<Fixed128>")]
  field10: Option<NonZeroU128>,
}

#[derive(Object)]
struct NullableInts {
  #[grost(tag = 1, nullable(scalar))]
  field1: Option<i8>,
  #[grost(tag = 2, nullable(scalar))]
  field2: Option<i16>,
  #[grost(tag = 3, nullable(scalar))]
  field3: Option<i32>,
  #[grost(tag = 4, nullable(scalar))]
  field4: Option<i64>,
  #[grost(tag = 5, nullable(scalar))]
  field5: Option<i128>,

  #[grost(tag = 6, nullable(scalar), wire_format = "Nullable<Varint>")]
  field6: Option<i8>,
  #[grost(tag = 7, nullable(scalar), wire_format = "Nullable<Fixed16>")]
  field7: Option<i16>,
  #[grost(tag = 8, nullable(scalar), wire_format = "Nullable<Fixed32>")]
  field8: Option<i32>,
  #[grost(tag = 9, nullable(scalar), wire_format = "Nullable<Fixed64>")]
  field9: Option<i64>,
  #[grost(tag = 10, nullable(scalar), wire_format = "Nullable<Fixed128>")]
  field10: Option<i128>,
}

#[derive(Object)]
struct NullableNonZeroInts {
  #[grost(tag = 1, nullable(scalar))]
  field1: Option<NonZeroI8>,
  #[grost(tag = 2, nullable(scalar))]
  field2: Option<NonZeroI16>,
  #[grost(tag = 3, nullable(scalar))]
  field3: Option<NonZeroI32>,
  #[grost(tag = 4, nullable(scalar))]
  field4: Option<NonZeroI64>,
  #[grost(tag = 5, nullable(scalar))]
  field5: Option<NonZeroI128>,

  #[grost(tag = 6, nullable(scalar), wire_format = "Nullable<Varint>")]
  field6: Option<NonZeroI8>,
  #[grost(tag = 7, nullable(scalar), wire_format = "Nullable<Fixed16>")]
  field7: Option<NonZeroI16>,
  #[grost(tag = 8, nullable(scalar), wire_format = "Nullable<Fixed32>")]
  field8: Option<NonZeroI32>,
  #[grost(tag = 9, nullable(scalar), wire_format = "Nullable<Fixed64>")]
  field9: Option<NonZeroI64>,
  #[grost(tag = 10, nullable(scalar), wire_format = "Nullable<Fixed128>")]
  field10: Option<NonZeroI128>,
}

#[derive(Object)]
struct NullableBool {
  #[grost(tag = 1, nullable(scalar))]
  field1: Option<bool>,
}

#[derive(Object)]
struct NullableChar {
  #[grost(tag = 1, nullable(scalar))]
  field1: Option<char>,
}

#[derive(Object)]
struct NullableFloats {
  #[grost(tag = 1, nullable(scalar))]
  field1: Option<f32>,
  #[grost(tag = 2, nullable(scalar))]
  field2: Option<f64>,
  #[cfg(feature = "half_2")]
  #[grost(tag = 3, nullable(scalar))]
  field3: Option<half_2::f16>,
  #[cfg(feature = "decimal_1")]
  #[grost(tag = 4, nullable(scalar))]
  field4: Option<rust_decimal_1::Decimal>,
}

#[derive(Object)]
struct NullableTime {
  #[grost(tag = 1, nullable(scalar))]
  #[cfg(feature = "time_0_3")]
  field1: Option<time_0_3::Date>,
  #[grost(tag = 2, nullable(scalar))]
  #[cfg(feature = "time_0_3")]
  field2: Option<time_0_3::Time>,
  #[grost(tag = 3, nullable(scalar))]
  #[cfg(feature = "time_0_3")]
  field3: Option<time_0_3::PrimitiveDateTime>,
  #[grost(tag = 4, nullable(scalar))]
  #[cfg(feature = "time_0_3")]
  field4: Option<time_0_3::UtcDateTime>,
  #[cfg(feature = "time_0_3")]
  #[grost(tag = 5, nullable(scalar))]
  field5: Option<time_0_3::Duration>,
  #[grost(tag = 6, nullable(scalar))]
  field6: Option<core::time::Duration>,
  #[cfg(feature = "chrono_0_4")]
  #[grost(tag = 7, nullable(scalar))]
  field7: Option<chrono_0_4::NaiveDate>,
  #[cfg(feature = "chrono_0_4")]
  #[grost(tag = 8, nullable(scalar))]
  field8: Option<chrono_0_4::NaiveTime>,
  #[cfg(feature = "chrono_0_4")]
  #[grost(tag = 9, nullable(scalar))]
  field9: Option<chrono_0_4::NaiveDateTime>,
  #[cfg(feature = "chrono_0_4")]
  #[grost(tag = 10, nullable(scalar))]
  field10: Option<chrono_0_4::Duration>,
  #[cfg(feature = "chrono_0_4")]
  #[grost(tag = 11, nullable(scalar))]
  field11: Option<chrono_0_4::DateTime<chrono_0_4::Utc>>,
  #[cfg(feature = "chrono-tz_0_10")]
  #[grost(tag = 12, nullable(scalar))]
  field12: Option<chrono_tz_0_10::Tz>,
}

#[cfg(feature = "bnum_0_13")]
#[derive(Object)]
struct NullableBUints {
  #[grost(tag = 1, nullable(scalar))]
  field1: Option<bnum_0_13::BUint<1>>,
  #[grost(tag = 2, nullable(scalar))]
  field2: Option<bnum_0_13::BUintD8<2>>,
  #[grost(tag = 3, nullable(scalar))]
  field3: Option<bnum_0_13::BUintD16<3>>,
  #[grost(tag = 4, nullable(scalar))]
  field4: Option<bnum_0_13::BUintD32<4>>,
}

#[cfg(feature = "bnum_0_13")]
#[derive(Object)]
struct NullableBInts {
  #[grost(tag = 1, nullable(scalar))]
  field1: Option<bnum_0_13::BInt<1>>,
  #[grost(tag = 2, nullable(scalar))]
  field2: Option<bnum_0_13::BIntD8<2>>,
  #[grost(tag = 3, nullable(scalar))]
  field3: Option<bnum_0_13::BIntD16<3>>,
  #[grost(tag = 4, nullable(scalar))]
  field4: Option<bnum_0_13::BIntD32<4>>,
}

#[cfg(feature = "ruint_1")]
#[derive(Object)]
struct NullableRUints {
  #[grost(tag = 1, nullable(scalar))]
  field0: Option<ruint_1::aliases::U0>,
  #[grost(tag = 2, nullable(scalar))]
  field1: Option<ruint_1::aliases::U1>,
  #[grost(tag = 3, nullable(scalar))]
  field2: Option<ruint_1::aliases::U16>,
  #[grost(tag = 4, nullable(scalar))]
  field3: Option<ruint_1::aliases::U32>,
  #[grost(tag = 5, nullable(scalar))]
  field4: Option<ruint_1::aliases::U64>,
  #[grost(tag = 6, nullable(scalar))]
  field5: Option<ruint_1::aliases::U128>,
  #[grost(tag = 7, nullable(scalar))]
  field6: Option<ruint_1::aliases::U256>,
  #[grost(tag = 8, nullable(scalar))]
  field7: Option<ruint_1::aliases::U512>,
}

#[cfg(feature = "arbitrary-int_1")]
#[derive(Object)]
struct NullableArbitraryInts {
  #[grost(tag = 1, nullable(scalar))]
  field0: Option<arbitrary_int_1::u1>,
  #[grost(tag = 2, nullable(scalar))]
  field1: Option<arbitrary_int_1::u2>,
  #[grost(tag = 3, nullable(scalar))]
  field2: Option<arbitrary_int_1::u3>,
  #[grost(tag = 4, nullable(scalar))]
  field3: Option<arbitrary_int_1::u4>,
  #[grost(tag = 5, nullable(scalar))]
  field4: Option<arbitrary_int_1::u5>,
  #[grost(tag = 6, nullable(scalar))]
  field5: Option<arbitrary_int_1::u6>,
}

#[derive(Object)]
struct Net {
  #[grost(tag = 1, nullable(scalar))]
  ip: Option<core::net::IpAddr>,
  #[grost(tag = 2, nullable(scalar))]
  ipv4: Option<core::net::Ipv4Addr>,
  #[grost(tag = 3, nullable(scalar))]
  ipv6: Option<core::net::Ipv6Addr>,
  #[grost(tag = 4, nullable(scalar))]
  socket: Option<core::net::SocketAddr>,
  #[grost(tag = 5, nullable(scalar))]
  socket_v6: Option<core::net::SocketAddrV6>,
  #[grost(tag = 6, nullable(scalar))]
  socket_v4: Option<core::net::SocketAddrV4>,

  #[grost(tag = 7, nullable(scalar), wire_format = "Nullable<Varint>")]
  ipv4_varint: Option<core::net::Ipv4Addr>,
  #[grost(tag = 8, nullable(scalar), wire_format = "Nullable<Varint>")]
  ipv6_varint: Option<core::net::Ipv6Addr>,
}

#[test]
fn compile() {}
