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
        type Encoded<'a>
          = Self
        where
          Self: Sized + 'a;

        type Borrowed<'a>
          = &'a Self
        where
          Self: 'a;

        type EncodedOwned
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
        const WIRE_TYPE: $crate::WireType = $crate::WireType::Zst;
      }

      impl $crate::__private::Encode for $ty {
        #[inline]
        fn encode(&self, _: &mut [u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn encoded_len(&self) -> ::core::primitive::usize {
          0
        }
      }

      impl<'de> $crate::__private::Decode<'de> for $ty {
        fn decode<B>(_: &'de [u8], _: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
        where
          Self: ::core::marker::Sized + 'de,
          B: $crate::__private::UnknownRefBuffer<'de>,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }
      }

      impl $crate::__private::DecodeOwned for $ty
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

      impl $crate::__private::PartialEncode for $ty {
        type Selection = ();

        #[inline]
        fn partial_encode(&self, _: &mut [::core::primitive::u8], _: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
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
///       from: |v: u8| Self(v);
///       to: |v: &Self| v.0;
///     }
///   },
///   u32 {
///     MyU32 {
///       from: |v: u32| Self(v);
///       to: |v: &Self| v.0;
///     },
///     MyOtherU32 {
///       from: |v: u32| {
///         Self {
///           low: v as u16,
///           high: (v >> 16) as u16,
///         }
///       };
///       to: |v: &Self| {
///         (v.low as u32) | ((v.high as u32) << 16)
///       };
///     }
///   }
/// }
/// ```
#[macro_export]
macro_rules! bridge {
  ($(
    $bridge: ty {
      $($ty:ty {
        from: $from:expr;
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::bridge!(@wirable $bridge {
          $ty,
        });

        $crate::bridge!(@encode $bridge {
          $ty {
            to: $to,
          }
        });

        $crate::bridge!(@partial_encode $bridge {
          $ty {
            to: $to,
          }
        });

        $crate::bridge!(@decode $bridge {
          $ty {
            from: $from,
          }
        });

        $crate::bridge!(@decode_owned $bridge {
          $ty {
            from: $from,
          }
        });
      )*
    )*
  };
  (@encode_impl $bridge:ty => $to:expr) => {
    fn encode(&self, context: &$crate::__private::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
      <$bridge as $crate::__private::Encode>::encode(&$to(self), context, buf)
    }

    fn encoded_len(&self, context: &$crate::__private::Context,) -> ::core::primitive::usize {
      <$bridge as $crate::__private::Encode>::encoded_len(&$to(self), context)
    }
  };
  (@encode $(
    $bridge:ty {
      $($ty:ty {
        to: $to:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::__private::Encode for $ty {
          $crate::bridge!(@encode_impl $bridge => $to);
        }
      )*
    )*
  };
  (@partial_encode_impl $bridge:ty => $to:expr) => {
    fn partial_encode(&self, context: &$crate::__private::Context, buf: &mut [::core::primitive::u8], selection: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
      <$bridge as $crate::__private::PartialEncode>::partial_encode(&$to(self), context, buf, selection)
    }

    fn partial_encoded_len(&self, context: &$crate::__private::Context, selection: &Self::Selection) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialEncode>::partial_encoded_len(&$to(self), context, selection)
    }
  };
  (@partial_encode $(
    $bridge:ty {
      $($ty:ty {
        to: $to:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::__private::PartialEncode for $ty {
          type Selection = <$bridge as $crate::__private::PartialEncode>::Selection;

          $crate::bridge!(@partial_encode_impl $bridge => $to);
        }
      )*
    )*
  };
  (@decode_impl $from:expr => $bridge:ty) => {
    fn decode<B, UB>(context: &$crate::__private::Context, src: &'de B, ub: &mut UB) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
      where
        Self: ::core::marker::Sized + 'de,
        B: $crate::__private::Buffer,
        UB: $crate::__private::UnknownRefBuffer<'de> {
      <$bridge as $crate::__private::Decode>::decode(context, src, ub).map(|(n, v)| (n, $from(v)))
    }
  };
  (@decode $(
    $bridge:ty {
      $($ty:ty {
        from: $from:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de> $crate::__private::Decode<'de> for $ty {
          $crate::bridge!(@decode_impl $from => $bridge);
        }
      )*
    )*
  };
  (@decode_owned_impl $from:expr => $bridge:ty) => {
    $crate::__bridge_owned_impl!($from => $bridge);
  };
  (@decode_owned $(
    $bridge:ty {
      $($ty:ty {
        from: $from:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::__private::DecodeOwned for $ty {
          $crate::bridge!(@decode_owned_impl $from => $bridge);
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

#[cfg(any(feature = "std", feature = "alloc"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __bridge_owned_impl {
  ($from:expr => $bridge:ty) => {
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn decode_from_bytes<U>(
      src: $crate::__private::bytes::Bytes,
      ub: &mut U,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      U: $crate::__private::UnknownBuffer<$crate::__private::bytes::Bytes>,
    {
      <$bridge as $crate::__private::DecodeOwned>::decode_from_bytes(src, ub)
        .map(|(n, v)| (n, $from(v)))
    }
  };
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
#[macro_export]
#[doc(hidden)]
macro_rules! __bridge_owned_impl {
  ($from:expr => $bridge:ty) => {};
}

/// A macro emits traits implementations for a type that through a bridge to another type.
///
/// This macro is similar to `bridge!`, but it uses a `TryFrom` pattern for decoding.
#[macro_export]
macro_rules! try_from_bridge {
  ($(
    $bridge: ty {
      $($ty:ty {
        try_from: $try_from:expr;
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::try_from_bridge!(@wirable $bridge {
          $ty,
        });

        $crate::try_from_bridge!(@encode $bridge {
          $ty {
            to: $to,
          }
        });

        $crate::try_from_bridge!(@partial_encode $bridge {
          $ty {
            to: $to,
          }
        });

        $crate::try_from_bridge!(@decode $bridge {
          $ty {
            from: $try_from,
          }
        });

        $crate::try_from_bridge!(@decode_owned $bridge {
          $ty {
            from: $try_from,
          }
        });
      )*
    )*
  };
  (@encode_impl $bridge:ty => $to:expr) => {
    fn encode(&self, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
      <$bridge as $crate::__private::Encode>::encode(&$to(self), buf)
    }

    fn encoded_len(&self) -> ::core::primitive::usize {
      <$bridge as $crate::__private::Encode>::encoded_len(&$to(self))
    }
  };
  (@encode $(
    $bridge:ty {
      $($ty:ty {
        to: $to:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::__private::Encode for $ty {
          $crate::try_from_bridge!(@encode_impl $bridge => $to);
        }
      )*
    )*
  };
  (@partial_encode_impl $bridge:ty => $to:expr) => {
    fn partial_encode(&self, buf: &mut [::core::primitive::u8], selection: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
      <$bridge as $crate::__private::PartialEncode>::partial_encode(&$to(self), buf, selection)
    }

    fn partial_encode_with_identifier(&self, identifier: $crate::__private::Identifier, buf: &mut [::core::primitive::u8], selection: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
      <$bridge as $crate::__private::PartialEncode>::partial_encode_with_identifier(&$to(self), identifier, buf, selection)
    }

    fn partial_encoded_len(&self, selection: &Self::Selection) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialEncode>::partial_encoded_len(&$to(self), selection)
    }

    fn partial_encoded_len_with_identifier(&self, identifier: $crate::__private::Identifier, selection: &Self::Selection) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialEncode>::partial_encoded_len_with_identifier(&$to(self), identifier, selection)
    }
  };
  (@partial_encode $(
    $bridge:ty {
      $($ty:ty {
        to: $to:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::__private::PartialEncode for $ty {
          type Selection = <$bridge as $crate::__private::PartialEncode>::Selection;

          $crate::try_from_bridge!(@partial_encode_impl $bridge => $to);
        }
      )*
    )*
  };
  (@decode_impl $try_from:expr => $bridge:ty) => {
    fn decode<B>(src: &'de [::core::primitive::u8], ub: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
      where
        Self: ::core::marker::Sized + 'de,
        B: $crate::__private::UnknownRefBuffer<'de> {
      <$bridge as $crate::__private::Decode>::decode(src, ub)
        .and_then(|(n, v)| $try_from(v).map(|v| (n, v)))
    }
  };
  (@decode $(
    $bridge:ty {
      $($ty:ty {
        from: $from:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de> $crate::__private::Decode<'de> for $ty {
          $crate::try_from_bridge!(@decode_impl $from => $bridge);
        }
      )*
    )*
  };
  (@decode_owned_impl $from:expr => $bridge:ty) => {
    $crate::__try_from_bridge_owned_impl!($from => $bridge);
  };
  (@decode_owned $(
    $bridge:ty {
      $($ty:ty {
        from: $from:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::__private::DecodeOwned for $ty {
          $crate::try_from_bridge!(@decode_owned_impl $from => $bridge);
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

#[cfg(any(feature = "std", feature = "alloc"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __try_from_bridge_owned_impl {
  ($from:expr => $bridge:ty) => {
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn decode_from_bytes<U>(
      src: $crate::__private::bytes::Bytes,
      ub: &mut U,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      U: $crate::__private::UnknownBuffer<$crate::__private::bytes::Bytes>,
    {
      <$bridge as $crate::__private::DecodeOwned>::decode_from_bytes(src, ub)
        .and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }
  };
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
#[macro_export]
#[doc(hidden)]
macro_rules! __try_from_bridge_owned_impl {
  ($from:expr => $bridge:ty) => {};
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
///     from_str: |s: &str| Ok(MyString(s.to_string()));
///     to_str: MyString::as_str;
///
///     type EncodedOwned = SmolStr {
///       from_ref: |s: &SmolStr| Ok(MyString(s.to_string()));
///       from: |s: SmolStr| Ok(MyString(s.to_string()));
///     }
///   }
/// );
/// ```
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
#[macro_export]
macro_rules! str_bridge {
  ($($ty:ty {
    from_str: $from_str: expr;
    to_str: $to_str: expr;

    type EncodedOwned = $owned_ty:ty {
      from_ref: $from_ref: expr;
      from: $from: expr;
    } $(;)?
  }), +$(,)?) => {
    $(
      impl $crate::Wirable for $ty {}

      $crate::str_bridge!(@encode $ty: $to_str);
      $crate::str_bridge!(@decode $ty: $from_str);
      $crate::str_bridge!(@decode_owned $ty: $from_str);

      impl $crate::__private::PartialEncode for $ty {
        $crate::partial_encode_primitives!(@impl);
      }

      impl $crate::__private::IntoTarget<$ty> for &::core::primitive::str {
        $crate::str_bridge!(@into_target_impl $ty: $from_str);
      }

      impl $crate::__private::TypeRef<$ty> for &::core::primitive::str {
        $crate::str_bridge!(@str_to_impl $ty: $from_str);
      }

      impl $crate::__private::TypeOwned<$ty> for &::core::primitive::str {
        $crate::str_bridge!(@str_to_impl $ty: $from_str);
      }

      impl $crate::__private::IntoTarget<$ty> for $owned_ty {
        $crate::str_bridge!(@into_target_impl $ty: $from);
      }

      impl $crate::__private::TypeOwned<$ty> for $owned_ty {
        $crate::str_bridge!(@to_impl $ty: $from_ref);
      }

      impl $crate::__private::Message for $ty {
        type Encoded<'a> = &'a ::core::primitive::str
        where
          Self: ::core::marker::Sized + 'a;

        type Borrowed<'a> = &'a $ty
        where
          Self: 'a;

        type EncodedOwned = $owned_ty
        where
          Self: ::core::marker::Sized + 'static;
      }
    )*
  };
  (@encode $ty:ty: $to_str:expr) => {
    impl $crate::__private::Encode for $ty {
      $crate::str_bridge!(@encode_impl $to_str);
    }
  };
  (@encode_impl $to_str:expr) => {
    #[inline]
    fn encode(&self, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
      <&::core::primitive::str as $crate::__private::Encode>::encode(&$to_str(self), buf)
    }

    #[inline]
    fn encoded_len(&self) -> ::core::primitive::usize {
      <&::core::primitive::str as $crate::__private::Encode>::encoded_len(&$to_str(self))
    }
  };
  (@decode $ty:ty: $from_str:expr) => {
    impl<'de> $crate::__private::Decode<'de> for $ty {
      $crate::str_bridge!(@decode_impl $from_str);
    }
  };
  (@decode_impl $from_str:expr) => {
    fn decode<B>(src: &'de [::core::primitive::u8], _: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      B: $crate::__private::UnknownRefBuffer<'de>
    {
      $crate::__private::from_utf8(&src)
        .map_err(|_| $crate::__private::DecodeError::custom("invalid UTF-8"))
        .and_then(|val| $from_str(val).map(|v| (src.len(), v)))
    }
  };
  (@decode_owned $ty:ty: $from_str:expr) => {
    impl $crate::__private::DecodeOwned for $ty {
      $crate::str_bridge!(@decode_owned_impl $from_str);
    }
  };
  (@decode_owned_impl $from_str:expr) => {
    fn decode_from_bytes<U>(
      src: $crate::__private::bytes::Bytes,
      _: &mut U,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      U: $crate::__private::UnknownBuffer<$crate::__private::bytes::Bytes>
    {
      $crate::__private::from_utf8(&src)
        .map_err(|_| $crate::__private::DecodeError::custom("invalid UTF-8"))
        .and_then(|val| $from_str(val).map(|v| (src.len(), v)))
    }
  };
  (@into_target_impl $ty:ty: $from:expr) => {
    fn into_target(self) -> ::core::result::Result<$ty, $crate::__private::DecodeError> {
      $from(self)
    }
  };
  (@str_to_impl $ty:ty: $from:expr) => {
    fn to(&self) -> ::core::result::Result<$ty, $crate::__private::DecodeError> {
      $from(*self)
    }
  };
  (@to_impl $ty:ty: $from:expr) => {
    fn to(&self) -> ::core::result::Result<$ty, $crate::__private::DecodeError> {
      $from(self)
    }
  };
}

/// A macro emits traits implementation for types underlying is a `&[u8]`.
///
/// This macro requires the types are `'static`
///
/// ## Example
///
/// ```rust
/// use grost::{bytes_bridge, bytes::Bytes};
///
/// struct MyVec(Vec<u8>);
///
/// impl MyVec {
///   fn as_bytes(&self) -> &[u8] {
///     self.0.as_slice()
///   }
/// }
///
/// bytes_bridge!(
///   MyVec {
///     from_bytes: |s: &str| Ok(MyVec(s.to_vec()));
///     to_bytes: MyVec::as_bytes;
///
///     type EncodedOwned = Bytes {
///       from_ref: |s: &Bytes| Ok(MyVec(s.to_vec()));
///       from: |s: Bytes| Ok(MyVec(s));
///     }
///   }
/// );
/// ```
#[cfg(any(feature = "alloc", feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "alloc", feature = "std"))))]
#[macro_export]
macro_rules! bytes_bridge {
  ($($ty:ty $([const $g:ident: usize])? {
    from_bytes: $from_bytes: expr;
    to_bytes: $to_bytes: expr;

    type EncodedOwned = $owned_ty:ty {
      from_ref: $from_ref: expr;
      from: $from: expr;
    } $(;)?
  }), +$(,)?) => {
    $(
      impl$(<const $g: ::core::primitive::usize>)? $crate::Wirable for $ty {}

      $crate::bytes_bridge!(@encode $ty $([const $g: usize])?: $to_bytes);
      $crate::bytes_bridge!(@decode $ty $([const $g: usize])?: $from_bytes);
      $crate::bytes_bridge!(@decode_owned  $ty $([const $g: usize])?: $from_bytes);

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::PartialEncode for $ty {
        $crate::partial_encode_primitives!(@impl);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::IntoTarget<$ty> for &[::core::primitive::u8] {
        $crate::bytes_bridge!(@into_target_impl $ty: $from_bytes);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::TypeRef<$ty> for &[::core::primitive::u8] {
        $crate::bytes_bridge!(@str_to_impl $ty: $from_bytes);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::TypeOwned<$ty> for &[::core::primitive::u8] {
        $crate::bytes_bridge!(@str_to_impl $ty: $from_bytes);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::IntoTarget<$ty> for $owned_ty {
        $crate::bytes_bridge!(@into_target_impl $ty: $from);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::TypeOwned<$ty> for $owned_ty {
        $crate::bytes_bridge!(@to_impl $ty: $from_ref);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::Message for $ty {
        type Encoded<'a> = &'a [::core::primitive::u8]
        where
          Self: ::core::marker::Sized + 'a;

        type Borrowed<'a> = &'a $ty
        where
          Self: 'a;

        type EncodedOwned = $owned_ty
        where
          Self: ::core::marker::Sized + 'static;
      }
    )*
  };
  (@encode $ty:ty $([const $g:ident: usize])?: $to_bytes:expr) => {
    impl $(<const $g: ::core::primitive::usize>)? $crate::__private::Encode for $ty {
      $crate::bytes_bridge!(@encode_impl $to_bytes);
    }
  };
  (@encode_impl $to_bytes:expr) => {
    #[inline]
    fn encode(&self, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
      <&[::core::primitive::u8] as $crate::__private::Encode>::encode(&$to_bytes(self), buf)
    }

    #[inline]
    fn encoded_len(&self) -> ::core::primitive::usize {
      <&[::core::primitive::u8] as $crate::__private::Encode>::encoded_len(&$to_bytes(self))
    }
  };
  (@decode $ty:ty $([const $g:ident: usize])?: $from_bytes:expr) => {
    impl<'de, $(const $g: usize)?> $crate::__private::Decode<'de> for $ty {
      $crate::bytes_bridge!(@decode_impl $from_bytes);
    }
  };
  (@decode_impl $from_bytes:expr) => {
    fn decode<B>(src: &'de [::core::primitive::u8], _: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      B: $crate::__private::UnknownRefBuffer<'de>
    {
      $from_bytes(src).map(|v| (src.len(), v))
    }
  };
  (@decode_owned $ty:ty $([const $g:ident: usize])?: $from_bytes:expr) => {
    impl$(<const $g: ::core::primitive::usize>)? $crate::__private::DecodeOwned for $ty {
      $crate::bytes_bridge!(@decode_owned_impl $from_bytes);
    }
  };
  (@decode_owned_impl $from_bytes:expr) => {
    fn decode_from_bytes<U>(
      src: $crate::__private::bytes::Bytes,
      _: &mut U,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      U: $crate::__private::UnknownBuffer<$crate::__private::bytes::Bytes>
    {
      $from_bytes(&src).map(|v| (src.len(), v))
    }
  };
  (@into_target_impl $ty:ty: $from:expr) => {
    fn into_target(self) -> ::core::result::Result<$ty, $crate::__private::DecodeError> {
      $from(self)
    }
  };
  (@str_to_impl $ty:ty: $from:expr) => {
    fn to(&self) -> ::core::result::Result<$ty, $crate::__private::DecodeError> {
      $from(*self)
    }
  };
  (@to_impl $ty:ty: $from:expr) => {
    fn to(&self) -> ::core::result::Result<$ty, $crate::__private::DecodeError> {
      $from(self)
    }
  };
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
          0 => $crate::__private::WireType::Varint,
          _ => $crate::__private::WireType::LengthDelimited,
        }
      };
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::Encode for $ty {
      fn encode(
        &self,
        buf: &mut [::core::primitive::u8],
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
        if N == 0 {
          return ::core::result::Result::Ok(0);
        }

        $crate::__private::Encode::encode(&self.as_bytes(), buf)
      }

      #[inline]
      fn encoded_len(&self) -> ::core::primitive::usize {
        if N == 0 {
          return 0;
        }

        $crate::__private::Encode::encoded_len(&self.as_bytes())
      }

      fn encoded_len_with_identifier(&self, identifier: $crate::__private::Identifier) -> ::core::primitive::usize {
        if N == 0 {
          return 0;
        }

        $crate::__private::Encode::encoded_len_with_identifier(&self.as_bytes(), identifier)
      }

      fn encode_with_identifier(
        &self,
        identifier: $crate::__private::Identifier,
        buf: &mut [::core::primitive::u8],
      ) -> Result<::core::primitive::usize, $crate::__private::EncodeError> {
        if N == 0 {
          return ::core::result::Result::Ok(0);
        }

        $crate::__private::Encode::encode_with_identifier(&self.as_bytes(), identifier, buf)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::PartialEncode for $ty {
      $crate::partial_encode_primitives!(@impl);
    }

    impl<'de, const $g: ::core::primitive::usize> $crate::__private::Decode<'de> for $ty {
      fn decode<B>(
        src: &'de [::core::primitive::u8],
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

    impl<const $g: ::core::primitive::usize> $crate::__private::DecodeOwned for $ty {
      $crate::__array_str_decoded_owned_impl!($new, $decode);
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
      type Encoded<'a>
        = &'a ::core::primitive::str
      where
        Self: ::core::marker::Sized + 'a;

      type Borrowed<'a>
        = &'a Self
      where
        Self: 'a;

      type EncodedOwned
        = Self
      where
        Self: ::core::marker::Sized + 'static;
    }
  };
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __array_str_decoded_owned_impl {
  ($new:expr, $decode:expr) => {
    #[cfg(any(feature = "std", feature = "alloc"))]
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

      $decode(::core::convert::AsRef::<[::core::primitive::u8]>::as_ref(
        &src,
      ))
    }
  };
}

#[cfg(not(any(feature = "alloc", feature = "std")))]
#[doc(hidden)]
#[macro_export]
macro_rules! __array_str_decoded_owned_impl {
  ($new:expr, $decode:expr) => {};
}

/// A macro emits traits implementations for a array-style `[u8]` type.
#[macro_export]
macro_rules! array_bytes {
  (
    impl<const $g:ident: usize> $ty:ty {
      fn new = $new:expr;

      fn from_bytes = $from_bytes:expr;

      fn decode = $decode:expr;

      fn as_bytes = $as_bytes:expr;
    }
  ) => {
    impl<const $g: ::core::primitive::usize> $crate::__private::Wirable for $ty {
      const WIRE_TYPE: $crate::__private::WireType = {
        match N {
          0 => $crate::__private::WireType::Zst,
          _ => $crate::__private::WireType::LengthDelimited,
        }
      };
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::Encode for $ty {
      fn encode(
        &self,
        buf: &mut [::core::primitive::u8],
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
        if N == 0 {
          return ::core::result::Result::Ok(0);
        }

        $crate::__private::Encode::encode(&$as_bytes(self), buf)
      }

      #[inline]
      fn encoded_len(&self) -> ::core::primitive::usize {
        if N == 0 {
          return 0;
        }

        $crate::__private::Encode::encoded_len(&$as_bytes(self))
      }

      fn encoded_len_with_identifier(&self, identifier: $crate::__private::Identifier) -> ::core::primitive::usize {
        if N == 0 {
          return 0;
        }

        $crate::__private::Encode::encoded_len_with_identifier(&$as_bytes(self), identifier)
      }

      fn encode_with_identifier(
        &self,
        identifier: $crate::__private::Identifier,
        buf: &mut [::core::primitive::u8],
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
        if N == 0 {
          return ::core::result::Result::Ok(0);
        }

        $crate::__private::Encode::encode_with_identifier(&$as_bytes(self), identifier, buf)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::PartialEncode for $ty {
      $crate::partial_encode_primitives!(@impl);
    }

    impl<'de, const $g: ::core::primitive::usize> $crate::__private::Decode<'de> for $ty {
      fn decode<B>(
        src: &'de [::core::primitive::u8],
        _: &mut B,
      ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
      where
        Self: ::core::marker::Sized + 'de,
        B: $crate::__private::UnknownRefBuffer<'de>,
      {
        if N == 0 {
          return ::core::result::Result::Ok((0, $new()));
        }

        if src.len() > N {
          return ::core::result::Result::Err($crate::__private::larger_than_array_capacity::<N>());
        }

        $decode(src)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::DecodeOwned for $ty {
      $crate::__array_bytes_decoded_owned_impl!($new, $decode);
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

    impl<const $g: ::core::primitive::usize> $crate::__private::IntoTarget<$ty>
      for &[::core::primitive::u8]
    {
      fn into_target(self) -> ::core::result::Result<$ty, $crate::__private::DecodeError> {
        $from_bytes(self)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::TypeRef<$ty>
      for &[::core::primitive::u8]
    {
      fn to(&self) -> ::core::result::Result<$ty, $crate::__private::DecodeError> {
        $from_bytes(*self)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::Message for $ty {
      type Encoded<'a>
        = &'a [::core::primitive::u8]
      where
        Self: ::core::marker::Sized + 'a;

      type Borrowed<'a>
        = &'a Self
      where
        Self: 'a;

      type EncodedOwned
        = Self
      where
        Self: ::core::marker::Sized + 'static;
    }
  };
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __array_bytes_decoded_owned_impl {
  ($new:expr, $decode:expr) => {
    #[cfg(any(feature = "std", feature = "alloc"))]
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
        return ::core::result::Result::Err($crate::__private::larger_than_array_capacity::<N>());
      }

      $decode(::core::convert::AsRef::<[::core::primitive::u8]>::as_ref(
        &src,
      ))
    }
  };
}

#[cfg(not(any(feature = "alloc", feature = "std")))]
#[doc(hidden)]
#[macro_export]
macro_rules! __array_bytes_decoded_owned_impl {
  ($new:expr, $decode:expr) => {};
}

/// A macro emits [`TypeRef`](super::TypeRef) implementations for `Self`
#[macro_export]
macro_rules! type_ref {
  ($($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::TypeRef<Self> for $ty {
        $crate::type_ref!(@copy_impl);
      }
    )*
  };
  (@clone $ty:ty $($([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::TypeRef<Self> for $ty {
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
  ($($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::TypeOwned<Self> for $ty {
        $crate::type_ref!(@copy_impl);
      }
    )*
  };
  (@clone $ty:ty $($([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::TypeOwned<Self> for $ty {
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
  ($($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? ),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::IntoTarget<Self> for $ty {
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
  ($($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $crate::type_ref!($($ty $([ $(const $g: usize),* ])?),+);
    $crate::type_owned!($($ty $([ $(const $g: usize),* ])?),+);
    $crate::into_target!($($ty $([ $(const $g: usize),* ])?),+);
  };
  (@clone $ty:ty $($([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $crate::type_ref!(@clone $($ty $([ $(const $g: usize),* ])?),+);
    $crate::type_owned!(@clone $($ty $([ $(const $g: usize),* ])?),+);
    $crate::into_target!($($ty $([ $(const $g: usize),* ])?),+);
  };
}

/// A macro emits [`Message`](super::Message) implementations for `Self`.
///
/// **NB:** this macro can only be used for types that implements [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! message {
  ($($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Message for $ty {
        $crate::message!(@impl);
      }
    )*

    $($crate::conversion!($ty $([ $(const $g: usize),* ])? );)*
  };
  (@impl) => {
    type Encoded<'a>
      = Self
    where
      Self: ::core::marker::Sized + 'a;

    type Borrowed<'a>
      = &'a Self
    where
      Self: 'a;

    type EncodedOwned
      = Self
    where
      Self: ::core::marker::Sized + 'static;
  };
}

/// A macro emits [`PartialEncode`](super::PartialEncode) implementations for primitive types.
#[macro_export]
macro_rules! partial_encode_primitives {
  (@impl) => {
    type Selection = ();

    fn partial_encode(&self, context: &$crate::__private::Context, buf: &mut [::core::primitive::u8],  _: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
      <Self as $crate::__private::Encode>::encode(self, context, buf)
    }

    fn partial_encoded_len(&self, context: &$crate::__private::Context, _: &Self::Selection) -> ::core::primitive::usize {
      <Self as $crate::__private::Encode>::encoded_len(self, context)
    }
  };
  ($($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialEncode for $ty {
        $crate::partial_encode_primitives!(@impl);
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
  (@merged) => {
    const WIRE_TYPE: $crate::WireType = $crate::WireType::Zst;
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
    (@$wire_varint:ident) <=> ($ty:ty $($([ $( const $g:ident: usize), +$(,)? ])?), +$(,)?)
  ),+$(,)?) => {
    $(
      $(
        impl$( < $(const $g: ::core::primitive::usize),* > )? $crate::Wirable for $ty {
          $crate::wirable!(@$wire_varint);
        }
      )*
    )*
  }
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __varint_decode_owned_impl {
  () => {
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn decode_from_bytes<U>(
      src: $crate::__private::bytes::Bytes,
      _: &mut U,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      U: $crate::__private::UnknownBuffer<$crate::__private::bytes::Bytes>,
    {
      $crate::__private::varing::Varint::decode(::core::convert::AsRef::as_ref(&src))
        .map_err(::core::convert::Into::into)
    }
  };
}

#[cfg(not(any(feature = "alloc", feature = "std")))]
#[doc(hidden)]
#[macro_export]
macro_rules! __varint_decode_owned_impl {
  () => {};
}

/// A macro emits traits implementations for primitive types that implements [`varing::Varint`](varing::Varint) and [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! varint {
  ($(
    $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?
  ),+$(,)?) => {
    $($crate::wirable!((@varint) <=> ($ty $([ $(const $g: usize),* ])?));)*
    $($crate::message!($ty $([$(const $g: usize),*])?);)*
    $($crate::partial_encode_primitives!($ty $([ $(const $g: usize),* ])?);)*
    $($crate::varint!(@encode $ty $([ $(const $g: usize),* ])?);)*
    $($crate::varint!(@decode $ty $([ $(const $g: usize),* ])?);)*
    $($crate::varint!(@decode_owned $ty $([ $(const $g: usize),* ])?);)*
  };
  (@encode $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Encode for $ty {
      $crate::varint!(@encode_impl);
    }
  };
  (@encode_impl) => {
    fn encode(&self, _: &$crate::__private::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<usize, $crate::__private::EncodeError> {
      $crate::__private::varing::Varint::encode(self, buf).map_err(::core::convert::Into::into)
    }

    fn encoded_len(&self, _: &$crate::__private::Context,) -> ::core::primitive::usize {
      $crate::__private::varing::Varint::encoded_len(self)
    }
  };
  (@decode $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl<'de, $($(const $g: ::core::primitive::usize),*)?> $crate::__private::Decode<'de> for $ty {
      $crate::varint!(@decode_impl);
    }
  };
  (@decode_impl) => {
    fn decode<B, UB>(_: &$crate::__private::Context, src: &'de B, _: &mut UB) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      B: $crate::__private::Buffer,
      UB: $crate::__private::UnknownRefBuffer<'de>,
    {
      $crate::__private::varing::Varint::decode(::core::convert::AsRef::as_ref(src)).map_err(::core::convert::Into::into)
    }
  };
  (@decode_owned $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::DecodeOwned for $ty {
      $crate::varint!(@decode_owned_impl);
    }
  };
  (@decode_owned_impl) => {
    $crate::__varint_decode_owned_impl!();
  };
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __fixed_decode_owned_impl {
  (@decode_owned_impl $size:literal { $from_slice:expr $(,)? }) => {
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn decode_from_bytes<U>(
      src: $crate::__private::bytes::Bytes,
      _: &mut U,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      U: $crate::__private::UnknownBuffer<$crate::__private::bytes::Bytes>,
    {
      const SIZE: ::core::primitive::usize = $size / 8;

      if src.len() < SIZE {
        return Err($crate::__private::DecodeError::buffer_underflow());
      }

      $from_slice(&src[..SIZE]).map(|val| (SIZE, val))
    }
  };
}

#[cfg(not(any(feature = "alloc", feature = "std")))]
#[doc(hidden)]
#[macro_export]
macro_rules! __fixed_decode_owned_impl {
  (@decode_owned_impl $size:literal { $from_slice:expr $(,)? }) => {};
}

/// A macro emits traits implementations for primitive types whose [`WireType`](crate::WireType) is `Fixed*` and implement [`Copy`](::core::marker::Copy).
///
/// - `to_bytes`: An expr that takes `&self` and `&mut [u8]` and returns a `Result<(), EncodeError>`.
///   the buf size is promised to be the same as the size of the `WireType::Fixed*`.
///
/// - `from_bytes`: An expr that takes `&[u8]` and returns a `Result<Self, DecodeError>`.
///   the buf size is promised to be the same as the size of the `WireType::Fixed*`.
///
/// ## Example
///
/// ```rust
/// use grost::fixed;
///
/// #[derive(Debug, Clone, Copy)]
/// struct MyF32(f32);
/// #[derive(Debug, Clone, Copy)]
/// struct MyF64(f64);
///
/// #[derive(Debug, Clone, Copy)]
/// struct MyFixedU32(u32);
///
/// fixed!(
///   32(
///     MyF32 {
///       from_bytes: |src: &[u8]| {
///         Ok(MyF32(f32::from_le_bytes(src.try_into().unwrap())))
///       },
///       to_bytes: |this: &Self, buf: &mut [u8]| {
///         buf.copy_from_slice(&this.0.to_le_bytes());
///         Ok(())
///       },
///     },
///     MyFixedU32 {
///       from_bytes: |src: &[u8]| {
///         Ok(MyFixedU32(u32::from_le_bytes(src.try_into().unwrap())))
///       },
///       to_bytes: |this: &Self, buf: &mut [u8]| {
///         buf.copy_from_slice(&this.0.to_le_bytes());
///         Ok(())
///       },
///     },
///   ),
///   64(MyF64 {
///     from_bytes: |src: &[u8]| {
///       Ok(MyF64(f64::from_le_bytes(src.try_into().unwrap())))
///     },
///     to_bytes: |this: &Self, buf: &mut [u8]| {
///       buf.copy_from_slice(&this.0.to_le_bytes());
///       Ok(())
///     },
///   }),
/// );
/// ```
#[macro_export]
macro_rules! fixed {
  ($(
    $size:literal(
      $(
        $ty:ty $([ $( const $g:ident: usize), +$(,)? ])? {
          from_bytes: $from_bytes:expr,
          to_bytes: $to_bytes:expr,
        }
      ), +$(,)?
    )
  ),+$(,)?) => {
    $(
      paste::paste!($($crate::wirable!((@[<fixed $size>]) <=> ($ty $([ $(const $g: usize),* ])?));)*);
      $($crate::message!($ty $([$(const $g: usize),*])?);)*
      $($crate::partial_encode_primitives!($ty $([ $(const $g: usize),* ])?);)*
      $($crate::fixed!(@encode $size ($ty $([ $(const $g: usize),* ])? { to_bytes: $to_bytes }));)*
      $($crate::fixed!(@decode $size ($ty $([ $(const $g: usize),* ])? { from_bytes: $from_bytes }));)*
      $($crate::fixed!(@decode_owned $size ($ty $([ $(const $g: usize),* ])? { from_bytes: $from_bytes } ));)*
    )*
  };
  (@encode $size:literal($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? {
    to_bytes: $to_bytes:expr $(,)?
  })) => {
    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Encode for $ty {
      $crate::fixed!(@encode_impl $size { $to_bytes });
    }
  };
  (@encode_impl $size:literal { $to_slice:expr $(,)? }) => {
    fn encode(&self, _: &$crate::__private::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
      const SIZE: ::core::primitive::usize = $size / 8;

      let buf_len = buf.len();
      if buf_len < $size {
        return Err($crate::__private::EncodeError::insufficient_buffer($size, buf_len));
      }

      $to_slice(self, &mut buf[..SIZE]).map(|_| SIZE)
    }

    fn encoded_len(&self, _: &$crate::__private::Context) -> ::core::primitive::usize {
      const SIZE: ::core::primitive::usize = $size / 8;
      SIZE
    }
  };
  (@decode $size:literal($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? { from_bytes: $from_bytes:expr $(,)? })) => {
    impl<'de, $($(const $g: ::core::primitive::usize),*)?> $crate::__private::Decode<'de> for $ty {
      $crate::fixed!(@decode_impl $size { $from_bytes });
    }
  };
  (@decode_impl $size:literal { $from_slice:expr $(,)? }) => {
    fn decode<B, UB>(_: &$crate::__private::Context, src: &'de B, _: &mut UB) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      B: $crate::__private::Buffer,
      UB: $crate::__private::UnknownRefBuffer<'de>,
    {
      const SIZE: ::core::primitive::usize = $size / 8;
      let src = ::core::convert::AsRef::as_ref(src);

      if src.len() < SIZE {
        return Err($crate::__private::DecodeError::buffer_underflow());
      }

      $from_slice(&src[..SIZE]).map(|val| (SIZE, val))
    }
  };
  (@decode_owned $size:literal($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? { from_bytes: $from_bytes:expr $(,)? })) => {
    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::DecodeOwned for $ty {
      $crate::fixed!(@decode_owned_impl $size { $from_bytes });
    }
  };
  (@decode_owned_impl $size:literal { $from_slice:expr $(,)? }) => {
    $crate::__fixed_decode_owned_impl!(@decode_owned_impl $size { $from_slice });
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

          $crate::WireType::Zst
        };
      }

      impl<T: ?::core::marker::Sized> $crate::__private::Encode for $ty {
        #[inline]
        fn encode(&self, _: &mut [u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn encoded_len(&self) -> ::core::primitive::usize {
          0
        }
      }

      impl<'de, T: ?::core::marker::Sized> $crate::__private::Decode<'de> for $ty {
        fn decode<B>(_: &'de [u8], _: &mut B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::DecodeError>
        where
          Self: ::core::marker::Sized + 'de,
          B: $crate::__private::UnknownRefBuffer<'de>,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }
      }

      impl<T: ?::core::marker::Sized> $crate::__private::DecodeOwned for $ty
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

      impl<T: ?::core::marker::Sized> $crate::__private::PartialEncode for $ty {
        type Selection = ();

        #[inline]
        fn partial_encode(&self, _: &mut [u8], _: &Self::Selection,) -> ::core::result::Result<::core::primitive::usize, $crate::__private::EncodeError> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn partial_encoded_len(&self, _: &Self::Selection,) -> ::core::primitive::usize {
          0
        }
      }

      impl<T: ?::core::marker::Sized> $crate::__private::Message for $ty {
        type Encoded<'a>
          = Self
        where
          Self: Sized + 'a;

        type Borrowed<'a>
          = &'a Self
        where
          Self: 'a;

        type EncodedOwned
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
