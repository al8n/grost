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

      impl $crate::__private::Message for $ty {
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

      impl $crate::__private::TypeOwned<Self> for $ty {
        fn to(&self) -> ::core::result::Result<Self, $crate::__private::DecodeError> {
          ::core::result::Result::Ok(::core::default::Default::default())
        }
      }

      impl $crate::__private::TypeRef<Self> for $ty {
        fn to(&self) -> ::core::result::Result<Self, $crate::__private::DecodeError> {
          ::core::result::Result::Ok(::core::default::Default::default())
        }
      }

      $crate::into_target!($ty);

      impl $crate::Wirable for $ty {
        const WIRE_TYPE: $crate::WireType = $crate::WireType::Merged;
      }

      impl $crate::__private::Serialize for $ty {
        #[inline]
        fn encode(&self, _: &mut [u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn encoded_len(&self) -> ::core::primitive::usize {
          0
        }
      }

      impl<'de> $crate::__private::Deserialize<'de> for $ty {
        fn decode<B>(_: &'de [u8], _: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
        where
          Self: ::core::marker::Sized + 'de,
          B: $crate::__private::UnknownRefBuffer<'de>,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }
      }

      impl $crate::__private::DeserializeOwned for $ty
      where
        Self: 'static,
      {
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn decode_from_bytes<U>(
          _: $crate::__private::bytes::Bytes,
          _: &mut U,
        ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
        where
          Self: ::core::marker::Sized + 'static,
          U: $crate::__private::UnknownBuffer<$crate::__private::bytes::Bytes>,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }
      }

      impl $crate::__private::PartialSerialize for $ty {
        type Selection = ();

        #[inline]
        fn partial_encode(&self, _: &Self::Selection, _: &mut [u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn partial_encoded_len(&self, _: &Self::Selection,) -> ::core::primitive::usize {
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
    fn encode(&self, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
      <$bridge as $crate::__private::Serialize>::encode(&$to(self), buf)
    }

    fn encoded_len(&self) -> ::core::primitive::usize {
      <$bridge as $crate::__private::Serialize>::encoded_len(&$to(self))
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
        impl $crate::__private::Serialize for $ty {
          $crate::bridge!(@serialize_impl $bridge => $to);
        }
      )*
    )*
  };
  (@partial_serialize_impl $bridge:ty => $to:expr) => {
    fn partial_encode(&self, selection: &Self::Selection, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
      <$bridge as $crate::__private::PartialSerialize>::partial_encode(&$to(self), selection, buf)
    }

    fn partial_encoded_len(&self, selection: &Self::Selection) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialSerialize>::partial_encoded_len(&$to(self), selection)
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
        impl $crate::__private::PartialSerialize for $ty {
          type Selection = <$bridge as $crate::__private::PartialSerialize>::Selection;

          $crate::bridge!(@partial_serialize_impl $bridge => $to);
        }
      )*
    )*
  };
  (@deserialize_impl $from:expr => $bridge:ty) => {
    fn decode<B>(src: &'de [::core::primitive::u8], ub: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
      where
        Self: ::core::marker::Sized + 'de,
        B: $crate::__private::UnknownRefBuffer<'de> {
      <$bridge as $crate::__private::Deserialize>::decode(src, ub).map(|(n, v)| (n, $from(v)))
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
        impl<'de> $crate::__private::Deserialize<'de> for $ty {
          $crate::bridge!(@deserialize_impl $from => $bridge);
        }
      )*
    )*
  };
  (@deserialize_owned_impl $from:expr => $bridge:ty) => {
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn decode_from_bytes<U>(
      src: $crate::__private::bytes::Bytes,
      ub: &mut U,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      U: $crate::__private::UnknownBuffer<$crate::__private::bytes::Bytes> {
      <$bridge as $crate::__private::DeserializeOwned>::decode_from_bytes(src, ub).map(|(n, v)| (n, $from(v)))
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
        impl $crate::__private::DeserializeOwned for $ty {
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

/// A macro emits traits implementation for types underlying is a `&str`.
///
/// This macro requires the types are `'static`
///
/// ## Example
///
/// ```rust
/// use grost::{str_bridge, smol_str::SmolStr};
///
/// struct MyString(String);
///
/// impl MyString {
///   fn as_str(&self) -> &str {
///     self.0.as_str()
///   }
/// }
///
/// str_bridge!(
///   MyString {
///     from_str: |s: &str| MyString(s.to_string()),
///     to_str: MyString::as_str,
///   }
/// );
/// ```
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
#[macro_export]
macro_rules! str_bridge {
  ($($ty:ty {
    from_str: $from_str: expr,
    to_str: $to_str: expr,
  }), +$(,)?) => {
    $(
      impl $crate::Wirable for $ty {}

      $crate::str_bridge!(@serialize $ty: $to_str);
      $crate::str_bridge!(@deserialize $ty: $from_str);
      $crate::str_bridge!(@deserialize_owned $ty: $from_str);
      $crate::str_bridge!(@str_conversion $ty: $from_str);
    )*
  };
  (@serialize $ty:ty: $to_str:expr) => {
    impl $crate::__private::Serialize for $ty {
      $crate::str_bridge!(@serialize_impl $to_str);
    }
  };
  (@serialize_impl $to_str:expr) => {
    #[inline]
    fn encode(&self, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
      <&::core::primitive::str as $crate::__private::Serialize>::encode(&$to_str(self), buf)
    }

    #[inline]
    fn encoded_len(&self) -> ::core::primitive::usize {
      <&::core::primitive::str as $crate::__private::Serialize>::encoded_len(&$to_str(self))
    }
  };
  (@deserialize $ty:ty: $from_str:expr) => {
    impl<'de> $crate::__private::Deserialize<'de> for $ty {
      $crate::str_bridge!(@deserialize_impl $from_str);
    }
  };
  (@deserialize_impl $from_str:expr) => {
    fn decode<B>(src: &'de [u8], _: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      B: $crate::__private::UnknownRefBuffer<'de>
    {
      $crate::__private::from_utf8(src)
        .map(|s| (src.len(), s))
        .map_err(|_| $crate::__private::DecodeError::custom("invalid UTF-8"))
        .map(|(read, val)| (read, $from_str(val)))
    }
  };
  (@deserialize_owned $ty:ty: $from_str:expr) => {
    impl $crate::__private::DeserializeOwned for $ty {
      $crate::str_bridge!(@deserialize_owned_impl $from_str);
    }
  };
  (@deserialize_owned_impl $from_str:expr) => {
    fn decode_from_bytes<U>(
      src: $crate::__private::bytes::Bytes,
      _: &mut U,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      U: $crate::__private::UnknownBuffer<$crate::__private::bytes::Bytes>
    {
      $crate::__private::from_utf8(&src)
        .map(|s| (src.len(), s))
        .map_err(|_| $crate::__private::DecodeError::custom("invalid UTF-8"))
        .map(|(read, val)| (read, $from_str(val)))
    }
  };
  (@str_conversion $ty:ty: $from_str:expr) => {
    impl $crate::__private::IntoTarget<$ty> for &::core::primitive::str {
      $crate::str_bridge!(@into_target_impl $ty: $from_str);
    }

    impl $crate::__private::TypeRef<$ty> for &::core::primitive::str {
      $crate::str_bridge!(@str_to_impl $ty: $from_str);
    }

    impl $crate::__private::TypeOwned<$ty> for &::core::primitive::str {
      $crate::str_bridge!(@str_to_impl $ty: $from_str);
    }
  };
  (@into_target_impl $ty:ty: $from:expr) => {
    fn into_target(self) -> ::core::result::Result<$ty, $crate::__private::DecodeError> {
      ::core::result::Result::Ok($from(self))
    }
  };
  (@str_to_impl $ty:ty: $from:expr) => {
    fn to(&self) -> ::core::result::Result<$ty, $crate::__private::DecodeError> {
      ::core::result::Result::Ok($from(*self))
    }
  };
  (@smolstr_to_impl $ty:ty: $from:expr) => {
    fn to(&self) -> ::core::result::Result<$ty, $crate::__private::DecodeError> {
      ::core::result::Result::Ok($from(self))
    }
  };
  (@smolstr_message $($ty:ty {
    from_ref: $from_ref: expr,
    from: $from: expr,
  }), +$(,)?) => {
    $(
      impl $crate::__private::IntoTarget<$ty> for $crate::__private::smol_str::SmolStr {
        $crate::str_bridge!(@into_target_impl $ty: $from);
      }

      impl $crate::__private::TypeOwned<$ty> for $crate::__private::smol_str::SmolStr {
        $crate::str_bridge!(@smolstr_to_impl $ty: $from_ref);
      }

      impl $crate::__private::Message for $ty {
        type Serialized<'a> = &'a ::core::primitive::str
        where
          Self: ::core::marker::Sized + 'a;

        type Borrowed<'a> = &'a $ty
        where
          Self: 'a;

        type SerializedOwned = $crate::__private::smol_str::SmolStr
        where
          Self: ::core::marker::Sized + 'static;
      }
    )*
  }
}

/// A macro emits traits implementations for a array-style `str` type.
#[macro_export]
macro_rules! array_str {
  (
    impl<const $g:ident: usize> $ty:ty {
      fn new = $new:expr;

      fn from_str = $from_str:expr;

      fn decode = $decode:expr;

      fn as_bytes = $as_bytes:expr;
    }
  ) => {
    impl<const $g: ::core::primitive::usize> $crate::__private::Wirable for $ty {
      const WIRE_TYPE: $crate::__private::WireType = {
        match N {
          0 => $crate::__private::WireType::Merged,
          _ => $crate::__private::WireType::LengthDelimited,
        }
      };
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::Serialize for $ty {
      fn encode(
        &self,
        buf: &mut [::core::primitive::u8],
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
        if N == 0 {
          return ::core::result::Result::Ok(0);
        }

        $crate::__private::Serialize::encode(&self.as_bytes(), buf)
      }

      #[inline]
      fn encoded_len(&self) -> ::core::primitive::usize {
        if N == 0 {
          return 0;
        }

        $crate::__private::Serialize::encoded_len(&self.as_bytes())
      }

      fn encoded_len_with_prefix(&self) -> ::core::primitive::usize {
        if N == 0 {
          return 0;
        }

        $crate::__private::Serialize::encoded_len_with_prefix(&self.as_bytes())
      }

      fn encode_with_prefix(
        &self,
        buf: &mut [::core::primitive::u8],
      ) -> Result<::core::primitive::usize, $crate::__private::EncodeError> {
        if N == 0 {
          return ::core::result::Result::Ok(0);
        }

        $crate::__private::Serialize::encode_with_prefix(&self.as_bytes(), buf)
      }
    }

    impl<'de, const $g: ::core::primitive::usize> $crate::__private::Deserialize<'de> for $ty {
      fn decode<B>(
        src: &'de [u8],
        _: &mut B,
      ) -> Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
      where
        Self: ::core::marker::Sized + 'de,
        B: $crate::__private::UnknownRefBuffer<'de>,
      {
        if N == 0 {
          return ::core::result::Result::Ok((0, $new()));
        }

        if src.len() > N {
          return ::core::result::Result::Err($crate::__private::larger_than_str_capacity::<N>());
        }

        $decode(src)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::DeserializeOwned for $ty {
      fn decode_from_bytes<U>(
        src: $crate::__private::bytes::Bytes,
        _: &mut U,
      ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
      where
        Self: ::core::marker::Sized + 'static,
        U: $crate::__private::UnknownBuffer<$crate::__private::bytes::Bytes>,
      {
        if N == 0 {
          return ::core::result::Result::Ok((0, $new()));
        }

        if src.len() > N {
          return ::core::result::Result::Err($crate::__private::larger_than_str_capacity::<N>());
        }

        $decode(::core::convert::AsRef::<[u8]>::as_ref(&src))
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::IntoTarget<Self> for $ty {
      fn into_target(self) -> ::core::result::Result<Self, $crate::__private::DecodeError> {
        ::core::result::Result::Ok(self)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::TypeRef<Self> for $ty {
      fn to(&self) -> ::core::result::Result<Self, $crate::__private::DecodeError> {
        ::core::result::Result::Ok(::core::clone::Clone::clone(self))
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::TypeOwned<Self> for $ty {
      fn to(&self) -> ::core::result::Result<Self, $crate::__private::DecodeError> {
        ::core::result::Result::Ok(::core::clone::Clone::clone(self))
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::IntoTarget<$ty> for &str {
      fn into_target(self) -> ::core::result::Result<$ty, $crate::__private::DecodeError> {
        $from_str(self)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::TypeRef<$ty> for &str {
      fn to(&self) -> ::core::result::Result<$ty, $crate::__private::DecodeError> {
        $from_str(*self)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::Message for $ty {
      type Serialized<'a>
        = &'a ::core::primitive::str
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
    }
  };
}

/// A macro emits [`TypeRef`](super::TypeRef) implementations for `Self`
#[macro_export]
macro_rules! type_ref {
  ($($ty:ty),+$(,)?) => {
    $(
      impl $crate::__private::TypeRef<Self> for $ty {
        $crate::type_ref!(@copy_impl);
      }
    )*
  };
  (@clone $($ty:ty),+$(,)?) => {
    $(
      impl $crate::__private::TypeRef<Self> for $ty {
        $crate::type_ref!(@clone_impl);
      }
    )*
  };
  (@copy_impl) => {
    fn to(&self) -> ::core::result::Result<Self, $crate::__private::DecodeError> {
      ::core::result::Result::Ok(*self)
    }
  };
  (@clone_impl) => {
    fn to(&self) -> ::core::result::Result<Self, $crate::__private::DecodeError> {
      ::core::result::Result::Ok(::core::clone::Clone::clone(self))
    }
  }
}

/// A macro emits [`TypeOwned`](super::TypeOwned) implementations for `Self`
#[macro_export]
macro_rules! type_owned {
  ($($ty:ty),+$(,)?) => {
    $(
      impl $crate::__private::TypeOwned<Self> for $ty {
        $crate::type_ref!(@copy_impl);
      }
    )*
  };
  (@clone $($ty:ty),+$(,)?) => {
    $(
      impl $crate::__private::TypeOwned<Self> for $ty {
        $crate::type_ref!(@clone_impl);
      }
    )*
  };
  (@copy_impl) => {
    fn to(&self) -> ::core::result::Result<Self, $crate::__private::DecodeError> {
      ::core::result::Result::Ok(*self)
    }
  };
  (@clone_impl) => {
    fn to(&self) -> ::core::result::Result<Self, $crate::__private::DecodeError> {
      ::core::result::Result::Ok(::core::clone::Clone::clone(self))
    }
  }
}

/// A macro emits [`IntoTarget`](super::IntoTarget) implementations for `Self`
#[macro_export]
macro_rules! into_target {
  ($($ty:ty),+$(,)?) => {
    $(
      impl $crate::__private::IntoTarget<Self> for $ty {
        $crate::into_target!(@impl);
      }
    )*
  };
  (@impl) => {
    fn into_target(self) -> ::core::result::Result<Self, $crate::__private::DecodeError> {
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
      impl $crate::__private::Message for $ty {
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

    fn partial_encode(&self, _: &Self::Selection, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
      <Self as $crate::__private::Serialize>::encode(self, buf)
    }

    fn partial_encoded_len(&self, _: &Self::Selection) -> ::core::primitive::usize {
      <Self as $crate::__private::Serialize>::encoded_len(self)
    }
  };
  ($($ty:ty),+$(,)?) => {
    $(
      impl $crate::__private::PartialSerialize for $ty {
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
macro_rules! varint {
  ($($ty:ty),+$(,)?) => {
    $crate::wirable!((@varint) <=> ($($ty,)*));
    $crate::message!($($ty),*);
    $crate::partial_serialize_primitives!($($ty),*);
    $crate::varint!(@serialize $($ty),+);
    $crate::varint!(@deserialize $($ty),+);
    $crate::varint!(@deserialize_owned $($ty),+);
  };
  (@serialize $($ty:ty), +$(,)?) => {
    $(
      impl $crate::__private::Serialize for $ty {
        $crate::varint!(@serialize_impl);
      }
    )*
  };
  (@serialize_impl) => {
    fn encode(&self, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<usize, $crate::__private::EncodeError> {
      $crate::__private::varing::Varint::encode(self, buf).map_err(::core::convert::Into::into)
    }

    fn encoded_len(&self) -> ::core::primitive::usize {
      $crate::__private::varing::Varint::encoded_len(self)
    }
  };
  (@deserialize $($ty:ty), +$(,)?) => {
    $(
      impl<'de> $crate::__private::Deserialize<'de> for $ty {
        $crate::varint!(@deserialize_impl);
      }
    )*
  };
  (@deserialize_impl) => {
    fn decode<B>(src: &'de [::core::primitive::u8], _: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      B: $crate::__private::UnknownRefBuffer<'de>,
    {
      $crate::__private::varing::Varint::decode(src).map_err(::core::convert::Into::into)
    }
  };
  (@deserialize_owned $($ty:ty), +$(,)?) => {
    $(
      impl $crate::__private::DeserializeOwned for $ty {
        $crate::varint!(@deserialize_owned_impl);
      }
    )*
  };
  (@deserialize_owned_impl) => {
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn decode_from_bytes<U>(
      src: $crate::__private::bytes::Bytes,
      _: &mut U,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      U: $crate::__private::UnknownBuffer<$crate::__private::bytes::Bytes>,
    {
      $crate::__private::varing::Varint::decode(::core::convert::AsRef::as_ref(&src)).map_err(::core::convert::Into::into)
    }
  };
}

/// A macro emits traits implementations for `PhantomData<T>` like types.
///
/// ## Example
///
/// ```rust
/// use grost::phantom;
/// use core::marker::PhantomData;
///
/// #[repr(transparent)]
/// struct MyPhantom<T: ?Sized>(PhantomData<T>);
///
/// impl<T: ?Sized> Default for MyPhantom<T> {
///   fn default() -> Self {
///     Self(PhantomData)
///   }
/// }
///
/// impl<T: ?Sized> Clone for MyPhantom<T> {
///  fn clone(&self) -> Self {
///    *self
///  }
/// }
///
/// impl<T: ?Sized> Copy for MyPhantom<T> {}
///
/// phantom!(MyPhantom<T>);
/// ```
macro_rules! phantom {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<T: ?::core::marker::Sized> $crate::Wirable for $ty {
        const WIRE_TYPE: $crate::WireType = {
          assert!(::core::mem::size_of::<Self>() == 0, "Not a zero-sized type");

          $crate::WireType::Merged
        };
      }

      impl<T: ?::core::marker::Sized> $crate::__private::Serialize for $ty {
        #[inline]
        fn encode(&self, _: &mut [u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn encoded_len(&self) -> ::core::primitive::usize {
          0
        }
      }

      impl<'de, T: ?::core::marker::Sized> $crate::__private::Deserialize<'de> for $ty {
        fn decode<B>(_: &'de [u8], _: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
        where
          Self: ::core::marker::Sized + 'de,
          B: $crate::__private::UnknownRefBuffer<'de>,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }
      }

      impl<T: ?::core::marker::Sized> $crate::__private::DeserializeOwned for $ty
      where
        Self: 'static,
      {
        #[cfg(any(feature = "std", feature = "alloc"))]
        #[inline]
        fn decode_from_bytes<U>(
          _: $crate::__private::bytes::Bytes,
          _: &mut U,
        ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
        where
          Self: ::core::marker::Sized + 'static,
          U: $crate::__private::UnknownBuffer<$crate::__private::bytes::Bytes>,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }
      }

      impl<T: ?::core::marker::Sized> $crate::__private::PartialSerialize for $ty {
        type Selection = ();

        #[inline]
        fn partial_encode(&self, _: &Self::Selection, _: &mut [u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn partial_encoded_len(&self, _: &Self::Selection,) -> ::core::primitive::usize {
          0
        }
      }

      impl<T: ?::core::marker::Sized> $crate::__private::Message for $ty {
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

      impl<T: ?::core::marker::Sized> $crate::__private::TypeOwned<Self> for $ty {
        fn to(&self) -> ::core::result::Result<Self, $crate::__private::DecodeError> {
          ::core::result::Result::Ok(::core::default::Default::default())
        }
      }

      impl<T: ?::core::marker::Sized> $crate::__private::TypeRef<Self> for $ty {
        fn to(&self) -> ::core::result::Result<Self, $crate::__private::DecodeError> {
          ::core::result::Result::Ok(::core::default::Default::default())
        }
      }

      impl<T: ?::core::marker::Sized> $crate::__private::IntoTarget<Self> for $ty {
        fn into_target(self) -> ::core::result::Result<Self, $crate::__private::DecodeError> {
          ::core::result::Result::Ok(self)
        }
      }
    )*
  };
}
