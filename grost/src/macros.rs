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

      impl $crate::__private::PartialMessage for $ty {
        type UnknownBuffer<B: ?::core::marker::Sized> = ();

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

      impl $crate::__private::Message for $ty {
        type Partial = Self;

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
        fn to(&self) -> ::core::result::Result<Self, $crate::__private::flavors::network::DecodeError> {
          ::core::result::Result::Ok(::core::default::Default::default())
        }
      }

      impl $crate::__private::TypeRef<Self> for $ty {
        fn to(&self) -> ::core::result::Result<Self, $crate::__private::flavors::network::DecodeError> {
          ::core::result::Result::Ok(::core::default::Default::default())
        }
      }

      $crate::into_target!($ty);

      impl $crate::Wirable for $ty {
        const WIRE_TYPE: $crate::__private::flavors::network::WireType = $crate::__private::flavors::network::WireType::Zst;
      }

      impl $crate::__private::Encode for $ty {
        #[inline]
        fn encode(&self, _: &$crate::__private::flavors::network::Context, _: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn encoded_len(&self, _: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
          0
        }
      }

      impl<'de> $crate::__private::Decode<'de, Self> for $ty {
        fn decode<UB>(_: &$crate::__private::flavors::network::Context, _: &'de [::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::flavors::network::DecodeError>
        where
          Self: ::core::marker::Sized + 'de,
          UB: $crate::__private::UnknownBuffer<&'de [::core::primitive::u8]> + 'de,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }
      }

      impl $crate::__private::DecodeOwned<Self> for $ty {
        fn decode_owned<B, UB>(ctx: &$crate::__private::flavors::network::Context, src: B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::flavors::network::DecodeError>
        where
          Self: ::core::marker::Sized + 'static,
          B: $crate::__private::Buffer + 'static,
          UB: $crate::__private::UnknownBuffer<B> + 'static,
        {
          <Self as $crate::__private::Decode<'_, Self>>::decode::<()>(ctx, src.as_bytes())
            .map(|(n, _)| (n, ::core::default::Default::default()))
        }
      }

      impl $crate::__private::PartialEncode for $ty {
        type Selection = ();

        #[inline]
        fn partial_encode(&self, ctx: &$crate::__private::flavors::network::Context, src: &mut [::core::primitive::u8], _: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
          <Self as $crate::__private::Encode>::encode(self, ctx, src)
        }

        #[inline]
        fn partial_encoded_len(&self, ctx: &$crate::__private::flavors::network::Context, _: &Self::Selection,) -> ::core::primitive::usize {
          <Self as $crate::__private::Encode>::encoded_len(self, ctx)
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
    $flavor:ty: $bridge: ty {
      $($ty:ty {
        from: $from:expr;
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::bridge!(@wirable $flavor: $bridge {
          $ty,
        });

        $crate::bridge!(@encode $flavor: $bridge {
          $ty {
            to: $to,
          }
        });

        $crate::bridge!(@partial_encode $flavor: $bridge {
          $ty {
            to: $to,
          }
        });

        $crate::bridge!(@decode $flavor: $bridge {
          $ty {
            from: $from,
          }
        });

        $crate::bridge!(@decode_owned $flavor: $bridge {
          $ty {
            from: $from,
          }
        });
      )*
    )*
  };
  (@encode_impl $flavor:ty: $bridge:ty => $to:expr) => {
    fn encode(&self, context: &<$flavor as $crate::__private::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <$bridge as $crate::__private::Encode<$flavor>>::encode(&$to(self), context, buf)
    }

    fn encoded_len(&self, context: &<$flavor as $crate::__private::Flavor>::Context,) -> ::core::primitive::usize {
      <$bridge as $crate::__private::Encode<$flavor>>::encoded_len(&$to(self), context)
    }
  };
  (@encode $flavor:ty: $(
    $bridge:ty {
      $($ty:ty {
        to: $to:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::__private::Encode<$flavor> for $ty {
          $crate::bridge!(@encode_impl $flavor: $bridge => $to);
        }
      )*
    )*
  };
  (@partial_encode_impl $flavor:ty: $bridge:ty => $to:expr) => {
    fn partial_encode(&self, context: &<$flavor as $crate::__private::Flavor>::Context, buf: &mut [::core::primitive::u8], selection: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <$bridge as $crate::__private::PartialEncode<$flavor>>::partial_encode(&$to(self), context, buf, selection)
    }

    fn partial_encoded_len(&self, context: &<$flavor as $crate::__private::Flavor>::Context, selection: &Self::Selection) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialEncode<$flavor>>::partial_encoded_len(&$to(self), context, selection)
    }
  };
  (@partial_encode $flavor:ty: $(
    $bridge:ty {
      $($ty:ty {
        to: $to:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::__private::PartialEncode<$flavor> for $ty {
          type Selection = <$bridge as $crate::__private::PartialEncode<$flavor>>::Selection;

          $crate::bridge!(@partial_encode_impl $flavor: $bridge => $to);
        }
      )*
    )*
  };
  (@decode_impl $flavor:ty: $from:expr => $bridge:ty) => {
    fn decode<UB>(context: &<$flavor as $crate::__private::Flavor>::Context, src: &'de [::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::UnknownBuffer<$flavor, &'de [::core::primitive::u8]> + 'de,
    {
      <$bridge as $crate::__private::Decode<$flavor, $bridge>>::decode::<UB>(context, src).map(|(n, v)| (n, $from(v)))
    }
  };
  (@decode $flavor:ty: $(
    $bridge:ty {
      $($ty:ty {
        from: $from:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de> $crate::__private::Decode<'de, $flavor, Self> for $ty {
          $crate::bridge!(@decode_impl $flavor: $from => $bridge);
        }
      )*
    )*
  };
  (@decode_owned_impl $flavor:ty: $from:expr => $bridge:ty) => {
    fn decode_owned<B, UB>(
      ctx: &<$flavor as $crate::__private::Flavor>::Context,
      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::Buffer + 'static,
      UB: $crate::__private::UnknownBuffer<$flavor, B> + 'static,
    {
      <$bridge as $crate::__private::DecodeOwned<$flavor, $bridge>>::decode_owned::<B, UB>(ctx, src)
        .map(|(n, v)| (n, $from(v)))
    }
  };
  (@decode_owned $flavor:ty: $(
    $bridge:ty {
      $($ty:ty {
        from: $from:expr $(,)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::__private::DecodeOwned<$flavor, Self> for $ty
        where
          $bridge: $crate::__private::DecodeOwned<$flavor, $bridge>,
        {
          $crate::bridge!(@decode_owned_impl $flavor: $from => $bridge);
        }
      )*
    )*
  };
  (@wirable $flavor:ty: $bridge:ty {
    $($ty:ty), +$(,)?
  }) => {
    $(
      impl $crate::Wirable<$flavor> for $ty {
        const WIRE_TYPE: <$flavor as $crate::__private::Flavor>::WireType = <$bridge as $crate::Wirable<$flavor>>::WIRE_TYPE;
      }
    )*
  };
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
    fn encode(&self, ctx: &$crate::__private::flavors::network::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
      <$bridge as $crate::__private::Encode>::encode(&$to(self), ctx, buf)
    }

    fn encoded_len(&self, ctx: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
      <$bridge as $crate::__private::Encode>::encoded_len(&$to(self), ctx)
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
    fn partial_encode(&self, ctx: &$crate::__private::flavors::network::Context, buf: &mut [::core::primitive::u8], selection: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
      <$bridge as $crate::__private::PartialEncode>::partial_encode(&$to(self), ctx, buf, selection)
    }

    fn partial_encoded_len(&self, ctx: &$crate::__private::flavors::network::Context, selection: &Self::Selection) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialEncode>::partial_encoded_len(&$to(self), ctx, selection)
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
    fn decode<UB>(ctx: &$crate::__private::flavors::network::Context, src: &'de [::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::flavors::network::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::UnknownBuffer<&'de [::core::primitive::u8]> + 'de,
    {
      <$bridge as $crate::__private::Decode<'_, $bridge>>::decode::<UB>(ctx, src)
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
        impl<'de> $crate::__private::Decode<'de, Self> for $ty
        where
          $bridge: $crate::__private::Decode<'de, $bridge>,
        {
          $crate::try_from_bridge!(@decode_impl $from => $bridge);
        }
      )*
    )*
  };
  (@decode_owned_impl $from:expr => $bridge:ty) => {
    fn decode_owned<B, UB>(
      ctx: &$crate::__private::flavors::network::Context,
      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::flavors::network::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::Buffer + 'static,
      UB: $crate::__private::UnknownBuffer<B> + 'static,
    {
      <$bridge as $crate::__private::DecodeOwned<$bridge>>::decode_owned::<B, UB>(ctx, src)
        .and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }
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
        impl $crate::__private::DecodeOwned<Self> for $ty
        where
          $bridge: $crate::__private::DecodeOwned<$bridge>,
        {
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
        const WIRE_TYPE: $crate::__private::flavors::network::WireType = <$bridge as $crate::Wirable>::WIRE_TYPE;
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
#[macro_export]
macro_rules! str_bridge {
  ($flavor:ty: $(($wire_type:expr):$ty:ty {
    from_str: $from_str: expr;
    to_str: $to_str: expr;

    type EncodedOwned = $owned_ty:ty {
      from_ref: $from_ref: expr;
      from: $from: expr;
    } $(;)?
  }), +$(,)?) => {
    $(
      impl $crate::Wirable<$flavor> for $ty {
        const WIRE_TYPE: <$flavor as $crate::__private::Flavor>::WireType = $wire_type;
      }

      $crate::str_bridge!(@encode $flavor: $ty: $to_str);
      $crate::str_bridge!(@decode $flavor: $ty: $from_str);
      $crate::str_bridge!(@decode_owned $flavor: $ty: $from_str);

      impl $crate::__private::PartialEncode<$flavor> for $ty {
        $crate::partial_encode_primitives!(@impl $flavor);
      }

      impl $crate::__private::IntoTarget<$flavor, $ty> for &::core::primitive::str {
        $crate::str_bridge!(@into_target_impl $flavor: $ty: $from_str);
      }

      impl $crate::__private::TypeRef<$flavor, $ty> for &::core::primitive::str {
        $crate::str_bridge!(@str_to_impl $flavor: $ty: $from_str);
      }

      impl $crate::__private::TypeOwned<$flavor, $ty> for &::core::primitive::str {
        $crate::str_bridge!(@str_to_impl $flavor: $ty: $from_str);
      }

      impl $crate::__private::IntoTarget<$flavor, $ty> for $owned_ty {
        $crate::str_bridge!(@into_target_impl $flavor: $ty: $from);
      }

      impl $crate::__private::TypeOwned<$flavor, $ty> for $owned_ty {
        $crate::str_bridge!(@to_impl $flavor: $ty: $from_ref);
      }

      impl $crate::__private::PartialMessage<$flavor> for $ty {
        type UnknownBuffer<B: ?::core::marker::Sized> = ();

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

      impl $crate::__private::Message<$flavor> for $ty {
        type Partial = Self;

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
  (@encode $flavor:ty: $ty:ty: $to_str:expr) => {
    impl $crate::__private::Encode<$flavor> for $ty {
      $crate::str_bridge!(@encode_impl $flavor: $to_str);
    }
  };
  (@encode_impl $flavor:ty: $to_str:expr) => {
    #[inline]
    fn encode(&self, context: &<$flavor as $crate::__private::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <&::core::primitive::str as $crate::__private::Encode<$flavor>>::encode(&$to_str(self), context, buf)
    }

    #[inline]
    fn encoded_len(&self, context: &<$flavor as $crate::__private::Flavor>::Context) -> ::core::primitive::usize {
      <&::core::primitive::str as $crate::__private::Encode<$flavor>>::encoded_len(&$to_str(self), context)
    }
  };
  (@decode $flavor:ty: $ty:ty: $from_str:expr) => {
    impl<'de> $crate::__private::Decode<'de, $flavor, Self> for $ty {
      $crate::str_bridge!(@decode_impl $flavor: $from_str);
    }
  };
  (@decode_impl $flavor:ty: $from_str:expr) => {
    fn decode<UB>(context: &<$flavor as $crate::__private::Flavor>::Context, src: &'de [::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::UnknownBuffer<$flavor, &'de [::core::primitive::u8]> + 'de,
    {
      <&::core::primitive::str as $crate::__private::Decode<'de, $flavor, &::core::primitive::str>>::decode::<UB>(context, src)
        .and_then(|(read, val)| $from_str(val).map(|v| (read, v)))
    }
  };
  (@decode_owned $flavor:ty: $ty:ty: $from_str:expr) => {
    impl $crate::__private::DecodeOwned<$flavor, Self> for $ty {
      $crate::str_bridge!(@decode_owned_impl $flavor: $from_str);
    }
  };
  (@decode_owned_impl $flavor:ty: $from_str:expr) => {
    fn decode_owned<B, UB>(
      context: &<$flavor as $crate::__private::Flavor>::Context,
      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::Buffer + 'static,
      UB: $crate::__private::UnknownBuffer<$flavor, B> + 'static,
    {
      <Self as $crate::__private::Decode<'_, $flavor, Self>>::decode::<()>(context, $crate::__private::Buffer::as_bytes(&src))
    }
  };
  (@into_target_impl $flavor:ty: $ty:ty: $from:expr) => {
    fn into_target(self) -> ::core::result::Result<$ty, <$flavor as $crate::__private::Flavor>::DecodeError> {
      $from(self)
    }
  };
  (@str_to_impl $flavor:ty: $ty:ty: $from:expr) => {
    fn to(&self) -> ::core::result::Result<$ty, <$flavor as $crate::__private::Flavor>::DecodeError> {
      $from(*self)
    }
  };
  (@to_impl $flavor:ty: $ty:ty: $from:expr) => {
    fn to(&self) -> ::core::result::Result<$ty, <$flavor as $crate::__private::Flavor>::DecodeError> {
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
#[macro_export]
macro_rules! bytes_bridge {
  ($flavor:ty: $(($wire_type:expr): $ty:ty $([const $g:ident: usize])? {
    from_bytes: $from_bytes: expr;
    to_bytes: $to_bytes: expr;

    type EncodedOwned = $owned_ty:ty {
      from_ref: $from_ref: expr;
      from: $from: expr;
    } $(;)?
  }), +$(,)?) => {
    $(
      impl$(<const $g: ::core::primitive::usize>)? $crate::Wirable<$flavor> for $ty {
        const WIRE_TYPE: <$flavor as $crate::__private::Flavor>::WireType = $wire_type;
      }

      $crate::bytes_bridge!(@encode $flavor: $ty $([const $g: usize])?: $to_bytes);
      $crate::bytes_bridge!(@decode $flavor: $ty $([const $g: usize])?: $from_bytes);

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::PartialEncode<$flavor> for $ty {
        $crate::partial_encode_primitives!(@impl $flavor);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::IntoTarget<$flavor, $ty> for &[::core::primitive::u8] {
        $crate::bytes_bridge!(@into_target_impl $flavor: $ty: $from_bytes);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::TypeRef<$flavor, $ty> for &[::core::primitive::u8] {
        $crate::bytes_bridge!(@str_to_impl $flavor: $ty: $from_bytes);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::TypeOwned<$flavor, $ty> for &[::core::primitive::u8] {
        $crate::bytes_bridge!(@str_to_impl $flavor: $ty: $from_bytes);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::IntoTarget<$flavor, $ty> for $owned_ty {
        $crate::bytes_bridge!(@into_target_impl $flavor: $ty: $from);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::TypeOwned<$flavor, $ty> for $owned_ty {
        $crate::bytes_bridge!(@to_impl $flavor: $ty: $from_ref);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::PartialMessage<$flavor> for $ty {
        type UnknownBuffer<B: ?::core::marker::Sized> = ();

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

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::Message<$flavor> for $ty {
        type Partial = Self;

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
  (@encode $flavor:ty: $ty:ty $([const $g:ident: usize])?: $to_bytes:expr) => {
    impl $(<const $g: ::core::primitive::usize>)? $crate::__private::Encode<$flavor> for $ty {
      $crate::bytes_bridge!(@encode_impl $flavor: $to_bytes);
    }
  };
  (@encode_impl $flavor:ty: $to_bytes:expr) => {
    #[inline]
    fn encode(&self, context: &<$flavor as $crate::__private::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <&[::core::primitive::u8] as $crate::__private::Encode<$flavor>>::encode(&$to_bytes(self), context, buf)
    }

    #[inline]
    fn encoded_len(&self, context: &<$flavor as $crate::__private::Flavor>::Context) -> ::core::primitive::usize {
      <&[::core::primitive::u8] as $crate::__private::Encode<$flavor>>::encoded_len(&$to_bytes(self), context)
    }
  };
  (@decode $flavor:ty: $ty:ty $([const $g:ident: usize])?: $from_bytes:expr) => {
    impl<'de, $(const $g: usize)?> $crate::__private::Decode<'de, $flavor, Self> for $ty {
      $crate::bytes_bridge!(@decode_impl $flavor: $from_bytes);
    }
  };
  (@decode_impl $flavor:ty: $from_bytes:expr) => {
    fn decode<UB>(context: &<$flavor as $crate::__private::Flavor>::Context, src: &'de [::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::UnknownBuffer<$flavor, &'de [::core::primitive::u8]> + 'de,
    {
      <&[::core::primitive::u8] as $crate::__private::Decode<'de, $flavor, &[::core::primitive::u8]>>::decode::<UB>(context, src)
        .and_then(|(read, val)| $from_bytes(val).map(|v| (read, v)))
    }
  };
  (@decode_owned $flavor:ty: $ty:ty $([const $g:ident: usize])?: $from_bytes:expr) => {
    impl $(< const $g: usize >)? $crate::__private::Decode<'de, $flavor, Self> for $ty {
      $crate::bytes_bridge!(@decode_owned_impl $from_bytes);
    }
  };
  (@decode_owned_impl $flavor:ty: $from_bytes:expr) => {
    fn decode_owned<B, UB>(context: &<$flavor as $crate::__private::Flavor>::Context, src: B) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::Buffer + 'static,
      UB: $crate::__private::UnknownBuffer<$flavor, B> + 'static,
    {
      <Self as $crate::__private::Decode<'_, $flavor, Self>>::decode::<()>(context, src.as_bytes())
        .and_then(|(read, val)| $from_bytes(val).map(|v| (read, v)))
    }
  };
  (@into_target_impl $flavor:ty: $ty:ty: $from:expr) => {
    fn into_target(self) -> ::core::result::Result<$ty, <$flavor as $crate::__private::Flavor>::DecodeError> {
      $from(self)
    }
  };
  (@str_to_impl $flavor:ty: $ty:ty: $from:expr) => {
    fn to(&self) -> ::core::result::Result<$ty, <$flavor as $crate::__private::Flavor>::DecodeError> {
      $from(*self)
    }
  };
  (@to_impl $flavor:ty: $ty:ty: $from:expr) => {
    fn to(&self) -> ::core::result::Result<$ty, <$flavor as $crate::__private::Flavor>::DecodeError> {
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
    impl<const $g: ::core::primitive::usize> $crate::__private::Wirable<$crate::__private::flavors::Network> for $ty {
      const WIRE_TYPE: $crate::__private::flavors::network::WireType = {
        match N {
          0 => $crate::__private::flavors::network::WireType::Varint,
          _ => $crate::__private::flavors::network::WireType::LengthDelimited,
        }
      };
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::Encode<$crate::__private::flavors::Network> for $ty {
      fn encode(
        &self,
        context: &$crate::__private::flavors::network::Context,
        buf: &mut [::core::primitive::u8],
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
        if N == 0 {
          return ::core::result::Result::Ok(0);
        }

        $crate::__private::Encode::encode(&self.as_bytes(), context, buf)
      }

      #[inline]
      fn encoded_len(&self, context: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
        if N == 0 {
          return 0;
        }

        $crate::__private::Encode::encoded_len(&self.as_bytes(), context)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::PartialEncode<$crate::__private::flavors::Network> for $ty {
      $crate::partial_encode_primitives!(@impl $crate::__private::flavors::Network);
    }

    impl<'de, const $g: ::core::primitive::usize> $crate::__private::Decode<'de, $crate::__private::flavors::Network, Self> for $ty {
      fn decode<UB>(
        context: &$crate::__private::flavors::network::Context,
        src: &'de [::core::primitive::u8],
      ) -> Result<(::core::primitive::usize, Self), $crate::__private::flavors::network::DecodeError>
      where
        Self: ::core::marker::Sized + 'de,
        UB: $crate::__private::UnknownBuffer<$crate::__private::flavors::Network, &'de [::core::primitive::u8]>,
      {
        if N == 0 {
          return ::core::result::Result::Ok((0, $new()));
        }

        $crate::__private::network::decode(context, src, |src| {
          if src.len() > N {
            return ::core::result::Result::Err($crate::__private::larger_than_str_capacity::<N>());
          }

          $decode(src)
        })
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::DecodeOwned<$crate::__private::flavors::Network, Self> for $ty {
      fn decode_owned<B, UB>(
        ctx: &$crate::__private::flavors::network::Context,
        src: B,
      ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::flavors::network::DecodeError>
      where
        Self: ::core::marker::Sized + 'static,
        B: $crate::__private::Buffer + 'static,
        UB: $crate::__private::UnknownBuffer<$crate::__private::flavors::Network, B> + 'static,
      {
        <Self as $crate::__private::Decode<'_, $crate::__private::flavors::Network, Self>>::decode::<()>(ctx, $crate::__private::Buffer::as_bytes(&src))
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::IntoTarget<$crate::__private::flavors::Network, Self> for $ty {
      fn into_target(self) -> ::core::result::Result<Self, $crate::__private::flavors::network::DecodeError> {
        ::core::result::Result::Ok(self)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::TypeRef<$crate::__private::flavors::Network, Self> for $ty {
      fn to(&self) -> ::core::result::Result<Self, $crate::__private::flavors::network::DecodeError> {
        ::core::result::Result::Ok(::core::clone::Clone::clone(self))
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::TypeOwned<$crate::__private::flavors::Network, Self> for $ty {
      fn to(&self) -> ::core::result::Result<Self, $crate::__private::flavors::network::DecodeError> {
        ::core::result::Result::Ok(::core::clone::Clone::clone(self))
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::IntoTarget<$crate::__private::flavors::Network, $ty> for &str {
      fn into_target(self) -> ::core::result::Result<$ty, $crate::__private::flavors::network::DecodeError> {
        $from_str(self)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::TypeRef<$crate::__private::flavors::Network, $ty> for &str {
      fn to(&self) -> ::core::result::Result<$ty, $crate::__private::flavors::network::DecodeError> {
        $from_str(*self)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::PartialMessage<$crate::__private::flavors::Network> for $ty {
      type UnknownBuffer<B: ?::core::marker::Sized> = ();

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

    impl<const $g: ::core::primitive::usize> $crate::__private::Message<$crate::__private::flavors::Network> for $ty {
      type Partial = Self;

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

/// A macro emits traits implementations for a array-style `[u8]` type for [`Network`](crate::flavors::Network) flavor.
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
    impl<const $g: ::core::primitive::usize> $crate::__private::Wirable<$crate::__private::flavors::network::Network> for $ty {
      const WIRE_TYPE: $crate::__private::flavors::network::WireType = {
        match N {
          0 => $crate::__private::flavors::network::WireType::Zst,
          _ => $crate::__private::flavors::network::WireType::LengthDelimited,
        }
      };
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::Encode<$crate::__private::flavors::Network> for $ty {
      fn encode(
        &self,
        context: &$crate::__private::flavors::network::Context,
        buf: &mut [::core::primitive::u8],
      ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
        if N == 0 {
          return ::core::result::Result::Ok(0);
        }

        $crate::__private::Encode::encode(&$as_bytes(self), context, buf)
      }

      #[inline]
      fn encoded_len(&self, context: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
        if N == 0 {
          return 0;
        }

        $crate::__private::Encode::encoded_len(&$as_bytes(self), context)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::PartialEncode<$crate::__private::flavors::Network> for $ty {
      $crate::partial_encode_primitives!(@impl $crate::__private::flavors::Network);
    }

    impl<'de, const $g: ::core::primitive::usize> $crate::__private::Decode<'de, $crate::__private::flavors::Network, Self> for $ty {
      fn decode<UB>(
        context: &$crate::__private::flavors::network::Context,
        src: &'de [::core::primitive::u8],
      ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::flavors::network::DecodeError>
      where
        Self: ::core::marker::Sized + 'de,
        UB: $crate::__private::UnknownBuffer<$crate::__private::flavors::Network, &'de [::core::primitive::u8]>,
      {
        if N == 0 {
          return ::core::result::Result::Ok((0, $new()));
        }

        $crate::__private::network::decode(context, src, |src| {
          if src.len() > N {
            return ::core::result::Result::Err($crate::__private::larger_than_array_capacity::<N>());
          }

          $decode(src)
        })
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::DecodeOwned<$crate::__private::flavors::Network, Self> for $ty {
      fn decode_owned<B, UB>(
        ctx: &$crate::__private::flavors::network::Context,
        src: B,
      ) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::flavors::network::DecodeError>
      where
        Self: ::core::marker::Sized + 'static,
        B: $crate::__private::Buffer + 'static,
        UB: $crate::__private::UnknownBuffer<$crate::__private::flavors::Network, B>,
      {
        <Self as $crate::__private::Decode<'_, $crate::__private::flavors::Network, Self>>::decode::<()>(ctx, $crate::__private::Buffer::as_bytes(&src))
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::IntoTarget<$crate::__private::flavors::Network, Self> for $ty {
      fn into_target(self) -> ::core::result::Result<Self, $crate::__private::flavors::network::DecodeError> {
        ::core::result::Result::Ok(self)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::TypeRef<$crate::__private::flavors::Network, Self> for $ty {
      fn to(&self) -> ::core::result::Result<Self, $crate::__private::flavors::network::DecodeError> {
        ::core::result::Result::Ok(::core::clone::Clone::clone(self))
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::TypeOwned<$crate::__private::flavors::Network, Self> for $ty {
      fn to(&self) -> ::core::result::Result<Self, $crate::__private::flavors::network::DecodeError> {
        ::core::result::Result::Ok(::core::clone::Clone::clone(self))
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::IntoTarget<$crate::__private::flavors::Network, $ty>
      for &[::core::primitive::u8]
    {
      fn into_target(self) -> ::core::result::Result<$ty, $crate::__private::flavors::network::DecodeError> {
        $from_bytes(self)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::TypeRef<$crate::__private::flavors::Network, $ty>
      for &[::core::primitive::u8]
    {
      fn to(&self) -> ::core::result::Result<$ty, $crate::__private::flavors::network::DecodeError> {
        $from_bytes(*self)
      }
    }

    impl<const $g: ::core::primitive::usize> $crate::__private::PartialMessage<$crate::__private::flavors::Network> for $ty {
      type UnknownBuffer<B: ?::core::marker::Sized> = ();

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

    impl<const $g: ::core::primitive::usize> $crate::__private::Message<$crate::__private::flavors::Network> for $ty {
      type Partial = Self;

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

/// A macro emits [`TypeRef`](super::TypeRef) implementations for `Self`
#[macro_export]
macro_rules! type_ref {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::TypeRef<$flavor, Self> for $ty {
        $crate::type_ref!(@copy_impl $flavor);
      }
    )*
  };
  (@clone $flavor:ty: $ty:ty $($([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::TypeRef<$flavor, Self> for $ty {
        $crate::type_ref!(@clone_impl $flavor);
      }
    )*
  };
  (@copy_impl $flavor:ty) => {
    fn to(&self) -> ::core::result::Result<Self, <$flavor as $crate::__private::Flavor>::DecodeError> {
      ::core::result::Result::Ok(*self)
    }
  };
  (@clone_impl $flavor:ty) => {
    fn to(&self) -> ::core::result::Result<Self, <$flavor as $crate::__private::Flavor>::DecodeError> {
      ::core::result::Result::Ok(::core::clone::Clone::clone(self))
    }
  }
}

/// A macro emits [`TypeOwned`](super::TypeOwned) implementations for `Self`
#[macro_export]
macro_rules! type_owned {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::TypeOwned<$flavor, Self> for $ty {
        $crate::type_ref!(@copy_impl $flavor);
      }
    )*
  };
  (@clone $flavor:ty: $ty:ty $($([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::TypeOwned<$flavor, Self> for $ty {
        $crate::type_ref!(@clone_impl);
      }
    )*
  };
  (@copy_impl $flavor:ty) => {
    fn to(&self) -> ::core::result::Result<Self, <$flavor as $crate::__private::Flavor>::DecodeError> {
      ::core::result::Result::Ok(*self)
    }
  };
  (@clone_impl $flavor:ty) => {
    fn to(&self) -> ::core::result::Result<Self, <$flavor as $crate::__private::Flavor>::DecodeError> {
      ::core::result::Result::Ok(::core::clone::Clone::clone(self))
    }
  }
}

/// A macro emits [`IntoTarget`](super::IntoTarget) implementations for `Self`
#[macro_export]
macro_rules! into_target {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? ),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::IntoTarget<$flavor, Self> for $ty {
        $crate::into_target!(@impl $flavor);
      }
    )*
  };
  (@impl $flavor:ty) => {
    fn into_target(self) -> ::core::result::Result<Self, <$flavor as $crate::__private::Flavor>::DecodeError> {
      ::core::result::Result::Ok(self)
    }
  }
}

/// A macro emits convertion traits implementations for `Self`
#[macro_export]
macro_rules! conversion {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $crate::type_ref!($flavor: $($ty $([ $(const $g: usize),* ])?),+);
    $crate::type_owned!($flavor: $($ty $([ $(const $g: usize),* ])?),+);
    $crate::into_target!($flavor: $($ty $([ $(const $g: usize),* ])?),+);
  };
  (@clone $flavor:ty: $ty:ty $($([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $crate::type_ref!(@clone $flavor: $($ty $([ $(const $g: usize),* ])?),+);
    $crate::type_owned!(@clone $flavor: $($ty $([ $(const $g: usize),* ])?),+);
    $crate::into_target!($($ty $flavor: $([ $(const $g: usize),* ])?),+);
  };
}

/// A macro emits [`Message`](super::Message) implementations for `Self`.
///
/// **NB:** this macro can only be used for types that implements [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! message {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Message<$flavor> for $ty {
        $crate::message!(@impl);
      }

      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialMessage<$flavor> for $ty {
        $crate::message!(@impl_partial);
      }
    )*

    $($crate::conversion!($flavor: $ty $([ $(const $g: usize),* ])? );)*
  };
  (@impl) => {
    type Partial = Self;

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
  (@impl_partial) => {
    type UnknownBuffer<B: ?::core::marker::Sized> = ();

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
  (@impl $flavor:ty) => {
    type Selection = ();

    fn partial_encode(&self, context: &<$flavor as $crate::__private::Flavor>::Context, buf: &mut [::core::primitive::u8],  _: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <Self as $crate::__private::Encode<$flavor>>::encode(self, context, buf)
    }

    fn partial_encoded_len(&self, context: &<$flavor as $crate::__private::Flavor>::Context, _: &Self::Selection) -> ::core::primitive::usize {
      <Self as $crate::__private::Encode<$flavor>>::encoded_len(self, context)
    }
  };
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialEncode<$flavor> for $ty {
        $crate::partial_encode_primitives!(@impl $flavor);
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
    const WIRE_TYPE: $crate::__private::flavors::network::WireType = $crate::__private::flavors::network::WireType::Byte;
  };
  (@varint) => {
    const WIRE_TYPE: $crate::__private::flavors::network::WireType = $crate::__private::flavors::network::WireType::Varint;
  };
  (@length_delimited) => {
    const WIRE_TYPE: $crate::__private::flavors::network::WireType = $crate::__private::flavors::network::WireType::LengthDelimited;
  };
  (@zst) => {
    const WIRE_TYPE: $crate::__private::flavors::network::WireType = $crate::__private::flavors::network::WireType::Zst;
  };
  (@fixed16) => {
    const WIRE_TYPE: $crate::__private::flavors::network::WireType = $crate::__private::flavors::network::WireType::Fixed16;
  };
  (@fixed32) => {
    const WIRE_TYPE: $crate::__private::flavors::network::WireType = $crate::__private::flavors::network::WireType::Fixed32;
  };
  (@fixed64) => {
    const WIRE_TYPE: $crate::__private::flavors::network::WireType = $crate::__private::flavors::network::WireType::Fixed64;
  };
  (@fixed128) => {
    const WIRE_TYPE: $crate::__private::flavors::network::WireType = $crate::__private::flavors::network::WireType::Fixed128;
  };
  ($(
    (@$wire_varint:ident) <=> ($ty:ty $($([ $( const $g:ident: usize), +$(,)? ])?), +$(,)?)
  ),+$(,)?) => {
    $(
      $(
        impl$( < $(const $g: ::core::primitive::usize),* > )? $crate::Wirable<$crate::__private::flavors::Network> for $ty {
          $crate::wirable!(@$wire_varint);
        }
      )*
    )*
  }
}

/// A macro emits traits implementations for primitive types that implements [`varing::Varint`](varing::Varint) and [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! varint {
  ($(
    $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?
  ),+$(,)?) => {
    $($crate::wirable!((@varint) <=> ($ty $([ $(const $g: usize),* ])?));)*
    $($crate::message!($crate::__private::flavors::Network: $ty $([$(const $g: usize),*])?);)*
    $($crate::partial_encode_primitives!($crate::__private::flavors::Network: $ty $([ $(const $g: usize),* ])?);)*
    $($crate::varint!(@encode $ty $([ $(const $g: usize),* ])?);)*
    $($crate::varint!(@decode $ty $([ $(const $g: usize),* ])?);)*
    $($crate::varint!(@decode_owned $ty $([ $(const $g: usize),* ])?);)*
  };
  (@encode $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Encode<$crate::__private::flavors::Network> for $ty {
      $crate::varint!(@encode_impl);
    }
  };
  (@encode_impl) => {
    fn encode(&self, ctx: &$crate::__private::flavors::network::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<usize, $crate::__private::flavors::network::EncodeError> {
      $crate::__private::network::encode(
        ctx,
        self,
        buf,
        |val, buf| $crate::__private::varing::Varint::encode(val, buf).map_err(::core::convert::Into::into),
        $crate::__private::varing::Varint::encoded_len,
      )
    }

    fn encoded_len(&self, ctx: &$crate::__private::flavors::network::Context,) -> ::core::primitive::usize {
      $crate::__private::network::encoded_len(ctx, self, $crate::__private::varing::Varint::encoded_len)
    }
  };
  (@decode $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl<'de, $($(const $g: ::core::primitive::usize),*)?> $crate::__private::Decode<'de, $crate::__private::flavors::Network, Self> for $ty {
      $crate::varint!(@decode_impl);
    }
  };
  (@decode_impl) => {
    fn decode<UB>(ctx: &$crate::__private::flavors::network::Context, src: &'de [::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::flavors::network::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::UnknownBuffer<$crate::__private::flavors::Network, &'de [::core::primitive::u8]> + 'de,
    {
      $crate::__private::network::decode(ctx, src, |buf| $crate::__private::varing::Varint::decode(buf).map_err(::core::convert::Into::into))
    }
  };
  (@decode_owned $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl$(<$(const $g: ::core::primitive::usize),*>)? $crate::__private::DecodeOwned<$crate::__private::flavors::Network, Self> for $ty {
      $crate::varint!(@decode_owned_impl);
    }
  };
  (@decode_owned_impl) => {
    fn decode_owned<B, UB>(ctx: &$crate::__private::flavors::network::Context, src: B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::flavors::network::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::Buffer + 'static,
      UB: $crate::__private::UnknownBuffer<$crate::__private::flavors::Network, B> + 'static,
    {
      <Self as $crate::__private::Decode<'_, $crate::__private::flavors::Network, Self>>::decode::<()>(ctx, $crate::__private::Buffer::as_bytes(&src))
    }
  };
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
      $($crate::message!($crate::__private::flavors::Network: $ty $([$(const $g: usize),*])?);)*
      $($crate::partial_encode_primitives!($crate::__private::flavors::Network: $ty $([ $(const $g: usize),* ])?);)*
      $($crate::fixed!(@encode $size ($ty $([ $(const $g: usize),* ])? { to_bytes: $to_bytes }));)*
      $($crate::fixed!(@decode $size ($ty $([ $(const $g: usize),* ])? { from_bytes: $from_bytes }));)*
      $($crate::fixed!(@decode_owned $size ($ty $([ $(const $g: usize),* ])? { from_bytes: $from_bytes } ));)*
    )*
  };
  (@encode $size:literal($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? {
    to_bytes: $to_bytes:expr $(,)?
  })) => {
    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Encode<$crate::__private::flavors::Network> for $ty {
      $crate::fixed!(@encode_impl $size { $to_bytes });
    }
  };
  (@encode_impl $size:literal { $to_slice:expr $(,)? }) => {
    fn encode(&self, ctx: &$crate::__private::flavors::network::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
      const SIZE: ::core::primitive::usize = $size / 8;
      $crate::__private::network::encode(ctx, self, buf, $to_slice, |_| SIZE)
    }

    fn encoded_len(&self, ctx: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
      const SIZE: ::core::primitive::usize = $size / 8;
      $crate::__private::network::encoded_len(ctx, self, |_| SIZE)
    }
  };
  (@decode $size:literal($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? { from_bytes: $from_bytes:expr $(,)? })) => {
    impl<'de, $($(const $g: ::core::primitive::usize),*)?> $crate::__private::Decode<'de, $crate::__private::flavors::Network, Self> for $ty {
      $crate::fixed!(@decode_impl $size { $from_bytes });
    }
  };
  (@decode_impl $size:literal { $from_slice:expr $(,)? }) => {
    fn decode<UB>(ctx: &$crate::__private::flavors::network::Context, src: &'de [::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::flavors::network::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::UnknownBuffer<$crate::__private::flavors::Network, &'de [::core::primitive::u8]> + 'de,
    {
      $crate::__private::network::decode(ctx, src, $from_slice)
    }
  };
  (@decode_owned $size:literal($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? { from_bytes: $from_bytes:expr $(,)? })) => {
    impl $(< $(const $g: ::core::primitive::usize),* >)? $crate::__private::DecodeOwned<$crate::__private::flavors::Network, Self> for $ty {
      $crate::fixed!(@decode_owned_impl $size { $from_bytes });
    }
  };
  (@decode_owned_impl $size:literal { $from_slice:expr $(,)? }) => {
    fn decode_owned<B, UB>(ctx: &$crate::__private::flavors::network::Context, src: B) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::flavors::network::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::Buffer + 'static,
      UB: $crate::__private::UnknownBuffer<$crate::__private::flavors::Network, B> + 'static,
    {
      <Self as $crate::__private::Decode<'_, $crate::__private::flavors::Network, _>>::decode::<()>(
        ctx,
        $crate::__private::Buffer::as_bytes(&src),
      )
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
        const WIRE_TYPE: $crate::__private::flavors::network::WireType = {
          assert!(::core::mem::size_of::<Self>() == 0, "Not a zero-sized type");

          $crate::__private::flavors::network::WireType::Zst
        };
      }

      impl<T: ?::core::marker::Sized> $crate::__private::Encode for $ty {
        #[inline]
        fn encode(&self, _: &$crate::__private::flavors::network::Context, _: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn encoded_len(&self, _: &$crate::__private::flavors::network::Context) -> ::core::primitive::usize {
          0
        }
      }

      impl<'de, T: ?::core::marker::Sized> $crate::__private::Decode<'de, Self> for $ty {
        fn decode<UB>(_: &$crate::__private::flavors::network::Context, _: &'de [::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), $crate::__private::flavors::network::DecodeError>
        where
          Self: ::core::marker::Sized + 'de,
          UB: $crate::__private::UnknownBuffer<&'de [::core::primitive::u8]> + 'de,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }
      }

      impl<T: ?::core::marker::Sized> $crate::__private::PartialEncode for $ty {
        type Selection = ();

        #[inline]
        fn partial_encode(&self, _: &$crate::__private::flavors::network::Context, _: &mut [u8], _: &Self::Selection,) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn partial_encoded_len(&self, _: &$crate::__private::flavors::network::Context, _: &Self::Selection,) -> ::core::primitive::usize {
          0
        }
      }

      impl<T: ?::core::marker::Sized> $crate::__private::PartialMessage for $ty {
        type UnknownBuffer<B: ?::core::marker::Sized> = ();

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

      impl<T: ?::core::marker::Sized> $crate::__private::Message for $ty {
        type Partial = Self;

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
        fn to(&self) -> ::core::result::Result<Self, $crate::__private::flavors::network::DecodeError> {
          ::core::result::Result::Ok(::core::default::Default::default())
        }
      }

      impl<T: ?::core::marker::Sized> $crate::__private::TypeRef<Self> for $ty {
        fn to(&self) -> ::core::result::Result<Self, $crate::__private::flavors::network::DecodeError> {
          ::core::result::Result::Ok(::core::default::Default::default())
        }
      }

      impl<T: ?::core::marker::Sized> $crate::__private::IntoTarget<Self> for $ty {
        fn into_target(self) -> ::core::result::Result<Self, $crate::__private::flavors::network::DecodeError> {
          ::core::result::Result::Ok(self)
        }
      }
    )*
  };
}
