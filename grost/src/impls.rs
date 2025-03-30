pub use map::*;
pub use set::*;
pub use string::*;
pub use vector::*;

macro_rules! wirable {
  (@byte) => {
    const WIRE_TYPE: $crate::WireType = $crate::WireType::Byte;
  };
  (@varint) => {
    const WIRE_TYPE: $crate::WireType = $crate::WireType::Varint;
  };
  (@length_delimited) => {
    const WIRE_TYPE: $crate::WireType = $crate::WireType::LengthDelimited;
  };
  (@nothing) => {
    const WIRE_TYPE: $crate::WireType = $crate::WireType::Merged;
  };
  (@fixed16) => {
    const WIRE_TYPE: $crate::WireType = $crate::WireType::Fixed16;
  };
  (@fixed32) => {
    const WIRE_TYPE: $crate::WireType = $crate::WireType::Fixed32;
  };
  (@fixed64) => {
    const WIRE_TYPE: $crate::WireType = $crate::WireType::Fixed64;
  };
  (@fixed128) => {
    const WIRE_TYPE: $crate::WireType = $crate::WireType::Fixed128;
  };
  (@outer ($wire_varint:ident) <=> ($($ty:ty), +$(,)?)) => {
    $(
      impl $crate::Wirable for $ty {
        wirable!(@$wire_varint);
      }
    )*
  }
}

macro_rules! impl_type_conversion_for_self {
  (@copy $($ty:ty), +$(,)?) => {
    $(
      impl $crate::TypeRef<Self> for $ty {
        fn to_target(&self) -> ::core::result::Result<Self, $crate::DecodeError> {
          ::core::result::Result::Ok(*self)
        }

        fn into_target(self) -> ::core::result::Result<Self, $crate::DecodeError> {
          ::core::result::Result::Ok(self)
        }
      }

      impl $crate::TypeOwned<Self> for $ty {
        fn to_target(&self) -> ::core::result::Result<Self, $crate::DecodeError> {
          ::core::result::Result::Ok(*self)
        }

        fn into_target(self) -> ::core::result::Result<Self, $crate::DecodeError> {
          ::core::result::Result::Ok(self)
        }
      }
    )*
  };
  (@clone $($ty:ty), +$(,)?) => {
    $(
      impl $crate::TypeRef<Self> for $ty {
        fn to_target(&self) -> ::core::result::Result<Self, $crate::DecodeError> {
          ::core::result::Result::Ok(self.clone())
        }

        fn into_target(self) -> ::core::result::Result<Self, $crate::DecodeError> {
          ::core::result::Result::Ok(self)
        }
      }

      impl $crate::TypeOwned<Self> for $ty {
        fn to_target(&self) -> ::core::result::Result<Self, $crate::DecodeError> {
          ::core::result::Result::Ok(self.clone())
        }

        fn into_target(self) -> ::core::result::Result<Self, $crate::DecodeError> {
          ::core::result::Result::Ok(self)
        }
      }
    )*
  };
}

macro_rules! impl_output_type_for_self {
  () => {
    type Serialized<'a>
      = Self
    where
      Self: Sized + 'a;

    type Borrowed<'a>
      = &'a Self
    where
      Self: 'a;

    type SerializedOwned
      = Self
    where
      Self: Sized + 'static;
  };
  (@owned_borrow) => {
    type Serialized<'a>
      = Self
    where
      Self: Sized + 'a;

    type Borrowed<'a>
      = Self
    where
      Self: 'a;

    type SerializedOwned
      = Self
    where
      Self: Sized + 'static;
  };
  (@outer ($($ty:ident), +$(,)?)) => {
    $(
      impl $crate::OutputType for $ty {
        impl_output_type_for_self!();
      }
    )*
  };
  (@outer & owned_borrow ($($ty:ident), +$(,)?)) => {
    $(
      impl $crate::OutputType for $ty {
        impl_output_type_for_self!(@owned_borrow);
      }
    )*
  };
}

macro_rules! impl_varing_types {
  (@serialize) => {
    fn encode(&self, _: $crate::Tag, buf: &mut [u8]) -> Result<usize, $crate::EncodeError> {
      varing::Varint::encode(self, buf).map_err(Into::into)
    }

    fn encoded_len(&self, _: $crate::Tag) -> usize {
      varing::Varint::encoded_len(self)
    }
  };
  (@deserialize) => {
    fn decode<B>(src: &'de [u8], _: &mut B) -> Result<(usize, Self), $crate::DecodeError>
    where
      Self: Sized + 'de,
      B: $crate::UnknownRefBuffer<'de>,
    {
      varing::Varint::decode(src).map_err(Into::into)
    }
  };
  (@deserialize_owned) => {
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn decode_from_bytes<U>(
      src: bytes_1::Bytes,
      _: &mut U,
    ) -> Result<(usize, Self), $crate::DecodeError>
    where
      Self: Sized + 'static,
      U: $crate::UnknownBuffer<bytes_1::Bytes>,
    {
      varing::Varint::decode(src.as_ref()).map_err(Into::into)
    }
  };
  (@output_type) => {
    type Serialized<'a>
      = Self
    where
      Self: Sized + 'a;

    type Borrowed<'a>
      = Self
    where
      Self: 'a;

    type SerializedOwned
      = Self
    where
      Self: Sized + 'static;
  };
  (@type_ref) => {
    fn to_target(&self) -> Result<Self, $crate::DecodeError> {
      Ok(*self)
    }

    fn into_target(self) -> Result<Self, $crate::DecodeError> {
      Ok(self)
    }
  };
  (@type_owned) => {
    fn to_target(&self) -> Result<Self, $crate::DecodeError> {
      Ok(*self)
    }

    fn into_target(self) -> Result<Self, $crate::DecodeError> {
      Ok(self)
    }
  };
}

mod builtin;
mod duration;
mod map;
mod net;
#[cfg(feature = "std")]
mod path;
mod set;
mod string;
mod vector;

/// Re-export types of [`arbitrary-int`](arbitrary_int_1).
#[cfg(feature = "arbitrary-int")]
pub mod arbitrary_int;
#[cfg(feature = "bigdecimal")]
mod bigdecimal;
#[cfg(feature = "chrono")]
mod chrono;
#[cfg(feature = "chrono-tz")]
mod chrono_tz;
#[cfg(feature = "decimal")]
mod decimal;
/// Re-export types of [`ruint`](ruint_1).
#[cfg(feature = "ruint")]
pub mod ruint;
#[cfg(feature = "url")]
mod url;
#[cfg(feature = "uuid")]
mod uuid;
