
/// A macro emits traits implementations for a zero-sized type.
/// 
/// ## Example
/// 
/// ```rust
/// use grost::zst;
/// 
/// #[derive(Default)]
/// struct MyZst;
/// 
/// #[derive(Default)]
/// #[repr(transparent)]
/// struct MyZstTwo(MyZst);
/// 
/// zst!(MyZst, MyZstTwo);
/// ```
#[macro_export]
macro_rules! zst {
  ($($ty:ty), +$(,)?) => {
    $(
      const _: () = {
        const fn __assert<T>() where T: ::core::default::Default {
          assert!(::core::mem::size_of::<T>() == 0, "Not a zero-sized type");
        }

        __assert::<$ty>();
      };

      impl $crate::Message for $ty {
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
      }

      impl $crate::TypeOwned<Self> for $ty {
        fn to(&self) -> ::core::result::Result<Self, $crate::DecodeError> {
          ::core::result::Result::Ok(::core::default::Default::default())
        }
      }

      impl $crate::TypeRef<Self> for $ty {
        fn to(&self) -> ::core::result::Result<Self, $crate::DecodeError> {
          ::core::result::Result::Ok(::core::default::Default::default())
        }
      }

      $crate::into_target!($ty);

      impl $crate::Wirable for $ty {
        const WIRE_TYPE: $crate::WireType = $crate::WireType::Merged;
      }
      
      impl $crate::Serialize for $ty {
        #[inline]
        fn encode(&self, _: $crate::Tag, _: &mut [u8]) -> ::core::result::Result<::core::primitive::usize, $crate::EncodeError> {
          ::core::result::Result::Ok(0)
        }
    
        #[inline]
        fn encoded_len(&self, _: $crate::Tag) -> ::core::primitive::usize {
          0
        }
      }
      
      impl<'de> $crate::Deserialize<'de> for $ty {
        fn decode<B>(_: &'de [u8], _: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::DecodeError>
        where
          Self: ::core::marker::Sized + 'de,
          B: $crate::UnknownRefBuffer<'de>,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }
      }
      
      impl $crate::DeserializeOwned for $ty
      where
        Self: 'static,
      {
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn decode_from_bytes<U>(
          _: $crate::bytes::Bytes,
          _: &mut U,
        ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::DecodeError>
        where
          Self: ::core::marker::Sized + 'static,
          U: $crate::UnknownBuffer<$crate::bytes::Bytes>,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }
      }

      impl $crate::PartialSerialize for $ty {
        type Selection = ();

        #[inline]
        fn partial_encode(&self, _: $crate::Tag, _: &Self::Selection, _: &mut [u8]) -> ::core::result::Result<::core::primitive::usize, $crate::EncodeError> {
          ::core::result::Result::Ok(0)
        }
      
        #[inline]
        fn partial_encoded_len(&self, _: $crate::Tag, _: &Self::Selection,) -> ::core::primitive::usize {
          0
        }
      }
    )*
  };
}

/// A macro emits traits implementations for a type that through a bridge to another type.
/// 
/// ## Example
/// 
/// ```rust
/// use grost::bridge;
/// 
/// struct MyU8(u8);
/// 
/// struct MyU32(u32);
/// 
/// struct MyOtherU32 {
///   low: u16,
///   high: u16,
/// }
/// 
/// // Examples of bridge implementations
/// bridge! {
///   u8 {
///     MyU8 {
///       from: |v: u8| Self(v),
///       to: |v: &Self| v.0,
///     }
///   },
///   u32 {
///     MyU32 {
///       from: |v: u32| Self(v),
///       to: |v: &Self| v.0,
///     },
///     MyOtherU32 {
///       from: |v: u32| {
///         Self {
///           low: v as u16,
///           high: (v >> 16) as u16,
///         }
///       },
///       to: |v: &Self| {
///         (v.low as u32) | ((v.high as u32) << 16)
///       },
///     }
///   }
/// }
/// ```
#[macro_export]
macro_rules! bridge {
  ($(
    $bridge: ty {
      $($ty:ty {
        from: $from:expr,
        to: $to:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::bridge!(@wirable $bridge {
          $ty,
        });

        $crate::bridge!(@serialize $bridge {
          $ty {
            to: $to,
          }
        });

        $crate::bridge!(@partial_serialize $bridge {
          $ty {
            to: $to,
          }
        });

        $crate::bridge!(@deserialize $bridge {
          $ty {
            from: $from,
          }
        });

        $crate::bridge!(@deserialize_owned $bridge {
          $ty {
            from: $from,
          }
        });
      )*
    )*
  };
  (@serialize_impl $bridge:ty => $to:expr) => {
    fn encode(&self, tag: $crate::Tag, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::EncodeError> {
      <$bridge as $crate::Serialize>::encode(&$to(self), tag, buf)
    }

    fn encoded_len(&self, tag: $crate::Tag) -> ::core::primitive::usize {
      <$bridge as $crate::Serialize>::encoded_len(&$to(self), tag)
    }
  };
  (@serialize $(
    $bridge:ty {
      $($ty:ty {
        to: $to:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::Serialize for $ty {
          $crate::bridge!(@serialize_impl $bridge => $to);
        }
      )*
    )*
  };
  (@partial_serialize_impl $bridge:ty => $to:expr) => {
    fn partial_encode(&self, tag: $crate::Tag, selection: &Self::Selection, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::EncodeError> {
      <$bridge as $crate::PartialSerialize>::partial_encode(&$to(self), tag, selection, buf)
    }

    fn partial_encoded_len(&self, tag: $crate::Tag, selection: &Self::Selection) -> ::core::primitive::usize {
      <$bridge as $crate::PartialSerialize>::partial_encoded_len(&$to(self), tag, selection)
    }
  };
  (@partial_serialize $(
    $bridge:ty {
      $($ty:ty {
        to: $to:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::PartialSerialize for $ty {
          type Selection = <$bridge as $crate::PartialSerialize>::Selection;

          $crate::bridge!(@partial_serialize_impl $bridge => $to);
        }
      )*
    )*
  };
  (@deserialize_impl $from:expr => $bridge:ty) => {
    fn decode<B>(src: &'de [::core::primitive::u8], ub: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::DecodeError>
      where
        Self: ::core::marker::Sized + 'de,
        B: $crate::UnknownRefBuffer<'de> {
      <$bridge as $crate::Deserialize>::decode(src, ub).map(|(n, v)| (n, $from(v)))
    }
  };
  (@deserialize $(
    $bridge:ty {
      $($ty:ty {
        from: $from:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de> $crate::Deserialize<'de> for $ty {
          $crate::bridge!(@deserialize_impl $from => $bridge);
        }
      )*
    )*
  };
  (@deserialize_owned_impl $from:expr => $bridge:ty) => {
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn decode_from_bytes<U>(
      src: $crate::bytes::Bytes,
      ub: &mut U,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      U: $crate::UnknownBuffer<$crate::bytes::Bytes> {
      <$bridge as $crate::DeserializeOwned>::decode_from_bytes(src, ub).map(|(n, v)| (n, $from(v)))
    }
  };
  (@deserialize_owned $(
    $bridge:ty {
      $($ty:ty {
        from: $from:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::DeserializeOwned for $ty {
          $crate::bridge!(@deserialize_owned_impl $from => $bridge);
        }
      )*
    )*
  };
  (@wirable $bridge:ty {
    $($ty:ty), +$(,)?
  }) => {
    $(
      impl $crate::Wirable for $ty {
        const WIRE_TYPE: $crate::WireType = <$bridge as $crate::Wirable>::WIRE_TYPE;
      }
    )*
  };
}

/// A macro emits [`TypeRef`](super::TypeRef) implementations for `Self`
#[macro_export]
macro_rules! type_ref {
  ($($ty:ty),+$(,)?) => {
    $(
      impl $crate::TypeRef<Self> for $ty {
        $crate::type_ref!(@copy_impl);
      }
    )*
  };
  (@clone $($ty:ty),+$(,)?) => {
    $(
      impl $crate::TypeRef<Self> for $ty {
        $crate::type_ref!(@clone_impl);
      }
    )*
  };
  (@copy_impl) => {
    fn to(&self) -> ::core::result::Result<Self, $crate::DecodeError> {
      ::core::result::Result::Ok(*self)
    }
  };
  (@clone_impl) => {
    fn to(&self) -> ::core::result::Result<Self, $crate::DecodeError> {
      ::core::result::Result::Ok(::core::clone::Clone::clone(self))
    }
  }
}

/// A macro emits [`TypeOwned`](super::TypeOwned) implementations for `Self`
#[macro_export]
macro_rules! type_owned {
  ($($ty:ty),+$(,)?) => {
    $(
      impl $crate::TypeOwned<Self> for $ty {
        $crate::type_ref!(@copy_impl);
      }
    )*
  };
  (@clone $($ty:ty),+$(,)?) => {
    $(
      impl $crate::TypeOwned<Self> for $ty {
        $crate::type_ref!(@clone_impl);
      }
    )*
  };
  (@copy_impl) => {
    fn to(&self) -> ::core::result::Result<Self, $crate::DecodeError> {
      ::core::result::Result::Ok(*self)
    }
  };
  (@clone_impl) => {
    fn to(&self) -> ::core::result::Result<Self, $crate::DecodeError> {
      ::core::result::Result::Ok(::core::clone::Clone::clone(self))
    }
  }
}

/// A macro emits [`IntoTarget`](super::IntoTarget) implementations for `Self`
#[macro_export]
macro_rules! into_target {
  ($($ty:ty),+$(,)?) => {
    $(
      impl $crate::IntoTarget<Self> for $ty {
        $crate::into_target!(@impl);
      }
    )*
  };
  (@impl) => {
    fn into_target(self) -> ::core::result::Result<Self, $crate::DecodeError> {
      ::core::result::Result::Ok(self)
    }
  }
}

/// A macro emits convertion traits implementations for `Self`
#[macro_export]
macro_rules! conversion {
  ($($ty:ty),+$(,)?) => {
    $crate::type_ref!($($ty),+);
    $crate::type_owned!($($ty),+);
    $crate::into_target!($($ty),+);
  };
  (@clone $($ty:ty),+$(,)?) => {
    $crate::type_ref!(@clone $($ty),+);
    $crate::type_owned!(@clone $($ty),+);
    $crate::into_target!($($ty),+);
  };
}

/// A macro emits [`Message`](super::Message) implementations for `Self`.
/// 
/// **NB:** this macro can only be used for types that implements [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! message {
  ($($ty:ty),+$(,)?) => {
    $(
      impl $crate::Message for $ty {
        $crate::message!(@impl);
      }
    )*

    $crate::conversion!($($ty),+);
  };
  (@impl) => {
    type Serialized<'a>
      = Self
    where
      Self: ::core::marker::Sized + 'a;

    type Borrowed<'a>
      = &'a Self
    where
      Self: 'a;

    type SerializedOwned
      = Self
    where
      Self: ::core::marker::Sized + 'static;
  };
}

/// A macro emits [`PartialSerialize`](super::PartialSerialize) implementations for primitive types.
#[macro_export]
macro_rules! partial_serialize_primitives {
  (@impl) => {
    type Selection = ();
  
    fn partial_encode(&self, tag: $crate::Tag, _: &Self::Selection, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::EncodeError> {
      <Self as $crate::Serialize>::encode(self, tag, buf)
    }

    fn partial_encoded_len(&self, tag: $crate::Tag, _: &Self::Selection) -> ::core::primitive::usize {
      <Self as $crate::Serialize>::encoded_len(self, tag)
    }
  };
  ($($ty:ty),+$(,)?) => {
    $(
      impl $crate::PartialSerialize for $ty {
        $crate::partial_serialize_primitives!(@impl);
      }
    )*
  };
}

/// A macro emits [`Wirable`](super::Wirable) implementations for given types.
/// 
/// ## Example
/// 
/// ```rust
/// use grost::wirable;
/// 
/// struct FixedU16(u16);
/// struct FixedU16Le(u16);
/// struct FixedU16Be(u16);
/// 
/// struct FixedU32(u32);
/// struct FixedU32Le(u32);
/// struct FixedU32Be(u32);
/// 
/// wirable! {
///   (@fixed16) <=> (FixedU16, FixedU16Le, FixedU16Be),
///   (@fixed32) <=> (FixedU32, FixedU32Le, FixedU32Be),
/// }
/// ```
#[macro_export]
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
  ($(
    (@$wire_varint:ident) <=> ($($ty:ty), +$(,)?)
  ),+$(,)?) => {
    $(
      $(
        impl $crate::Wirable for $ty {
          $crate::wirable!(@$wire_varint);
        }
      )*
    )*
  }
}

/// A macro emits traits implementations for primitive types that implements [`varing::Varint`](varing::Varint) and [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! varing {
  ($($ty:ty),+$(,)?) => {
    $crate::wirable!((@varint) <=> ($($ty,)*));
    $crate::message!($($ty),*);
    $crate::partial_serialize_primitives!($($ty),*);
    $crate::varing!(@serialize $($ty),+);
    $crate::varing!(@deserialize $($ty),+);
    $crate::varing!(@deserialize_owned $($ty),+);
  };
  (@serialize $($ty:ty), +$(,)?) => {
    $(
      impl $crate::Serialize for $ty {
        $crate::varing!(@serialize_impl);
      }
    )*
  };
  (@serialize_impl) => {
    fn encode(&self, _: $crate::Tag, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<usize, $crate::EncodeError> {
      $crate::__private::varing::Varint::encode(self, buf).map_err(::core::convert::Into::into)
    }

    fn encoded_len(&self, _: $crate::Tag) -> ::core::primitive::usize {
      $crate::__private::varing::Varint::encoded_len(self)
    }
  };
  (@deserialize $($ty:ty), +$(,)?) => {
    $(
      impl<'de> $crate::Deserialize<'de> for $ty {
        $crate::varing!(@deserialize_impl);
      }
    )*
  };
  (@deserialize_impl) => {
    fn decode<B>(src: &'de [::core::primitive::u8], _: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      B: $crate::UnknownRefBuffer<'de>,
    {
      $crate::__private::varing::Varint::decode(src).map_err(::core::convert::Into::into)
    }
  };
  (@deserialize_owned $($ty:ty), +$(,)?) => {
    $(
      impl $crate::DeserializeOwned for $ty {
        $crate::varing!(@deserialize_owned_impl);
      }
    )*
  };
  (@deserialize_owned_impl) => {
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn decode_from_bytes<U>(
      src: $crate::bytes::Bytes,
      _: &mut U,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      U: $crate::UnknownBuffer<$crate::bytes::Bytes>,
    {
      $crate::__private::varing::Varint::decode(::core::convert::AsRef::as_ref(&src)).map_err(::core::convert::Into::into)
    }
  };
}
