/// A macro emits traits implementations for primitive types that implements [`varing::Varint`](varing::Varint) and [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! network_varint {
  ($(
    $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?
  ),+$(,)?) => {
    $crate::varint!(
      $crate::__private::flavors::Network:$crate::__private::flavors::network::Varint {
        $(
          $ty $([ $(const $g: usize),* ])?
        ),+
      }
    );
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
    $($crate::decoded_state!(@scalar &'a $flavor: $ty $([ $(const $g: usize),* ])? as $wf);)*
    $($crate::flatten_state!($ty $([ $(const $g: usize),* ])?);)*
    $($crate::default_wire_format!($flavor: $ty $([$(const $g: usize),*])? as $wf);)*
    $($crate::partial_encode_scalar!($flavor: $ty $([ $(const $g: usize),* ])? as $wf);)*
    $($crate::varint!(@encode $flavor:$wf:$ty $([ $(const $g: usize),* ])?);)*
    $($crate::varint!(@decode $flavor:$wf:$ty $([ $(const $g: usize),* ])?);)*
  };
  (@without_flatten_state $flavor:ty:$wf:ty {
    $(
      $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?
    ),+$(,)?
  }) => {
    $($crate::selectable!(@scalar $flavor: $ty $([ $(const $g: usize),* ])?);)*
    $($crate::decoded_state!(@scalar &'a $flavor: $ty $([ $(const $g: usize),* ])? as $wf);)*
    $($crate::default_wire_format!($flavor: $ty $([$(const $g: usize),*])? as $wf);)*
    $($crate::partial_encode_scalar!($flavor: $ty $([ $(const $g: usize),* ])? as $wf);)*
    $($crate::varint!(@encode $flavor:$wf:$ty $([ $(const $g: usize),* ])?);)*
    $($crate::varint!(@decode $flavor:$wf:$ty $([ $(const $g: usize),* ])?);)*
  };
  (@encode $flavor:ty:$wf:ty:$ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Encode<$flavor, $wf> for $ty {
      $crate::varint!(@encode_impl $flavor:$wf);
    }
  };
  (@encode_impl $flavor:ty:$wf:ty) => {
    fn encode(&self, _: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      $crate::__private::varing::Varint::encode(self, buf).map_err(::core::convert::Into::into)
    }

    fn encoded_len(&self, _: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      $crate::__private::varing::Varint::encoded_len(self)
    }

    fn encode_length_delimited(&self, ctx: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      <Self as $crate::__private::Encode<$flavor, $wf>>::encode(self, ctx, buf)
    }

    fn encoded_length_delimited_len(&self, ctx: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      <Self as $crate::__private::Encode<$flavor, $wf>>::encoded_len(self, ctx)
    }
  };
  (@decode $flavor:ty:$wf:ty:$ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl<'de, UB, $($(const $g: ::core::primitive::usize),*)?> $crate::__private::Decode<'de, $flavor, $wf, Self, UB> for $ty {
      $crate::varint!(@decode_impl $flavor:$wf);
    }
  };
  (@decode_impl $flavor:ty:$wf:ty) => {
    fn decode<B>(_: &<$flavor as $crate::__private::flavors::Flavor>::Context, src: B) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      B: $crate::__private::Buf<'de>,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::flavors::Flavor>::Unknown<B>> + 'de,
    {
      $crate::__private::varing::Varint::decode(src.as_bytes()).map_err(::core::convert::Into::into)
    }
  };
}

/// A macro emits [`PartialEncode`](super::encode::PartialEncode) implementations for scalar types.
#[macro_export]
macro_rules! partial_encode_scalar {
  (@impl $flavor:ty as $format:ty) => {
    fn partial_encode(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut [::core::primitive::u8], s: &Self::Selector) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      if *s {
        <Self as $crate::__private::Encode<$flavor, $format>>::encode(self, context, buf)
      } else {
        ::core::result::Result::Ok(0)
      }
    }

    fn partial_encoded_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, s: &Self::Selector) -> ::core::primitive::usize {
      if *s {
        <Self as $crate::__private::Encode<$flavor, $format>>::encoded_len(self, context)
      } else {
        0
      }
    }

    fn partial_encode_length_delimited(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut [::core::primitive::u8], s: &Self::Selector) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      if *s {
        <Self as $crate::__private::Encode<$flavor, $format>>::encode_length_delimited(self, context, buf)
      } else {
        ::core::result::Result::Ok(0)
      }
    }

    fn partial_encoded_length_delimited_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, s: &Self::Selector) -> ::core::primitive::usize {
      if *s {
        <Self as $crate::__private::Encode<$flavor, $format>>::encoded_length_delimited_len(self, context)
      } else {
        0
      }
    }
  };
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialEncode<$flavor, $format> for $ty {
        $crate::partial_encode_scalar!(@impl $flavor as $format);
      }
    )*
  };
}

/// A macro emits [`PartialDecode`](super::decode::PartialDecode) implementations for scalar types.
///
/// This macro require the given types already implement [`Decode`](super::decode::Decode) for the same
/// flavor and wire format.
#[macro_export]
macro_rules! partial_decode_scalar {
  (@impl $flavor:ty as $format:ty => $on_empty:expr ) => {
    fn partial_decode<B>(
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: B,
      s: &Self::Selector
    ) -> ::core::result::Result<(::core::primitive::usize, ::core::option::Option<Self>), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      B: $crate::__private::Buf<'de>,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::flavors::Flavor>::Unknown<B>> + 'de,
    {
      if <Self::Selector as $crate::__private::Selector<$flavor>>::is_empty(s) {
        ($on_empty)(context, buf.as_bytes())
      } else {
        <Self as $crate::__private::Decode<'_, $flavor, $format, Self, UB>>::decode(context, buf).map(|(read, val)| (read, ::core::option::Option::Some(val)))
      }
    }
  };
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty => $on_empty:expr ),+$(,)?) => {
    $(
      impl<'de, UB, $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::PartialDecode<'de, $flavor, $format, Self, UB> for $ty {
        $crate::partial_decode_scalar!(@impl $flavor as $format => $on_empty );
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
      impl<__GROST_WIRE_FORMAT__: ?::core::marker::Sized, $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::selection::Selectable<$flavor, __GROST_WIRE_FORMAT__> for $ty {
        type Selector = ::core::primitive::bool;
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
        impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::selection::Selectable<$flavor, $wf> for $ty {
          type Selector = <$bridge as $crate::__private::selection::Selectable<$flavor, $wf>>::Selector;
        }
      )*
    )*
  };
}

/// A macro emits basic [`State<Decoded<'_, Flavor, WireFormat>>`](super::State) implementations for `Self`
#[macro_export]
macro_rules! decoded_state {
  (&$lifetime:lifetime $flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $wf:ty => $target:ty ),+$(,)?) => {
    $(
      impl<$lifetime, $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::State<$crate::__private::convert::Decoded<$lifetime, $flavor, $wf>> for $ty {
        type Input = & $lifetime [u8];
        type Output = $target;
      }
    )*
  };
  (@scalar &$lifetime:lifetime $flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $wf:ty),+$(,)?) => {
    $crate::decoded_state!(& $lifetime $flavor: $($ty $([ $(const $g: usize),* ])? as $wf => $ty),+);
  };
}

/// A macro emits [`impl Reflectable<Self> for Reflection<Self, Type, Flavor>`](super::reflection::Reflectable) implementations for `Self`
#[macro_export]
macro_rules! type_reflection {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? => $expr:expr),+$(,)?) => {
    $(
      impl$(<$(const $g: ::core::primitive::usize),*>)? $crate::__private::reflection::Reflectable<$ty> for $crate::__private::reflection::TypeReflection<$ty> {
        type Reflection = $crate::__private::reflection::Type;

        const REFLECTION: &Self::Reflection = &{
          $expr
        };
      }
    )*
  };
}

/// A macro emits basic [`State<Flatten<_>>`](super::State) implementations for `Self`
#[macro_export]
macro_rules! flatten_state {
  ($($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl<S: ?::core::marker::Sized, $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::State<
        $crate::__private::convert::Flatten<S>
      > for $ty {
        type Input = $ty;
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

      $crate::decode_bridge!($flavor: $bridge {
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

      $crate::try_decode_bridge!($flavor: $bridge {
        $($ty $([ $(const $g: usize),* ])? as $format {
          convert: $from;
        },)*
      });
    )*
  };
  (@without_decode_owned $(
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
        })*
      });

      $crate::try_decode_bridge!(@without_decode_owned $flavor: $bridge {
        $($ty $([ $(const $g: usize),* ])? as $format {
          convert: $from;
        })*
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
    fn encode(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      <$bridge as $crate::__private::Encode<$flavor, $format>>::encode(&$to(self), context, buf)
    }

    fn encoded_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      <$bridge as $crate::__private::Encode<$flavor, $format>>::encoded_len(&$to(self), context)
    }

    fn encode_length_delimited(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      <$bridge as $crate::__private::Encode<$flavor, $format>>::encode_length_delimited(&$to(self), context, buf)
    }

    fn encoded_length_delimited_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      <$bridge as $crate::__private::Encode<$flavor, $format>>::encoded_length_delimited_len(&$to(self), context)
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
        impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Encode<$flavor, $format> for $ty {
          $crate::encode_bridge!(@encode_impl $flavor: $bridge as $format => $to);
        }
      )*
    )*
  };
  (@partial_encode_impl $flavor:ty: $bridge:ty as $format:ty => $to:expr) => {
    fn partial_encode(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut [::core::primitive::u8],
      selector: &Self::Selector,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      <$bridge as $crate::__private::PartialEncode<$flavor, $format>>::partial_encode(&$to(self), context, buf, selector)
    }

    fn partial_encoded_len(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      selector: &Self::Selector,
    ) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialEncode<$flavor, $format>>::partial_encoded_len(&$to(self), context, selector)
    }

    fn partial_encode_length_delimited(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut [::core::primitive::u8],
      selector: &Self::Selector,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error> {
      <$bridge as $crate::__private::PartialEncode<$flavor, $format>>::partial_encode_length_delimited(&$to(self), context, buf, selector)
    }

    fn partial_encoded_length_delimited_len(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      selector: &Self::Selector,
    ) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialEncode<$flavor, $format>>::partial_encoded_length_delimited_len(&$to(self), context, selector)
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
        impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialEncode<$flavor, $format> for $ty {
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
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de, UB, $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::Decode<'de, $flavor, $format, Self, UB> for $ty {
          $crate::decode_bridge!(@decode_impl $flavor: $from => $bridge as $format);
        }
      )*
    )*
  };
  (@decode_impl $flavor:ty: $from:expr => $bridge:ty as $format:ty) => {
    fn decode<B>(
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      B: $crate::__private::Buf<'de>,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::flavors::Flavor>::Unknown<B>> + 'de,
    {
      <$bridge as $crate::__private::Decode<$flavor, $format, $bridge, UB>>::decode::<B>(context, src).map(|(n, v)| (n, $from(v)))
    }

    fn decode_length_delimited<B>(
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      B: $crate::__private::Buf<'de>,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::flavors::Flavor>::Unknown<B>> + 'de,
    {
      <$bridge as $crate::__private::Decode<$flavor, $format, $bridge, UB>>::decode_length_delimited::<B>(context, src).map(|(n, v)| (n, $from(v)))
    }
  };
}

/// A macro emits [`Decode`](super::decode::Decode) implementations for `Self`.
#[macro_export]
macro_rules! try_decode_bridge {
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::try_decode_bridge!(@decode $flavor: $bridge {
          $ty $([ $(const $g: usize),* ])? as $format {
            try_from: $from;
          }
        });
      )*
    )*
  };
  (@without_decode_owned $(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::try_decode_bridge!(@decode $flavor: $bridge {
          $ty $([ $(const $g: usize),* ])? as $format {
            try_from: $from;
          }
        });
      )*
    )*
  };
  (@decode_impl $flavor:ty: $from:expr => $bridge:ty as $format:ty) => {
    fn decode<B>(
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      B: $crate::__private::Buf<'de>,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::flavors::Flavor>::Unknown<B>> + 'de,
    {
      <$bridge as $crate::__private::Decode<$flavor, $format, $bridge, UB>>::decode(context, src).and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }

    fn decode_length_delimited<B>(
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      B: $crate::__private::Buf<'de>,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::flavors::Flavor>::Unknown<B>> + 'de,
    {
      <$bridge as $crate::__private::Decode<$flavor, $format, $bridge, UB>>::decode_length_delimited(context, src).and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }
  };
  (@decode $flavor:ty: $(
    $bridge:ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        try_from: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de, UB> $crate::__private::Decode<'de, $flavor, $format, Self, UB> for $ty {
          $crate::try_decode_bridge!(@decode_impl $flavor: $from => $bridge as $format);
        }
      )*
    )*
  };
}

/// A macro emits [`DecodeOwned`](super::decode::DecodeOwned) implementations for `Self`.
#[macro_export]
macro_rules! try_decode_owned_bridge {
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::try_decode_owned_bridge!(@decode_owned $flavor: $bridge {
          $ty $([ $(const $g: usize),* ])? as $format {
            try_from: $from;
          }
        });
      )*
    )*
  };
  (@decode_owned_impl $flavor:ty: $from:expr => $bridge:ty as $format:ty) => {
    fn decode_owned<D>(
      ctx: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      src: D,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'static,
      D: $crate::__private::BytesBuffer + 'static,
      B: $crate::__private::Buffer<<$flavor as $crate::__private::flavors::Flavor>::Unknown<D>> + 'static,
    {
      <$bridge as $crate::__private::DecodeOwned<$flavor, $format, $bridge, B>>::decode_owned::<D>(ctx, src)
        .and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }

    fn decode_length_delimited_owned<D>(
      ctx: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      src: D,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'static,
      D: $crate::__private::BytesBuffer + 'static,
      B: $crate::__private::Buffer<<$flavor as $crate::__private::flavors::Flavor>::Unknown<D>> + 'static,
    {
      <$bridge as $crate::__private::DecodeOwned<$flavor, $format, $bridge, B>>::decode_length_delimited_owned::<D>(ctx, src)
        .and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }
  };
  (@decode_owned $flavor:ty: $(
    $bridge:ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        try_from: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<B> $crate::__private::DecodeOwned<$flavor, $format, Self, B> for $ty
        where
          $bridge: $crate::__private::DecodeOwned<$flavor, $format, $bridge, B>,
        {
          $crate::try_decode_owned_bridge!(@decode_owned_impl $flavor: $from => $bridge as $format);
        }
      )*
    )*
  };
}

/// A macro emits traits implementations for a zero-sized type.
///
/// ## Example
///
/// ```rust
/// use grost_proto::network_zst;
///
/// #[derive(Default)]
/// struct MyZst;
///
/// #[derive(Default)]
/// #[repr(transparent)]
/// struct MyZstTwo(MyZst);
///
/// network_zst!(MyZst, MyZstTwo);
/// ```
#[macro_export]
macro_rules! network_zst {
  ($($ty:ty), +$(,)?) => {
    $(
      const _: () = {
        const fn __assert<T>() where T: ::core::default::Default {
          assert!(::core::mem::size_of::<T>() == 0, "Not a zero-sized type");
        }

        __assert::<$ty>();
      };

      impl $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Zst> for $ty {
        #[inline]
        fn encode(
          &self,
          _: &$crate::__private::flavors::network::Context,
          _: &mut [::core::primitive::u8]
        ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::Error> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn encode_length_delimited(
          &self,
          _: &$crate::__private::flavors::network::Context,
          _: &mut [::core::primitive::u8]
        ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::Error> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn encoded_len(
          &self,
          _: &$crate::__private::flavors::network::Context,
        ) -> ::core::primitive::usize {
          0
        }

        #[inline]
        fn encoded_length_delimited_len(
          &self,
          _: &$crate::__private::flavors::network::Context,
        ) -> ::core::primitive::usize {
          0
        }
      }

      impl<'de, UB> $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::Zst, Self, UB> for $ty {
        fn decode<B>(
          _: &$crate::__private::flavors::network::Context,
          _: B,
        ) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::Error>
        where
          Self: ::core::marker::Sized + 'de,
          B: $crate::__private::Buf<'de>,
          UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'de,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }

        fn decode_length_delimited<B>(
          ctx: &$crate::__private::flavors::network::Context,
          src: B,
        ) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::Error>
        where
          Self: ::core::marker::Sized + 'de,
          B: $crate::__private::Buf<'de>,
          UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'de,
        {
          <Self as $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::Zst, Self, UB>>::decode(ctx, src)
            .map(|(n, _)| (n, ::core::default::Default::default()))
        }
      }

      $crate::selectable!(@scalar $crate::__private::flavors::Network: $ty);
      $crate::partial_encode_scalar!($crate::__private::flavors::Network: $ty as $crate::__private::flavors::network::Zst);
    )*
  };
}

/// A macro emits traits implementations for `PhantomData<T>` like types.
///
/// ## Example
///
/// ```rust
/// use grost_proto::network_phantom;
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
/// network_phantom!(MyPhantom<T>);
/// ```
#[macro_export]
macro_rules! network_phantom {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<T: ?::core::marker::Sized> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Zst> for $ty {
        #[inline]
        fn encode(
          &self,
          _: &$crate::__private::flavors::network::Context,
          _: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::Error> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn encoded_len(
          &self,
          _: &$crate::__private::flavors::network::Context,
        ) -> ::core::primitive::usize {
          0
        }

        #[inline]
        fn encode_length_delimited(
          &self,
          _: &$crate::__private::flavors::network::Context,
          _: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::Error> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn encoded_length_delimited_len(
          &self,
          _: &$crate::__private::flavors::network::Context,
        ) -> ::core::primitive::usize {
          0
        }
      }

      impl<T: ?::core::marker::Sized, W: ?::core::marker::Sized> $crate::__private::selection::Selectable<$crate::__private::flavors::Network, W> for $ty {
        type Selector = ::core::primitive::bool;
      }

      impl<T: ?::core::marker::Sized> $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Zst> for $ty {
        $crate::partial_encode_scalar!(@impl $crate::__private::flavors::Network as $crate::__private::flavors::network::Zst);
      }

      impl<'de, T: ?::core::marker::Sized, UB> $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::Zst, Self, UB> for $ty {
        fn decode<B>(
          _: &$crate::__private::flavors::network::Context,
          _: B,
        ) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::Error>
        where
          Self: ::core::marker::Sized + 'de,
          B: $crate::__private::Buf<'de>,
          UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'de,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }

        fn decode_length_delimited<B>(
          ctx: &$crate::__private::flavors::network::Context,
          src: B,
        ) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::Error>
        where
          Self: ::core::marker::Sized + 'de,
          B: $crate::__private::Buf<'de>,
          UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'de,
        {
          <Self as $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::Zst, Self, UB>>::decode(ctx, src)
        }
      }
    )*
  };
}
