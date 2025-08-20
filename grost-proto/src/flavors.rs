use crate::error::ParseWireTypeError;

use super::{
  error::{DecodeError, ParseTagError},
  identifier::Identifier,
};

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
        pub const fn try_from_u8(value: u8) -> Result<Self, $crate::__private::error::ParseWireTypeError> {
          Ok(match value {
            $(
              $value => Self::[< $ty: camel >],
            )*
            _ => return Err($crate::__private::error::ParseWireTypeError::new(value)),
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
        type Error = $crate::__private::error::ParseWireTypeError;

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
  type WireType: Copy
    + Eq
    + core::hash::Hash
    + core::fmt::Debug
    + core::fmt::Display
    + TryFrom<u8, Error = ParseWireTypeError>;

  /// The context used for this flavor.
  #[cfg(not(feature = "quickcheck"))]
  type Context;
  /// The context used for this flavor.
  #[cfg(feature = "quickcheck")]
  type Context: quickcheck::Arbitrary;

  /// The name of the flavor.
  const NAME: &'static str;

  /// Try to peek the raw data according to the wire type.
  ///
  /// If the given buffer does not contain enough data to determine the length of the next data,
  /// it should return an error.
  ///
  /// Returns the number of bytes for the next data.
  fn peek_raw<B>(
    ctx: &Self::Context,
    wire_type: Self::WireType,
    buf: &B,
  ) -> Result<usize, DecodeError<Self>>
  where
    B: crate::buffer::Chunk;
}
