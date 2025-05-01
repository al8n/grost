use crate::flavors::WireFormat;

use super::Network;

macro_rules! impl_from_wire_format {
  ($(
    $(#[$meta:meta])*
    $ty:literal
  ),+$(,)?) => {
    paste::paste! {
      $(
        $(
          #[$meta]
        )*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display)]
        #[display($ty)]
        pub struct [< $ty: camel >];

        impl WireFormat<Network> for [< $ty: camel >] {
          const WIRE_TYPE: WireType = WireType::[< $ty: camel >];
        }

        impl From<[< $ty: camel >]> for WireType {
          fn from(_: [< $ty: camel >]) -> Self {
            Self::[< $ty: camel >]
          }
        }
      )*
    }
  };
}

impl_from_wire_format!(
  /// The zero-sized type wire format
  "zst",
  /// The length-delimited encoding/decoding wire format
  "length-delimited",
  /// The varint encoding/decoding wire format
  "varint",
  /// The fixed 8-bit length encoding/decoding wire format
  "fixed8",
  /// The fixed 16-bit length encoding/decoding wire format
  "fixed16",
  /// The fixed 32-bit length encoding/decoding wire format
  "fixed32",
  /// The fixed 64-bit length encoding/decoding wire format
  "fixed64",
  /// The fixed 128-bit length encoding/decoding wire format
  "fixed128",
);

/// A wire type for encoding and decoding messages.
/// 
/// This is a sum type that holds all the [`WireFormat`]s for [`Network`] flavor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, derive_more::IsVariant, derive_more::Display)]
#[repr(u8)]
#[display("{str}", str = self.as_str())]
pub enum WireType {
  /// Zero size wire type.
  ///
  /// This wire type requires no information included.
  Zst = 0,
  /// A varint wire type, which is a variable-length encoding for integers.
  Varint = 1,
  /// A length-delimited wire type.
  LengthDelimited = 2,
  /// A fixed 8-bit wire type.
  Fixed8 = 3,
  /// A fixed 16-bit wire type.
  Fixed16 = 4,
  /// A fixed 32-bit wire type.
  Fixed32 = 5,
  /// A fixed 64-bit wire type.
  Fixed64 = 6,
  /// A fixed 128-bit wire type.
  Fixed128 = 7,
}

impl WireType {
  /// Decode a wire type from a byte.
  ///
  /// ## Panics
  ///
  /// Panics if the value is not a valid wire type.
  pub const fn from_u8_unchecked(value: u8) -> Self {
    match value {
      0 => Self::LengthDelimited,
      1 => Self::Varint,
      2 => Self::Fixed8,
      3 => Self::Fixed16,
      4 => Self::Fixed32,
      5 => Self::Fixed64,
      6 => Self::Fixed128,
      7 => Self::Zst,
      _ => panic!("invalid wire type value"),
    }
  }

  /// Decode a wire type from a byte.
  #[inline]
  pub const fn try_from_u8(value: u8) -> Result<Self, u8> {
    Ok(match value {
      0 => Self::LengthDelimited,
      1 => Self::Varint,
      2 => Self::Fixed8,
      3 => Self::Fixed16,
      4 => Self::Fixed32,
      5 => Self::Fixed64,
      6 => Self::Fixed128,
      7 => Self::Zst,
      _ => return Err(value),
    })
  }

  /// Convert the wire type to a byte.
  #[inline]
  pub const fn as_u8(&self) -> u8 {
    *self as u8
  }

  /// Returns the [`WireType`] as a `&'static str`.
  #[inline]
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::LengthDelimited => "length-delimited",
      Self::Varint => "varint",
      Self::Fixed8 => "fixed8",
      Self::Fixed16 => "fixed16",
      Self::Fixed32 => "fixed32",
      Self::Fixed64 => "fixed64",
      Self::Fixed128 => "fixed128",
      Self::Zst => "zst",
    }
  }
}

impl core::convert::TryFrom<u8> for WireType {
  type Error = u8;

  #[inline]
  fn try_from(value: u8) -> Result<Self, Self::Error> {
    Self::try_from_u8(value)
  }
}
