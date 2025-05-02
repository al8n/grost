use crate::flavors::WireFormat;

use super::Network;

macro_rules! impl_from_wire_format {
  ($(
    $(#[$meta:meta])*
    $ty:literal = $value:literal
  ),+$(,)?) => {
    paste::paste! {
      $(
        $(
          #[$meta]
        )*
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display)]
        #[display($ty)]
        pub struct [< $ty: camel >];

        impl WireFormat<Network> for [< $ty: camel >] {
          const WIRE_TYPE: WireType = WireType::[< $ty: camel >];
          const NAME: &'static str = $ty;
        }

        impl From<[< $ty: camel >]> for WireType {
          fn from(_: [< $ty: camel >]) -> Self {
            Self::[< $ty: camel >]
          }
        }
      )*

      /// A wire type for encoding and decoding messages.
      ///
      /// This is a sum type that holds all the [`WireFormat`]s for [`Network`] flavor.
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, derive_more::IsVariant, derive_more::Display)]
      #[repr(u8)]
      #[display("{str}", str = self.as_str())]
      pub enum WireType {
        $(
          $(
            #[$meta]
          )*
          [< $ty: camel >] = $value,
        )*
      }

      impl WireType {
        /// Decode a wire type from a byte.
        ///
        /// ## Panics
        ///
        /// Panics if the value is not a valid wire type.
        pub const fn from_u8_unchecked(value: u8) -> Self {
          match value {
            $(
              $value => Self::[< $ty: camel >],
            )*
            _ => panic!("invalid wire type value"),
          }
        }

        /// Decode a wire type from a byte.
        #[inline]
        pub const fn try_from_u8(value: u8) -> Result<Self, u8> {
          Ok(match value {
            $(
              $value => Self::[< $ty: camel >],
            )*
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
            $(
              Self::[< $ty: camel >] => $ty,
            )*
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
    }
  };
}

impl_from_wire_format!(
  /// The zero-sized type wire format, this wire format requires no information included.
  "zst" = 0,
  /// The varint encoding/decoding wire format
  "varint" = 1,
  /// The length-delimited encoding/decoding wire format
  "length-delimited" = 2,
  /// The fixed 8-bit length encoding/decoding wire format
  "fixed8" = 3,
  /// The fixed 16-bit length encoding/decoding wire format
  "fixed16" = 4,
  /// The fixed 32-bit length encoding/decoding wire format
  "fixed32" = 5,
  /// The fixed 64-bit length encoding/decoding wire format
  "fixed64" = 6,
  /// The fixed 128-bit length encoding/decoding wire format
  "fixed128" = 7,
);
