use super::{buffer::ReadBuf, error::ParseTagError};

pub use varing::{DecodeError as DecodeVarintError, EncodeError as EncodeVarintError};

pub use groto::Groto;
pub use wire_format::*;

mod wire_format;

macro_rules! wire_format {
  ($name:ident<$flavor:ty> $(
    $(#[$meta:meta])*
    $ty:literal = $fixed_length:expr
  ),+$(,)?) => {
    paste::paste! {
      $(
        $(
          #[$meta]
        )*
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display)]
        #[display($ty)]
        pub struct [< $ty: camel >];

        impl $crate::flavors::WireFormat<$flavor> for [< $ty: camel >] {
          const WIRE_TYPE: $name = $name::[< $ty: camel >];
          const INSTANCE: Self = [< $ty: camel >];
          const FIXED_LENGTH: Option<usize> = $fixed_length;
        }

        impl From<[< $ty: camel >]> for $name {
          fn from(_: [< $ty: camel >]) -> Self {
            Self::[< $ty: camel >]
          }
        }
      )*
    }
  };
}

macro_rules! wire_type {
  (enum $name:ident<$flavor:ty> {
    $(
      $(#[$meta:meta])*
      $ty:literal = $value:literal
    ),+$(,)?
  }) => {
    paste::paste! {
      #[doc = "The error when parsing a [`" $name "`]"]
      #[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
      #[error("invalid selector wire type value: {0}")]
      pub struct [< Parse $name:camel Error >](pub(crate) u8);

      impl [< Parse $name:camel Error >] {
        /// Returns the invalid selector wire type value.
        #[inline]
        pub const fn value(&self) -> u8 {
          self.0
        }
      }

      /// A wire type for encoding and decoding messages.
      ///
      #[doc = "This is a sum type that holds all the [`WireFormat`](crate::flavors::WireFormat)s for [`" $flavor "`](" $flavor ") flavor"]
      #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, derive_more::IsVariant, derive_more::Display)]
      #[repr(u8)]
      #[display("{str}", str = self.as_str())]
      pub enum $name {
        $(
          $(
            #[$meta]
          )*
          [< $ty: camel >] = $value,
        )*
      }

      impl $name {
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
        pub const fn try_from_u8(value: u8) -> Result<Self, [< Parse $name:camel Error >]> {
          Ok(match value {
            $(
              $value => Self::[< $ty: camel >],
            )*
            _ => return Err([< Parse $name:camel Error >](value)),
          })
        }

        /// Convert the wire type to a byte.
        #[inline]
        pub const fn as_u8(&self) -> u8 {
          *self as u8
        }

        /// Returns `&'static str` representation of the wire type.
        #[inline]
        pub const fn as_str(&self) -> &'static str {
          match self {
            $(
              Self::[< $ty: camel >] => $ty,
            )*
          }
        }
      }

      impl core::convert::TryFrom<u8> for $name {
        type Error = [< Parse $name:camel Error >];

        #[inline]
        fn try_from(value: u8) -> Result<Self, Self::Error> {
          Self::try_from_u8(value)
        }
      }
    }
  };
}

/// The groto flavor
pub mod groto;

/// The flavor for encoding and decoding selector
pub mod selector;

/// The identifier
pub trait Identifier<F: Flavor + ?Sized>: Copy + core::fmt::Debug + core::fmt::Display {
  /// Returns the wire type of the identifier.
  fn wire_type(&self) -> F::WireType;

  /// Returns the tag of the identifier.
  fn tag(&self) -> F::Tag;

  /// Encode the identifier into a buffer.
  fn encode(&self, dst: &mut [u8]) -> Result<usize, F::Error>;

  /// Return the length of the encoded identifier.
  fn encoded_len(&self) -> usize;

  /// Decode the identifier from a buffer.
  fn decode<'de, B>(buf: B) -> Result<(usize, Self), F::Error>
  where
    B: ReadBuf + Sized + 'de,
    Self: Sized;
}

#[cfg(any(feature = "alloc", feature = "std"))]
const _: () = {
  use super::*;
  use std::{boxed::Box, rc::Rc, sync::Arc};

  macro_rules! default_wire_format {
    (impl $trait:ident $(< $($g:ident),+$(,)? >)? for $ty:ty) => {
      impl<T, F> $crate::__private::flavors::$trait<F> for $ty
      where
        T: $trait<F> + ?Sized,
        F: Flavor + ?Sized,
      {
        type Format $(< $($g),*>)? = T::Format $(< $($g),*>)? $( where $($g: WireFormat<F> + 'static),* )?;
      }
    };
    (@builtins $($t:ident $(< $($g:ident),+$(,)? >)?),+$(,)?) => {
      $(
        default_wire_format!(
          impl $t $(<$($g),*>)? for Box<T>
        );
        default_wire_format!(
          impl $t $(<$($g),*>)? for Rc<T>
        );
        default_wire_format!(
          impl $t $(<$($g),*>)? for Arc<T>
        );

        #[cfg(feature = "triomphe_0_1")]
        default_wire_format!(
          impl $t $(<$($g),*>)? for triomphe_0_1::Arc<T>
        );
      )*
    };
  }

  default_wire_format!(
    @builtins
    DefaultScalarWireFormat,
    DefaultBytesWireFormat,
    DefaultObjectWireFormat,
    DefaultListWireFormat<V>,
    DefaultSetWireFormat<V>,
    DefaultMapWireFormat<K, V>,
    DefaultNullableWireFormat<V>,
    DefaultFlattenWireFormat<V>,
    DefaultEnumWireFormat,
    DefaultUnionWireFormat,
  );
};

/// The error for the flavor.
pub trait FlavorError<F: ?Sized + Flavor>:
  core::error::Error + From<super::error::Error<F>>
{
  /// Update the error with the required and remaining buffer capacity.
  fn update_insufficient_buffer(&mut self, required: usize, remaining: usize);
}

/// The flavor of the encoding and decoding.
pub trait Flavor: core::fmt::Debug + 'static {
  /// The identifier used for this flavor.
  type Identifier: Identifier<Self>;

  /// The tag used for this flavor.
  type Tag: Copy
    + Eq
    + core::hash::Hash
    + core::fmt::Debug
    + core::fmt::Display
    + TryFrom<u32, Error = ParseTagError>;

  /// The wire type used for this flavor.
  ///
  /// A wire type is typically a sum type of all possible [`WireFormat`]s supported by this flavor.
  type WireType: Copy + Eq + core::hash::Hash + core::fmt::Debug + core::fmt::Display;

  /// The context used for this flavor.
  #[cfg(not(feature = "quickcheck"))]
  type Context;
  /// The context used for this flavor.
  #[cfg(feature = "quickcheck")]
  type Context: quickcheck::Arbitrary;

  /// The error for this flavor.
  type Error: FlavorError<Self>;

  /// The name of the flavor.
  const NAME: &'static str;

  // /// Encodes the unknown value into a buffer.
  // fn encode_unknown<'de, B>(
  //   ctx: &Self::Context,
  //   value: &Self::Unknown<B>,
  //   buf: &mut [u8],
  // ) -> Result<usize, Self::Error>
  // where
  //   B: ReadBuf + 'de;

  // /// Returns the length of the encoded unknown value.
  // fn encoded_unknown_len<'de, B>(ctx: &Self::Context, value: &Self::Unknown<B>) -> usize
  // where
  //   B: ReadBuf + 'de;

  // /// Decodes an unknown value from a buffer.
  // ///
  // /// This function is used as a handler for unknown identifiers when decoding
  // /// a message. It is called when the identifier is not recognized by the
  // /// flavor.
  // fn decode_unknown<'de, B>(
  //   ctx: &Self::Context,
  //   buf: B,
  // ) -> Result<(usize, Self::Unknown<B>), Self::Error>
  // where
  //   B: ReadBuf + 'de;

  /// Try to peek the raw data according to the wire type.
  ///
  /// If the given buffer does not contain enough data to determine the length of the next data,
  /// it should return an error.
  ///
  /// Returns the number of bytes for the next data.
  fn peek_raw(
    ctx: &Self::Context,
    wire_type: Self::WireType,
    buf: &[u8],
  ) -> Result<usize, Self::Error>;
}
