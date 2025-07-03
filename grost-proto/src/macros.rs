/// A macro emits traits implementations for primitive types that implements [`varing::Varint`](varing::Varint) and [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! groto_varint {
  ($(
    $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?
  ),+$(,)?) => {
    $crate::varint!(
      $crate::__private::flavors::Groto:
        $crate::__private::flavors::groto::Varint {
          $(
            $ty $([ $(const $g: usize),* ])?
          ),+
        }
    );

    $(
      // impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::convert::Transform<$ty, ::core::option::Option<$ty>, $crate::__private::flavors::groto::Nullable<$crate::__private::flavors::groto::Varint>, $crate::__private::flavors::Groto> for ::core::option::Option<$ty> {
      //   fn transform(input: $ty) -> ::core::result::Result<Self, <$crate::__private::flavors::Groto as $crate::__private::flavors::Flavor>::Error> {
      //     ::core::result::Result::Ok(::core::option::Option::Some(input))
      //   }
      // }

      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::convert::Transform<$ty, ::core::option::Option<$ty>, $crate::__private::flavors::groto::Nullable<$crate::__private::flavors::groto::Varint>, $crate::__private::flavors::Groto> for $ty {
        fn transform(input: $ty) -> ::core::result::Result<::core::option::Option<Self>, <$crate::__private::flavors::Groto as $crate::__private::flavors::Flavor>::Error> {
          ::core::result::Result::Ok(::core::option::Option::Some(input))
        }
      }
    )*
  };
}

/// A macro emits traits implementations for primitive types that implements [`varing::Varint`](varing::Varint) and [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! varint {
  ($flavor:ty:$wf:ty {
    $(
      $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?
    ),+$(,)?
  }) => {
    $($crate::selectable!(@scalar $flavor: $ty $([ $(const $g: usize),* ])?);)*
    $($crate::partial_ref_state!(@scalar &'a $flavor: $ty $([ $(const $g: usize),* ])? as $wf);)*
    $($crate::partial_state!(@scalar $flavor: $ty $([ $(const $g: usize),* ])?);)*
    $($crate::flatten_state!($ty $([ $(const $g: usize),* ])?);)*
    $($crate::default_wire_format!($flavor: $ty $([$(const $g: usize),*])? as $wf);)*
    $($crate::partial_encode_scalar!($flavor: $ty $([ $(const $g: usize),* ])? as $wf);)*
    $($crate::varint!(@encode $flavor:$wf:$ty $([ $(const $g: usize),* ])?);)*
    $($crate::varint!(@decode $flavor:$wf:$ty $([ $(const $g: usize),* ])?);)*
    $($crate::identity_transform!($flavor {
      $ty $([ $(const $g: usize),* ])? as $wf,
    });)*

    $(
      impl $(< $(const $g: ::core::primitive::usize),* > )? $crate::__private::convert::PartialTransform<$ty, ::core::option::Option<$ty>, $wf, $flavor> for $ty {
        fn partial_transform(input: Self, selector: &bool) -> ::core::result::Result<::core::option::Option<$ty>, <$flavor as $crate::__private::flavors::Flavor>::Error>
        {
          if $crate::__private::selection::Selector::<$flavor>::is_empty(selector) {
            return ::core::result::Result::Ok(::core::option::Option::None);
          }

          ::core::result::Result::Ok(::core::option::Option::Some(input))
        }
      }

      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::convert::PartialTransform<::core::option::Option<Self>, ::core::option::Option<Self>, $crate::__private::flavors::groto::Nullable<$wf>, $flavor,> for $ty {
        fn partial_transform(input: ::core::option::Option<Self>, selector: &<Self as $crate::__private::selection::Selectable<$flavor>>::Selector) -> ::core::result::Result<::core::option::Option<Self>, <$flavor as $crate::__private::flavors::Flavor>::Error>
        {
          match input {
            ::core::option::Option::None => ::core::result::Result::Ok(::core::option::Option::None),
            ::core::option::Option::Some(input) => {
              if $crate::__private::selection::Selector::<$flavor>::is_empty(selector) {
                return ::core::result::Result::Ok(::core::option::Option::None);
              }

              ::core::result::Result::Ok(::core::option::Option::Some(input))
            }
          }
        }
      }
    )*
  };
  (@without_flatten_state $flavor:ty:$wf:ty {
    $(
      $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?
    ),+$(,)?
  }) => {
    $($crate::selectable!(@scalar $flavor: $ty $([ $(const $g: usize),* ])?);)*
    $($crate::partial_ref_state!(@scalar &'a $flavor: $ty $([ $(const $g: usize),* ])? as $wf);)*
    $($crate::default_wire_format!($flavor: $ty $([$(const $g: usize),*])? as $wf);)*
    $($crate::partial_encode_scalar!($flavor: $ty $([ $(const $g: usize),* ])? as $wf);)*
    $($crate::varint!(@encode $flavor:$wf:$ty $([ $(const $g: usize),* ])?);)*
    $($crate::varint!(@decode $flavor:$wf:$ty $([ $(const $g: usize),* ])?);)*
    $($crate::identity_transform!($flavor {
      $ty $([ $(const $g: usize),* ])? as $wf,
    });)*
  };
  (@encode $flavor:ty:$wf:ty:$ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Encode<$wf, $flavor> for $ty {
      $crate::varint!(@encode_impl $flavor:$wf);
    }
  };
  (@encode_impl $flavor:ty:$wf:ty) => {
    fn encode_raw(&self, _: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      $crate::__private::varing::Varint::encode(self, buf).map_err(::core::convert::Into::into)
    }

    fn encoded_raw_len(&self, _: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      $crate::__private::varing::Varint::encoded_len(self)
    }

    fn encode(&self, _: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      $crate::__private::varing::Varint::encode(self, buf).map_err(::core::convert::Into::into)
    }

    fn encoded_len(&self, _: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      $crate::__private::varing::Varint::encoded_len(self)
    }

    fn encode_length_delimited(&self, ctx: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      <Self as $crate::__private::Encode<$wf, $flavor>>::encode(self, ctx, buf)
    }

    fn encoded_length_delimited_len(&self, ctx: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      <Self as $crate::__private::Encode<$wf, $flavor>>::encoded_len(self, ctx)
    }
  };
  (@decode $flavor:ty:$wf:ty:$ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl<'de, RB, B, $($(const $g: ::core::primitive::usize),*)?> $crate::__private::decode::Decode<'de, Self, $wf, RB, B, $flavor> for $ty {
      $crate::varint!(@decode_impl $flavor:$wf);
    }
  };
  (@decode_impl $flavor:ty:$wf:ty) => {
    fn decode(_: &<$flavor as $crate::__private::flavors::Flavor>::Context, src: RB) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      RB: $crate::__private::ReadBuf,
      B: $crate::__private::Buffer<<$flavor as $crate::__private::flavors::Flavor>::Unknown<RB>> + 'de,
    {
      $crate::__private::varing::Varint::decode(src.as_bytes()).map_err(::core::convert::Into::into)
    }
  };
}

/// A macro emits [`PartialEncode`](super::encode::PartialEncode) implementations for scalar types.
#[macro_export]
macro_rules! partial_encode_scalar {
  (@impl $flavor:ty as $format:ty) => {
    fn partial_encode_raw(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut [::core::primitive::u8], s: &Self::Selector) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      if *s {
        <Self as $crate::__private::Encode<$format, $flavor>>::encode_raw(self, context, buf)
      } else {
        ::core::result::Result::Ok(0)
      }
    }

    fn partial_encoded_raw_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, s: &Self::Selector) -> ::core::primitive::usize {
      if *s {
        <Self as $crate::__private::Encode<$format, $flavor>>::encoded_raw_len(self, context)
      } else {
        0
      }
    }

    fn partial_encode(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut [::core::primitive::u8], s: &Self::Selector) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      if *s {
        <Self as $crate::__private::Encode<$format, $flavor>>::encode(self, context, buf)
      } else {
        ::core::result::Result::Ok(0)
      }
    }

    fn partial_encoded_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, s: &Self::Selector) -> ::core::primitive::usize {
      if *s {
        <Self as $crate::__private::Encode<$format, $flavor>>::encoded_len(self, context)
      } else {
        0
      }
    }

    fn partial_encode_length_delimited(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut [::core::primitive::u8], s: &Self::Selector) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      if *s {
        <Self as $crate::__private::Encode<$format, $flavor>>::encode_length_delimited(self, context, buf)
      } else {
        ::core::result::Result::Ok(0)
      }
    }

    fn partial_encoded_length_delimited_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, s: &Self::Selector) -> ::core::primitive::usize {
      if *s {
        <Self as $crate::__private::Encode<$format, $flavor>>::encoded_length_delimited_len(self, context)
      } else {
        0
      }
    }
  };
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialEncode<$format, $flavor> for $ty {
        $crate::partial_encode_scalar!(@impl $flavor as $format);
      }
    )*
  };
}

/// A macro emits [`Transform`](super::convert::Transform) implementations for the `Self`
#[macro_export]
macro_rules! identity_transform {
  ($flavor:ty {
    $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $wf:ty), +$(,)?
  }) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::convert::Transform<$ty, $ty, $wf, $flavor> for $ty {
        fn transform(input: $ty) -> ::core::result::Result<Self, <$flavor as $crate::__private::flavors::Flavor>::Error> {
          ::core::result::Result::Ok(input)
        }
      }
    )*
  };
}

/// A macro emits [`Transform`](super::convert::Transform) implementations for the `Self` for `Groto` flavor.
#[macro_export]
macro_rules! groto_identity_transform {
  ($($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $wf:ty), +$(,)?) => {
    $($crate::identity_transform!($crate::__private::flavors::Groto {
      $ty $([ $(const $g: usize),* ])? as $wf,
    });)*

    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::convert::Transform<$ty, ::core::option::Option<$ty>, $crate::__private::flavors::groto::Nullable<$wf>, $crate::__private::flavors::Groto> for ::core::option::Option<$ty> {
        fn transform(input: $ty) -> ::core::result::Result<Self, <$crate::__private::flavors::Groto as $crate::__private::flavors::Flavor>::Error> {
          ::core::result::Result::Ok(::core::option::Option::Some(input))
        }
      }

      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::convert::Transform<$ty, ::core::option::Option<$ty>, $crate::__private::flavors::groto::Nullable<$wf>, $crate::__private::flavors::Groto> for $ty {
        fn transform(input: $ty) -> ::core::result::Result<::core::option::Option<Self>, <$crate::__private::flavors::Groto as $crate::__private::flavors::Flavor>::Error> {
          ::core::result::Result::Ok(::core::option::Option::Some(input))
        }
      }
    )*
  };
}

/// A macro emits [`Selectable`](super::selector::Selectable) implementations for scalar types.
#[macro_export]
macro_rules! selectable {
  (@scalar $flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)?])?),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl$(< $(const $g: ::core::primitive::usize),* >)? $crate::__private::selection::Selectable<$flavor> for $ty {
        type Selector = ::core::primitive::bool;

        fn is_empty(&self) -> bool {
          false
        }
      }
    )*
  };
  (@bridge $flavor:ty: $(
    $bridge: ty [
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $wf:ty), +$(,)?
    ]
  ),+$(,)?) => {
    $(
      $(
        impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::selection::Selectable<$flavor> for $ty {
          type Selector = <$bridge as $crate::__private::selection::Selectable<$flavor>>::Selector;
        }
      )*
    )*
  };
}

/// A macro emits basic [`State<PartialRef<'_, Flavor, WireFormat, Src, UB>>`](super::State) implementations for `Self`
#[macro_export]
macro_rules! partial_ref_state {
  (&$lifetime:lifetime $flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $wf:ty => $target:ty ),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<$lifetime, __GROST_READ_BUF__, __GROST_BUFFER__, $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::State<$crate::__private::convert::PartialRef<$lifetime, __GROST_READ_BUF__, __GROST_BUFFER__, $wf, $flavor>> for $ty {
        type Output = $target;
      }
    )*
  };
  (@scalar &$lifetime:lifetime $flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $wf:ty),+$(,)?) => {
    $crate::partial_ref_state!(& $lifetime $flavor: $($ty $([ $(const $g: usize),* ])? as $wf => $ty),+);
  };
}

/// A macro emits basic [`State<Partial<Flavor, WireFormat>>`](super::State) implementations for `Self`
#[macro_export]
macro_rules! partial_state {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? => $target:ty ),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl$(< $(const $g: ::core::primitive::usize),* > )? $crate::__private::State<$crate::__private::convert::Partial<$flavor>> for $ty {
        type Output = $target;
      }
    )*
  };
  (@scalar $flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $crate::partial_state!($flavor: $($ty $([ $(const $g: usize),* ])? => $ty),+);
  };
}

/// A macro emits [`impl Reflectable<Self> for Reflection<Self, Type, Flavor>`](super::reflection::Reflectable) implementations for `Self`
#[macro_export]
macro_rules! type_reflection {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? => $expr:expr),+$(,)?) => {
    $(
      impl$(<$(const $g: ::core::primitive::usize),*>)? $crate::__private::reflection::Reflectable<$ty> for $crate::__private::reflection::SchemaTypeReflection<$ty> {
        type Reflection = $crate::__private::reflection::SchemaType;

        const REFLECTION: &Self::Reflection = &{
          $expr
        };
      }
    )*
  };
}

/// A macro emits basic [`State<Flattened<_>>`](super::State) implementations for `Self`
#[macro_export]
macro_rules! flatten_state {
  ($($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl<S: ?::core::marker::Sized, $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::State<
        $crate::__private::convert::Flattened<S>
      > for $ty {
        type Output = $ty;
      }
    )*
  };
  (@scalar $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $crate::flatten_state!($($ty $([ $(const $g: usize),* ])? => $ty),+);
  };
}

/// A macro emits [`DefaultWireFormat`](crate::flavors::DefaultWireFormat) implementations.
#[macro_export]
macro_rules! default_wire_format {
  ($(
    $flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty); +$(;)?
  ),+$(,)?) => {
    $(
      $(
        impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::flavors::DefaultWireFormat<$flavor> for $ty {
          type Format = $format;
        }
      )*
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
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        from: $from:expr;
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $crate::encode_bridge!($flavor: $bridge {
        $($ty $([ $(const $g: usize),* ])? as $format {
          convert: $to;
        },)*
      });

      $crate::decode_bridge!($flavor: $bridge => $bridge {
        $($ty $([ $(const $g: usize),* ])? as $format {
          convert: $from;
        },)*
      });
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
macro_rules! try_from_bridge {
  ($(
    $flavor:ty: $bridge: ty => $output:ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        try_from: $from:expr;
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $crate::encode_bridge!($flavor: $bridge {
        $($ty $([ $(const $g: usize),* ])? as $format {
          convert: $to;
        },)*
      });

      $crate::try_decode_bridge!($flavor: $bridge => $output {
        $($ty $([ $(const $g: usize),* ])? as $format {
          convert: $from;
        },)*
      });
    )*
  };
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        try_from: $from:expr;
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $crate::encode_bridge!($flavor: $bridge {
        $($ty $([ $(const $g: usize),* ])? as $format {
          convert: $to;
        },)*
      });

      $crate::try_decode_bridge!($flavor: $bridge => $bridge {
        $($ty $([ $(const $g: usize),* ])? as $format {
          convert: $from;
        },)*
      });
    )*
  };
}

/// A macro emits [`Encode`](super::encode::Encode) implementations for `Self`.
#[macro_export]
macro_rules! encode_bridge {
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        convert: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::encode_bridge!(@encode $flavor: $bridge {
          $ty $([ $(const $g: usize),* ])? as $format {
            to: $to;
          }
        });

        $crate::encode_bridge!(@partial_encode $flavor: $bridge {
          $ty $([ $(const $g: usize),* ])? as $format {
            to: $to;
          }
        });
      )*
    )*
  };
  (@encode_impl $flavor:ty: $bridge:ty as $format:ty => $to:expr) => {
    fn encode_raw(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      <$bridge as $crate::__private::Encode<$format, $flavor>>::encode_raw(&$to(self), context, buf)
    }

    fn encoded_raw_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      <$bridge as $crate::__private::Encode<$format, $flavor>>::encoded_raw_len(&$to(self), context)
    }

    fn encode(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      <$bridge as $crate::__private::Encode<$format, $flavor>>::encode(&$to(self), context, buf)
    }

    fn encoded_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      <$bridge as $crate::__private::Encode<$format, $flavor>>::encoded_len(&$to(self), context)
    }

    fn encode_length_delimited(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      <$bridge as $crate::__private::Encode<$format, $flavor>>::encode_length_delimited(&$to(self), context, buf)
    }

    fn encoded_length_delimited_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      <$bridge as $crate::__private::Encode<$format, $flavor>>::encoded_length_delimited_len(&$to(self), context)
    }
  };
  (@encode $flavor:ty: $(
    $bridge:ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Encode<$format, $flavor> for $ty {
          $crate::encode_bridge!(@encode_impl $flavor: $bridge as $format => $to);
        }
      )*
    )*
  };
  (@partial_encode_impl $flavor:ty: $bridge:ty as $format:ty => $to:expr) => {
    fn partial_encode_raw(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut [::core::primitive::u8],
      selector: &Self::Selector,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      <$bridge as $crate::__private::PartialEncode<$format, $flavor>>::partial_encode_raw(&$to(self), context, buf, selector)
    }

    fn partial_encoded_raw_len(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      selector: &Self::Selector,
    ) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialEncode<$format, $flavor>>::partial_encoded_raw_len(&$to(self), context, selector)
    }

    fn partial_encode(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut [::core::primitive::u8],
      selector: &Self::Selector,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      <$bridge as $crate::__private::PartialEncode<$format, $flavor>>::partial_encode(&$to(self), context, buf, selector)
    }

    fn partial_encoded_len(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      selector: &Self::Selector,
    ) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialEncode<$format, $flavor>>::partial_encoded_len(&$to(self), context, selector)
    }

    fn partial_encode_length_delimited(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut [::core::primitive::u8],
      selector: &Self::Selector,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      <$bridge as $crate::__private::PartialEncode<$format, $flavor>>::partial_encode_length_delimited(&$to(self), context, buf, selector)
    }

    fn partial_encoded_length_delimited_len(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      selector: &Self::Selector,
    ) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialEncode<$format, $flavor>>::partial_encoded_length_delimited_len(&$to(self), context, selector)
    }
  };
  (@partial_encode $flavor:ty: $(
    $bridge:ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialEncode<$format, $flavor> for $ty {
          $crate::encode_bridge!(@partial_encode_impl $flavor: $bridge as $format => $to);
        }
      )*
    )*
  };
}

/// A macro emits [`Decode`](super::decode::Decode) implementations for `Self`.
#[macro_export]
macro_rules! decode_bridge {
  ($(
    $flavor:ty: $bridge: ty => $output: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de, RB, B, $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::decode::Decode<
          'de,
          Self,
          $format,
          RB,
          B,
          $flavor,
        > for $ty {
          $crate::decode_bridge!(@decode_impl $flavor: $from => $bridge as $format => $output);
        }
      )*
    )*
  };
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de, RB, B, $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::decode::Decode<
          'de,
          $flavor,
          $format,
          Self,
          B,
          UB
        > for $ty {
          $crate::decode_bridge!(@decode_impl $flavor: $from => $bridge as $format => $bridge);
        }
      )*
    )*
  };
  (@decode_impl $flavor:ty: $from:expr => $bridge:ty as $format:ty => $output:ty) => {
    fn decode(
      context: &'de <$flavor as $crate::__private::flavors::Flavor>::Context,
      src: RB,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      RB: $crate::__private::ReadBuf + 'de,
      B: $crate::__private::Buffer<<$flavor as $crate::__private::flavors::Flavor>::Unknown<RB>> + 'de,
    {
      <$bridge as $crate::__private::decode::Decode<'de, $output, $format, RB, B, $flavor>>::decode(context, src).map(|(n, v)| (n, $from(v)))
    }

    fn decode_length_delimited(
      context: &'de <$flavor as $crate::__private::flavors::Flavor>::Context,
      src: RB,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      RB: $crate::__private::ReadBuf + 'de,
      B: $crate::__private::Buffer<<$flavor as $crate::__private::flavors::Flavor>::Unknown<RB>> + 'de,
    {
      <$bridge as $crate::__private::decode::Decode<'de, $output, $format, RB, B, $flavor>>::decode_length_delimited(context, src).map(|(n, v)| (n, $from(v)))
    }
  };
}

/// A macro emits [`Decode`](super::decode::Decode) implementations for `Self`.
#[macro_export]
macro_rules! try_decode_bridge {
  ($(
    $flavor:ty: $bridge: ty => $output: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de, RB, B> $crate::__private::decode::Decode<'de, Self, $format, RB, B, $flavor> for $ty {
          $crate::try_decode_bridge!(@decode_impl $flavor: $from => $bridge as $format => $output);
        }
      )*
    )*
  };
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de, RB, B> $crate::__private::decode::Decode<'de, Self, $format, RB, B, $flavor> for $ty {
          $crate::try_decode_bridge!(@decode_impl $flavor: $from => $bridge as $format => $bridge);
        }
      )*
    )*
  };
  (@decode_impl $flavor:ty: $from:expr => $bridge:ty as $format:ty => $output:ty) => {
    fn decode(
      context: &'de <$flavor as $crate::__private::flavors::Flavor>::Context,
      src: RB,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      RB: $crate::__private::ReadBuf + 'de,
      B: $crate::__private::Buffer<<$flavor as $crate::__private::flavors::Flavor>::Unknown<RB>> + 'de,
    {
      <$bridge as $crate::__private::decode::Decode<'de, $output, $format, RB, B, $flavor>>::decode(context, src).and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }

    fn decode_length_delimited(
      context: &'de <$flavor as $crate::__private::flavors::Flavor>::Context,
      src: RB,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      RB: $crate::__private::ReadBuf + 'de,
      B: $crate::__private::Buffer<<$flavor as $crate::__private::flavors::Flavor>::Unknown<RB>> + 'de,
    {
      <$bridge as $crate::__private::decode::Decode<$output, $format, RB, B, $flavor>>::decode_length_delimited(context, src).and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }
  };
  (@decode $flavor:ty: $(
    $bridge:ty => $output:ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        try_from: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de, RB, B> $crate::__private::decode::Decode<'de, Self, $format, RB, B, $flavor> for $ty {
          $crate::try_decode_bridge!(@decode_impl $flavor: $from => $bridge as $format => $output);
        }
      )*
    )*
  };
}
